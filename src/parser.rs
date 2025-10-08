use ahash::AHashMap as HashMap;

/// FIX protocol constants
pub const SOH: char = '\x01'; // Start of header delimiter

/// FIX versions
pub const FIXT_1_1: &str = "FIXT.1.1";
pub const FIX_5_0_SP2: &str = "FIX.5.0SP2";

/// ID type for referencing group entries in the arena
pub type GroupEntryId = usize;

/// A single entry in a repeating group, which can contain nested groups
#[derive(Debug, Clone)]
pub struct GroupEntry {
    /// Fields belonging to this group entry
    pub fields: HashMap<u32, String>,
    /// Nested repeating groups: NoXXX tag -> Vec of child entry IDs
    pub nested_groups: HashMap<u32, Vec<GroupEntryId>>,
}

impl GroupEntry {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            nested_groups: HashMap::new(),
        }
    }
}

/// Represents a parsed FIX message as a collection of tag-value pairs
#[derive(Debug, Clone)]
pub struct RawFixMessage {
    pub fields: HashMap<u32, String>,
    /// Top-level repeating groups: NoXXX tag -> Vec of entry IDs
    pub groups: HashMap<u32, Vec<GroupEntryId>>,
    /// Arena storing all group entries (flat storage for cache efficiency)
    pub group_arena: Vec<GroupEntry>,
}

impl RawFixMessage {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            groups: HashMap::new(),
            group_arena: Vec::new(),
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

    /// Get the entry IDs for a top-level repeating group
    pub fn get_group(&self, num_in_group_tag: u32) -> Option<&Vec<GroupEntryId>> {
        self.groups.get(&num_in_group_tag)
    }

    /// Get a group entry from the arena by ID
    pub fn get_group_entry(&self, entry_id: GroupEntryId) -> Option<&GroupEntry> {
        self.group_arena.get(entry_id)
    }

    /// Get a mutable group entry from the arena by ID
    pub fn get_group_entry_mut(&mut self, entry_id: GroupEntryId) -> Option<&mut GroupEntry> {
        self.group_arena.get_mut(entry_id)
    }

    /// Set a top-level repeating group
    pub fn set_group(&mut self, num_in_group_tag: u32, entry_ids: Vec<GroupEntryId>) {
        self.groups.insert(num_in_group_tag, entry_ids);
    }

    /// Parse a FIX message from wire format
    /// Format: "8=FIXT.1.1\x019=...\x0135=...\x011128=9\x01..."
    /// Also supports pipe-delimited format for debugging: "8=FIXT.1.1|9=...|35=...|"
    /// Note: Tag 1128 (ApplVerID) indicates FIX 5.0 SP2
    ///
    /// Supports repeating groups: when a NoXXX tag is encountered, subsequent fields
    /// are grouped by the delimiter tag until the expected count is reached.
    pub fn parse(input: &str) -> Result<Self, FixParseError> {
        use crate::groups::{is_num_in_group_tag, get_delimiter_tag, get_member_tags};

        let mut fields = HashMap::new();
        let mut groups = HashMap::new();
        let mut group_arena = Vec::new();

        // Auto-detect delimiter: SOH (\x01) or pipe (|)
        let delimiter = if input.contains(SOH) {
            SOH
        } else if input.contains('|') {
            '|'
        } else {
            SOH // default
        };

        // Parse all tag=value pairs into a vector first
        let mut tag_value_pairs: Vec<(u32, String)> = Vec::new();
        let mut msg_type: Option<String> = None;

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

            // Capture message type (tag 35) for context-specific group parsing
            if tag == 35 {
                msg_type = Some(value.clone());
            }

            tag_value_pairs.push((tag, value));
        }

        // Get message type as &str for group lookups
        let msg_type_str = msg_type.as_deref();

