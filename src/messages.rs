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
    // Pre-Trade Quotation Messages
    MassQuote,                // i (lowercase)
    MassQuoteAcknowledgement, // b (lowercase)
    Quote,                    // S
    QuoteAcknowledgment,      // CW
    QuoteCancel,              // Z
    QuoteRequest,             // R
    QuoteRequestReject,       // AG
    QuoteResponse,            // AJ
    QuoteStatusReport,        // AI
    QuoteStatusRequest,       // a (lowercase)
    RFQRequest,               // AH
    // Pre-Trade Market Structure Messages
    MarketDefinition,         // BU
    MarketDefinitionRequest,  // BT
    MarketDefinitionUpdateReport, // BV
    TradingSessionList,       // BJ
    TradingSessionListRequest, // BI
    TradingSessionListUpdateReport, // BS
    TradingSessionStatus,     // h (lowercase)
    TradingSessionStatusRequest, // g (lowercase)
    // Pre-Trade Securities Reference Messages
    DerivativeSecurityList,   // AA
    DerivativeSecurityListRequest, // z (lowercase)
    SecurityDefinition,       // d (lowercase)
    SecurityDefinitionRequest, // c (lowercase)
    SecurityDefinitionUpdateReport, // BP
    SecurityList,             // y (lowercase)
    SecurityListRequest,      // x (lowercase)
    SecurityListUpdateReport, // BK
    SecurityMassStatus,       // CO
    SecurityMassStatusRequest, // CN
    SecurityStatus,           // f (lowercase)
    SecurityStatusRequest,    // e (lowercase)
    SecurityTypeRequest,      // v (lowercase)
    SecurityTypes,            // w (lowercase)
    // Post-Trade Messages
    AccountSummaryReport,                      // CQ
    AdjustedPositionReport,                    // BL
    AllocationInstruction,                     // J
    AllocationInstructionAck,                  // P
    AllocationInstructionAlert,                // BM
    AllocationInstructionAlertRequest,         // DU
    AllocationInstructionAlertRequestAck,      // DV
    AllocationReport,                          // AS
    AllocationReportAck,                       // AT
    AssignmentReport,                          // AW
    Confirmation,                              // AK
    ConfirmationAck,                           // AU
    ConfirmationRequest,                       // BH
    ContraryIntentionReport,                   // BO
    PositionMaintenanceReport,                 // AM
    PositionMaintenanceRequest,                // AL
    PositionReport,                            // AP
    PositionTransferInstruction,               // DL
    PositionTransferInstructionAck,            // DM
    PositionTransferReport,                    // DN
    RequestForPositions,                       // AN
    RequestForPositionsAck,                    // AO
    SettlementInstructionRequest,              // AV
    SettlementInstructions,                    // T
    SettlementObligationReport,                // BQ
    TradeCaptureReport,                        // AE
    TradeCaptureReportAck,                     // AR
    TradeCaptureReportRequest,                 // AD
    TradeCaptureReportRequestAck,              // AQ
    TradeMatchReport,                          // DC
    TradeMatchReportAck,                       // DD
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
            MsgType::MassQuote => "i",
            MsgType::MassQuoteAcknowledgement => "b",
            MsgType::Quote => "S",
            MsgType::QuoteAcknowledgment => "CW",
            MsgType::QuoteCancel => "Z",
            MsgType::QuoteRequest => "R",
            MsgType::QuoteRequestReject => "AG",
            MsgType::QuoteResponse => "AJ",
            MsgType::QuoteStatusReport => "AI",
            MsgType::QuoteStatusRequest => "a",
            MsgType::RFQRequest => "AH",
            MsgType::MarketDefinition => "BU",
            MsgType::MarketDefinitionRequest => "BT",
            MsgType::MarketDefinitionUpdateReport => "BV",
            MsgType::TradingSessionList => "BJ",
            MsgType::TradingSessionListRequest => "BI",
            MsgType::TradingSessionListUpdateReport => "BS",
            MsgType::TradingSessionStatus => "h",
            MsgType::TradingSessionStatusRequest => "g",
            MsgType::DerivativeSecurityList => "AA",
            MsgType::DerivativeSecurityListRequest => "z",
            MsgType::SecurityDefinition => "d",
            MsgType::SecurityDefinitionRequest => "c",
            MsgType::SecurityDefinitionUpdateReport => "BP",
            MsgType::SecurityList => "y",
            MsgType::SecurityListRequest => "x",
            MsgType::SecurityListUpdateReport => "BK",
            MsgType::SecurityMassStatus => "CO",
            MsgType::SecurityMassStatusRequest => "CN",
            MsgType::SecurityStatus => "f",
            MsgType::SecurityStatusRequest => "e",
            MsgType::SecurityTypeRequest => "v",
            MsgType::SecurityTypes => "w",
            MsgType::AccountSummaryReport => "CQ",
            MsgType::AdjustedPositionReport => "BL",
            MsgType::AllocationInstruction => "J",
            MsgType::AllocationInstructionAck => "P",
            MsgType::AllocationInstructionAlert => "BM",
            MsgType::AllocationInstructionAlertRequest => "DU",
            MsgType::AllocationInstructionAlertRequestAck => "DV",
            MsgType::AllocationReport => "AS",
            MsgType::AllocationReportAck => "AT",
            MsgType::AssignmentReport => "AW",
            MsgType::Confirmation => "AK",
            MsgType::ConfirmationAck => "AU",
            MsgType::ConfirmationRequest => "BH",
            MsgType::ContraryIntentionReport => "BO",
            MsgType::PositionMaintenanceReport => "AM",
            MsgType::PositionMaintenanceRequest => "AL",
            MsgType::PositionReport => "AP",
            MsgType::PositionTransferInstruction => "DL",
            MsgType::PositionTransferInstructionAck => "DM",
            MsgType::PositionTransferReport => "DN",
            MsgType::RequestForPositions => "AN",
            MsgType::RequestForPositionsAck => "AO",
            MsgType::SettlementInstructionRequest => "AV",
            MsgType::SettlementInstructions => "T",
            MsgType::SettlementObligationReport => "BQ",
            MsgType::TradeCaptureReport => "AE",
            MsgType::TradeCaptureReportAck => "AR",
            MsgType::TradeCaptureReportRequest => "AD",
            MsgType::TradeCaptureReportRequestAck => "AQ",
            MsgType::TradeMatchReport => "DC",
            MsgType::TradeMatchReportAck => "DD",
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
            "i" => Some(MsgType::MassQuote),
            "b" => Some(MsgType::MassQuoteAcknowledgement),
            "S" => Some(MsgType::Quote),
            "CW" => Some(MsgType::QuoteAcknowledgment),
            "Z" => Some(MsgType::QuoteCancel),
            "R" => Some(MsgType::QuoteRequest),
            "AG" => Some(MsgType::QuoteRequestReject),
            "AJ" => Some(MsgType::QuoteResponse),
            "AI" => Some(MsgType::QuoteStatusReport),
            "a" => Some(MsgType::QuoteStatusRequest),
            "AH" => Some(MsgType::RFQRequest),
            "BU" => Some(MsgType::MarketDefinition),
            "BT" => Some(MsgType::MarketDefinitionRequest),
            "BV" => Some(MsgType::MarketDefinitionUpdateReport),
            "BJ" => Some(MsgType::TradingSessionList),
            "BI" => Some(MsgType::TradingSessionListRequest),
            "BS" => Some(MsgType::TradingSessionListUpdateReport),
            "h" => Some(MsgType::TradingSessionStatus),
            "g" => Some(MsgType::TradingSessionStatusRequest),
            "AA" => Some(MsgType::DerivativeSecurityList),
            "z" => Some(MsgType::DerivativeSecurityListRequest),
            "d" => Some(MsgType::SecurityDefinition),
            "c" => Some(MsgType::SecurityDefinitionRequest),
            "BP" => Some(MsgType::SecurityDefinitionUpdateReport),
            "y" => Some(MsgType::SecurityList),
            "x" => Some(MsgType::SecurityListRequest),
            "BK" => Some(MsgType::SecurityListUpdateReport),
            "CO" => Some(MsgType::SecurityMassStatus),
            "CN" => Some(MsgType::SecurityMassStatusRequest),
            "f" => Some(MsgType::SecurityStatus),
            "e" => Some(MsgType::SecurityStatusRequest),
            "v" => Some(MsgType::SecurityTypeRequest),
            "w" => Some(MsgType::SecurityTypes),
            "CQ" => Some(MsgType::AccountSummaryReport),
            "BL" => Some(MsgType::AdjustedPositionReport),
            "J" => Some(MsgType::AllocationInstruction),
            "P" => Some(MsgType::AllocationInstructionAck),
            "BM" => Some(MsgType::AllocationInstructionAlert),
            "DU" => Some(MsgType::AllocationInstructionAlertRequest),
            "DV" => Some(MsgType::AllocationInstructionAlertRequestAck),
            "AS" => Some(MsgType::AllocationReport),
            "AT" => Some(MsgType::AllocationReportAck),
            "AW" => Some(MsgType::AssignmentReport),
            "AK" => Some(MsgType::Confirmation),
            "AU" => Some(MsgType::ConfirmationAck),
            "BH" => Some(MsgType::ConfirmationRequest),
            "BO" => Some(MsgType::ContraryIntentionReport),
            "AM" => Some(MsgType::PositionMaintenanceReport),
            "AL" => Some(MsgType::PositionMaintenanceRequest),
            "AP" => Some(MsgType::PositionReport),
            "DL" => Some(MsgType::PositionTransferInstruction),
            "DM" => Some(MsgType::PositionTransferInstructionAck),
            "DN" => Some(MsgType::PositionTransferReport),
            "AN" => Some(MsgType::RequestForPositions),
            "AO" => Some(MsgType::RequestForPositionsAck),
            "AV" => Some(MsgType::SettlementInstructionRequest),
            "T" => Some(MsgType::SettlementInstructions),
            "BQ" => Some(MsgType::SettlementObligationReport),
            "AE" => Some(MsgType::TradeCaptureReport),
            "AR" => Some(MsgType::TradeCaptureReportAck),
            "AD" => Some(MsgType::TradeCaptureReportRequest),
            "AQ" => Some(MsgType::TradeCaptureReportRequestAck),
            "DC" => Some(MsgType::TradeMatchReport),
            "DD" => Some(MsgType::TradeMatchReportAck),
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
    // Pre-Trade Quotation Messages
    MassQuote(crate::message_types::quotation::MassQuote),
    MassQuoteAcknowledgement(crate::message_types::quotation::MassQuoteAcknowledgement),
    Quote(crate::message_types::quotation::Quote),
    QuoteAcknowledgment(crate::message_types::quotation::QuoteAcknowledgment),
    QuoteCancel(crate::message_types::quotation::QuoteCancel),
    QuoteRequest(crate::message_types::quotation::QuoteRequest),
    QuoteRequestReject(crate::message_types::quotation::QuoteRequestReject),
    QuoteResponse(crate::message_types::quotation::QuoteResponse),
    QuoteStatusReport(crate::message_types::quotation::QuoteStatusReport),
    QuoteStatusRequest(crate::message_types::quotation::QuoteStatusRequest),
    RFQRequest(crate::message_types::quotation::RFQRequest),
    // Pre-Trade Market Structure Messages
    MarketDefinition(crate::message_types::market_structure::MarketDefinition),
    MarketDefinitionRequest(crate::message_types::market_structure::MarketDefinitionRequest),
    MarketDefinitionUpdateReport(crate::message_types::market_structure::MarketDefinitionUpdateReport),
    TradingSessionList(crate::message_types::market_structure::TradingSessionList),
    TradingSessionListRequest(crate::message_types::market_structure::TradingSessionListRequest),
    TradingSessionListUpdateReport(crate::message_types::market_structure::TradingSessionListUpdateReport),
    TradingSessionStatus(crate::message_types::market_structure::TradingSessionStatus),
    TradingSessionStatusRequest(crate::message_types::market_structure::TradingSessionStatusRequest),
    // Pre-Trade Securities Reference Messages
    DerivativeSecurityList(crate::message_types::securities_reference::DerivativeSecurityList),
    DerivativeSecurityListRequest(crate::message_types::securities_reference::DerivativeSecurityListRequest),
    SecurityDefinition(crate::message_types::securities_reference::SecurityDefinition),
    SecurityDefinitionRequest(crate::message_types::securities_reference::SecurityDefinitionRequest),
    SecurityDefinitionUpdateReport(crate::message_types::securities_reference::SecurityDefinitionUpdateReport),
    SecurityList(crate::message_types::securities_reference::SecurityList),
    SecurityListRequest(crate::message_types::securities_reference::SecurityListRequest),
    SecurityListUpdateReport(crate::message_types::securities_reference::SecurityListUpdateReport),
    SecurityMassStatus(crate::message_types::securities_reference::SecurityMassStatus),
    SecurityMassStatusRequest(crate::message_types::securities_reference::SecurityMassStatusRequest),
    SecurityStatus(crate::message_types::securities_reference::SecurityStatus),
    SecurityStatusRequest(crate::message_types::securities_reference::SecurityStatusRequest),
    SecurityTypeRequest(crate::message_types::securities_reference::SecurityTypeRequest),
    SecurityTypes(crate::message_types::securities_reference::SecurityTypes),
    // Post-Trade Messages
    AccountSummaryReport(crate::message_types::account_reporting::AccountSummaryReport),
    AdjustedPositionReport(crate::message_types::position_maintenance::AdjustedPositionReport),
    AllocationInstruction(crate::message_types::allocation::AllocationInstruction),
    AllocationInstructionAck(crate::message_types::allocation::AllocationInstructionAck),
    AllocationInstructionAlert(crate::message_types::allocation::AllocationInstructionAlert),
    AllocationInstructionAlertRequest(crate::message_types::allocation::AllocationInstructionAlertRequest),
    AllocationInstructionAlertRequestAck(crate::message_types::allocation::AllocationInstructionAlertRequestAck),
    AllocationReport(crate::message_types::allocation::AllocationReport),
    AllocationReportAck(crate::message_types::allocation::AllocationReportAck),
    AssignmentReport(crate::message_types::position_maintenance::AssignmentReport),
    Confirmation(crate::message_types::confirmation::Confirmation),
    ConfirmationAck(crate::message_types::confirmation::ConfirmationAck),
    ConfirmationRequest(crate::message_types::confirmation::ConfirmationRequest),
    ContraryIntentionReport(crate::message_types::position_maintenance::ContraryIntentionReport),
    PositionMaintenanceReport(crate::message_types::position_maintenance::PositionMaintenanceReport),
    PositionMaintenanceRequest(crate::message_types::position_maintenance::PositionMaintenanceRequest),
    PositionReport(crate::message_types::position_maintenance::PositionReport),
    PositionTransferInstruction(crate::message_types::position_maintenance::PositionTransferInstruction),
    PositionTransferInstructionAck(crate::message_types::position_maintenance::PositionTransferInstructionAck),
    PositionTransferReport(crate::message_types::position_maintenance::PositionTransferReport),
    RequestForPositions(crate::message_types::position_maintenance::RequestForPositions),
    RequestForPositionsAck(crate::message_types::position_maintenance::RequestForPositionsAck),
    SettlementInstructionRequest(crate::message_types::settlement_instruction::SettlementInstructionRequest),
    SettlementInstructions(crate::message_types::settlement_instruction::SettlementInstructions),
    SettlementObligationReport(crate::message_types::settlement_instruction::SettlementObligationReport),
    TradeCaptureReport(crate::message_types::trade_capture_reporting::TradeCaptureReport),
    TradeCaptureReportAck(crate::message_types::trade_capture_reporting::TradeCaptureReportAck),
    TradeCaptureReportRequest(crate::message_types::trade_capture_reporting::TradeCaptureReportRequest),
    TradeCaptureReportRequestAck(crate::message_types::trade_capture_reporting::TradeCaptureReportRequestAck),
    TradeMatchReport(crate::message_types::trade_capture_reporting::TradeMatchReport),
    TradeMatchReportAck(crate::message_types::trade_capture_reporting::TradeMatchReportAck),
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
            FixMessage::MassQuote(_) => MsgType::MassQuote,
            FixMessage::MassQuoteAcknowledgement(_) => MsgType::MassQuoteAcknowledgement,
            FixMessage::Quote(_) => MsgType::Quote,
            FixMessage::QuoteAcknowledgment(_) => MsgType::QuoteAcknowledgment,
            FixMessage::QuoteCancel(_) => MsgType::QuoteCancel,
            FixMessage::QuoteRequest(_) => MsgType::QuoteRequest,
            FixMessage::QuoteRequestReject(_) => MsgType::QuoteRequestReject,
            FixMessage::QuoteResponse(_) => MsgType::QuoteResponse,
            FixMessage::QuoteStatusReport(_) => MsgType::QuoteStatusReport,
            FixMessage::QuoteStatusRequest(_) => MsgType::QuoteStatusRequest,
            FixMessage::RFQRequest(_) => MsgType::RFQRequest,
            FixMessage::MarketDefinition(_) => MsgType::MarketDefinition,
            FixMessage::MarketDefinitionRequest(_) => MsgType::MarketDefinitionRequest,
            FixMessage::MarketDefinitionUpdateReport(_) => MsgType::MarketDefinitionUpdateReport,
            FixMessage::TradingSessionList(_) => MsgType::TradingSessionList,
            FixMessage::TradingSessionListRequest(_) => MsgType::TradingSessionListRequest,
            FixMessage::TradingSessionListUpdateReport(_) => MsgType::TradingSessionListUpdateReport,
            FixMessage::TradingSessionStatus(_) => MsgType::TradingSessionStatus,
            FixMessage::TradingSessionStatusRequest(_) => MsgType::TradingSessionStatusRequest,
            FixMessage::DerivativeSecurityList(_) => MsgType::DerivativeSecurityList,
            FixMessage::DerivativeSecurityListRequest(_) => MsgType::DerivativeSecurityListRequest,
            FixMessage::SecurityDefinition(_) => MsgType::SecurityDefinition,
            FixMessage::SecurityDefinitionRequest(_) => MsgType::SecurityDefinitionRequest,
            FixMessage::SecurityDefinitionUpdateReport(_) => MsgType::SecurityDefinitionUpdateReport,
            FixMessage::SecurityList(_) => MsgType::SecurityList,
            FixMessage::SecurityListRequest(_) => MsgType::SecurityListRequest,
            FixMessage::SecurityListUpdateReport(_) => MsgType::SecurityListUpdateReport,
            FixMessage::SecurityMassStatus(_) => MsgType::SecurityMassStatus,
            FixMessage::SecurityMassStatusRequest(_) => MsgType::SecurityMassStatusRequest,
            FixMessage::SecurityStatus(_) => MsgType::SecurityStatus,
            FixMessage::SecurityStatusRequest(_) => MsgType::SecurityStatusRequest,
            FixMessage::SecurityTypeRequest(_) => MsgType::SecurityTypeRequest,
            FixMessage::SecurityTypes(_) => MsgType::SecurityTypes,
            FixMessage::AccountSummaryReport(_) => MsgType::AccountSummaryReport,
            FixMessage::AdjustedPositionReport(_) => MsgType::AdjustedPositionReport,
            FixMessage::AllocationInstruction(_) => MsgType::AllocationInstruction,
            FixMessage::AllocationInstructionAck(_) => MsgType::AllocationInstructionAck,
            FixMessage::AllocationInstructionAlert(_) => MsgType::AllocationInstructionAlert,
            FixMessage::AllocationInstructionAlertRequest(_) => MsgType::AllocationInstructionAlertRequest,
            FixMessage::AllocationInstructionAlertRequestAck(_) => MsgType::AllocationInstructionAlertRequestAck,
            FixMessage::AllocationReport(_) => MsgType::AllocationReport,
            FixMessage::AllocationReportAck(_) => MsgType::AllocationReportAck,
            FixMessage::AssignmentReport(_) => MsgType::AssignmentReport,
            FixMessage::Confirmation(_) => MsgType::Confirmation,
            FixMessage::ConfirmationAck(_) => MsgType::ConfirmationAck,
            FixMessage::ConfirmationRequest(_) => MsgType::ConfirmationRequest,
            FixMessage::ContraryIntentionReport(_) => MsgType::ContraryIntentionReport,
            FixMessage::PositionMaintenanceReport(_) => MsgType::PositionMaintenanceReport,
            FixMessage::PositionMaintenanceRequest(_) => MsgType::PositionMaintenanceRequest,
            FixMessage::PositionReport(_) => MsgType::PositionReport,
            FixMessage::PositionTransferInstruction(_) => MsgType::PositionTransferInstruction,
            FixMessage::PositionTransferInstructionAck(_) => MsgType::PositionTransferInstructionAck,
            FixMessage::PositionTransferReport(_) => MsgType::PositionTransferReport,
            FixMessage::RequestForPositions(_) => MsgType::RequestForPositions,
            FixMessage::RequestForPositionsAck(_) => MsgType::RequestForPositionsAck,
            FixMessage::SettlementInstructionRequest(_) => MsgType::SettlementInstructionRequest,
            FixMessage::SettlementInstructions(_) => MsgType::SettlementInstructions,
            FixMessage::SettlementObligationReport(_) => MsgType::SettlementObligationReport,
            FixMessage::TradeCaptureReport(_) => MsgType::TradeCaptureReport,
            FixMessage::TradeCaptureReportAck(_) => MsgType::TradeCaptureReportAck,
            FixMessage::TradeCaptureReportRequest(_) => MsgType::TradeCaptureReportRequest,
            FixMessage::TradeCaptureReportRequestAck(_) => MsgType::TradeCaptureReportRequestAck,
            FixMessage::TradeMatchReport(_) => MsgType::TradeMatchReport,
            FixMessage::TradeMatchReportAck(_) => MsgType::TradeMatchReportAck,
        }
    }
}
