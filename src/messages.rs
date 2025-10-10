use crate::types::*;
use crate::parser::{RawFixMessage, FixParseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// FIX Message Types (MsgType field 35)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgType {
    Logon,                    // A
    Logout,                   // 5
    Heartbeat,                // 0
    TestRequest,              // 1
    ResendRequest,            // 2
    Reject,                   // 3
    NewOrderSingle,           // D
    ExecutionReport,          // 8
    OrderCancelReject,        // 9
    OrderCancelRequest,       // F
    OrderCancelReplaceRequest, // G
    OrderStatusRequest,       // H
    DontKnowTrade,            // Q
    ExecutionAcknowledgement, // BN
    OrderMassCancelRequest,   // q (lowercase)
    OrderMassCancelReport,    // r (lowercase)
    NewOrderCross,            // s (lowercase)
    CrossOrderCancelRequest,  // u (lowercase)
    CrossOrderCancelReplaceRequest, // t (lowercase)
    MarketDataRequest,        // V
    MarketDataSnapshotFullRefresh, // W
    MarketDataRequestReject,  // Y
    // Program Trading / List Trading
    NewOrderList,             // E
    ListStatus,               // N
    ListExecute,              // L
    ListCancelRequest,        // K
    ListStatusRequest,        // M
    BidRequest,               // k (lowercase)
    BidResponse,              // l (lowercase)
    ListStrikePrice,          // m (lowercase)
    // Mass Order Messages
    MassOrder,                // DJ
    MassOrderAck,             // DK
    OrderMassActionRequest,   // CA
    OrderMassActionReport,    // BZ
    OrderMassStatusRequest,   // AF
    // Multileg Order Messages
    NewOrderMultileg,         // AB
    MultilegOrderCancelReplace, // AC
    // Infrastructure Messages
    BusinessMessageReject,    // j
    NetworkCounterpartySystemStatusRequest,  // BC
    NetworkCounterpartySystemStatusResponse, // BD
    ApplicationMessageRequest,    // BW
    ApplicationMessageRequestAck, // BX
    ApplicationMessageReport,     // BY
    UserRequest,                  // BE
    UserResponse,                 // BF
    UserNotification,             // CB
}

