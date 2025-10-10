//! Program Trading / List Trading message types
//!
//! This module implements FIX 5.0 SP2 Program Trading messages, which allow
//! institutions to submit, execute, and monitor baskets of orders as atomic units.
//!
//! ## Message Types
//! - NewOrderList (E): Submit multiple orders as a list/basket
//! - ListStatus (N): Report status of list orders
//! - ListExecute (L): Execute a previously submitted list
//! - ListCancelRequest (K): Cancel an entire list
//! - ListStatusRequest (M): Request status of a list
//! - BidRequest (k): Request bids for a program trade
//! - BidResponse (l): Provide bid responses
//! - ListStrikePrice (m): Exchange strike prices for principal trades

use crate::parser::{RawFixMessage, FixParseError};
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// NewOrderList (MsgType = E)
// ============================================================================

/// NewOrderList (E) - Submit multiple orders as a list/basket
///
/// This is the foundational message for Program Trading, allowing submission
/// of multiple orders as a single atomic unit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewOrderList {
    /// ListID (Tag 66) - Required - Unique identifier for this list
    pub list_id: String,

    /// BidID (Tag 390) - Conditionally required for disclosed bid
    pub bid_id: Option<String>,

    /// ClientBidID (Tag 391)
    pub client_bid_id: Option<String>,

    /// ProgRptReqs (Tag 414)
    pub prog_rpt_reqs: Option<ProgRptReqs>,

    /// BidType (Tag 394) - Required - Type of bid
    pub bid_type: BidType,

    /// ProgPeriodInterval (Tag 415)
    pub prog_period_interval: Option<u32>,

    /// CancellationRights (Tag 480)
    pub cancellation_rights: Option<char>,

    /// MoneyLaunderingStatus (Tag 481)
    pub money_laundering_status: Option<char>,

    /// RegistID (Tag 513)
    pub regist_id: Option<String>,

    /// ListExecInstType (Tag 433)
    pub list_exec_inst_type: Option<ListExecInstType>,

    /// ListExecInst (Tag 69)
    pub list_exec_inst: Option<String>,

    /// EncodedListExecInstLen (Tag 352)
    pub encoded_list_exec_inst_len: Option<u32>,

    /// EncodedListExecInst (Tag 353)
    pub encoded_list_exec_inst: Option<Vec<u8>>,

    /// AllowableOneSidednessPct (Tag 765)
    pub allowable_one_sidedness_pct: Option<f64>,

    /// AllowableOneSidednessValue (Tag 766)
    pub allowable_one_sidedness_value: Option<f64>,

    /// AllowableOneSidednessCurr (Tag 767)
    pub allowable_one_sidedness_curr: Option<String>,

    /// TotNoOrders (Tag 68) - Required - Total number of orders
    pub tot_no_orders: u32,

    /// LastFragment (Tag 893)
    pub last_fragment: Option<bool>,

    // Note: ListOrdGrp - Repeating group of orders (Tag 73 = NoOrders)
    // This is stored in the RawFixMessage.groups field with key 73
    // Each entry contains the order details with nested Parties and PreAllocGrp
}

