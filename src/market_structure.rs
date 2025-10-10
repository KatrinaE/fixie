//! Market Structure Reference Data Messages
//!
//! This module implements FIX 5.0 SP2 Market Structure messages (Pre-Trade category),
//! which provide information about trading sessions and market definitions.
//!
//! ## Message Types
//! - TradingSessionStatusRequest (g): Request for trading session status
//! - TradingSessionStatus (h): Status information about a trading session
//! - TradingSessionListRequest (BI): Request for list of trading sessions
//! - TradingSessionList (BJ): List of trading sessions
//! - TradingSessionListUpdateReport (BS): Updates to trading session list
//! - MarketDefinitionRequest (BT): Request for market definition
//! - MarketDefinition (BU): Market definition details
//! - MarketDefinitionUpdateReport (BV): Updates to market definition
//!
//! ## Use Cases
//! - Requesting and receiving trading session status information
//! - Discovering available trading sessions
//! - Understanding market structures and trading rules
//! - Monitoring session state transitions (pre-open, open, close, etc.)

use crate::parser::{RawFixMessage, FixParseError};
use crate::types::{TradSesStatus, TradSesMethod, TradSesMode, TradSesStatusRejReason, SubscriptionRequestType};
use serde::{Deserialize, Serialize};

// ============================================================================
// TradingSessionStatusRequest (MsgType = g)
// ============================================================================

/// TradingSessionStatusRequest (g) - Request information on the status of a market
///
/// Used to request information on the status of a trading session or market.
/// Can be used as a subscription to receive unsolicited updates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradingSessionStatusRequest {
    /// TradSesReqID (Tag 335) - Required - Unique identifier for this request
    pub trad_ses_req_id: String,

    /// SubscriptionRequestType (Tag 263) - Required - Type of request (Snapshot/Subscribe/Unsubscribe)
    pub subscription_request_type: SubscriptionRequestType,

    /// MarketID (Tag 1301) - Market for which status is requested
    pub market_id: Option<String>,

    /// MarketSegmentID (Tag 1300) - Market segment for which status is requested
    pub market_segment_id: Option<String>,

    /// TradingSessionID (Tag 336) - Trading session for which status is requested
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (Tag 625) - Trading session sub-identifier
    pub trading_session_sub_id: Option<String>,

    /// TradSesMethod (Tag 338) - Method of trading
    pub trad_ses_method: Option<TradSesMethod>,

    /// TradSesMode (Tag 339) - Trading session mode
    pub trad_ses_mode: Option<TradSesMode>,

    /// SecurityExchange (Tag 207) - Exchange (deprecated in FIX.5.0SP1, use MarketID instead)
    pub security_exchange: Option<String>,
}

impl TradingSessionStatusRequest {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(TradingSessionStatusRequest {
            trad_ses_req_id: raw.get_field(335)
                .ok_or_else(|| FixParseError::MissingRequiredField(335))?
                .to_string(),
            subscription_request_type: raw.get_field(263)
                .and_then(|s| s.chars().next())
                .and_then(SubscriptionRequestType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(263))?,
            market_id: raw.get_field(1301).map(|s| s.to_string()),
            market_segment_id: raw.get_field(1300).map(|s| s.to_string()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            trading_session_sub_id: raw.get_field(625).map(|s| s.to_string()),
            trad_ses_method: raw.get_field(338)
                .and_then(|s| s.chars().next())
                .and_then(TradSesMethod::from_fix),
            trad_ses_mode: raw.get_field(339)
                .and_then(|s| s.chars().next())
                .and_then(TradSesMode::from_fix),
            security_exchange: raw.get_field(207).map(|s| s.to_string()),
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "g".to_string());

        // Required fields
        msg.set_field(335, self.trad_ses_req_id.clone());
        msg.set_field(263, self.subscription_request_type.to_fix().to_string());

        // Optional fields
        if let Some(ref market_id) = self.market_id {
            msg.set_field(1301, market_id.clone());
        }
        if let Some(ref segment_id) = self.market_segment_id {
            msg.set_field(1300, segment_id.clone());
        }
        if let Some(ref session_id) = self.trading_session_id {
            msg.set_field(336, session_id.clone());
        }
        if let Some(ref sub_id) = self.trading_session_sub_id {
            msg.set_field(625, sub_id.clone());
        }
        if let Some(method) = self.trad_ses_method {
            msg.set_field(338, method.to_fix().to_string());
        }
        if let Some(mode) = self.trad_ses_mode {
            msg.set_field(339, mode.to_fix().to_string());
        }
        if let Some(ref exchange) = self.security_exchange {
            msg.set_field(207, exchange.clone());
        }

        msg
    }
}

// ============================================================================
// TradingSessionStatus (MsgType = h)
// ============================================================================

/// TradingSessionStatus (h) - Provides information on the status of a market
///
/// Sent in response to TradingSessionStatusRequest or as an unsolicited update
/// when subscribed. Contains current status and timing information for a trading session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradingSessionStatus {
    /// TradSesReqID (Tag 335) - Request ID if responding to a request
    pub trad_ses_req_id: Option<String>,

