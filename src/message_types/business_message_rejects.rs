use crate::types::*;
use crate::parser::{RawFixMessage, FixParseError};
use serde::{Deserialize, Serialize};

/// BusinessMessageReject (MsgType j)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMessageReject {
    pub ref_seq_num: Option<u32>,              // Tag 45: MsgSeqNum of rejected message
    pub ref_msg_type: String,                  // Tag 372: MsgType of rejected message (required)
    pub business_reject_ref_id: Option<String>, // Tag 379: ID of rejected message (e.g., ClOrdID)
    pub business_reject_reason: BusinessRejectReason, // Tag 380: Reason for rejection (required)
    pub text: Option<String>,                  // Tag 58: Free text explanation
    pub encoded_text_len: Option<u32>,         // Tag 354: Encoded text length
    pub encoded_text: Option<Vec<u8>>,         // Tag 355: Encoded text data
}

impl BusinessMessageReject {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "j".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        if let Some(ref_seq_num) = self.ref_seq_num {
            msg.set_field(45, ref_seq_num.to_string());
        }
        msg.set_field(372, self.ref_msg_type.clone());
        if let Some(business_reject_ref_id) = &self.business_reject_ref_id {
            msg.set_field(379, business_reject_ref_id.clone());
        }
        msg.set_field(380, self.business_reject_reason.to_fix().to_string());
        if let Some(text) = &self.text {
            msg.set_field(58, text.clone());
        }
        if let Some(encoded_text_len) = self.encoded_text_len {
            msg.set_field(354, encoded_text_len.to_string());
        }
        if let Some(encoded_text) = &self.encoded_text {
            msg.set_field(355, String::from_utf8_lossy(encoded_text).to_string());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let ref_seq_num = raw.get_field_as::<u32>(45);

        let ref_msg_type = raw.get_field(372)
            .ok_or(FixParseError::MissingRequiredField(372))?
            .to_string();

        let business_reject_ref_id = raw.get_field(379).map(|s| s.to_string());

        let business_reject_reason_char = raw.get_field(380)
            .ok_or(FixParseError::MissingRequiredField(380))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 380,
                value: "".to_string(),
                error: "Empty business reject reason".to_string(),
            })?;
        let business_reject_reason = BusinessRejectReason::from_fix(business_reject_reason_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 380,
                value: business_reject_reason_char.to_string(),
                error: "Invalid business reject reason".to_string(),
            })?;

        let text = raw.get_field(58).map(|s| s.to_string());
        let encoded_text_len = raw.get_field_as::<u32>(354);
        let encoded_text = raw.get_field(355).map(|s| s.as_bytes().to_vec());

        Ok(BusinessMessageReject {
            ref_seq_num,
            ref_msg_type,
            business_reject_ref_id,
            business_reject_reason,
            text,
            encoded_text_len,
            encoded_text,
        })
    }
}
