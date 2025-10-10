use crate::types::*;
use crate::parser::{RawFixMessage, FixParseError};
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

/// Top-level FIX message enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixMessage {
    Logon(Logon),
    Logout(Logout),
    Heartbeat(Heartbeat),
    NewOrderSingle(crate::message_types::single_general_order_handling::NewOrderSingle),
    OrderCancelRequest(crate::message_types::single_general_order_handling::OrderCancelRequest),
    OrderCancelReplaceRequest(crate::message_types::single_general_order_handling::OrderCancelReplaceRequest),
    OrderStatusRequest(crate::message_types::single_general_order_handling::OrderStatusRequest),
    DontKnowTrade(DontKnowTrade),
    ExecutionReport(crate::message_types::single_general_order_handling::ExecutionReport),
    ExecutionAcknowledgement(ExecutionAcknowledgement),
    OrderCancelReject(crate::message_types::single_general_order_handling::OrderCancelReject),
    OrderMassCancelRequest(crate::message_types::order_mass_handling::OrderMassCancelRequest),
    OrderMassCancelReport(crate::message_types::order_mass_handling::OrderMassCancelReport),
    NewOrderCross(crate::message_types::order_cross_handling::NewOrderCross),
    CrossOrderCancelRequest(crate::message_types::order_cross_handling::CrossOrderCancelRequest),
    CrossOrderCancelReplaceRequest(crate::message_types::order_cross_handling::CrossOrderCancelReplaceRequest),
    MarketDataRequest(crate::message_types::market_data::MarketDataRequest),
    MarketDataSnapshotFullRefresh(crate::message_types::market_data::MarketDataSnapshotFullRefresh),
    MarketDataRequestReject(crate::message_types::market_data::MarketDataRequestReject),
    // Program Trading
    NewOrderList(crate::message_types::program_trading::NewOrderList),
    ListStatus(crate::message_types::program_trading::ListStatus),
    // Mass Order Messages
    MassOrder(crate::message_types::mass_orders::MassOrder),
    MassOrderAck(crate::message_types::mass_orders::MassOrderAck),
    OrderMassActionRequest(crate::message_types::mass_orders::OrderMassActionRequest),
    OrderMassActionReport(crate::message_types::mass_orders::OrderMassActionReport),
    OrderMassStatusRequest(crate::message_types::mass_orders::OrderMassStatusRequest),
    // Multileg Order Messages
    NewOrderMultileg(crate::message_types::multileg_orders::NewOrderMultileg),
    MultilegOrderCancelReplace(crate::message_types::multileg_orders::MultilegOrderCancelReplace),
    // Infrastructure Messages
    BusinessMessageReject(BusinessMessageReject),
    NetworkCounterpartySystemStatusRequest(crate::message_types::network_status::NetworkCounterpartySystemStatusRequest),
    NetworkCounterpartySystemStatusResponse(crate::message_types::network_status::NetworkCounterpartySystemStatusResponse),
    ApplicationMessageRequest(crate::message_types::application_sequencing::ApplicationMessageRequest),
    ApplicationMessageRequestAck(crate::message_types::application_sequencing::ApplicationMessageRequestAck),
    ApplicationMessageReport(crate::message_types::application_sequencing::ApplicationMessageReport),
    UserRequest(crate::message_types::user_management::UserRequest),
    UserResponse(crate::message_types::user_management::UserResponse),
    UserNotification(crate::message_types::user_management::UserNotification),
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
