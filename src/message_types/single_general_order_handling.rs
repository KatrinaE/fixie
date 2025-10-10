use crate::types::*;
use crate::parser::{RawFixMessage, FixParseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// NewOrderSingle (MsgType D)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderSingle {
    pub cl_ord_id: String,      // Tag 11: Client order ID
    pub symbol: String,          // Tag 55: Ticker symbol
    pub side: Side,              // Tag 54: 1=Buy, 2=Sell
    pub transact_time: DateTime<Utc>, // Tag 60: Time of order creation
    pub ord_type: OrdType,       // Tag 40: 1=Market, 2=Limit
    pub order_qty: f64,          // Tag 38: Quantity
    pub price: Option<f64>,      // Tag 44: Price (required for limit orders)
}

impl NewOrderSingle {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "D".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(55, self.symbol.clone());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        msg.set_field(40, self.ord_type.to_fix().to_string());
        msg.set_field(38, self.order_qty.to_string());
        if let Some(price) = self.price {
            msg.set_field(44, price.to_string());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cl_ord_id = raw.get_field(11)
            .ok_or(FixParseError::MissingRequiredField(11))?
            .to_string();

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

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let ord_type_char = raw.get_field(40)
            .ok_or(FixParseError::MissingRequiredField(40))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: "".to_string(),
                error: "Empty order type".to_string(),
            })?;
        let ord_type = OrdType::from_fix(ord_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: ord_type_char.to_string(),
                error: "Invalid order type".to_string(),
            })?;

        let order_qty: f64 = raw.get_field_as(38)
            .ok_or(FixParseError::MissingRequiredField(38))?;

        let price = raw.get_field_as(44);

        Ok(NewOrderSingle {
            cl_ord_id,
            symbol,
            side,
            transact_time,
            ord_type,
            order_qty,
            price,
        })
    }
}

/// OrderCancelRequest (MsgType F)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancelRequest {
    pub orig_cl_ord_id: String,  // Tag 41: Original client order ID
    pub cl_ord_id: String,       // Tag 11: New client order ID for this cancel
    pub symbol: String,          // Tag 55: Symbol
    pub side: Side,              // Tag 54: Side
    pub transact_time: DateTime<Utc>, // Tag 60: Time of cancel request
}

impl OrderCancelRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "F".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(41, self.orig_cl_ord_id.clone());
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(55, self.symbol.clone());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let orig_cl_ord_id = raw.get_field(41)
            .ok_or(FixParseError::MissingRequiredField(41))?
            .to_string();

        let cl_ord_id = raw.get_field(11)
            .ok_or(FixParseError::MissingRequiredField(11))?
            .to_string();

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

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        Ok(OrderCancelRequest {
            orig_cl_ord_id,
            cl_ord_id,
            symbol,
            side,
            transact_time,
        })
    }
}

/// OrderCancelReplaceRequest (MsgType G)
/// Used to change the parameters of an existing order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancelReplaceRequest {
    pub orig_cl_ord_id: String,  // Tag 41: Original client order ID being replaced
    pub cl_ord_id: String,       // Tag 11: New client order ID for this replacement
    pub symbol: String,          // Tag 55: Symbol
    pub side: Side,              // Tag 54: Side
    pub transact_time: DateTime<Utc>, // Tag 60: Time of replace request
    pub ord_type: OrdType,       // Tag 40: Order type
    pub order_qty: f64,          // Tag 38: New quantity
    pub price: Option<f64>,      // Tag 44: New price (required for limit orders)
}

impl OrderCancelReplaceRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "G".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(41, self.orig_cl_ord_id.clone());
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(55, self.symbol.clone());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        msg.set_field(40, self.ord_type.to_fix().to_string());
        msg.set_field(38, self.order_qty.to_string());
        if let Some(price) = self.price {
            msg.set_field(44, price.to_string());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let orig_cl_ord_id = raw.get_field(41)
            .ok_or(FixParseError::MissingRequiredField(41))?
            .to_string();

        let cl_ord_id = raw.get_field(11)
            .ok_or(FixParseError::MissingRequiredField(11))?
            .to_string();

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

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let ord_type_char = raw.get_field(40)
            .ok_or(FixParseError::MissingRequiredField(40))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: "".to_string(),
                error: "Empty order type".to_string(),
            })?;
        let ord_type = OrdType::from_fix(ord_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: ord_type_char.to_string(),
                error: "Invalid order type".to_string(),
            })?;

        let order_qty: f64 = raw.get_field_as(38)
            .ok_or(FixParseError::MissingRequiredField(38))?;

        let price = raw.get_field_as(44);

        Ok(OrderCancelReplaceRequest {
            orig_cl_ord_id,
            cl_ord_id,
            symbol,
            side,
            transact_time,
            ord_type,
            order_qty,
            price,
        })
    }
}

/// OrderStatusRequest (MsgType H)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusRequest {
    pub cl_ord_id: String,       // Tag 11: Client order ID (required)
    pub symbol: String,          // Tag 55: Instrument symbol (required)
    pub side: Side,              // Tag 54: Side of order (required)
    pub order_id: Option<String>, // Tag 37: Order ID (optional)
}