impl NewOrderList {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(NewOrderList {
            list_id: raw.get_field(66)
                .ok_or_else(|| FixParseError::MissingRequiredField(66))?
                .to_string(),
            bid_id: raw.get_field(390).map(|s| s.to_string()),
            client_bid_id: raw.get_field(391).map(|s| s.to_string()),
            prog_rpt_reqs: raw.get_field(414)
                .and_then(|s| s.chars().next())
                .and_then(ProgRptReqs::from_fix),
            bid_type: raw.get_field(394)
                .and_then(|s| s.chars().next())
                .and_then(BidType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(394))?,
            prog_period_interval: raw.get_field(415)
                .and_then(|s| s.parse().ok()),
            cancellation_rights: raw.get_field(480)
                .and_then(|s| s.chars().next()),
            money_laundering_status: raw.get_field(481)
                .and_then(|s| s.chars().next()),
            regist_id: raw.get_field(513).map(|s| s.to_string()),
            list_exec_inst_type: raw.get_field(433)
                .and_then(|s| s.chars().next())
                .and_then(ListExecInstType::from_fix),
            list_exec_inst: raw.get_field(69).map(|s| s.to_string()),
            encoded_list_exec_inst_len: raw.get_field(352)
                .and_then(|s| s.parse().ok()),
            encoded_list_exec_inst: None, // TODO: handle binary data
            allowable_one_sidedness_pct: raw.get_field(765)
                .and_then(|s| s.parse().ok()),
            allowable_one_sidedness_value: raw.get_field(766)
                .and_then(|s| s.parse().ok()),
            allowable_one_sidedness_curr: raw.get_field(767).map(|s| s.to_string()),
            tot_no_orders: raw.get_field(68)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(68))?,
            last_fragment: raw.get_field(893)
                .map(|s| s == "Y"),
        })
        // Note: ListOrdGrp (73) is accessed via raw.groups.get(&73)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "E".to_string());

        // Required fields
        msg.set_field(66, self.list_id.clone());
        msg.set_field(394, self.bid_type.to_fix().to_string());
        msg.set_field(68, self.tot_no_orders.to_string());

        // Optional fields
        if let Some(ref bid_id) = self.bid_id {
            msg.set_field(390, bid_id.clone());
        }
        if let Some(ref client_bid_id) = self.client_bid_id {
            msg.set_field(391, client_bid_id.clone());
        }
        if let Some(prog_rpt_reqs) = self.prog_rpt_reqs {
            msg.set_field(414, prog_rpt_reqs.to_fix().to_string());
        }
        if let Some(interval) = self.prog_period_interval {
            msg.set_field(415, interval.to_string());
        }
        if let Some(rights) = self.cancellation_rights {
            msg.set_field(480, rights.to_string());
        }
        if let Some(status) = self.money_laundering_status {
            msg.set_field(481, status.to_string());
        }
        if let Some(ref regist_id) = self.regist_id {
            msg.set_field(513, regist_id.clone());
        }
        if let Some(inst_type) = self.list_exec_inst_type {
            msg.set_field(433, inst_type.to_fix().to_string());
        }
        if let Some(ref inst) = self.list_exec_inst {
            msg.set_field(69, inst.clone());
        }
        if let Some(pct) = self.allowable_one_sidedness_pct {
            msg.set_field(765, pct.to_string());
        }
        if let Some(value) = self.allowable_one_sidedness_value {
            msg.set_field(766, value.to_string());
        }
        if let Some(ref curr) = self.allowable_one_sidedness_curr {
            msg.set_field(767, curr.clone());
        }
        if let Some(last_frag) = self.last_fragment {
            msg.set_field(893, if last_frag { "Y".to_string() } else { "N".to_string() });
        }

        // Note: ListOrdGrp (73) should be added via msg.groups
        msg
    }
}

// ============================================================================
// ListStatus (MsgType = N)
// ============================================================================

/// ListStatus (N) - Report status of list orders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListStatus {
    /// ListID (Tag 66) - Required
    pub list_id: String,

    /// ListStatusType (Tag 429) - Required
    pub list_status_type: ListStatusType,

    /// NoRpts (Tag 82) - Required
    pub no_rpts: u32,

    /// ListOrderStatus (Tag 431) - Required
    pub list_order_status: ListOrderStatus,

    /// RptSeq (Tag 83) - Required
    pub rpt_seq: u32,

    /// ListStatusText (Tag 444)
    pub list_status_text: Option<String>,

    /// EncodedListStatusTextLen (Tag 445)
    pub encoded_list_status_text_len: Option<u32>,

    /// EncodedListStatusText (Tag 446)
    pub encoded_list_status_text: Option<Vec<u8>>,

    /// TransactTime (Tag 60)
    pub transact_time: Option<String>,

    /// TotNoOrders (Tag 68) - Required
    pub tot_no_orders: u32,

    /// LastFragment (Tag 893)
    pub last_fragment: Option<bool>,

    // Note: OrdListStatGrp - Repeating group (Tag 73 = NoOrders)
    // Stored in RawFixMessage.groups with key 73
}

