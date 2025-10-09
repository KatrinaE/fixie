use crate::parser::{RawFixMessage, FixParseError};
use crate::types::{ApplReqType, ApplResponseType, ApplReportType};
use serde::{Deserialize, Serialize};

/// ApplicationMessageRequest (MsgType BW)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMessageRequest {
    pub appl_req_id: String,              // Required - Tag 1346
    pub appl_req_type: ApplReqType,       // Required - Tag 1347
    pub appl_id: Option<String>,          // Tag 1180
    pub appl_begin_seq_num: Option<u64>,  // Tag 1182
    pub appl_end_seq_num: Option<u64>,    // Tag 1183
    // NoApplIDs (1351) group accessed via RawFixMessage.groups.get(&1351)
}

impl ApplicationMessageRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BW".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(1346, self.appl_req_id.clone());
        msg.set_field(1347, self.appl_req_type.to_fix().to_string());
        if let Some(appl_id) = &self.appl_id {
            msg.set_field(1180, appl_id.clone());
        }
        if let Some(seq) = self.appl_begin_seq_num {
            msg.set_field(1182, seq.to_string());
        }
        if let Some(seq) = self.appl_end_seq_num {
            msg.set_field(1183, seq.to_string());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let appl_req_id = raw.get_field(1346)
            .ok_or(FixParseError::MissingRequiredField(1346))?
            .to_string();

        let appl_req_type_char = raw.get_field(1347)
            .ok_or(FixParseError::MissingRequiredField(1347))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 1347,
                value: "".to_string(),
                error: "Empty appl req type".to_string(),
            })?;
        let appl_req_type = ApplReqType::from_fix(appl_req_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 1347,
                value: appl_req_type_char.to_string(),
                error: "Invalid appl req type".to_string(),
            })?;

        let appl_id = raw.get_field(1180).map(|s| s.to_string());
        let appl_begin_seq_num = raw.get_field_as::<u64>(1182);
        let appl_end_seq_num = raw.get_field_as::<u64>(1183);

        Ok(ApplicationMessageRequest {
            appl_req_id,
            appl_req_type,
            appl_id,
            appl_begin_seq_num,
            appl_end_seq_num,
        })
    }
}

/// ApplicationMessageRequestAck (MsgType BX)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMessageRequestAck {
    pub appl_response_id: String,           // Required - Tag 1353
    pub appl_req_id: Option<String>,        // Tag 1346
    pub appl_response_type: Option<ApplResponseType>, // Tag 1348
    pub appl_response_error: Option<u32>,   // Tag 1354
    pub text: Option<String>,               // Tag 58
}

impl ApplicationMessageRequestAck {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BX".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(1353, self.appl_response_id.clone());
        if let Some(appl_req_id) = &self.appl_req_id {
            msg.set_field(1346, appl_req_id.clone());
        }
        if let Some(response_type) = self.appl_response_type {
            msg.set_field(1348, response_type.to_fix().to_string());
        }
        if let Some(error) = self.appl_response_error {
            msg.set_field(1354, error.to_string());
        }
        if let Some(text) = &self.text {
            msg.set_field(58, text.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let appl_response_id = raw.get_field(1353)
            .ok_or(FixParseError::MissingRequiredField(1353))?
            .to_string();

        let appl_req_id = raw.get_field(1346).map(|s| s.to_string());

        let appl_response_type = raw.get_field(1348)
            .and_then(|s| s.chars().next())
            .and_then(ApplResponseType::from_fix);

        let appl_response_error = raw.get_field_as::<u32>(1354);
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(ApplicationMessageRequestAck {
            appl_response_id,
            appl_req_id,
            appl_response_type,
            appl_response_error,
            text,
        })
    }
}

/// ApplicationMessageReport (MsgType BY)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationMessageReport {
    pub appl_report_id: String,             // Required - Tag 1356
    pub appl_report_type: ApplReportType,   // Required - Tag 1426
    pub appl_req_id: Option<String>,        // Tag 1346
    pub appl_begin_seq_num: Option<u64>,    // Tag 1182
    pub appl_end_seq_num: Option<u64>,      // Tag 1183
    pub text: Option<String>,               // Tag 58
    // NoApplIDs (1351) group accessed via RawFixMessage.groups.get(&1351)
}