impl MsgType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            MsgType::Logon => "A",
            MsgType::Logout => "5",
            MsgType::Heartbeat => "0",
            MsgType::TestRequest => "1",
            MsgType::ResendRequest => "2",
            MsgType::Reject => "3",
            MsgType::NewOrderSingle => "D",
            MsgType::ExecutionReport => "8",
            MsgType::OrderCancelReject => "9",
            MsgType::OrderCancelRequest => "F",
            MsgType::OrderCancelReplaceRequest => "G",
            MsgType::OrderStatusRequest => "H",
            MsgType::DontKnowTrade => "Q",
            MsgType::ExecutionAcknowledgement => "BN",
            MsgType::OrderMassCancelRequest => "q",
            MsgType::OrderMassCancelReport => "r",
            MsgType::NewOrderCross => "s",
            MsgType::CrossOrderCancelRequest => "u",
            MsgType::CrossOrderCancelReplaceRequest => "t",
            MsgType::MarketDataRequest => "V",
            MsgType::MarketDataSnapshotFullRefresh => "W",
            MsgType::MarketDataRequestReject => "Y",
            MsgType::NewOrderList => "E",
            MsgType::ListStatus => "N",
            MsgType::ListExecute => "L",
            MsgType::ListCancelRequest => "K",
            MsgType::ListStatusRequest => "M",
            MsgType::BidRequest => "k",
            MsgType::BidResponse => "l",
            MsgType::ListStrikePrice => "m",
            MsgType::MassOrder => "DJ",
            MsgType::MassOrderAck => "DK",
            MsgType::OrderMassActionRequest => "CA",
            MsgType::OrderMassActionReport => "BZ",
            MsgType::OrderMassStatusRequest => "AF",
            MsgType::NewOrderMultileg => "AB",
            MsgType::MultilegOrderCancelReplace => "AC",
            MsgType::BusinessMessageReject => "j",
            MsgType::NetworkCounterpartySystemStatusRequest => "BC",
            MsgType::NetworkCounterpartySystemStatusResponse => "BD",
            MsgType::ApplicationMessageRequest => "BW",
            MsgType::ApplicationMessageRequestAck => "BX",
            MsgType::ApplicationMessageReport => "BY",
            MsgType::UserRequest => "BE",
            MsgType::UserResponse => "BF",
            MsgType::UserNotification => "CB",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "A" => Some(MsgType::Logon),
            "5" => Some(MsgType::Logout),
            "0" => Some(MsgType::Heartbeat),
            "1" => Some(MsgType::TestRequest),
            "2" => Some(MsgType::ResendRequest),
            "3" => Some(MsgType::Reject),
            "D" => Some(MsgType::NewOrderSingle),
            "8" => Some(MsgType::ExecutionReport),
            "9" => Some(MsgType::OrderCancelReject),
            "F" => Some(MsgType::OrderCancelRequest),
            "G" => Some(MsgType::OrderCancelReplaceRequest),
            "H" => Some(MsgType::OrderStatusRequest),
            "Q" => Some(MsgType::DontKnowTrade),
            "BN" => Some(MsgType::ExecutionAcknowledgement),
            "q" => Some(MsgType::OrderMassCancelRequest),
            "r" => Some(MsgType::OrderMassCancelReport),
            "s" => Some(MsgType::NewOrderCross),
            "u" => Some(MsgType::CrossOrderCancelRequest),
            "t" => Some(MsgType::CrossOrderCancelReplaceRequest),
            "V" => Some(MsgType::MarketDataRequest),
            "W" => Some(MsgType::MarketDataSnapshotFullRefresh),
            "Y" => Some(MsgType::MarketDataRequestReject),
            "E" => Some(MsgType::NewOrderList),
            "N" => Some(MsgType::ListStatus),
            "L" => Some(MsgType::ListExecute),
            "K" => Some(MsgType::ListCancelRequest),
            "M" => Some(MsgType::ListStatusRequest),
            "k" => Some(MsgType::BidRequest),
            "l" => Some(MsgType::BidResponse),
            "m" => Some(MsgType::ListStrikePrice),
            "DJ" => Some(MsgType::MassOrder),
            "DK" => Some(MsgType::MassOrderAck),
            "CA" => Some(MsgType::OrderMassActionRequest),
            "BZ" => Some(MsgType::OrderMassActionReport),
            "AF" => Some(MsgType::OrderMassStatusRequest),
            "AB" => Some(MsgType::NewOrderMultileg),
            "AC" => Some(MsgType::MultilegOrderCancelReplace),
            "j" => Some(MsgType::BusinessMessageReject),
            "BC" => Some(MsgType::NetworkCounterpartySystemStatusRequest),
            "BD" => Some(MsgType::NetworkCounterpartySystemStatusResponse),
            "BW" => Some(MsgType::ApplicationMessageRequest),
            "BX" => Some(MsgType::ApplicationMessageRequestAck),
            "BY" => Some(MsgType::ApplicationMessageReport),
            "BE" => Some(MsgType::UserRequest),
            "BF" => Some(MsgType::UserResponse),
            "CB" => Some(MsgType::UserNotification),
            _ => None,
        }
    }
}

/// FIX Session Messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logon {
    pub encrypt_method: i32,  // Tag 98: 0 = None
    pub heart_bt_int: i32,    // Tag 108: Heartbeat interval in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logout {
    pub text: Option<String>, // Tag 58: Optional logout reason
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub test_req_id: Option<String>, // Tag 112: Required if responding to TestRequest
}

/// FIX Application Messages - Orders

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

/// ExecutionAcknowledgement (MsgType BN)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionAcknowledgement {
    pub order_id: String,            // Tag 37: Order ID (required)
    pub exec_id: String,             // Tag 17: Execution ID (required)
    pub exec_ack_status: ExecAckStatus, // Tag 1036: Status (required)
    pub symbol: String,              // Tag 55: Symbol (required)
    pub side: Side,                  // Tag 54: Side (required)
    pub order_qty: f64,              // Tag 38: Order quantity (required)
    pub cl_ord_id: Option<String>,   // Tag 11: Client order ID (conditionally required)
    pub dk_reason: Option<DKReason>, // Tag 127: DK reason (required when ExecAckStatus=DontKnow)
    pub last_qty: Option<f64>,       // Tag 32: Last quantity
    pub last_px: Option<f64>,        // Tag 31: Last price
    pub cum_qty: Option<f64>,        // Tag 14: Cumulative quantity
    pub avg_px: Option<f64>,         // Tag 6: Average price
    pub text: Option<String>,        // Tag 58: Free format text
}