    /// TradingSessionID (Tag 336) - Required - Identifier for trading session
    pub trading_session_id: String,

    /// TradingSessionSubID (Tag 625) - Trading session sub-identifier
    pub trading_session_sub_id: Option<String>,

    /// TradSesMethod (Tag 338) - Method of trading
    pub trad_ses_method: Option<TradSesMethod>,

    /// TradSesMode (Tag 339) - Trading session mode
    pub trad_ses_mode: Option<TradSesMode>,

    /// UnsolicitedIndicator (Tag 325) - Whether message is unsolicited
    pub unsolicited_indicator: Option<bool>,

    /// TradSesStatus (Tag 340) - Required - Current status of the trading session
    pub trad_ses_status: TradSesStatus,

    /// TradSesStatusRejReason (Tag 567) - Reason for rejection if status is RequestRejected
    pub trad_ses_status_rej_reason: Option<TradSesStatusRejReason>,

    /// TradSesStartTime (Tag 341) - Time session is scheduled to start
    pub trad_ses_start_time: Option<String>,

    /// TradSesOpenTime (Tag 342) - Time session opened/will open
    pub trad_ses_open_time: Option<String>,

    /// TradSesPreCloseTime (Tag 343) - Time session pre-close period begins
    pub trad_ses_pre_close_time: Option<String>,

    /// TradSesCloseTime (Tag 344) - Time session closes/will close
    pub trad_ses_close_time: Option<String>,

    /// TradSesEndTime (Tag 345) - Time session is scheduled to end
    pub trad_ses_end_time: Option<String>,

    /// TotalVolumeTraded (Tag 387) - Total volume traded during session
    pub total_volume_traded: Option<f64>,

    /// Text (Tag 58) - Free-form text
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354) - Length of encoded text
    pub encoded_text_len: Option<u32>,

    /// EncodedText (Tag 355) - Encoded text
    pub encoded_text: Option<Vec<u8>>,

    /// MarketID (Tag 1301) - Market identifier
    pub market_id: Option<String>,

    /// MarketSegmentID (Tag 1300) - Market segment identifier
    pub market_segment_id: Option<String>,

    /// TradSesEvent (Tag 1368) - Trading session event
    pub trad_ses_event: Option<u32>,
}

