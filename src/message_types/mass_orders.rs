//! Mass Order message types
//!
//! This module implements FIX 5.0 SP2 Mass Order messages, which provide
//! bulk order management capabilities for multiple unrelated orders.
//!
//! ## Message Types
//! - OrderMassStatusRequest (AF): Request status for orders matching criteria
//! - OrderMassActionRequest (CA): Request suspend/release/cancel for matching orders
//! - OrderMassActionReport (BZ): Acknowledge OrderMassActionRequest
//! - MassOrder (DJ): Add, modify, or delete multiple unrelated orders
//! - MassOrderAck (DK): Acknowledge MassOrder message

use crate::parser::{RawFixMessage, FixParseError};
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// OrderMassStatusRequest (MsgType = AF)
// ============================================================================

/// OrderMassStatusRequest (AF) - Request status for orders matching criteria
///
/// This is the simplest Mass Order message. It requests status for orders
/// matching specific criteria. Responses come as ExecutionReport (8) messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderMassStatusRequest {
    /// MassStatusReqID (Tag 584) - Required - Unique request identifier
    pub mass_status_req_id: String,

    /// MassStatusReqType (Tag 585) - Required - Type of status request
    pub mass_status_req_type: MassStatusReqType,

    /// ClOrdID (Tag 11) - Optional - Specific order ID (when requesting status for one order)
    pub cl_ord_id: Option<String>,

    /// Account (Tag 1) - Optional - Account filter
    pub account: Option<String>,

    /// TradingSessionID (Tag 336) - Optional - Trading session filter
    pub trading_session_id: Option<String>,

    /// Symbol (Tag 55) - Optional - Symbol filter
    pub symbol: Option<String>,

    /// UnderlyingSymbol (Tag 311) - Optional - Underlying symbol filter
    pub underlying_symbol: Option<String>,

    /// Side (Tag 54) - Optional - Side filter
    pub side: Option<Side>,
}

impl OrderMassStatusRequest {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(OrderMassStatusRequest {
            mass_status_req_id: raw.get_field(584)
                .ok_or_else(|| FixParseError::MissingRequiredField(584))?
                .to_string(),
            mass_status_req_type: raw.get_field(585)
                .and_then(|s| s.chars().next())
                .and_then(MassStatusReqType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(585))?,
            cl_ord_id: raw.get_field(11).map(|s| s.to_string()),
            account: raw.get_field(1).map(|s| s.to_string()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            underlying_symbol: raw.get_field(311).map(|s| s.to_string()),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix),
        })
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AF".to_string());

        // Required fields
        msg.set_field(584, self.mass_status_req_id.clone());
        msg.set_field(585, self.mass_status_req_type.to_fix().to_string());

        // Optional fields
        if let Some(ref cl_ord_id) = self.cl_ord_id {
            msg.set_field(11, cl_ord_id.clone());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(ref trading_session_id) = self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(ref underlying_symbol) = self.underlying_symbol {
            msg.set_field(311, underlying_symbol.clone());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }

        msg
    }
}

// ============================================================================
// OrderMassActionRequest (MsgType = CA)
// ============================================================================

/// OrderMassActionRequest (CA) - Request suspend, release, or cancel for orders
///
/// Requests a bulk action (suspend/release/cancel) on orders matching criteria.
/// Response is OrderMassActionReport (BZ).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderMassActionRequest {
    /// ClOrdID (Tag 11) - Required - Unique ID for this request
    pub cl_ord_id: String,

    /// MassActionType (Tag 1373) - Required - Type of mass action
    pub mass_action_type: MassActionType,

    /// MassActionScope (Tag 1374) - Optional - Scope of mass action
    pub mass_action_scope: Option<u32>,

    /// MarketID (Tag 1301) - Optional - Market filter
    pub market_id: Option<String>,

    /// MarketSegmentID (Tag 1300) - Optional - Market segment filter
    pub market_segment_id: Option<String>,

    /// TradingSessionID (Tag 336) - Optional - Trading session filter
    pub trading_session_id: Option<String>,

    /// Symbol (Tag 55) - Optional - Symbol filter
    pub symbol: Option<String>,

    /// SecurityType (Tag 167) - Optional - Security type filter
    pub security_type: Option<String>,

    /// Side (Tag 54) - Optional - Side filter
    pub side: Option<Side>,

    /// TransactTime (Tag 60) - Required - Time of request
    pub transact_time: String,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: TargetMarketSegmentGrp - Repeating group (Tag 1793 = NoTargetMarketSegments)
    // This is stored in the RawFixMessage.groups field with key 1793
}