impl ExecutionAcknowledgement {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BN".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(37, self.order_id.clone());
        msg.set_field(17, self.exec_id.clone());
        msg.set_field(1036, self.exec_ack_status.to_fix().to_string());
        msg.set_field(55, self.symbol.clone());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(38, self.order_qty.to_string());
        if let Some(cl_ord_id) = &self.cl_ord_id {
            msg.set_field(11, cl_ord_id.clone());
        }
        if let Some(dk_reason) = self.dk_reason {
            msg.set_field(127, dk_reason.to_fix().to_string());
        }
        if let Some(last_qty) = self.last_qty {
            msg.set_field(32, last_qty.to_string());
        }
        if let Some(last_px) = self.last_px {
            msg.set_field(31, last_px.to_string());
        }
        if let Some(cum_qty) = self.cum_qty {
            msg.set_field(14, cum_qty.to_string());
        }
        if let Some(avg_px) = self.avg_px {
            msg.set_field(6, avg_px.to_string());
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

        let exec_ack_status_char = raw.get_field(1036)
            .ok_or(FixParseError::MissingRequiredField(1036))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 1036,
                value: "".to_string(),
                error: "Empty exec ack status".to_string(),
            })?;
        let exec_ack_status = ExecAckStatus::from_fix(exec_ack_status_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 1036,
                value: exec_ack_status_char.to_string(),
                error: "Invalid exec ack status".to_string(),
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

        let cl_ord_id = raw.get_field(11).map(|s| s.to_string());

        let dk_reason = if let Some(dk_char_str) = raw.get_field(127) {
            let dk_char = dk_char_str.chars().next()
                .ok_or(FixParseError::InvalidValue {
                    tag: 127,
                    value: "".to_string(),
                    error: "Empty DK reason".to_string(),
                })?;
            Some(DKReason::from_fix(dk_char)
                .ok_or(FixParseError::InvalidValue {
                    tag: 127,
                    value: dk_char.to_string(),
                    error: "Invalid DK reason".to_string(),
                })?)
        } else {
            None
        };

        let last_qty = raw.get_field_as(32);
        let last_px = raw.get_field_as(31);
        let cum_qty = raw.get_field_as(14);
        let avg_px = raw.get_field_as(6);
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(ExecutionAcknowledgement {
            order_id,
            exec_id,
            exec_ack_status,
            symbol,
            side,
            order_qty,
            cl_ord_id,
            dk_reason,
            last_qty,
            last_px,
            cum_qty,
            avg_px,
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

/// Order Mass Handling Messages

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

/// Cross Order Handling Messages

/// NewOrderCross (MsgType s) - Submit a cross order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOrderCross {
    pub cross_id: String,                              // Tag 548
    pub cross_type: CrossType,                         // Tag 549
    pub cross_prioritization: CrossPrioritization,     // Tag 550
    pub symbol: String,                                // Tag 55
    pub ord_type: OrdType,                             // Tag 40
    pub transact_time: DateTime<Utc>,                  // Tag 60
    // Note: sides would be in the NoSides(552) repeating group
    // For now we store them in the RawFixMessage groups
    pub price: Option<f64>,                            // Tag 44
    pub text: Option<String>,                          // Tag 58
}

impl NewOrderCross {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        raw.set_field(8, "FIXT.1.1".to_string());
        raw.set_field(35, "s".to_string());
        raw.set_field(1128, "9".to_string()); // FIX 5.0 SP2

        raw.set_field(548, self.cross_id.clone());
        raw.set_field(549, self.cross_type.to_fix().to_string());
        raw.set_field(550, self.cross_prioritization.to_fix().to_string());
        raw.set_field(55, self.symbol.clone());
        raw.set_field(40, self.ord_type.to_fix().to_string());

        let transact_time_str = self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string();
        raw.set_field(60, transact_time_str);

        if let Some(price) = self.price {
            raw.set_field(44, price.to_string());
        }

        if let Some(ref text) = self.text {
            raw.set_field(58, text.clone());
        }

        raw
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cross_id = raw.get_field(548)
            .ok_or(FixParseError::MissingRequiredField(548))?
            .to_string();

        let cross_type_char = raw.get_field(549)
            .ok_or(FixParseError::MissingRequiredField(549))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: String::new(),
                error: "Empty cross type".to_string(),
            })?;
        let cross_type = CrossType::from_fix(cross_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: cross_type_char.to_string(),
                error: "Invalid cross type".to_string(),
            })?;

        let cross_prioritization_char = raw.get_field(550)
            .ok_or(FixParseError::MissingRequiredField(550))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: String::new(),
                error: "Empty cross prioritization".to_string(),
            })?;
        let cross_prioritization = CrossPrioritization::from_fix(cross_prioritization_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: cross_prioritization_char.to_string(),
                error: "Invalid cross prioritization".to_string(),
            })?;

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let ord_type_char = raw.get_field(40)
            .ok_or(FixParseError::MissingRequiredField(40))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: String::new(),
                error: "Empty ord type".to_string(),
            })?;
        let ord_type = OrdType::from_fix(ord_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: ord_type_char.to_string(),
                error: "Invalid ord type".to_string(),
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

        let price = raw.get_field(44).and_then(|s| s.parse().ok());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(NewOrderCross {
            cross_id,
            cross_type,
            cross_prioritization,
            symbol,
            ord_type,
            transact_time,
            price,
            text,
        })
    }
}