impl ListStatus {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(ListStatus {
            list_id: raw.get_field(66)
                .ok_or_else(|| FixParseError::MissingRequiredField(66))?
                .to_string(),
            list_status_type: raw.get_field(429)
                .and_then(|s| s.chars().next())
                .and_then(ListStatusType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(429))?,
            no_rpts: raw.get_field(82)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(82))?,
            list_order_status: raw.get_field(431)
                .and_then(|s| s.chars().next())
                .and_then(ListOrderStatus::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(431))?,
            rpt_seq: raw.get_field(83)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(83))?,
            list_status_text: raw.get_field(444).map(|s| s.to_string()),
            encoded_list_status_text_len: raw.get_field(445)
                .and_then(|s| s.parse().ok()),
            encoded_list_status_text: None,
            transact_time: raw.get_field(60).map(|s| s.to_string()),
            tot_no_orders: raw.get_field(68)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(68))?,
            last_fragment: raw.get_field(893).map(|s| s == "Y"),
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(35, "N".to_string());
        msg.set_field(66, self.list_id.clone());
        msg.set_field(429, self.list_status_type.to_fix().to_string());
        msg.set_field(82, self.no_rpts.to_string());
        msg.set_field(431, self.list_order_status.to_fix().to_string());
        msg.set_field(83, self.rpt_seq.to_string());
        msg.set_field(68, self.tot_no_orders.to_string());

        if let Some(ref text) = self.list_status_text {
            msg.set_field(444, text.clone());
        }
        if let Some(ref time) = self.transact_time {
            msg.set_field(60, time.clone());
        }
        if let Some(last_frag) = self.last_fragment {
            msg.set_field(893, if last_frag { "Y".to_string() } else { "N".to_string() });
        }

        msg
    }
}

// ============================================================================
// Simpler Messages
// ============================================================================

/// ListExecute (L) - Execute a previously submitted list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListExecute {
    /// ListID (Tag 66) - Required
    pub list_id: String,

    /// ClientBidID (Tag 391)
    pub client_bid_id: Option<String>,

    /// BidID (Tag 390)
    pub bid_id: Option<String>,

    /// TransactTime (Tag 60) - Required
    pub transact_time: String,

    /// Text (Tag 58)
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354)
    pub encoded_text_len: Option<u32>,

    /// EncodedText (Tag 355)
    pub encoded_text: Option<Vec<u8>>,
}

/// ListCancelRequest (K) - Cancel an entire list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListCancelRequest {
    /// ListID (Tag 66) - Required
    pub list_id: String,

    /// TransactTime (Tag 60) - Required
    pub transact_time: String,

    /// TradeOriginationDate (Tag 229)
    pub trade_origination_date: Option<String>,

    /// TradeDate (Tag 75)
    pub trade_date: Option<String>,

    /// Text (Tag 58)
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354)
    pub encoded_text_len: Option<u32>,

    /// EncodedText (Tag 355)
    pub encoded_text: Option<Vec<u8>>,
}

/// ListStatusRequest (M) - Request status of a list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListStatusRequest {
    /// ListID (Tag 66) - Required
    pub list_id: String,

    /// Text (Tag 58)
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354)
    pub encoded_text_len: Option<u32>,

    /// EncodedText (Tag 355)
    pub encoded_text: Option<Vec<u8>>,
}

// Note: BidRequest, BidResponse, and ListStrikePrice are more complex
// and would be implemented with full field mappings following the same pattern

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_list_round_trip() {
        let msg = NewOrderList {
            list_id: "LIST123".to_string(),
            bid_id: Some("BID456".to_string()),
            client_bid_id: None,
            prog_rpt_reqs: Some(ProgRptReqs::BuySideRequests),
            bid_type: BidType::Disclosed,
            prog_period_interval: Some(30),
            cancellation_rights: None,
            money_laundering_status: Some('Y'),
            regist_id: None,
            list_exec_inst_type: Some(ListExecInstType::Immediate),
            list_exec_inst: None,
            encoded_list_exec_inst_len: None,
            encoded_list_exec_inst: None,
            allowable_one_sidedness_pct: Some(5.0),
            allowable_one_sidedness_value: None,
            allowable_one_sidedness_curr: None,
            tot_no_orders: 10,
            last_fragment: Some(true),
        };

        let raw = msg.to_raw();
        let parsed = NewOrderList::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.list_id, "LIST123");
        assert_eq!(parsed.bid_id, Some("BID456".to_string()));
        assert_eq!(parsed.bid_type, BidType::Disclosed);
        assert_eq!(parsed.tot_no_orders, 10);
    }

    #[test]
    fn test_list_status_round_trip() {
        let msg = ListStatus {
            list_id: "LIST123".to_string(),
            list_status_type: ListStatusType::Ack,
            no_rpts: 1,
            list_order_status: ListOrderStatus::InBiddingProcess,
            rpt_seq: 1,
            list_status_text: Some("Processing".to_string()),
            encoded_list_status_text_len: None,
            encoded_list_status_text: None,
            transact_time: Some("20250108-12:00:00".to_string()),
            tot_no_orders: 10,
            last_fragment: Some(true),
        };

        let raw = msg.to_raw();
        let parsed = ListStatus::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.list_id, "LIST123");
        assert_eq!(parsed.list_status_type, ListStatusType::Ack);
        assert_eq!(parsed.tot_no_orders, 10);
    }
}