impl TradingSessionStatus {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(TradingSessionStatus {
            trad_ses_req_id: raw.get_field(335).map(|s| s.to_string()),
            trading_session_id: raw.get_field(336)
                .ok_or_else(|| FixParseError::MissingRequiredField(336))?
                .to_string(),
            trading_session_sub_id: raw.get_field(625).map(|s| s.to_string()),
            trad_ses_method: raw.get_field(338)
                .and_then(|s| s.chars().next())
                .and_then(TradSesMethod::from_fix),
            trad_ses_mode: raw.get_field(339)
                .and_then(|s| s.chars().next())
                .and_then(TradSesMode::from_fix),
            unsolicited_indicator: raw.get_field(325).map(|s| s == "Y"),
            trad_ses_status: raw.get_field(340)
                .and_then(|s| s.chars().next())
                .and_then(TradSesStatus::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(340))?,
            trad_ses_status_rej_reason: raw.get_field(567)
                .and_then(TradSesStatusRejReason::from_fix),
            trad_ses_start_time: raw.get_field(341).map(|s| s.to_string()),
            trad_ses_open_time: raw.get_field(342).map(|s| s.to_string()),
            trad_ses_pre_close_time: raw.get_field(343).map(|s| s.to_string()),
            trad_ses_close_time: raw.get_field(344).map(|s| s.to_string()),
            trad_ses_end_time: raw.get_field(345).map(|s| s.to_string()),
            total_volume_traded: raw.get_field(387).and_then(|s| s.parse().ok()),
            text: raw.get_field(58).map(|s| s.to_string()),
            encoded_text_len: raw.get_field(354).and_then(|s| s.parse().ok()),
            encoded_text: None, // Binary data handling TBD
            market_id: raw.get_field(1301).map(|s| s.to_string()),
            market_segment_id: raw.get_field(1300).map(|s| s.to_string()),
            trad_ses_event: raw.get_field(1368).and_then(|s| s.parse().ok()),
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "h".to_string());

        // Required fields
        msg.set_field(336, self.trading_session_id.clone());
        msg.set_field(340, self.trad_ses_status.to_fix().to_string());

        // Optional fields
        if let Some(ref req_id) = self.trad_ses_req_id {
            msg.set_field(335, req_id.clone());
        }
        if let Some(ref sub_id) = self.trading_session_sub_id {
            msg.set_field(625, sub_id.clone());
        }
        if let Some(method) = self.trad_ses_method {
            msg.set_field(338, method.to_fix().to_string());
        }
        if let Some(mode) = self.trad_ses_mode {
            msg.set_field(339, mode.to_fix().to_string());
        }
        if let Some(unsol) = self.unsolicited_indicator {
            msg.set_field(325, if unsol { "Y" } else { "N" }.to_string());
        }
        if let Some(reason) = self.trad_ses_status_rej_reason {
            msg.set_field(567, reason.to_fix().to_string());
        }
        if let Some(ref start) = self.trad_ses_start_time {
            msg.set_field(341, start.clone());
        }
        if let Some(ref open) = self.trad_ses_open_time {
            msg.set_field(342, open.clone());
        }
        if let Some(ref pre_close) = self.trad_ses_pre_close_time {
            msg.set_field(343, pre_close.clone());
        }
        if let Some(ref close) = self.trad_ses_close_time {
            msg.set_field(344, close.clone());
        }
        if let Some(ref end) = self.trad_ses_end_time {
            msg.set_field(345, end.clone());
        }
        if let Some(volume) = self.total_volume_traded {
            msg.set_field(387, volume.to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }
        if let Some(len) = self.encoded_text_len {
            msg.set_field(354, len.to_string());
        }
        if let Some(ref market_id) = self.market_id {
            msg.set_field(1301, market_id.clone());
        }
        if let Some(ref segment_id) = self.market_segment_id {
            msg.set_field(1300, segment_id.clone());
        }
        if let Some(event) = self.trad_ses_event {
            msg.set_field(1368, event.to_string());
        }

        msg
    }
}

// ============================================================================
// TradingSessionListRequest (MsgType = BI)
// ============================================================================

/// TradingSessionListRequest (BI) - Request for list of trading sessions
///
/// Used to request a list of trading sessions available. The response is a
/// TradingSessionList message containing details about all matching sessions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradingSessionListRequest {
    /// TradSesReqID (Tag 335) - Required - Unique identifier for this request
    pub trad_ses_req_id: String,

    /// SubscriptionRequestType (Tag 263) - Required - Type of request (Snapshot/Subscribe/Unsubscribe)
    pub subscription_request_type: SubscriptionRequestType,

    /// TradingSessionID (Tag 336) - Specific trading session to request
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (Tag 625) - Trading session sub-identifier
    pub trading_session_sub_id: Option<String>,

    /// SecurityExchange (Tag 207) - Exchange identifier
    pub security_exchange: Option<String>,

    /// TradSesMethod (Tag 338) - Method of trading
    pub trad_ses_method: Option<TradSesMethod>,

    /// TradSesMode (Tag 339) - Trading session mode
    pub trad_ses_mode: Option<TradSesMode>,
}

