//! Market Data Messages
//!
//! This module implements FIX 5.0 SP2 Market Data messages (Pre-Trade category),
//! which allow market participants to request and receive market data snapshots and updates.
//!
//! ## Message Types
//! - MarketDataRequest (V): Request for market data
//! - MarketDataSnapshotFullRefresh (W): Full snapshot of current market data
//! - MarketDataRequestReject (Y): Rejection of a market data request
//!
//! ## Use Cases
//! - Real-time market data subscriptions
//! - Snapshot requests for current market state
//! - Level 1 and Level 2 (market depth) data
//! - Bid/Offer/Trade data streaming

use crate::parser::{RawFixMessage, FixParseError};
use crate::types::{MDReqRejReason, MDUpdateType, SubscriptionRequestType};
use serde::{Deserialize, Serialize};

// ============================================================================
// MarketDataRequest (MsgType = V)
// ============================================================================

/// MarketDataRequest (V) - Request for market data
///
/// Used to subscribe to or request a snapshot of market data for one or more instruments.
/// Supports both snapshot and subscription-based (streaming) market data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketDataRequest {
    /// MDReqID (Tag 262) - Required - Unique identifier for this market data request
    pub md_req_id: String,

    /// SubscriptionRequestType (Tag 263) - Required - Type of request (Snapshot/Subscribe/Unsubscribe)
    pub subscription_request_type: SubscriptionRequestType,

    /// MarketDepth (Tag 264) - Required - Depth of market data (0=full book, 1=top of book, N=N levels)
    pub market_depth: u32,

    /// MDUpdateType (Tag 265) - Type of market data update (FullRefresh/Incremental)
    pub md_update_type: Option<MDUpdateType>,

    /// AggregatedBook (Tag 266) - Whether book is aggregated by price level
    pub aggregated_book: Option<bool>,

    /// OpenCloseSettleFlag (Tag 286) - Which prices to return
    pub open_close_settle_flag: Option<String>,

    /// Scope (Tag 546) - Scope of the request (Local/National/Global)
    pub scope: Option<String>,

    /// MDImplicitDelete (Tag 547) - Whether to implicitly delete stale entries
    pub md_implicit_delete: Option<bool>,

    // Note: NoRelatedSym(146) and NoMDEntryTypes(267) repeating groups
    // are accessed via RawFixMessage.groups
}

impl MarketDataRequest {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(MarketDataRequest {
            md_req_id: raw.get_field(262)
                .ok_or_else(|| FixParseError::MissingRequiredField(262))?
                .to_string(),
            subscription_request_type: raw.get_field(263)
                .and_then(|s| s.chars().next())
                .and_then(SubscriptionRequestType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(263))?,
            market_depth: raw.get_field(264)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(264))?,
            md_update_type: raw.get_field(265)
                .and_then(|s| s.chars().next())
                .and_then(MDUpdateType::from_fix),
            aggregated_book: raw.get_field(266).map(|s| s == "Y"),
            open_close_settle_flag: raw.get_field(286).map(|s| s.to_string()),
            scope: raw.get_field(546).map(|s| s.to_string()),
            md_implicit_delete: raw.get_field(547).map(|s| s == "Y"),
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "V".to_string());

        // Required fields
        msg.set_field(262, self.md_req_id.clone());
        msg.set_field(263, self.subscription_request_type.to_fix().to_string());
        msg.set_field(264, self.market_depth.to_string());

        // Optional fields
        if let Some(update_type) = self.md_update_type {
            msg.set_field(265, update_type.to_fix().to_string());
        }
        if let Some(aggregated) = self.aggregated_book {
            msg.set_field(266, if aggregated { "Y" } else { "N" }.to_string());
        }
        if let Some(ref flag) = self.open_close_settle_flag {
            msg.set_field(286, flag.clone());
        }
        if let Some(ref scope) = self.scope {
            msg.set_field(546, scope.clone());
        }
        if let Some(implicit_delete) = self.md_implicit_delete {
            msg.set_field(547, if implicit_delete { "Y" } else { "N" }.to_string());
        }

        msg
    }
}

// ============================================================================
// MarketDataSnapshotFullRefresh (MsgType = W)
// ============================================================================