/// CrossOrderCancelRequest (MsgType u) - Cancel a cross order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossOrderCancelRequest {
    pub cross_id: String,                              // Tag 548
    pub orig_cross_id: String,                         // Tag 551
    pub cross_type: CrossType,                         // Tag 549
    pub cross_prioritization: CrossPrioritization,     // Tag 550
    pub symbol: String,                                // Tag 55
    pub transact_time: DateTime<Utc>,                  // Tag 60
    // Note: sides would be in the NoSides(552) repeating group
    pub order_id: Option<String>,                      // Tag 37
    pub text: Option<String>,                          // Tag 58
}

impl CrossOrderCancelRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        raw.set_field(8, "FIXT.1.1".to_string());
        raw.set_field(35, "u".to_string());
        raw.set_field(1128, "9".to_string()); // FIX 5.0 SP2

        raw.set_field(548, self.cross_id.clone());
        raw.set_field(551, self.orig_cross_id.clone());
        raw.set_field(549, self.cross_type.to_fix().to_string());
        raw.set_field(550, self.cross_prioritization.to_fix().to_string());
        raw.set_field(55, self.symbol.clone());

        let transact_time_str = self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string();
        raw.set_field(60, transact_time_str);

        if let Some(ref order_id) = self.order_id {
            raw.set_field(37, order_id.clone());
        }

        if let Some(ref text) = self.text {
            raw.set_field(58, text.clone());
        }

        raw
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cross_id = raw.get_field(548)
            .ok_or(FixParseError::MissingRequiredField(548))?
            .to_string();

        let orig_cross_id = raw.get_field(551)
            .ok_or(FixParseError::MissingRequiredField(551))?
            .to_string();

        let cross_type_char = raw.get_field(549)
            .ok_or(FixParseError::MissingRequiredField(549))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: String::new(),
                error: "Empty cross type".to_string(),
            })?;
        let cross_type = CrossType::from_fix(cross_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: cross_type_char.to_string(),
                error: "Invalid cross type".to_string(),
            })?;

        let cross_prioritization_char = raw.get_field(550)
            .ok_or(FixParseError::MissingRequiredField(550))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: String::new(),
                error: "Empty cross prioritization".to_string(),
            })?;
        let cross_prioritization = CrossPrioritization::from_fix(cross_prioritization_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: cross_prioritization_char.to_string(),
                error: "Invalid cross prioritization".to_string(),
            })?;

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let order_id = raw.get_field(37).map(|s| s.to_string());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(CrossOrderCancelRequest {
            cross_id,
            orig_cross_id,
            cross_type,
            cross_prioritization,
            symbol,
            transact_time,
            order_id,
            text,
        })
    }
}

/// CrossOrderCancelReplaceRequest (MsgType t) - Modify a cross order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossOrderCancelReplaceRequest {
    pub cross_id: String,                              // Tag 548
    pub orig_cross_id: String,                         // Tag 551
    pub cross_type: CrossType,                         // Tag 549
    pub cross_prioritization: CrossPrioritization,     // Tag 550
    pub symbol: String,                                // Tag 55
    pub ord_type: OrdType,                             // Tag 40
    pub transact_time: DateTime<Utc>,                  // Tag 60
    // Note: sides would be in the NoSides(552) repeating group
    pub price: Option<f64>,                            // Tag 44
    pub text: Option<String>,                          // Tag 58
}