        // Process pairs, detecting and handling repeating groups
        let mut i = 0;
        while i < tag_value_pairs.len() {
            let (tag, value) = &tag_value_pairs[i];

            // Check if this is a NoXXX (num-in-group) tag
            if is_num_in_group_tag(*tag, msg_type_str) {
                let num_in_group_tag = *tag;
                let count: usize = value.parse()
                    .map_err(|_| FixParseError::InvalidValue {
                        tag: *tag,
                        value: value.clone(),
                        error: "Invalid group count".to_string(),
                    })?;

                // Store the NoXXX field itself
                fields.insert(*tag, value.clone());

                if let Some(delimiter_tag) = get_delimiter_tag(num_in_group_tag, msg_type_str) {
                    if let Some(member_tags) = get_member_tags(num_in_group_tag, msg_type_str) {
                        // Parse the repeating group entries (flat, no nesting yet)
                        let mut group_entry_ids = Vec::new();
                        i += 1; // Move past the NoXXX tag

                        for _ in 0..count {
                            let mut entry_fields = HashMap::new();

                            // Collect tags for this group entry until we hit the delimiter again or run out
                            while i < tag_value_pairs.len() {
                                let (entry_tag, entry_value) = &tag_value_pairs[i];

                                // Check if this tag belongs to the group
                                if member_tags.contains(entry_tag) {
                                    entry_fields.insert(*entry_tag, entry_value.clone());
                                    i += 1;

                                    // If we've collected all tags for this entry or hit the delimiter for next entry
                                    if i < tag_value_pairs.len() && tag_value_pairs[i].0 == delimiter_tag && !entry_fields.is_empty() {
                                        break;
                                    }
                                } else {
                                    // Tag doesn't belong to this group, we're done with this entry
                                    break;
                                }
                            }

                            if !entry_fields.is_empty() {
                                // Create GroupEntry and add to arena
                                let entry_id = group_arena.len();
                                group_arena.push(GroupEntry {
                                    fields: entry_fields,
                                    nested_groups: HashMap::new(), // No nesting in Goal 2
                                });
                                group_entry_ids.push(entry_id);
                            }
                        }

                        groups.insert(num_in_group_tag, group_entry_ids);
                        continue; // Don't increment i, we already moved it
                    }
                }
            } else {
                // Regular field, not part of a group
                fields.insert(*tag, value.clone());
            }

            i += 1;
        }

        // Validate required header fields
        if !fields.contains_key(&8) {
            return Err(FixParseError::MissingRequiredField(8)); // BeginString
        }
        if !fields.contains_key(&35) {
            return Err(FixParseError::MissingRequiredField(35)); // MsgType
        }

        Ok(Self {
            fields,
            groups,
            group_arena,
        })
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
        // Also handle repeating groups
        use crate::groups::{is_num_in_group_tag, get_member_tags};

        // Get message type for context-specific group encoding
        let msg_type_str = self.fields.get(&35).map(|s| s.as_str());

        let mut sorted_tags: Vec<&u32> = self.fields.keys().collect();
        sorted_tags.sort();