/// MarketDataSnapshotFullRefresh (W) - Full snapshot of current market data
///
/// Sent as a response to MarketDataRequest or as an unsolicited update.
/// Contains a complete snapshot of current market data for an instrument.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketDataSnapshotFullRefresh {
    /// MDReqID (Tag 262) - Conditionally required - Request ID from original MarketDataRequest
    pub md_req_id: Option<String>,

    // Instrument identification
    /// Symbol (Tag 55)
    pub symbol: Option<String>,

    /// SecurityID (Tag 48)
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22)
    pub security_id_source: Option<String>,

    /// SecurityType (Tag 167)
    pub security_type: Option<String>,

    // Optional fields
    /// FinancialStatus (Tag 291) - Financial status indicator
    pub financial_status: Option<String>,

    /// CorporateAction (Tag 292) - Corporate action indicator
    pub corporate_action: Option<String>,

    /// NetChgPrevDay (Tag 451) - Net change from previous day
    pub net_chg_prev_day: Option<f64>,

    /// MDStreamID (Tag 1500) - Identifier for the market data stream
    pub md_stream_id: Option<String>,

    /// MarketID (Tag 1301) - Market identifier
    pub market_id: Option<String>,

    /// MarketSegmentID (Tag 1300) - Market segment identifier
    pub market_segment_id: Option<String>,

    /// TradingSessionID (Tag 336)
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (Tag 625)
    pub trading_session_sub_id: Option<String>,

    /// ClearingBusinessDate (Tag 715)
    pub clearing_business_date: Option<String>,

    // Note: NoMDEntries(268) repeating group accessed via RawFixMessage.groups
}

impl MarketDataSnapshotFullRefresh {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(MarketDataSnapshotFullRefresh {
            md_req_id: raw.get_field(262).map(|s| s.to_string()),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            security_type: raw.get_field(167).map(|s| s.to_string()),
            financial_status: raw.get_field(291).map(|s| s.to_string()),
            corporate_action: raw.get_field(292).map(|s| s.to_string()),
            net_chg_prev_day: raw.get_field(451).and_then(|s| s.parse().ok()),
            md_stream_id: raw.get_field(1500).map(|s| s.to_string()),
            market_id: raw.get_field(1301).map(|s| s.to_string()),
            market_segment_id: raw.get_field(1300).map(|s| s.to_string()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            trading_session_sub_id: raw.get_field(625).map(|s| s.to_string()),
            clearing_business_date: raw.get_field(715).map(|s| s.to_string()),
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "W".to_string());

        // Optional fields (note: at minimum need symbol or security ID to identify instrument)
        if let Some(ref req_id) = self.md_req_id {
            msg.set_field(262, req_id.clone());
        }
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(ref sec_id) = self.security_id {
            msg.set_field(48, sec_id.clone());
        }
        if let Some(ref src) = self.security_id_source {
            msg.set_field(22, src.clone());
        }
        if let Some(ref sec_type) = self.security_type {
            msg.set_field(167, sec_type.clone());
        }
        if let Some(ref status) = self.financial_status {
            msg.set_field(291, status.clone());
        }
        if let Some(ref action) = self.corporate_action {
            msg.set_field(292, action.clone());
        }
        if let Some(chg) = self.net_chg_prev_day {
            msg.set_field(451, chg.to_string());
        }
        if let Some(ref stream_id) = self.md_stream_id {
            msg.set_field(1500, stream_id.clone());
        }
        if let Some(ref mkt_id) = self.market_id {
            msg.set_field(1301, mkt_id.clone());
        }
        if let Some(ref seg_id) = self.market_segment_id {
            msg.set_field(1300, seg_id.clone());
        }
        if let Some(ref sess_id) = self.trading_session_id {
            msg.set_field(336, sess_id.clone());
        }
        if let Some(ref sub_id) = self.trading_session_sub_id {
            msg.set_field(625, sub_id.clone());
        }
        if let Some(ref date) = self.clearing_business_date {
            msg.set_field(715, date.clone());
        }

        msg
    }
}

// ============================================================================
// MarketDataRequestReject (MsgType = Y)
// ============================================================================

/// MarketDataRequestReject (Y) - Rejection of a market data request
///
/// Sent in response to a MarketDataRequest to indicate that the request was rejected.
/// Includes reason code and optional text explaining the rejection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarketDataRequestReject {
    /// MDReqID (Tag 262) - Required - Request ID from original MarketDataRequest
    pub md_req_id: String,