impl OrderMassActionRequest {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(OrderMassActionRequest {
            cl_ord_id: raw.get_field(11)
                .ok_or_else(|| FixParseError::MissingRequiredField(11))?
                .to_string(),
            mass_action_type: raw.get_field(1373)
                .and_then(|s| s.chars().next())
                .and_then(MassActionType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(1373))?,
            mass_action_scope: raw.get_field(1374)
                .and_then(|s| s.parse().ok()),
            market_id: raw.get_field(1301).map(|s| s.to_string()),
            market_segment_id: raw.get_field(1300).map(|s| s.to_string()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_type: raw.get_field(167).map(|s| s.to_string()),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix),
            transact_time: raw.get_field(60)
                .ok_or_else(|| FixParseError::MissingRequiredField(60))?
                .to_string(),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: TargetMarketSegmentGrp (1793) is accessed via raw.groups.get(&1793)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "CA".to_string());

        // Required fields
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(1373, self.mass_action_type.to_fix().to_string());
        msg.set_field(60, self.transact_time.clone());

        // Optional fields
        if let Some(scope) = self.mass_action_scope {
            msg.set_field(1374, scope.to_string());
        }
        if let Some(ref market_id) = self.market_id {
            msg.set_field(1301, market_id.clone());
        }
        if let Some(ref market_segment_id) = self.market_segment_id {
            msg.set_field(1300, market_segment_id.clone());
        }
        if let Some(ref trading_session_id) = self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(ref security_type) = self.security_type {
            msg.set_field(167, security_type.clone());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: TargetMarketSegmentGrp (1793) should be added via msg.groups
        msg
    }
}

// ============================================================================
// OrderMassActionReport (MsgType = BZ)
// ============================================================================

/// OrderMassActionReport (BZ) - Acknowledge OrderMassActionRequest
///
/// Reports the result of an OrderMassActionRequest, including affected/not-affected
/// market segments and total order counts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderMassActionReport {
    /// ClOrdID (Tag 11) - Optional - From request
    pub cl_ord_id: Option<String>,

    /// MassActionReportID (Tag 1369) - Required - Unique report identifier
    pub mass_action_report_id: String,

    /// MassActionType (Tag 1373) - Optional - Echo from request
    pub mass_action_type: Option<MassActionType>,

    /// MassActionScope (Tag 1374) - Optional - Echo from request
    pub mass_action_scope: Option<u32>,

    /// MassActionResponse (Tag 1375) - Required - Accepted/Rejected
    pub mass_action_response: MassActionResponse,

    /// MassActionRejectReason (Tag 1376) - Optional - Rejection reason
    pub mass_action_reject_reason: Option<u32>,

    /// TotalAffectedOrders (Tag 533) - Optional - Total number of affected orders
    pub total_affected_orders: Option<u32>,

    /// MarketID (Tag 1301) - Optional - Market ID
    pub market_id: Option<String>,

    /// MarketSegmentID (Tag 1300) - Optional - Market segment ID
    pub market_segment_id: Option<String>,

    /// TradingSessionID (Tag 336) - Optional - Trading session
    pub trading_session_id: Option<String>,

    /// Symbol (Tag 55) - Optional - Symbol
    pub symbol: Option<String>,

    /// SecurityType (Tag 167) - Optional - Security type
    pub security_type: Option<String>,

    /// Side (Tag 54) - Optional - Side
    pub side: Option<Side>,