        for tag in sorted_tags {
            // Skip standard header/trailer fields already added
            if *tag == 8 || *tag == 9 || *tag == 10 || header_tags.contains(tag) {
                continue;
            }

            // Check if this is a NoXXX tag with repeating group data
            if is_num_in_group_tag(*tag, msg_type_str) {
                // Output the NoXXX tag first
                if let Some(value) = self.fields.get(tag) {
                    body.push_str(&format!("{}={}{}", tag, value, SOH));
                }

                // Then output the group entries
                if let Some(group_entry_ids) = self.groups.get(tag) {
                    if let Some(member_tags) = get_member_tags(*tag, msg_type_str) {
                        for &entry_id in group_entry_ids {
                            if let Some(entry) = self.group_arena.get(entry_id) {
                                // Output tags in the order defined in member_tags
                                for member_tag in member_tags {
                                    if let Some(value) = entry.fields.get(member_tag) {
                                        body.push_str(&format!("{}={}{}", member_tag, value, SOH));
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                // Regular field
                if let Some(value) = self.fields.get(tag) {
                    body.push_str(&format!("{}={}{}", tag, value, SOH));
                }
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

    #[test]
    fn test_parse_repeating_group() {
        // Message with Party repeating group: 453=2 (NoPartyIDs)
        let input = format!(
            "8=FIXT.1.1{}35=D{}453=2{}448=TRADER1{}447=D{}452=1{}448=DESK22{}447=D{}452=24{}55=MSFT{}",
            SOH, SOH, SOH, SOH, SOH, SOH, SOH, SOH, SOH, SOH
        );
        let msg = RawFixMessage::parse(&input).unwrap();

        // Check regular fields
        assert_eq!(msg.get_field(35), Some("D"));
        assert_eq!(msg.get_field(55), Some("MSFT"));
        assert_eq!(msg.get_field(453), Some("2")); // NoPartyIDs

        // Check repeating group
        let parties = msg.get_group(453).expect("Should have parties group");
        assert_eq!(parties.len(), 2);

        // First party
        let party1 = msg.get_group_entry(parties[0]).unwrap();
        assert_eq!(party1.fields.get(&448), Some(&"TRADER1".to_string()));
        assert_eq!(party1.fields.get(&447), Some(&"D".to_string()));
        assert_eq!(party1.fields.get(&452), Some(&"1".to_string()));

        // Second party
        let party2 = msg.get_group_entry(parties[1]).unwrap();
        assert_eq!(party2.fields.get(&448), Some(&"DESK22".to_string()));
        assert_eq!(party2.fields.get(&447), Some(&"D".to_string()));
        assert_eq!(party2.fields.get(&452), Some(&"24".to_string()));
    }

    #[test]
    fn test_encode_repeating_group() {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "D".to_string());
        msg.set_field(55, "MSFT".to_string());
        msg.set_field(453, "2".to_string()); // NoPartyIDs

        // Create party group entries
        let mut party1_fields = HashMap::new();
        party1_fields.insert(448, "TRADER1".to_string());
        party1_fields.insert(447, "D".to_string());
        party1_fields.insert(452, "1".to_string());

        let mut party2_fields = HashMap::new();
        party2_fields.insert(448, "DESK22".to_string());
        party2_fields.insert(447, "D".to_string());
        party2_fields.insert(452, "24".to_string());

        // Add to arena
        let party1_id = msg.group_arena.len();
        msg.group_arena.push(GroupEntry {
            fields: party1_fields,
            nested_groups: HashMap::new(),
        });

        let party2_id = msg.group_arena.len();
        msg.group_arena.push(GroupEntry {
            fields: party2_fields,
            nested_groups: HashMap::new(),
        });

        msg.set_group(453, vec![party1_id, party2_id]);

        let encoded = msg.encode();

        // Verify the encoded message contains all the fields
        assert!(encoded.contains("453=2"));
        assert!(encoded.contains("448=TRADER1"));
        assert!(encoded.contains("447=D"));
        assert!(encoded.contains("452=1"));
        assert!(encoded.contains("448=DESK22"));
        assert!(encoded.contains("452=24"));
    }

    #[test]
    fn test_repeating_group_roundtrip() {
        let mut original = RawFixMessage::new();
        original.set_field(8, "FIXT.1.1".to_string());
        original.set_field(35, "D".to_string());
        original.set_field(55, "MSFT".to_string());
        original.set_field(453, "2".to_string());

        let mut party1_fields = HashMap::new();
        party1_fields.insert(448, "TRADER1".to_string());
        party1_fields.insert(447, "D".to_string());
        party1_fields.insert(452, "1".to_string());

        let mut party2_fields = HashMap::new();
        party2_fields.insert(448, "DESK22".to_string());
        party2_fields.insert(447, "D".to_string());
        party2_fields.insert(452, "24".to_string());

        // Add to arena
        let party1_id = original.group_arena.len();
        original.group_arena.push(GroupEntry {
            fields: party1_fields,
            nested_groups: HashMap::new(),
        });

        let party2_id = original.group_arena.len();
        original.group_arena.push(GroupEntry {
            fields: party2_fields,
            nested_groups: HashMap::new(),
        });

        original.set_group(453, vec![party1_id, party2_id]);

        let encoded = original.encode();
        let parsed = RawFixMessage::parse(&encoded).unwrap();

        // Check fields
        assert_eq!(parsed.get_field(35), Some("D"));
        assert_eq!(parsed.get_field(55), Some("MSFT"));
        assert_eq!(parsed.get_field(453), Some("2"));

        // Check groups
        let parties = parsed.get_group(453).unwrap();
        assert_eq!(parties.len(), 2);

        let party1 = parsed.get_group_entry(parties[0]).unwrap();
        assert_eq!(party1.fields.get(&448), Some(&"TRADER1".to_string()));

        let party2 = parsed.get_group_entry(parties[1]).unwrap();
        assert_eq!(party2.fields.get(&448), Some(&"DESK22".to_string()));
    }
}