    /// MDReqRejReason (Tag 281) - Reason for rejection
    pub md_req_rej_reason: Option<MDReqRejReason>,

    /// Text (Tag 58) - Free-form text describing reason for rejection
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354)
    pub encoded_text_len: Option<u32>,

    /// EncodedText (Tag 355)
    pub encoded_text: Option<Vec<u8>>,

    // Note: NoAltMDSource(816) repeating group can be accessed via RawFixMessage.groups
}

impl MarketDataRequestReject {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(MarketDataRequestReject {
            md_req_id: raw.get_field(262)
                .ok_or_else(|| FixParseError::MissingRequiredField(262))?
                .to_string(),
            md_req_rej_reason: raw.get_field(281)
                .and_then(MDReqRejReason::from_fix),
            text: raw.get_field(58).map(|s| s.to_string()),
            encoded_text_len: raw.get_field(354).and_then(|s| s.parse().ok()),
            encoded_text: None, // Binary data handling TBD
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "Y".to_string());

        // Required fields
        msg.set_field(262, self.md_req_id.clone());

        // Optional fields
        if let Some(reason) = self.md_req_rej_reason {
            msg.set_field(281, reason.to_fix().to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }
        if let Some(len) = self.encoded_text_len {
            msg.set_field(354, len.to_string());
        }

        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_data_request_round_trip() {
        let req = MarketDataRequest {
            md_req_id: "MDREQ123".to_string(),
            subscription_request_type: SubscriptionRequestType::SnapshotPlusUpdates,
            market_depth: 1, // Top of book
            md_update_type: Some(MDUpdateType::IncrementalRefresh),
            aggregated_book: Some(true),
            open_close_settle_flag: None,
            scope: Some("1".to_string()), // Local
            md_implicit_delete: Some(false),
        };

        let raw = req.to_raw();
        let parsed = MarketDataRequest::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.md_req_id, "MDREQ123");
        assert_eq!(parsed.subscription_request_type, SubscriptionRequestType::SnapshotPlusUpdates);
        assert_eq!(parsed.market_depth, 1);
        assert_eq!(parsed.md_update_type, Some(MDUpdateType::IncrementalRefresh));
        assert_eq!(parsed.aggregated_book, Some(true));
    }

    #[test]
    fn test_market_data_snapshot_round_trip() {
        let snapshot = MarketDataSnapshotFullRefresh {
            md_req_id: Some("MDREQ123".to_string()),
            symbol: Some("AAPL".to_string()),
            security_id: Some("037833100".to_string()),
            security_id_source: Some("1".to_string()),
            security_type: Some("CS".to_string()),
            financial_status: Some("1".to_string()),
            corporate_action: None,
            net_chg_prev_day: Some(2.50),
            md_stream_id: Some("STREAM001".to_string()),
            market_id: Some("XNAS".to_string()),
            market_segment_id: Some("MAIN".to_string()),
            trading_session_id: Some("CORE".to_string()),
            trading_session_sub_id: None,
            clearing_business_date: Some("20250110".to_string()),
        };

        let raw = snapshot.to_raw();
        let parsed = MarketDataSnapshotFullRefresh::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.symbol, Some("AAPL".to_string()));
        assert_eq!(parsed.security_id, Some("037833100".to_string()));
        assert_eq!(parsed.net_chg_prev_day, Some(2.50));
        assert_eq!(parsed.market_id, Some("XNAS".to_string()));
    }

    #[test]
    fn test_market_data_request_reject_round_trip() {
        let reject = MarketDataRequestReject {
            md_req_id: "MDREQ456".to_string(),
            md_req_rej_reason: Some(MDReqRejReason::UnknownSymbol),
            text: Some("Symbol not found".to_string()),
            encoded_text_len: None,
            encoded_text: None,
        };

        let raw = reject.to_raw();
        let parsed = MarketDataRequestReject::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.md_req_id, "MDREQ456");
        assert_eq!(parsed.md_req_rej_reason, Some(MDReqRejReason::UnknownSymbol));
        assert_eq!(parsed.text, Some("Symbol not found".to_string()));
    }
}
