use crate::parser::{RawFixMessage, FixParseError};
use crate::types::{NetworkRequestType, NetworkStatusResponseType};
use serde::{Deserialize, Serialize};

/// NetworkCounterpartySystemStatusRequest (MsgType BC)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCounterpartySystemStatusRequest {
    pub network_req_id: String,                // Required - Tag 933
    pub network_request_type: NetworkRequestType, // Required - Tag 935
    // NoCompIDs (936) group accessed via RawFixMessage.groups.get(&936)
}

impl NetworkCounterpartySystemStatusRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BC".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(933, self.network_req_id.clone());
        msg.set_field(935, self.network_request_type.to_fix().to_string());
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let network_req_id = raw.get_field(933)
            .ok_or(FixParseError::MissingRequiredField(933))?
            .to_string();

        let network_request_type_char = raw.get_field(935)
            .ok_or(FixParseError::MissingRequiredField(935))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 935,
                value: "".to_string(),
                error: "Empty network request type".to_string(),
            })?;
        let network_request_type = NetworkRequestType::from_fix(network_request_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 935,
                value: network_request_type_char.to_string(),
                error: "Invalid network request type".to_string(),
            })?;

        Ok(NetworkCounterpartySystemStatusRequest {
            network_req_id,
            network_request_type,
        })
    }
}

/// NetworkCounterpartySystemStatusResponse (MsgType BD)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCounterpartySystemStatusResponse {
    pub network_resp_id: String,               // Required - Tag 932
    pub network_request_id: Option<String>,    // Tag 933
    pub network_response_type: NetworkStatusResponseType, // Required - Tag 937
    pub last_network_resp_id: Option<String>,  // Tag 934
    // NoCompIDs (936) group accessed via RawFixMessage.groups.get(&936)
}

impl NetworkCounterpartySystemStatusResponse {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BD".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(932, self.network_resp_id.clone());
        if let Some(network_request_id) = &self.network_request_id {
            msg.set_field(933, network_request_id.clone());
        }
        msg.set_field(937, self.network_response_type.to_fix().to_string());
        if let Some(last_network_resp_id) = &self.last_network_resp_id {
            msg.set_field(934, last_network_resp_id.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let network_resp_id = raw.get_field(932)
            .ok_or(FixParseError::MissingRequiredField(932))?
            .to_string();

        let network_request_id = raw.get_field(933).map(|s| s.to_string());

        let network_response_type_char = raw.get_field(937)
            .ok_or(FixParseError::MissingRequiredField(937))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 937,
                value: "".to_string(),
                error: "Empty network response type".to_string(),
            })?;
        let network_response_type = NetworkStatusResponseType::from_fix(network_response_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 937,
                value: network_response_type_char.to_string(),
                error: "Invalid network response type".to_string(),
            })?;

        let last_network_resp_id = raw.get_field(934).map(|s| s.to_string());

        Ok(NetworkCounterpartySystemStatusResponse {
            network_resp_id,
            network_request_id,
            network_response_type,
            last_network_resp_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_status_request_round_trip() {
        let request = NetworkCounterpartySystemStatusRequest {
            network_req_id: "REQ123".to_string(),
            network_request_type: NetworkRequestType::Snapshot,
        };

        // Convert to raw
        let raw = request.to_raw();

        // Verify fields
        assert_eq!(raw.get_field(35), Some("BC"));
        assert_eq!(raw.get_field(933), Some("REQ123"));
        assert_eq!(raw.get_field(935), Some("1"));

        // Parse back
        let parsed = NetworkCounterpartySystemStatusRequest::from_raw(&raw)
            .expect("Failed to parse NetworkCounterpartySystemStatusRequest");

        // Verify round-trip
        assert_eq!(parsed.network_req_id, "REQ123");
        assert_eq!(parsed.network_request_type, NetworkRequestType::Snapshot);
    }

    #[test]
    fn test_network_status_response_round_trip() {
        let response = NetworkCounterpartySystemStatusResponse {
            network_resp_id: "RESP456".to_string(),
            network_request_id: Some("REQ123".to_string()),
            network_response_type: NetworkStatusResponseType::Full,
            last_network_resp_id: Some("RESP455".to_string()),
        };

        // Convert to raw
        let raw = response.to_raw();

        // Verify fields
        assert_eq!(raw.get_field(35), Some("BD"));
        assert_eq!(raw.get_field(932), Some("RESP456"));
        assert_eq!(raw.get_field(933), Some("REQ123"));
        assert_eq!(raw.get_field(937), Some("1"));
        assert_eq!(raw.get_field(934), Some("RESP455"));

        // Parse back
        let parsed = NetworkCounterpartySystemStatusResponse::from_raw(&raw)
            .expect("Failed to parse NetworkCounterpartySystemStatusResponse");

        // Verify round-trip
        assert_eq!(parsed.network_resp_id, "RESP456");
        assert_eq!(parsed.network_request_id, Some("REQ123".to_string()));
        assert_eq!(parsed.network_response_type, NetworkStatusResponseType::Full);
        assert_eq!(parsed.last_network_resp_id, Some("RESP455".to_string()));
    }
}
