use crate::types::*;
use crate::parser::{RawFixMessage, FixParseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// OrderMassCancelRequest (MsgType q)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMassCancelRequest {
    pub cl_ord_id: String,                           // Tag 11: Unique ID of mass cancel request (required)
    pub mass_cancel_request_type: MassCancelRequestType, // Tag 530: Type of cancellation (required)
    pub transact_time: DateTime<Utc>,                // Tag 60: Time of request (required)
    pub symbol: Option<String>,                      // Tag 55: Symbol (optional)
    pub side: Option<Side>,                          // Tag 54: Side (optional)
    pub trading_session_id: Option<String>,          // Tag 336: Trading session ID (optional)
    pub text: Option<String>,                        // Tag 58: Free text (optional)
}

impl OrderMassCancelRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "q".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(530, self.mass_cancel_request_type.to_fix().to_string());
        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        if let Some(symbol) = &self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(trading_session_id) = &self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(text) = &self.text {
            msg.set_field(58, text.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cl_ord_id = raw.get_field(11)
            .ok_or(FixParseError::MissingRequiredField(11))?
            .to_string();

        let mass_cancel_request_type_char = raw.get_field(530)
            .ok_or(FixParseError::MissingRequiredField(530))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 530,
                value: "".to_string(),
                error: "Empty mass cancel request type".to_string(),
            })?;
        let mass_cancel_request_type = MassCancelRequestType::from_fix(mass_cancel_request_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 530,
                value: mass_cancel_request_type_char.to_string(),
                error: "Invalid mass cancel request type".to_string(),
            })?;

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let symbol = raw.get_field(55).map(|s| s.to_string());

        let side = if let Some(side_str) = raw.get_field(54) {
            let side_char = side_str.chars().next()
                .ok_or(FixParseError::InvalidValue {
                    tag: 54,
                    value: "".to_string(),
                    error: "Empty side".to_string(),
                })?;
            Some(Side::from_fix(side_char)
                .ok_or(FixParseError::InvalidValue {
                    tag: 54,
                    value: side_char.to_string(),
                    error: "Invalid side".to_string(),
                })?)
        } else {
            None
        };

        let trading_session_id = raw.get_field(336).map(|s| s.to_string());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(OrderMassCancelRequest {
            cl_ord_id,
            mass_cancel_request_type,
            transact_time,
            symbol,
            side,
            trading_session_id,
            text,
        })
    }
}

/// OrderMassCancelReport (MsgType r)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderMassCancelReport {
    pub order_id: String,                            // Tag 37: Order ID (required)
    pub mass_cancel_request_type: MassCancelRequestType, // Tag 530: Request type (required)
    pub mass_cancel_response: MassCancelResponse,    // Tag 531: Response (required)
    pub cl_ord_id: Option<String>,                   // Tag 11: Client order ID (optional)
    pub mass_cancel_reject_reason: Option<i32>,      // Tag 532: Reject reason (optional)
    pub total_affected_orders: Option<i32>,          // Tag 533: Total affected orders (optional)
    pub symbol: Option<String>,                      // Tag 55: Symbol (optional)
    pub side: Option<Side>,                          // Tag 54: Side (optional)
    pub trading_session_id: Option<String>,          // Tag 336: Trading session ID (optional)
    pub transact_time: Option<DateTime<Utc>>,        // Tag 60: Transaction time (optional)
    pub text: Option<String>,                        // Tag 58: Free text (optional)
}

impl OrderMassCancelReport {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "r".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(37, self.order_id.clone());
        msg.set_field(530, self.mass_cancel_request_type.to_fix().to_string());
        msg.set_field(531, self.mass_cancel_response.to_fix().to_string());
        if let Some(cl_ord_id) = &self.cl_ord_id {
            msg.set_field(11, cl_ord_id.clone());
        }
        if let Some(reject_reason) = self.mass_cancel_reject_reason {
            msg.set_field(532, reject_reason.to_string());
        }
        if let Some(total) = self.total_affected_orders {
            msg.set_field(533, total.to_string());
        }
        if let Some(symbol) = &self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(trading_session_id) = &self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(transact_time) = &self.transact_time {
            msg.set_field(60, transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
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

        let mass_cancel_request_type_char = raw.get_field(530)
            .ok_or(FixParseError::MissingRequiredField(530))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 530,
                value: "".to_string(),
                error: "Empty mass cancel request type".to_string(),
            })?;
        let mass_cancel_request_type = MassCancelRequestType::from_fix(mass_cancel_request_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 530,
                value: mass_cancel_request_type_char.to_string(),
                error: "Invalid mass cancel request type".to_string(),
            })?;

        let mass_cancel_response_char = raw.get_field(531)
            .ok_or(FixParseError::MissingRequiredField(531))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 531,
                value: "".to_string(),
                error: "Empty mass cancel response".to_string(),
            })?;
        let mass_cancel_response = MassCancelResponse::from_fix(mass_cancel_response_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 531,
                value: mass_cancel_response_char.to_string(),
                error: "Invalid mass cancel response".to_string(),
            })?;

        let cl_ord_id = raw.get_field(11).map(|s| s.to_string());
        let mass_cancel_reject_reason = raw.get_field_as(532);
        let total_affected_orders = raw.get_field_as(533);
        let symbol = raw.get_field(55).map(|s| s.to_string());

        let side = if let Some(side_str) = raw.get_field(54) {
            let side_char = side_str.chars().next()
                .ok_or(FixParseError::InvalidValue {
                    tag: 54,
                    value: "".to_string(),
                    error: "Empty side".to_string(),
                })?;
            Some(Side::from_fix(side_char)
                .ok_or(FixParseError::InvalidValue {
                    tag: 54,
                    value: side_char.to_string(),
                    error: "Invalid side".to_string(),
                })?)
        } else {
            None
        };

        let trading_session_id = raw.get_field(336).map(|s| s.to_string());

        let transact_time = if let Some(transact_time_str) = raw.get_field(60) {
            let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
                .map_err(|e| FixParseError::InvalidValue {
                    tag: 60,
                    value: transact_time_str.to_string(),
                    error: e.to_string(),
                })?;
            Some(DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc))
        } else {
            None
        };

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(OrderMassCancelReport {
            order_id,
            mass_cancel_request_type,
            mass_cancel_response,
            cl_ord_id,
            mass_cancel_reject_reason,
            total_affected_orders,
            symbol,
            side,
            trading_session_id,
            transact_time,
            text,
        })
    }
}