impl TradingSessionListRequest {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(TradingSessionListRequest {
            trad_ses_req_id: raw.get_field(335)
                .ok_or_else(|| FixParseError::MissingRequiredField(335))?
                .to_string(),
            subscription_request_type: raw.get_field(263)
                .and_then(|s| s.chars().next())
                .and_then(SubscriptionRequestType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(263))?,
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            trading_session_sub_id: raw.get_field(625).map(|s| s.to_string()),
            security_exchange: raw.get_field(207).map(|s| s.to_string()),
            trad_ses_method: raw.get_field(338)
                .and_then(|s| s.chars().next())
                .and_then(TradSesMethod::from_fix),
            trad_ses_mode: raw.get_field(339)
                .and_then(|s| s.chars().next())
                .and_then(TradSesMode::from_fix),
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "BI".to_string());

        // Required fields
        msg.set_field(335, self.trad_ses_req_id.clone());
        msg.set_field(263, self.subscription_request_type.to_fix().to_string());

        // Optional fields
        if let Some(ref session_id) = self.trading_session_id {
            msg.set_field(336, session_id.clone());
        }
        if let Some(ref sub_id) = self.trading_session_sub_id {
            msg.set_field(625, sub_id.clone());
        }
        if let Some(ref exchange) = self.security_exchange {
            msg.set_field(207, exchange.clone());
        }
        if let Some(method) = self.trad_ses_method {
            msg.set_field(338, method.to_fix().to_string());
        }
        if let Some(mode) = self.trad_ses_mode {
            msg.set_field(339, mode.to_fix().to_string());
        }

        msg
    }
}

// ============================================================================
// TradingSessionList (MsgType = BJ)
// ============================================================================

/// TradingSessionList (BJ) - List of trading sessions
///
/// Sent in response to TradingSessionListRequest. Contains information about
/// one or more trading sessions. The TrdSessLstGrp repeating group (NoTradingSessions=386)
/// contains the detailed session information and is accessed via RawFixMessage.groups.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradingSessionList {
    /// TradSesReqID (Tag 335) - Request ID from the TradingSessionListRequest
    pub trad_ses_req_id: Option<String>,

    // Note: NoTradingSessions (386) and TrdSessLstGrp repeating group
    // are accessed via RawFixMessage.groups
}

impl TradingSessionList {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(TradingSessionList {
            trad_ses_req_id: raw.get_field(335).map(|s| s.to_string()),
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "BJ".to_string());

        // Optional fields
        if let Some(ref req_id) = self.trad_ses_req_id {
            msg.set_field(335, req_id.clone());
        }

        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trading_session_status_request_round_trip() {
        let req = TradingSessionStatusRequest {
            trad_ses_req_id: "TSREQ001".to_string(),
            subscription_request_type: SubscriptionRequestType::Snapshot,
            market_id: Some("XNAS".to_string()),
            market_segment_id: Some("MAIN".to_string()),
            trading_session_id: Some("CORE".to_string()),
            trading_session_sub_id: None,
            trad_ses_method: Some(TradSesMethod::Electronic),
            trad_ses_mode: Some(TradSesMode::Production),
            security_exchange: None,
        };

        let raw = req.to_raw();
        let parsed = TradingSessionStatusRequest::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.trad_ses_req_id, "TSREQ001");
        assert_eq!(parsed.subscription_request_type, SubscriptionRequestType::Snapshot);
        assert_eq!(parsed.market_id, Some("XNAS".to_string()));
        assert_eq!(parsed.market_segment_id, Some("MAIN".to_string()));
        assert_eq!(parsed.trading_session_id, Some("CORE".to_string()));
        assert_eq!(parsed.trad_ses_method, Some(TradSesMethod::Electronic));
        assert_eq!(parsed.trad_ses_mode, Some(TradSesMode::Production));
    }