impl ApplicationMessageReport {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BY".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(1356, self.appl_report_id.clone());
        msg.set_field(1426, self.appl_report_type.to_fix().to_string());
        if let Some(appl_req_id) = &self.appl_req_id {
            msg.set_field(1346, appl_req_id.clone());
        }
        if let Some(seq) = self.appl_begin_seq_num {
            msg.set_field(1182, seq.to_string());
        }
        if let Some(seq) = self.appl_end_seq_num {
            msg.set_field(1183, seq.to_string());
        }
        if let Some(text) = &self.text {
            msg.set_field(58, text.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let appl_report_id = raw.get_field(1356)
            .ok_or(FixParseError::MissingRequiredField(1356))?
            .to_string();

        let appl_report_type_char = raw.get_field(1426)
            .ok_or(FixParseError::MissingRequiredField(1426))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 1426,
                value: "".to_string(),
                error: "Empty appl report type".to_string(),
            })?;
        let appl_report_type = ApplReportType::from_fix(appl_report_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 1426,
                value: appl_report_type_char.to_string(),
                error: "Invalid appl report type".to_string(),
            })?;

        let appl_req_id = raw.get_field(1346).map(|s| s.to_string());
        let appl_begin_seq_num = raw.get_field_as::<u64>(1182);
        let appl_end_seq_num = raw.get_field_as::<u64>(1183);
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(ApplicationMessageReport {
            appl_report_id,
            appl_report_type,
            appl_req_id,
            appl_begin_seq_num,
            appl_end_seq_num,
            text,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_message_request_round_trip() {
        let request = ApplicationMessageRequest {
            appl_req_id: "REQ001".to_string(),
            appl_req_type: ApplReqType::Retransmission,
            appl_id: Some("APP1".to_string()),
            appl_begin_seq_num: Some(100),
            appl_end_seq_num: Some(200),
        };

        let raw = request.to_raw();
        assert_eq!(raw.get_field(35), Some("BW"));
        assert_eq!(raw.get_field(1346), Some("REQ001"));
        assert_eq!(raw.get_field(1347), Some("0"));

        let parsed = ApplicationMessageRequest::from_raw(&raw)
            .expect("Failed to parse");
        assert_eq!(parsed.appl_req_id, "REQ001");
        assert_eq!(parsed.appl_req_type, ApplReqType::Retransmission);
    }

    #[test]
    fn test_application_message_request_ack_round_trip() {
        let ack = ApplicationMessageRequestAck {
            appl_response_id: "RESP001".to_string(),
            appl_req_id: Some("REQ001".to_string()),
            appl_response_type: Some(ApplResponseType::RequestSuccessfullyProcessed),
            appl_response_error: None,
            text: Some("Success".to_string()),
        };

        let raw = ack.to_raw();
        assert_eq!(raw.get_field(35), Some("BX"));
        assert_eq!(raw.get_field(1353), Some("RESP001"));

        let parsed = ApplicationMessageRequestAck::from_raw(&raw)
            .expect("Failed to parse");
        assert_eq!(parsed.appl_response_id, "RESP001");
    }

    #[test]
    fn test_application_message_report_round_trip() {
        let report = ApplicationMessageReport {
            appl_report_id: "REPORT001".to_string(),
            appl_report_type: ApplReportType::LastMessageSent,
            appl_req_id: Some("REQ001".to_string()),
            appl_begin_seq_num: Some(1),
            appl_end_seq_num: Some(500),
            text: Some("End of messages".to_string()),
        };

        let raw = report.to_raw();
        assert_eq!(raw.get_field(35), Some("BY"));
        assert_eq!(raw.get_field(1356), Some("REPORT001"));
        assert_eq!(raw.get_field(1426), Some("1"));

        let parsed = ApplicationMessageReport::from_raw(&raw)
            .expect("Failed to parse");
        assert_eq!(parsed.appl_report_id, "REPORT001");
        assert_eq!(parsed.appl_report_type, ApplReportType::LastMessageSent);
    }
}
