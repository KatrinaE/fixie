use ahash::AHashMap as HashMap;

/// FIX protocol constants
pub const SOH: char = '\x01'; // Start of header delimiter

/// FIX versions
pub const FIXT_1_1: &str = "FIXT.1.1";
pub const FIX_5_0_SP2: &str = "FIX.5.0SP2";

/// Represents a parsed FIX message as a collection of tag-value pairs
#[derive(Debug, Clone)]
pub struct RawFixMessage {
    pub fields: HashMap<u32, String>,
}

impl RawFixMessage {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn set_field(&mut self, tag: u32, value: String) {
        self.fields.insert(tag, value);
    }

    pub fn get_field(&self, tag: u32) -> Option<&str> {
        self.fields.get(&tag).map(|s| s.as_str())
    }

    pub fn get_field_as<T>(&self, tag: u32) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.get_field(tag)?.parse().ok()
    }

    /// Parse a FIX message from wire format
    /// Format: "8=FIXT.1.1\x019=...\x0135=...\x011128=9\x01..."
    /// Also supports pipe-delimited format for debugging: "8=FIXT.1.1|9=...|35=...|"
    /// Note: Tag 1128 (ApplVerID) indicates FIX 5.0 SP2
    pub fn parse(input: &str) -> Result<Self, FixParseError> {
        let mut fields = HashMap::new();

        // Auto-detect delimiter: SOH (\x01) or pipe (|)
        let delimiter = if input.contains(SOH) {
            SOH
        } else if input.contains('|') {
            '|'
        } else {
            SOH // default
        };

        // Split by delimiter
        for field in input.split(delimiter) {
            if field.is_empty() {
                continue;
            }

            // Each field is "tag=value"
            let parts: Vec<&str> = field.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err(FixParseError::InvalidField(field.to_string()));
            }

            let tag: u32 = parts[0]
                .parse()
                .map_err(|_| FixParseError::InvalidTag(parts[0].to_string()))?;
            let value = parts[1].to_string();

            fields.insert(tag, value);
        }

        // Validate required header fields
        if !fields.contains_key(&8) {
            return Err(FixParseError::MissingRequiredField(8)); // BeginString
        }
        if !fields.contains_key(&35) {
            return Err(FixParseError::MissingRequiredField(35)); // MsgType
        }

        Ok(Self { fields })
    }

    /// Encode to FIX wire format (FIXT 1.1)
    /// This builds the message with proper header/trailer and checksum
    pub fn encode(&self) -> String {
        let mut body = String::new();

	// Pre-allocate message capacity
	let approx_size = self.fields.iter()
            .map(|(tag, val)| 6 + val.len()) // tag + "=" + SOH + value
            .sum::<usize>() + 50; // header/trailer overhead
	let mut message = String::with_capacity(approx_size);

        // Standard Header fields (in order per FIXT 1.1 spec)
        // After BeginString(8) and BodyLength(9), we need:
        // MsgType(35), SenderCompID(49), TargetCompID(56), MsgSeqNum(34), SendingTime(52)
        // Then ApplVerID(1128) for FIX 5.0 messages
        let header_tags = [35, 49, 56, 34, 52, 1128];
        for tag in &header_tags {
            if let Some(value) = self.fields.get(tag) {
                body.push_str(&format!("{}={}{}", tag, value, SOH));
            }
        }

        // Body fields (all other fields except BeginString(8), BodyLength(9), CheckSum(10))
        let mut sorted_tags: Vec<&u32> = self.fields.keys().collect();
        sorted_tags.sort();

        for tag in sorted_tags {
            // Skip standard header/trailer fields already added
            if *tag == 8 || *tag == 9 || *tag == 10 || header_tags.contains(tag) {
                continue;
            }
            if let Some(value) = self.fields.get(tag) {
                body.push_str(&format!("{}={}{}", tag, value, SOH));
            }
        }

        // Calculate body length (bytes in body)
        let body_length = body.len();

        // Build header with FIXT.1.1
        let default_begin_string = FIXT_1_1.to_string();
        let begin_string = self.fields.get(&8).unwrap_or(&default_begin_string);
        message = format!("8={}{}", begin_string, SOH);
        message.push_str(&format!("9={}{}", body_length, SOH));
        message.push_str(&body);

        // Calculate checksum (sum of all bytes mod 256)
        let checksum: u32 = message.bytes().map(|b| b as u32).sum::<u32>() % 256;
        message.push_str(&format!("10={:03}{}", checksum, SOH));

        message
    }
}

impl Default for RawFixMessage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum FixParseError {
    InvalidField(String),
    InvalidTag(String),
    MissingRequiredField(u32),
    InvalidValue { tag: u32, value: String, error: String },
}

impl std::fmt::Display for FixParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixParseError::InvalidField(field) => write!(f, "Invalid FIX field: {}", field),
            FixParseError::InvalidTag(tag) => write!(f, "Invalid FIX tag: {}", tag),
            FixParseError::MissingRequiredField(tag) => {
                write!(f, "Missing required FIX field: {}", tag)
            }
            FixParseError::InvalidValue { tag, value, error } => {
                write!(f, "Invalid value for tag {}: '{}' ({})", tag, value, error)
            }
        }
    }
}

impl std::error::Error for FixParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_fixt_message() {
        let input = format!(
            "8=FIXT.1.1{}9=100{}35=D{}1128=9{}55=EURUSD{}54=1{}",
            SOH, SOH, SOH, SOH, SOH, SOH
        );
        let msg = RawFixMessage::parse(&input).unwrap();

        assert_eq!(msg.get_field(8), Some("FIXT.1.1"));
        assert_eq!(msg.get_field(35), Some("D"));
        assert_eq!(msg.get_field(1128), Some("9")); // ApplVerID=9 (FIX 5.0 SP2)
        assert_eq!(msg.get_field(55), Some("EURUSD"));
        assert_eq!(msg.get_field(54), Some("1"));
    }

    #[test]
    fn test_encode_message() {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "D".to_string());
        msg.set_field(1128, "9".to_string()); // FIX 5.0 SP2
        msg.set_field(55, "EURUSD".to_string());
        msg.set_field(54, "1".to_string());

        let encoded = msg.encode();
        assert!(encoded.starts_with("8=FIXT.1.1"));
        assert!(encoded.contains("35=D"));
        assert!(encoded.contains("1128=9"));
        assert!(encoded.contains("55=EURUSD"));
        assert!(encoded.contains("54=1"));
    }

    #[test]
    fn test_roundtrip() {
        let mut original = RawFixMessage::new();
        original.set_field(8, "FIXT.1.1".to_string());
        original.set_field(35, "D".to_string());
        original.set_field(1128, "9".to_string());
        original.set_field(55, "EURUSD".to_string());

        let encoded = original.encode();
        let parsed = RawFixMessage::parse(&encoded).unwrap();

        assert_eq!(parsed.get_field(8), Some("FIXT.1.1"));
        assert_eq!(parsed.get_field(35), Some("D"));
        assert_eq!(parsed.get_field(1128), Some("9"));
        assert_eq!(parsed.get_field(55), Some("EURUSD"));
    }
}