impl CrossOrderCancelReplaceRequest {
    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        raw.set_field(8, "FIXT.1.1".to_string());
        raw.set_field(35, "t".to_string());
        raw.set_field(1128, "9".to_string()); // FIX 5.0 SP2

        raw.set_field(548, self.cross_id.clone());
        raw.set_field(551, self.orig_cross_id.clone());
        raw.set_field(549, self.cross_type.to_fix().to_string());
        raw.set_field(550, self.cross_prioritization.to_fix().to_string());
        raw.set_field(55, self.symbol.clone());
        raw.set_field(40, self.ord_type.to_fix().to_string());

        let transact_time_str = self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string();
        raw.set_field(60, transact_time_str);

        if let Some(price) = self.price {
            raw.set_field(44, price.to_string());
        }

        if let Some(ref text) = self.text {
            raw.set_field(58, text.clone());
        }

        raw
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let cross_id = raw.get_field(548)
            .ok_or(FixParseError::MissingRequiredField(548))?
            .to_string();

        let orig_cross_id = raw.get_field(551)
            .ok_or(FixParseError::MissingRequiredField(551))?
            .to_string();

        let cross_type_char = raw.get_field(549)
            .ok_or(FixParseError::MissingRequiredField(549))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: String::new(),
                error: "Empty cross type".to_string(),
            })?;
        let cross_type = CrossType::from_fix(cross_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 549,
                value: cross_type_char.to_string(),
                error: "Invalid cross type".to_string(),
            })?;

        let cross_prioritization_char = raw.get_field(550)
            .ok_or(FixParseError::MissingRequiredField(550))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: String::new(),
                error: "Empty cross prioritization".to_string(),
            })?;
        let cross_prioritization = CrossPrioritization::from_fix(cross_prioritization_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 550,
                value: cross_prioritization_char.to_string(),
                error: "Invalid cross prioritization".to_string(),
            })?;

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let ord_type_char = raw.get_field(40)
            .ok_or(FixParseError::MissingRequiredField(40))?
            .chars().next()
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: String::new(),
                error: "Empty ord type".to_string(),
            })?;
        let ord_type = OrdType::from_fix(ord_type_char)
            .ok_or(FixParseError::InvalidValue {
                tag: 40,
                value: ord_type_char.to_string(),
                error: "Invalid ord type".to_string(),
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

        let price = raw.get_field(44).and_then(|s| s.parse().ok());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(CrossOrderCancelReplaceRequest {
            cross_id,
            orig_cross_id,
            cross_type,
            cross_prioritization,
            symbol,
            ord_type,
            transact_time,
            price,
            text,
        })
    }
}

/// Top-level FIX message enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixMessage {
    Logon(Logon),
    Logout(Logout),
    Heartbeat(Heartbeat),
    NewOrderSingle(NewOrderSingle),
    OrderCancelRequest(OrderCancelRequest),
    OrderCancelReplaceRequest(OrderCancelReplaceRequest),
    OrderStatusRequest(OrderStatusRequest),
    DontKnowTrade(DontKnowTrade),
    ExecutionReport(ExecutionReport),
    ExecutionAcknowledgement(ExecutionAcknowledgement),
    OrderCancelReject(OrderCancelReject),
    OrderMassCancelRequest(OrderMassCancelRequest),
    OrderMassCancelReport(OrderMassCancelReport),
    NewOrderCross(NewOrderCross),
    CrossOrderCancelRequest(CrossOrderCancelRequest),
    CrossOrderCancelReplaceRequest(CrossOrderCancelReplaceRequest),
    MarketDataRequest(crate::market_data::MarketDataRequest),
    MarketDataSnapshotFullRefresh(crate::market_data::MarketDataSnapshotFullRefresh),
    MarketDataRequestReject(crate::market_data::MarketDataRequestReject),
    // Program Trading
    NewOrderList(crate::program_trading::NewOrderList),
    ListStatus(crate::program_trading::ListStatus),
    // Mass Order Messages
    MassOrder(crate::mass_orders::MassOrder),
    MassOrderAck(crate::mass_orders::MassOrderAck),
    OrderMassActionRequest(crate::mass_orders::OrderMassActionRequest),
    OrderMassActionReport(crate::mass_orders::OrderMassActionReport),
    OrderMassStatusRequest(crate::mass_orders::OrderMassStatusRequest),
    // Multileg Order Messages
    NewOrderMultileg(crate::multileg_orders::NewOrderMultileg),
    MultilegOrderCancelReplace(crate::multileg_orders::MultilegOrderCancelReplace),
    // Infrastructure Messages
    BusinessMessageReject(BusinessMessageReject),
    NetworkCounterpartySystemStatusRequest(crate::network_status::NetworkCounterpartySystemStatusRequest),
    NetworkCounterpartySystemStatusResponse(crate::network_status::NetworkCounterpartySystemStatusResponse),
    ApplicationMessageRequest(crate::application_sequencing::ApplicationMessageRequest),
    ApplicationMessageRequestAck(crate::application_sequencing::ApplicationMessageRequestAck),
    ApplicationMessageReport(crate::application_sequencing::ApplicationMessageReport),
    UserRequest(crate::user_management::UserRequest),
    UserResponse(crate::user_management::UserResponse),
    UserNotification(crate::user_management::UserNotification),
}