    #[test]
    fn test_trading_session_status_round_trip() {
        let status = TradingSessionStatus {
            trad_ses_req_id: Some("TSREQ001".to_string()),
            trading_session_id: "CORE".to_string(),
            trading_session_sub_id: None,
            trad_ses_method: Some(TradSesMethod::Electronic),
            trad_ses_mode: Some(TradSesMode::Production),
            unsolicited_indicator: Some(false),
            trad_ses_status: TradSesStatus::Open,
            trad_ses_status_rej_reason: None,
            trad_ses_start_time: Some("20251010-09:00:00".to_string()),
            trad_ses_open_time: Some("20251010-09:30:00".to_string()),
            trad_ses_pre_close_time: Some("20251010-15:50:00".to_string()),
            trad_ses_close_time: Some("20251010-16:00:00".to_string()),
            trad_ses_end_time: Some("20251010-16:30:00".to_string()),
            total_volume_traded: Some(1234567.89),
            text: Some("Market is open".to_string()),
            encoded_text_len: None,
            encoded_text: None,
            market_id: Some("XNAS".to_string()),
            market_segment_id: Some("MAIN".to_string()),
            trad_ses_event: None,
        };

        let raw = status.to_raw();
        let parsed = TradingSessionStatus::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.trad_ses_req_id, Some("TSREQ001".to_string()));
        assert_eq!(parsed.trading_session_id, "CORE");
        assert_eq!(parsed.trad_ses_method, Some(TradSesMethod::Electronic));
        assert_eq!(parsed.trad_ses_mode, Some(TradSesMode::Production));
        assert_eq!(parsed.unsolicited_indicator, Some(false));
        assert_eq!(parsed.trad_ses_status, TradSesStatus::Open);
        assert_eq!(parsed.trad_ses_start_time, Some("20251010-09:00:00".to_string()));
        assert_eq!(parsed.total_volume_traded, Some(1234567.89));
        assert_eq!(parsed.text, Some("Market is open".to_string()));
        assert_eq!(parsed.market_id, Some("XNAS".to_string()));
    }

    #[test]
    fn test_trading_session_status_rejection() {
        let status = TradingSessionStatus {
            trad_ses_req_id: Some("TSREQ002".to_string()),
            trading_session_id: "INVALID".to_string(),
            trading_session_sub_id: None,
            trad_ses_method: None,
            trad_ses_mode: None,
            unsolicited_indicator: Some(false),
            trad_ses_status: TradSesStatus::RequestRejected,
            trad_ses_status_rej_reason: Some(TradSesStatusRejReason::UnknownOrInvalidTradingSessionID),
            trad_ses_start_time: None,
            trad_ses_open_time: None,
            trad_ses_pre_close_time: None,
            trad_ses_close_time: None,
            trad_ses_end_time: None,
            total_volume_traded: None,
            text: Some("Unknown trading session ID".to_string()),
            encoded_text_len: None,
            encoded_text: None,
            market_id: None,
            market_segment_id: None,
            trad_ses_event: None,
        };

        let raw = status.to_raw();
        let parsed = TradingSessionStatus::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.trad_ses_status, TradSesStatus::RequestRejected);
        assert_eq!(parsed.trad_ses_status_rej_reason, Some(TradSesStatusRejReason::UnknownOrInvalidTradingSessionID));
        assert_eq!(parsed.text, Some("Unknown trading session ID".to_string()));
    }

    #[test]
    fn test_trading_session_list_request_round_trip() {
        let req = TradingSessionListRequest {
            trad_ses_req_id: "TSLISTREQ001".to_string(),
            subscription_request_type: SubscriptionRequestType::Snapshot,
            trading_session_id: Some("CORE".to_string()),
            trading_session_sub_id: None,
            security_exchange: Some("XNAS".to_string()),
            trad_ses_method: Some(TradSesMethod::Electronic),
            trad_ses_mode: Some(TradSesMode::Production),
        };

        let raw = req.to_raw();
        let parsed = TradingSessionListRequest::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.trad_ses_req_id, "TSLISTREQ001");
        assert_eq!(parsed.subscription_request_type, SubscriptionRequestType::Snapshot);
        assert_eq!(parsed.trading_session_id, Some("CORE".to_string()));
        assert_eq!(parsed.security_exchange, Some("XNAS".to_string()));
        assert_eq!(parsed.trad_ses_method, Some(TradSesMethod::Electronic));
        assert_eq!(parsed.trad_ses_mode, Some(TradSesMode::Production));
    }

    #[test]
    fn test_trading_session_list_round_trip() {
        let list = TradingSessionList {
            trad_ses_req_id: Some("TSLISTREQ001".to_string()),
        };

        let raw = list.to_raw();
        let parsed = TradingSessionList::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.trad_ses_req_id, Some("TSLISTREQ001".to_string()));
    }
}