impl OrderStatusRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "H".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(55, self.symbol.clone());
        msg.set_field(54, self.side.to_fix().to_string());
        if let Some(order_id) = &self.order_id {
            msg.set_field(37, order_id.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cl_ord_id = raw.get_field(11)
            .ok_or(FixParseError::MissingRequiredField(11))?
            .to_string();

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

        let order_id = raw.get_field(37).map(|s| s.to_string());

        Ok(OrderStatusRequest {
            cl_ord_id,
            symbol,
            side,
            order_id,
        })
    }
}

/// ExecutionReport (MsgType 8)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionReport {
    pub order_id: String,        // Tag 37: Exchange order ID
    pub cl_ord_id: String,       // Tag 11: Client order ID
    pub exec_id: String,         // Tag 17: Unique execution ID
    pub exec_type: ExecType,     // Tag 150: Execution type
    pub ord_status: OrdStatus,   // Tag 39: Order status
    pub symbol: String,          // Tag 55: Symbol
    pub side: Side,              // Tag 54: Side
    pub leaves_qty: f64,         // Tag 151: Quantity open for further execution
    pub cum_qty: f64,            // Tag 14: Total filled quantity
    pub avg_px: f64,             // Tag 6: Average fill price
    pub last_qty: Option<f64>,   // Tag 32: Quantity filled in this execution (for fills)
    pub last_px: Option<f64>,    // Tag 31: Price of this execution (for fills)
    pub transact_time: DateTime<Utc>, // Tag 60: Time of execution
    pub text: Option<String>,    // Tag 58: Free text (for rejections)
}

impl ExecutionReport {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let order_id = raw.get_field(37)
            .ok_or(FixParseError::MissingRequiredField(37))?
            .to_string();

        let cl_ord_id = raw.get_field(11)
            .ok_or(FixParseError::MissingRequiredField(11))?
            .to_string();

        let exec_id = raw.get_field(17)
            .ok_or(FixParseError::MissingRequiredField(17))?
            .to_string();

        let exec_type_char = raw.get_field(150)
            .ok_or(FixParseError::MissingRequiredField(150))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 150,
                value: "".to_string(),
                error: "Empty exec type".to_string(),
            })?;
        let exec_type = ExecType::from_fix(exec_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 150,
                value: exec_type_char.to_string(),
                error: "Invalid exec type".to_string(),
            })?;

        let ord_status_char = raw.get_field(39)
            .ok_or(FixParseError::MissingRequiredField(39))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 39,
                value: "".to_string(),
                error: "Empty order status".to_string(),
            })?;
        let ord_status = OrdStatus::from_fix(ord_status_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 39,
                value: ord_status_char.to_string(),
                error: "Invalid order status".to_string(),
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

        let leaves_qty: f64 = raw.get_field_as(151)
            .ok_or(FixParseError::MissingRequiredField(151))?;

        let cum_qty: f64 = raw.get_field_as(14)
            .ok_or(FixParseError::MissingRequiredField(14))?;

        let avg_px: f64 = raw.get_field_as(6)
            .ok_or(FixParseError::MissingRequiredField(6))?;

        let last_qty: Option<f64> = raw.get_field_as(32);
        let last_px: Option<f64> = raw.get_field_as(31);

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(ExecutionReport {
            order_id,
            cl_ord_id,
            exec_id,
            exec_type,
            ord_status,
            symbol,
            side,
            leaves_qty,
            cum_qty,
            avg_px,
            last_qty,
            last_px,
            transact_time,
            text,
        })
    }
}

/// OrderCancelReject (MsgType 9)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCancelReject {
    pub order_id: String,        // Tag 37: Exchange order ID
    pub cl_ord_id: String,       // Tag 11: Client order ID from cancel request
    pub orig_cl_ord_id: String,  // Tag 41: Original client order ID
    pub ord_status: OrdStatus,   // Tag 39: Current order status
    pub cxl_rej_reason: i32,     // Tag 102: Reason for rejection
    pub text: Option<String>,    // Tag 58: Free text
}

impl OrderCancelReject {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "9".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(37, self.order_id.clone());
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(41, self.orig_cl_ord_id.clone());
        msg.set_field(39, self.ord_status.to_fix().to_string());
        msg.set_field(102, self.cxl_rej_reason.to_string());
        if let Some(text) = &self.text {
            msg.set_field(58, text.clone());
        }
        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let order_id = raw.get_field(37)
            .ok_or(FixParseError::MissingRequiredField(37))?
            .to_string();

        let cl_ord_id = raw.get_field(11)
            .ok_or(FixParseError::MissingRequiredField(11))?
            .to_string();

        let orig_cl_ord_id = raw.get_field(41)
            .ok_or(FixParseError::MissingRequiredField(41))?
            .to_string();

        let ord_status_char = raw.get_field(39)
            .ok_or(FixParseError::MissingRequiredField(39))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 39,
                value: "".to_string(),
                error: "Empty order status".to_string(),
            })?;
        let ord_status = OrdStatus::from_fix(ord_status_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 39,
                value: ord_status_char.to_string(),
                error: "Invalid order status".to_string(),
            })?;

        let cxl_rej_reason: i32 = raw.get_field_as(102)
            .ok_or(FixParseError::MissingRequiredField(102))?;

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(OrderCancelReject {
            order_id,
            cl_ord_id,
            orig_cl_ord_id,
            ord_status,
            cxl_rej_reason,
            text,
        })
    }
}