impl FixMessage {
    pub fn msg_type(&self) -> MsgType {
        match self {
            FixMessage::Logon(_) => MsgType::Logon,
            FixMessage::Logout(_) => MsgType::Logout,
            FixMessage::Heartbeat(_) => MsgType::Heartbeat,
            FixMessage::NewOrderSingle(_) => MsgType::NewOrderSingle,
            FixMessage::OrderCancelRequest(_) => MsgType::OrderCancelRequest,
            FixMessage::OrderCancelReplaceRequest(_) => MsgType::OrderCancelReplaceRequest,
            FixMessage::OrderStatusRequest(_) => MsgType::OrderStatusRequest,
            FixMessage::DontKnowTrade(_) => MsgType::DontKnowTrade,
            FixMessage::ExecutionReport(_) => MsgType::ExecutionReport,
            FixMessage::ExecutionAcknowledgement(_) => MsgType::ExecutionAcknowledgement,
            FixMessage::OrderCancelReject(_) => MsgType::OrderCancelReject,
            FixMessage::OrderMassCancelRequest(_) => MsgType::OrderMassCancelRequest,
            FixMessage::OrderMassCancelReport(_) => MsgType::OrderMassCancelReport,
            FixMessage::NewOrderCross(_) => MsgType::NewOrderCross,
            FixMessage::CrossOrderCancelRequest(_) => MsgType::CrossOrderCancelRequest,
            FixMessage::CrossOrderCancelReplaceRequest(_) => MsgType::CrossOrderCancelReplaceRequest,
            FixMessage::MarketDataRequest(_) => MsgType::MarketDataRequest,
            FixMessage::MarketDataSnapshotFullRefresh(_) => MsgType::MarketDataSnapshotFullRefresh,
            FixMessage::MarketDataRequestReject(_) => MsgType::MarketDataRequestReject,
            FixMessage::NewOrderList(_) => MsgType::NewOrderList,
            FixMessage::ListStatus(_) => MsgType::ListStatus,
            FixMessage::MassOrder(_) => MsgType::MassOrder,
            FixMessage::MassOrderAck(_) => MsgType::MassOrderAck,
            FixMessage::OrderMassActionRequest(_) => MsgType::OrderMassActionRequest,
            FixMessage::OrderMassActionReport(_) => MsgType::OrderMassActionReport,
            FixMessage::OrderMassStatusRequest(_) => MsgType::OrderMassStatusRequest,
            FixMessage::NewOrderMultileg(_) => MsgType::NewOrderMultileg,
            FixMessage::MultilegOrderCancelReplace(_) => MsgType::MultilegOrderCancelReplace,
            FixMessage::BusinessMessageReject(_) => MsgType::BusinessMessageReject,
            FixMessage::NetworkCounterpartySystemStatusRequest(_) => MsgType::NetworkCounterpartySystemStatusRequest,
            FixMessage::NetworkCounterpartySystemStatusResponse(_) => MsgType::NetworkCounterpartySystemStatusResponse,
            FixMessage::ApplicationMessageRequest(_) => MsgType::ApplicationMessageRequest,
            FixMessage::ApplicationMessageRequestAck(_) => MsgType::ApplicationMessageRequestAck,
            FixMessage::ApplicationMessageReport(_) => MsgType::ApplicationMessageReport,
            FixMessage::UserRequest(_) => MsgType::UserRequest,
            FixMessage::UserResponse(_) => MsgType::UserResponse,
            FixMessage::UserNotification(_) => MsgType::UserNotification,
        }
    }
}
