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
    // Pre-Trade Indication Messages
    Advertisement,            // 7
    CrossRequest,             // DS
    CrossRequestAck,          // DT
    IOI,                      // 6
    // Pre-Trade Event Communication Messages
    Email,                    // C
    News,                     // B
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
            MsgType::Advertisement => "7",
            MsgType::CrossRequest => "DS",
            MsgType::CrossRequestAck => "DT",
            MsgType::IOI => "6",
            MsgType::Email => "C",
            MsgType::News => "B",
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
            "7" => Some(MsgType::Advertisement),
            "DS" => Some(MsgType::CrossRequest),
            "DT" => Some(MsgType::CrossRequestAck),
            "6" => Some(MsgType::IOI),
            "C" => Some(MsgType::Email),
            "B" => Some(MsgType::News),
            _ => None,
        }
    }
}

/// Top-level FIX message enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FixMessage {
    Logon(crate::message_types::infrastructure::Logon),
    Logout(crate::message_types::infrastructure::Logout),
    Heartbeat(crate::message_types::infrastructure::Heartbeat),
    NewOrderSingle(crate::message_types::single_general_order_handling::NewOrderSingle),
    OrderCancelRequest(crate::message_types::single_general_order_handling::OrderCancelRequest),
    OrderCancelReplaceRequest(crate::message_types::single_general_order_handling::OrderCancelReplaceRequest),
    OrderStatusRequest(crate::message_types::single_general_order_handling::OrderStatusRequest),
    DontKnowTrade(crate::message_types::confirmations::DontKnowTrade),
    ExecutionReport(crate::message_types::single_general_order_handling::ExecutionReport),
    ExecutionAcknowledgement(crate::message_types::single_general_order_handling::ExecutionAcknowledgement),
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
    BusinessMessageReject(crate::message_types::business_message_rejects::BusinessMessageReject),
    NetworkCounterpartySystemStatusRequest(crate::message_types::network_status::NetworkCounterpartySystemStatusRequest),
    NetworkCounterpartySystemStatusResponse(crate::message_types::network_status::NetworkCounterpartySystemStatusResponse),
    ApplicationMessageRequest(crate::message_types::application_sequencing::ApplicationMessageRequest),
    ApplicationMessageRequestAck(crate::message_types::application_sequencing::ApplicationMessageRequestAck),
    ApplicationMessageReport(crate::message_types::application_sequencing::ApplicationMessageReport),
    UserRequest(crate::message_types::user_management::UserRequest),
    UserResponse(crate::message_types::user_management::UserResponse),
    UserNotification(crate::message_types::user_management::UserNotification),
    // Pre-Trade Indication Messages
    Advertisement(crate::message_types::indication::Advertisement),
    CrossRequest(crate::message_types::indication::CrossRequest),
    CrossRequestAck(crate::message_types::indication::CrossRequestAck),
    IOI(crate::message_types::indication::IOI),
    // Pre-Trade Event Communication Messages
    Email(crate::message_types::event_communication::Email),
    News(crate::message_types::event_communication::News),
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
            FixMessage::Advertisement(_) => MsgType::Advertisement,
            FixMessage::CrossRequest(_) => MsgType::CrossRequest,
            FixMessage::CrossRequestAck(_) => MsgType::CrossRequestAck,
            FixMessage::IOI(_) => MsgType::IOI,
            FixMessage::Email(_) => MsgType::Email,
            FixMessage::News(_) => MsgType::News,
        }
    }
}
