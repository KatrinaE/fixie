use crate::types::*;
use crate::parser::{RawFixMessage, FixParseError};
use serde::{Deserialize, Serialize};

/// DontKnowTrade (MsgType Q)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DontKnowTrade {
    pub order_id: String,        // Tag 37: Broker order ID
    pub exec_id: String,         // Tag 17: Execution ID
    pub dk_reason: DKReason,     // Tag 127: Reason for DK
    pub symbol: String,          // Tag 55: Symbol
    pub side: Side,              // Tag 54: Side
    pub order_qty: f64,          // Tag 38: Order quantity
    pub last_qty: Option<f64>,   // Tag 32: Last quantity (if specified on ExecutionRpt)
    pub last_px: Option<f64>,    // Tag 31: Last price (if specified on ExecutionRpt)
    pub text: Option<String>,    // Tag 58: Free format text
}

impl DontKnowTrade {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "Q".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(37, self.order_id.clone());
        msg.set_field(17, self.exec_id.clone());
        msg.set_field(127, self.dk_reason.to_fix().to_string());
        msg.set_field(55, self.symbol.clone());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(38, self.order_qty.to_string());
        if let Some(last_qty) = self.last_qty {
            msg.set_field(32, last_qty.to_string());
        }
        if let Some(last_px) = self.last_px {
            msg.set_field(31, last_px.to_string());
        }
        if let Some(text) = &self.text {
            msg.set_field(58, text.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let order_id = raw.get_field(37)
            .ok_or(FixParseError::MissingRequiredField(37))?
            .to_string();

        let exec_id = raw.get_field(17)
            .ok_or(FixParseError::MissingRequiredField(17))?
            .to_string();

        let dk_reason_char = raw.get_field(127)
            .ok_or(FixParseError::MissingRequiredField(127))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 127,
                value: "".to_string(),
                error: "Empty DK reason".to_string(),
            })?;
        let dk_reason = DKReason::from_fix(dk_reason_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 127,
                value: dk_reason_char.to_string(),
                error: "Invalid DK reason".to_string(),
            })?;

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let side_char = raw.get_field(54)
            .ok_or(FixParseError::MissingRequiredField(54))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 54,
                value: "".to_string(),
                error: "Empty side".to_string(),
            })?;
        let side = Side::from_fix(side_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 54,
                value: side_char.to_string(),
                error: "Invalid side".to_string(),
            })?;

        let order_qty: f64 = raw.get_field_as(38)
            .ok_or(FixParseError::MissingRequiredField(38))?;

        let last_qty = raw.get_field_as(32);
        let last_px = raw.get_field_as(31);
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(DontKnowTrade {
            order_id,
            exec_id,
            dk_reason,
            symbol,
            side,
            order_qty,
            last_qty,
            last_px,
            text,
        })
    }
}