    /// TransactTime (Tag 60) - Required - Time of report
    pub transact_time: String,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: AffectedMarketSegmentGrp - Repeating group (Tag 1793 = NoAffectedMarketSegments)
    // Note: NotAffectedMarketSegmentGrp - Repeating group (Tag 1794 = NoNotAffMarketSegments)
    // These are stored in the RawFixMessage.groups field
}

impl OrderMassActionReport {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(OrderMassActionReport {
            cl_ord_id: raw.get_field(11).map(|s| s.to_string()),
            mass_action_report_id: raw.get_field(1369)
                .ok_or_else(|| FixParseError::MissingRequiredField(1369))?
                .to_string(),
            mass_action_type: raw.get_field(1373)
                .and_then(|s| s.chars().next())
                .and_then(MassActionType::from_fix),
            mass_action_scope: raw.get_field(1374)
                .and_then(|s| s.parse().ok()),
            mass_action_response: raw.get_field(1375)
                .and_then(|s| s.chars().next())
                .and_then(MassActionResponse::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(1375))?,
            mass_action_reject_reason: raw.get_field(1376)
                .and_then(|s| s.parse().ok()),
            total_affected_orders: raw.get_field(533)
                .and_then(|s| s.parse().ok()),
            market_id: raw.get_field(1301).map(|s| s.to_string()),
            market_segment_id: raw.get_field(1300).map(|s| s.to_string()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_type: raw.get_field(167).map(|s| s.to_string()),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix),
            transact_time: raw.get_field(60)
                .ok_or_else(|| FixParseError::MissingRequiredField(60))?
                .to_string(),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: AffectedMarketSegmentGrp (1793) is accessed via raw.groups.get(&1793)
        // Note: NotAffectedMarketSegmentGrp (1794) is accessed via raw.groups.get(&1794)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "BZ".to_string());

        // Required fields
        msg.set_field(1369, self.mass_action_report_id.clone());
        msg.set_field(1375, self.mass_action_response.to_fix().to_string());
        msg.set_field(60, self.transact_time.clone());

        // Optional fields
        if let Some(ref cl_ord_id) = self.cl_ord_id {
            msg.set_field(11, cl_ord_id.clone());
        }
        if let Some(mass_action_type) = self.mass_action_type {
            msg.set_field(1373, mass_action_type.to_fix().to_string());
        }
        if let Some(scope) = self.mass_action_scope {
            msg.set_field(1374, scope.to_string());
        }
        if let Some(reason) = self.mass_action_reject_reason {
            msg.set_field(1376, reason.to_string());
        }
        if let Some(total) = self.total_affected_orders {
            msg.set_field(533, total.to_string());
        }
        if let Some(ref market_id) = self.market_id {
            msg.set_field(1301, market_id.clone());
        }
        if let Some(ref market_segment_id) = self.market_segment_id {
            msg.set_field(1300, market_segment_id.clone());
        }
        if let Some(ref trading_session_id) = self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(ref security_type) = self.security_type {
            msg.set_field(167, security_type.clone());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: AffectedMarketSegmentGrp (1793) should be added via msg.groups
        // Note: NotAffectedMarketSegmentGrp (1794) should be added via msg.groups
        msg
    }
}

// ============================================================================
// MassOrder (MsgType = DJ)
// ============================================================================

/// MassOrder (DJ) - Add, modify, or delete multiple unrelated orders
///
/// This is the most complex Mass Order message. It can add, modify, or delete
/// multiple unrelated orders in a single message using OrderEntryGrp.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MassOrder {
    /// MassOrderID (Tag 2436) - Required - Unique identifier for this mass order
    pub mass_order_id: String,

    /// OrderEntryAction (Tag 2429) - Required - Action to perform
    pub order_entry_action: OrderEntryAction,

    /// SenderCompID (Tag 49) - Optional - Message sender
    pub sender_comp_id: Option<String>,

    /// TargetCompID (Tag 56) - Optional - Message target
    pub target_comp_id: Option<String>,

    /// OrderResponseLevel (Tag 2427) - Optional - Level of acknowledgement detail
    pub order_response_level: Option<OrderResponseLevel>,

    /// TransactTime (Tag 60) - Optional - Transaction time
    pub transact_time: Option<String>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: OrderEntryGrp - Repeating group of orders (Tag 2430 = NoOrderEntries)
    // This is stored in the RawFixMessage.groups field with key 2430
    // Each entry contains full order details with nested Parties and PreAllocGrp
}

impl MassOrder {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(MassOrder {
            mass_order_id: raw.get_field(2436)
                .ok_or_else(|| FixParseError::MissingRequiredField(2436))?
                .to_string(),
            order_entry_action: raw.get_field(2429)
                .and_then(|s| s.chars().next())
                .and_then(OrderEntryAction::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(2429))?,
            sender_comp_id: raw.get_field(49).map(|s| s.to_string()),
            target_comp_id: raw.get_field(56).map(|s| s.to_string()),
            order_response_level: raw.get_field(2427)
                .and_then(|s| s.chars().next())
                .and_then(OrderResponseLevel::from_fix),
            transact_time: raw.get_field(60).map(|s| s.to_string()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: OrderEntryGrp (2430) is accessed via raw.groups.get(&2430)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "DJ".to_string());

        // Required fields
        msg.set_field(2436, self.mass_order_id.clone());
        msg.set_field(2429, self.order_entry_action.to_fix().to_string());

        // Optional fields
        if let Some(ref sender_comp_id) = self.sender_comp_id {
            msg.set_field(49, sender_comp_id.clone());
        }
        if let Some(ref target_comp_id) = self.target_comp_id {
            msg.set_field(56, target_comp_id.clone());
        }
        if let Some(response_level) = self.order_response_level {
            msg.set_field(2427, response_level.to_fix().to_string());
        }
        if let Some(ref transact_time) = self.transact_time {
            msg.set_field(60, transact_time.clone());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: OrderEntryGrp (2430) should be added via msg.groups
        msg
    }
}

// ============================================================================
// MassOrderAck (MsgType = DK)
// ============================================================================

/// MassOrderAck (DK) - Acknowledge MassOrder message
///
/// Provides acknowledgement status for each order entry in a MassOrder.
/// Individual execution details come via ExecutionReport (8) messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MassOrderAck {
    /// MassOrderID (Tag 2436) - Required - Echo from MassOrder
    pub mass_order_id: String,

    /// OrderEntryAction (Tag 2429) - Optional - Echo from MassOrder
    pub order_entry_action: Option<OrderEntryAction>,

    /// OrderResponseLevel (Tag 2427) - Optional - Level of acknowledgement detail
    pub order_response_level: Option<OrderResponseLevel>,

    /// SenderCompID (Tag 49) - Optional - Message sender
    pub sender_comp_id: Option<String>,

    /// TargetCompID (Tag 56) - Optional - Message target
    pub target_comp_id: Option<String>,

    /// TransactTime (Tag 60) - Optional - Transaction time
    pub transact_time: Option<String>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: OrderEntryAckGrp - Repeating group (Tag 2430 = NoOrderEntries)
    // This is stored in the RawFixMessage.groups field with key 2430
    // Each entry contains acknowledgement status for an order
}

impl MassOrderAck {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(MassOrderAck {
            mass_order_id: raw.get_field(2436)
                .ok_or_else(|| FixParseError::MissingRequiredField(2436))?
                .to_string(),
            order_entry_action: raw.get_field(2429)
                .and_then(|s| s.chars().next())
                .and_then(OrderEntryAction::from_fix),
            order_response_level: raw.get_field(2427)
                .and_then(|s| s.chars().next())
                .and_then(OrderResponseLevel::from_fix),
            sender_comp_id: raw.get_field(49).map(|s| s.to_string()),
            target_comp_id: raw.get_field(56).map(|s| s.to_string()),
            transact_time: raw.get_field(60).map(|s| s.to_string()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: OrderEntryAckGrp (2430) is accessed via raw.groups.get(&2430)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "DK".to_string());

        // Required fields
        msg.set_field(2436, self.mass_order_id.clone());

        // Optional fields
        if let Some(order_entry_action) = self.order_entry_action {
            msg.set_field(2429, order_entry_action.to_fix().to_string());
        }
        if let Some(response_level) = self.order_response_level {
            msg.set_field(2427, response_level.to_fix().to_string());
        }
        if let Some(ref sender_comp_id) = self.sender_comp_id {
            msg.set_field(49, sender_comp_id.clone());
        }
        if let Some(ref target_comp_id) = self.target_comp_id {
            msg.set_field(56, target_comp_id.clone());
        }
        if let Some(ref transact_time) = self.transact_time {
            msg.set_field(60, transact_time.clone());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: OrderEntryAckGrp (2430) should be added via msg.groups
        msg
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_mass_status_request_round_trip() {
        let req = OrderMassStatusRequest {
            mass_status_req_id: "STATUS001".to_string(),
            mass_status_req_type: MassStatusReqType::StatusForOrdersForSecurity,
            cl_ord_id: Some("ORD123".to_string()),
            account: Some("ACCT001".to_string()),
            trading_session_id: None,
            symbol: Some("AAPL".to_string()),
            underlying_symbol: None,
            side: Some(Side::Buy),
        };

        let raw = req.to_raw();
        let parsed = OrderMassStatusRequest::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed, req);
    }

    #[test]
    fn test_order_mass_action_request_round_trip() {
        let req = OrderMassActionRequest {
            cl_ord_id: "ACTION001".to_string(),
            mass_action_type: MassActionType::SuspendOrders,
            mass_action_scope: Some(1),
            market_id: Some("XNAS".to_string()),
            market_segment_id: None,
            trading_session_id: None,
            symbol: Some("MSFT".to_string()),
            security_type: None,
            side: Some(Side::Sell),
            transact_time: "20251008-12:00:00.000".to_string(),
            text: Some("Suspend all MSFT sell orders".to_string()),
        };

        let raw = req.to_raw();
        let parsed = OrderMassActionRequest::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed, req);
    }

    #[test]
    fn test_order_mass_action_report_round_trip() {
        let report = OrderMassActionReport {
            cl_ord_id: Some("ACTION001".to_string()),
            mass_action_report_id: "REPORT001".to_string(),
            mass_action_type: Some(MassActionType::SuspendOrders),
            mass_action_scope: Some(1),
            mass_action_response: MassActionResponse::Accepted,
            mass_action_reject_reason: None,
            total_affected_orders: Some(25),
            market_id: Some("XNAS".to_string()),
            market_segment_id: None,
            trading_session_id: None,
            symbol: Some("MSFT".to_string()),
            security_type: None,
            side: Some(Side::Sell),
            transact_time: "20251008-12:00:01.000".to_string(),
            text: Some("25 orders suspended".to_string()),
        };

        let raw = report.to_raw();
        let parsed = OrderMassActionReport::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed, report);
    }

    #[test]
    fn test_mass_order_round_trip() {
        let order = MassOrder {
            mass_order_id: "MASS001".to_string(),
            order_entry_action: OrderEntryAction::Add,
            sender_comp_id: Some("TRADER1".to_string()),
            target_comp_id: Some("MARKET1".to_string()),
            order_response_level: Some(OrderResponseLevel::AckEachOrder),
            transact_time: Some("20251008-12:00:00.000".to_string()),
            text: Some("Bulk order submission".to_string()),
        };

        let raw = order.to_raw();
        let parsed = MassOrder::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed, order);
    }

    #[test]
    fn test_mass_order_ack_round_trip() {
        let ack = MassOrderAck {
            mass_order_id: "MASS001".to_string(),
            order_entry_action: Some(OrderEntryAction::Add),
            order_response_level: Some(OrderResponseLevel::AckEachOrder),
            sender_comp_id: Some("MARKET1".to_string()),
            target_comp_id: Some("TRADER1".to_string()),
            transact_time: Some("20251008-12:00:01.000".to_string()),
            text: Some("All orders accepted".to_string()),
        };

        let raw = ack.to_raw();
        let parsed = MassOrderAck::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed, ack);
    }
}
