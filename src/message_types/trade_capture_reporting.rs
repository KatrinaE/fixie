use crate::types::*;
use crate::parser::{RawFixMessage, FixParseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Trade Capture Reporting Messages
// Implementation of FIX 5.0 SP2 Post-Trade Trade Capture Reporting messages
// ============================================================================

/// TradeCaptureReport (MsgType=AE)
///
/// Used to report trade details to a counterparty or market data system.
/// Can be used to report new trades, update existing trades, or cancel trades.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCaptureReport {
    /// TradeReportID (571) - Unique identifier for this trade report
    pub trade_report_id: String,

    /// TradeReportTransType (487) - Type of trade report transaction
    /// 0=New, 1=Cancel, 2=Replace, 3=Release, 4=Reverse
    pub trade_report_trans_type: Option<i32>,

    /// TradeReportType (856) - Type of trade report
    pub trade_report_type: Option<TradeReportType>,

    /// TrdType (828) - Type of trade
    pub trd_type: Option<TrdType>,

    /// TrdSubType (829) - Sub-type of trade
    pub trd_sub_type: Option<TrdSubType>,

    /// TradeRequestID (568) - Reference to the trade request that triggered this report
    pub trade_request_id: Option<String>,

    /// ExecID (17) - Execution ID
    pub exec_id: Option<String>,

    /// ExecType (150) - Execution type
    pub exec_type: Option<ExecType>,

    /// PreviouslyReported (570) - Indicates if this trade was previously reported
    /// Y=Trade was previously reported, N=Trade is being reported for the first time
    pub previously_reported: Option<bool>,

    /// Symbol (55) - Ticker symbol
    pub symbol: String,

    /// SecurityID (48) - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Source of security identifier
    pub security_id_source: Option<String>,

    /// LastQty (32) - Quantity traded
    pub last_qty: f64,

    /// LastPx (31) - Price of the trade
    pub last_px: f64,

    /// TradeDate (75) - Date of the trade
    pub trade_date: Option<String>,

    /// TransactTime (60) - Time of trade execution
    pub transact_time: DateTime<Utc>,

    /// SettlType (63) - Settlement type
    pub settl_type: Option<String>,

    /// SettlDate (64) - Settlement date
    pub settl_date: Option<String>,

    /// MatchStatus (573) - Status of trade matching
    pub match_status: Option<MatchStatus>,

    /// MatchType (574) - Type of match algorithm used
    pub match_type: Option<MatchType>,

    /// Text (58) - Free format text
    pub text: Option<String>,
}

impl TradeCaptureReport {
    /// Create a new TradeCaptureReport
    pub fn new(
        trade_report_id: String,
        symbol: String,
        last_qty: f64,
        last_px: f64,
        transact_time: DateTime<Utc>,
    ) -> Self {
        Self {
            trade_report_id,
            trade_report_trans_type: None,
            trade_report_type: None,
            trd_type: None,
            trd_sub_type: None,
            trade_request_id: None,
            exec_id: None,
            exec_type: None,
            previously_reported: None,
            symbol,
            security_id: None,
            security_id_source: None,
            last_qty,
            last_px,
            trade_date: None,
            transact_time,
            settl_type: None,
            settl_date: None,
            match_status: None,
            match_type: None,
            text: None,
        }
    }

    /// Set the trade report transaction type
    pub fn with_trade_report_trans_type(mut self, trade_report_trans_type: i32) -> Self {
        self.trade_report_trans_type = Some(trade_report_trans_type);
        self
    }

    /// Set the trade report type
    pub fn with_trade_report_type(mut self, trade_report_type: TradeReportType) -> Self {
        self.trade_report_type = Some(trade_report_type);
        self
    }

    /// Set the trade type
    pub fn with_trd_type(mut self, trd_type: TrdType) -> Self {
        self.trd_type = Some(trd_type);
        self
    }

    /// Set the trade sub-type
    pub fn with_trd_sub_type(mut self, trd_sub_type: TrdSubType) -> Self {
        self.trd_sub_type = Some(trd_sub_type);
        self
    }

    /// Set the trade request ID
    pub fn with_trade_request_id(mut self, trade_request_id: String) -> Self {
        self.trade_request_id = Some(trade_request_id);
        self
    }

    /// Set the execution ID
    pub fn with_exec_id(mut self, exec_id: String) -> Self {
        self.exec_id = Some(exec_id);
        self
    }

    /// Set the execution type
    pub fn with_exec_type(mut self, exec_type: ExecType) -> Self {
        self.exec_type = Some(exec_type);
        self
    }

    /// Set previously reported flag
    pub fn with_previously_reported(mut self, previously_reported: bool) -> Self {
        self.previously_reported = Some(previously_reported);
        self
    }

    /// Set security ID
    pub fn with_security_id(mut self, security_id: String, source: String) -> Self {
        self.security_id = Some(security_id);
        self.security_id_source = Some(source);
        self
    }

    /// Set trade date
    pub fn with_trade_date(mut self, trade_date: String) -> Self {
        self.trade_date = Some(trade_date);
        self
    }

    /// Set settlement type
    pub fn with_settl_type(mut self, settl_type: String) -> Self {
        self.settl_type = Some(settl_type);
        self
    }

    /// Set settlement date
    pub fn with_settl_date(mut self, settl_date: String) -> Self {
        self.settl_date = Some(settl_date);
        self
    }

    /// Set match status
    pub fn with_match_status(mut self, match_status: MatchStatus) -> Self {
        self.match_status = Some(match_status);
        self
    }

    /// Set match type
    pub fn with_match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = Some(match_type);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "AE".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)

        msg.set_field(571, self.trade_report_id.clone());

        if let Some(trans_type) = self.trade_report_trans_type {
            msg.set_field(487, trans_type.to_string());
        }
        if let Some(report_type) = self.trade_report_type {
            msg.set_field(856, report_type.to_fix().to_string());
        }
        if let Some(trd_type) = self.trd_type {
            msg.set_field(828, trd_type.to_fix().to_string());
        }
        if let Some(trd_sub_type) = self.trd_sub_type {
            msg.set_field(829, trd_sub_type.to_fix().to_string());
        }
        if let Some(ref trade_req_id) = self.trade_request_id {
            msg.set_field(568, trade_req_id.clone());
        }
        if let Some(ref exec_id) = self.exec_id {
            msg.set_field(17, exec_id.clone());
        }
        if let Some(exec_type) = self.exec_type {
            msg.set_field(150, exec_type.to_fix().to_string());
        }
        if let Some(prev_reported) = self.previously_reported {
            msg.set_field(570, if prev_reported { "Y" } else { "N" }.to_string());
        }

        msg.set_field(55, self.symbol.clone());

        if let Some(ref sec_id) = self.security_id {
            msg.set_field(48, sec_id.clone());
        }
        if let Some(ref sec_id_src) = self.security_id_source {
            msg.set_field(22, sec_id_src.clone());
        }

        msg.set_field(32, self.last_qty.to_string());
        msg.set_field(31, self.last_px.to_string());

        if let Some(ref trade_date) = self.trade_date {
            msg.set_field(75, trade_date.clone());
        }

        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());

        if let Some(ref settl_type) = self.settl_type {
            msg.set_field(63, settl_type.clone());
        }
        if let Some(ref settl_date) = self.settl_date {
            msg.set_field(64, settl_date.clone());
        }
        if let Some(match_status) = self.match_status {
            msg.set_field(573, match_status.to_fix().to_string());
        }
        if let Some(match_type) = self.match_type {
            msg.set_field(574, match_type.to_fix().to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    /// Parse from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let trade_report_id = raw.get_field(571)
            .ok_or(FixParseError::MissingRequiredField(571))?
            .to_string();

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let last_qty: f64 = raw.get_field_as(32)
            .ok_or(FixParseError::MissingRequiredField(32))?;

        let last_px: f64 = raw.get_field_as(31)
            .ok_or(FixParseError::MissingRequiredField(31))?;

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let trade_report_trans_type = raw.get_field_as(487);

        let trade_report_type = raw.get_field(856)
            .and_then(|s| TradeReportType::from_fix(s));

        let trd_type = raw.get_field(828)
            .and_then(|s| TrdType::from_fix(s));

        let trd_sub_type = raw.get_field(829)
            .and_then(|s| TrdSubType::from_fix(s));

        let trade_request_id = raw.get_field(568).map(|s| s.to_string());
        let exec_id = raw.get_field(17).map(|s| s.to_string());

        let exec_type = raw.get_field(150)
            .and_then(|s| s.chars().next())
            .and_then(|c| ExecType::from_fix(c));

        let previously_reported = raw.get_field(570)
            .map(|s| s == "Y");

        let security_id = raw.get_field(48).map(|s| s.to_string());
        let security_id_source = raw.get_field(22).map(|s| s.to_string());
        let trade_date = raw.get_field(75).map(|s| s.to_string());
        let settl_type = raw.get_field(63).map(|s| s.to_string());
        let settl_date = raw.get_field(64).map(|s| s.to_string());

        let match_status = raw.get_field(573)
            .and_then(|s| MatchStatus::from_fix(s));

        let match_type = raw.get_field(574)
            .and_then(|s| MatchType::from_fix(s));

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(TradeCaptureReport {
            trade_report_id,
            trade_report_trans_type,
            trade_report_type,
            trd_type,
            trd_sub_type,
            trade_request_id,
            exec_id,
            exec_type,
            previously_reported,
            symbol,
            security_id,
            security_id_source,
            last_qty,
            last_px,
            trade_date,
            transact_time,
            settl_type,
            settl_date,
            match_status,
            match_type,
            text,
        })
    }
}

/// TradeCaptureReportAck (MsgType=AR)
///
/// Acknowledgement of a Trade Capture Report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCaptureReportAck {
    /// TradeReportID (571) - Reference to the trade report being acknowledged
    pub trade_report_id: String,

    /// TradeReportTransType (487) - Type of trade report transaction
    pub trade_report_trans_type: Option<i32>,

    /// TradeReportType (856) - Type of trade report
    pub trade_report_type: Option<TradeReportType>,

    /// TrdType (828) - Type of trade
    pub trd_type: Option<TrdType>,

    /// ExecID (17) - Execution ID
    pub exec_id: Option<String>,

    /// TradeReportStatus (939) - Status of the trade report
    /// 0=Accepted, 1=Rejected, 2=Accepted with errors, 3=Pending
    pub trade_report_status: i32,

    /// TradeReportRejectReason (751) - Reason for rejection (if rejected)
    pub trade_report_reject_reason: Option<i32>,

    /// TransactTime (60) - Time of acknowledgement
    pub transact_time: DateTime<Utc>,

    /// Text (58) - Free format text
    pub text: Option<String>,
}

impl TradeCaptureReportAck {
    /// Create a new TradeCaptureReportAck
    pub fn new(
        trade_report_id: String,
        trade_report_status: i32,
        transact_time: DateTime<Utc>,
    ) -> Self {
        Self {
            trade_report_id,
            trade_report_trans_type: None,
            trade_report_type: None,
            trd_type: None,
            exec_id: None,
            trade_report_status,
            trade_report_reject_reason: None,
            transact_time,
            text: None,
        }
    }

    /// Set the trade report transaction type
    pub fn with_trade_report_trans_type(mut self, trade_report_trans_type: i32) -> Self {
        self.trade_report_trans_type = Some(trade_report_trans_type);
        self
    }

    /// Set the trade report type
    pub fn with_trade_report_type(mut self, trade_report_type: TradeReportType) -> Self {
        self.trade_report_type = Some(trade_report_type);
        self
    }

    /// Set the trade type
    pub fn with_trd_type(mut self, trd_type: TrdType) -> Self {
        self.trd_type = Some(trd_type);
        self
    }

    /// Set the execution ID
    pub fn with_exec_id(mut self, exec_id: String) -> Self {
        self.exec_id = Some(exec_id);
        self
    }

    /// Set the trade report reject reason
    pub fn with_trade_report_reject_reason(mut self, reason: i32) -> Self {
        self.trade_report_reject_reason = Some(reason);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "AR".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)

        msg.set_field(571, self.trade_report_id.clone());

        if let Some(trans_type) = self.trade_report_trans_type {
            msg.set_field(487, trans_type.to_string());
        }
        if let Some(report_type) = self.trade_report_type {
            msg.set_field(856, report_type.to_fix().to_string());
        }
        if let Some(trd_type) = self.trd_type {
            msg.set_field(828, trd_type.to_fix().to_string());
        }
        if let Some(ref exec_id) = self.exec_id {
            msg.set_field(17, exec_id.clone());
        }

        msg.set_field(939, self.trade_report_status.to_string());

        if let Some(reject_reason) = self.trade_report_reject_reason {
            msg.set_field(751, reject_reason.to_string());
        }

        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());

        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    /// Parse from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let trade_report_id = raw.get_field(571)
            .ok_or(FixParseError::MissingRequiredField(571))?
            .to_string();

        let trade_report_status: i32 = raw.get_field_as(939)
            .ok_or(FixParseError::MissingRequiredField(939))?;

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let trade_report_trans_type = raw.get_field_as(487);

        let trade_report_type = raw.get_field(856)
            .and_then(|s| TradeReportType::from_fix(s));

        let trd_type = raw.get_field(828)
            .and_then(|s| TrdType::from_fix(s));

        let exec_id = raw.get_field(17).map(|s| s.to_string());
        let trade_report_reject_reason = raw.get_field_as(751);
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(TradeCaptureReportAck {
            trade_report_id,
            trade_report_trans_type,
            trade_report_type,
            trd_type,
            exec_id,
            trade_report_status,
            trade_report_reject_reason,
            transact_time,
            text,
        })
    }
}

/// TradeCaptureReportRequest (MsgType=AD)
///
/// Request for trade capture reports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCaptureReportRequest {
    /// TradeRequestID (568) - Unique identifier for this request
    pub trade_request_id: String,

    /// TradeRequestType (569) - Type of trade capture report request
    pub trade_request_type: TradeRequestType,

    /// SubscriptionRequestType (263) - Subscription type
    /// 0=Snapshot, 1=Snapshot + Updates, 2=Disable previous snapshot + updates
    pub subscription_request_type: Option<char>,

    /// Symbol (55) - Ticker symbol (optional, for filtering)
    pub symbol: Option<String>,

    /// SecurityID (48) - Security identifier (optional, for filtering)
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Source of security identifier
    pub security_id_source: Option<String>,

    /// TradeDate (75) - Trade date for filtering
    pub trade_date: Option<String>,

    /// TransactTime (60) - Time of request
    pub transact_time: DateTime<Utc>,

    /// Text (58) - Free format text
    pub text: Option<String>,
}

impl TradeCaptureReportRequest {
    /// Create a new TradeCaptureReportRequest
    pub fn new(
        trade_request_id: String,
        trade_request_type: TradeRequestType,
        transact_time: DateTime<Utc>,
    ) -> Self {
        Self {
            trade_request_id,
            trade_request_type,
            subscription_request_type: None,
            symbol: None,
            security_id: None,
            security_id_source: None,
            trade_date: None,
            transact_time,
            text: None,
        }
    }

    /// Set the subscription request type
    pub fn with_subscription_request_type(mut self, subscription_request_type: char) -> Self {
        self.subscription_request_type = Some(subscription_request_type);
        self
    }

    /// Set the symbol filter
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set security ID filter
    pub fn with_security_id(mut self, security_id: String, source: String) -> Self {
        self.security_id = Some(security_id);
        self.security_id_source = Some(source);
        self
    }

    /// Set trade date filter
    pub fn with_trade_date(mut self, trade_date: String) -> Self {
        self.trade_date = Some(trade_date);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "AD".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)

        msg.set_field(568, self.trade_request_id.clone());
        msg.set_field(569, self.trade_request_type.to_fix().to_string());

        if let Some(sub_req_type) = self.subscription_request_type {
            msg.set_field(263, sub_req_type.to_string());
        }
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(ref sec_id) = self.security_id {
            msg.set_field(48, sec_id.clone());
        }
        if let Some(ref sec_id_src) = self.security_id_source {
            msg.set_field(22, sec_id_src.clone());
        }
        if let Some(ref trade_date) = self.trade_date {
            msg.set_field(75, trade_date.clone());
        }

        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());

        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    /// Parse from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let trade_request_id = raw.get_field(568)
            .ok_or(FixParseError::MissingRequiredField(568))?
            .to_string();

        let trade_request_type_str = raw.get_field(569)
            .ok_or(FixParseError::MissingRequiredField(569))?;
        let trade_request_type = TradeRequestType::from_fix(trade_request_type_str)
            .ok_or(FixParseError::InvalidValue {
                tag: 569,
                value: trade_request_type_str.to_string(),
                error: "Invalid TradeRequestType".to_string(),
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

        let subscription_request_type = raw.get_field(263)
            .and_then(|s| s.chars().next());

        let symbol = raw.get_field(55).map(|s| s.to_string());
        let security_id = raw.get_field(48).map(|s| s.to_string());
        let security_id_source = raw.get_field(22).map(|s| s.to_string());
        let trade_date = raw.get_field(75).map(|s| s.to_string());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(TradeCaptureReportRequest {
            trade_request_id,
            trade_request_type,
            subscription_request_type,
            symbol,
            security_id,
            security_id_source,
            trade_date,
            transact_time,
            text,
        })
    }
}

/// TradeCaptureReportRequestAck (MsgType=AQ)
///
/// Acknowledgement of a Trade Capture Report Request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeCaptureReportRequestAck {
    /// TradeRequestID (568) - Reference to the trade request being acknowledged
    pub trade_request_id: String,

    /// TradeRequestType (569) - Type of trade capture report request
    pub trade_request_type: TradeRequestType,

    /// TradeRequestResult (749) - Result of the trade request
    pub trade_request_result: TradeRequestResult,

    /// TradeRequestStatus (750) - Status of the trade request
    pub trade_request_status: TradeRequestStatus,

    /// SubscriptionRequestType (263) - Subscription type
    pub subscription_request_type: Option<char>,

    /// TransactTime (60) - Time of acknowledgement
    pub transact_time: DateTime<Utc>,

    /// Text (58) - Free format text
    pub text: Option<String>,
}

impl TradeCaptureReportRequestAck {
    /// Create a new TradeCaptureReportRequestAck
    pub fn new(
        trade_request_id: String,
        trade_request_type: TradeRequestType,
        trade_request_result: TradeRequestResult,
        trade_request_status: TradeRequestStatus,
        transact_time: DateTime<Utc>,
    ) -> Self {
        Self {
            trade_request_id,
            trade_request_type,
            trade_request_result,
            trade_request_status,
            subscription_request_type: None,
            transact_time,
            text: None,
        }
    }

    /// Set the subscription request type
    pub fn with_subscription_request_type(mut self, subscription_request_type: char) -> Self {
        self.subscription_request_type = Some(subscription_request_type);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "AQ".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)

        msg.set_field(568, self.trade_request_id.clone());
        msg.set_field(569, self.trade_request_type.to_fix().to_string());
        msg.set_field(749, self.trade_request_result.to_fix().to_string());
        msg.set_field(750, self.trade_request_status.to_fix().to_string());

        if let Some(sub_req_type) = self.subscription_request_type {
            msg.set_field(263, sub_req_type.to_string());
        }

        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());

        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    /// Parse from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let trade_request_id = raw.get_field(568)
            .ok_or(FixParseError::MissingRequiredField(568))?
            .to_string();

        let trade_request_type_str = raw.get_field(569)
            .ok_or(FixParseError::MissingRequiredField(569))?;
        let trade_request_type = TradeRequestType::from_fix(trade_request_type_str)
            .ok_or(FixParseError::InvalidValue {
                tag: 569,
                value: trade_request_type_str.to_string(),
                error: "Invalid TradeRequestType".to_string(),
            })?;

        let trade_request_result_str = raw.get_field(749)
            .ok_or(FixParseError::MissingRequiredField(749))?;
        let trade_request_result = TradeRequestResult::from_fix(trade_request_result_str)
            .ok_or(FixParseError::InvalidValue {
                tag: 749,
                value: trade_request_result_str.to_string(),
                error: "Invalid TradeRequestResult".to_string(),
            })?;

        let trade_request_status_str = raw.get_field(750)
            .ok_or(FixParseError::MissingRequiredField(750))?;
        let trade_request_status = TradeRequestStatus::from_fix(trade_request_status_str)
            .ok_or(FixParseError::InvalidValue {
                tag: 750,
                value: trade_request_status_str.to_string(),
                error: "Invalid TradeRequestStatus".to_string(),
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

        let subscription_request_type = raw.get_field(263)
            .and_then(|s| s.chars().next());

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(TradeCaptureReportRequestAck {
            trade_request_id,
            trade_request_type,
            trade_request_result,
            trade_request_status,
            subscription_request_type,
            transact_time,
            text,
        })
    }
}

/// TradeMatchReport (MsgType=DC)
///
/// Reports matched trade details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeMatchReport {
    /// TradeReportID (571) - Unique identifier for this trade match report
    pub trade_report_id: String,

    /// TradeReportType (856) - Type of trade report
    pub trade_report_type: Option<TradeReportType>,

    /// TrdType (828) - Type of trade
    pub trd_type: Option<TrdType>,

    /// TrdSubType (829) - Sub-type of trade
    pub trd_sub_type: Option<TrdSubType>,

    /// MatchStatus (573) - Status of trade matching
    pub match_status: MatchStatus,

    /// MatchType (574) - Type of match algorithm used
    pub match_type: Option<MatchType>,

    /// Symbol (55) - Ticker symbol
    pub symbol: String,

    /// SecurityID (48) - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Source of security identifier
    pub security_id_source: Option<String>,

    /// LastQty (32) - Quantity traded
    pub last_qty: f64,

    /// LastPx (31) - Price of the trade
    pub last_px: f64,

    /// TradeDate (75) - Date of the trade
    pub trade_date: Option<String>,

    /// TransactTime (60) - Time of trade execution
    pub transact_time: DateTime<Utc>,

    /// SettlDate (64) - Settlement date
    pub settl_date: Option<String>,

    /// Text (58) - Free format text
    pub text: Option<String>,
}

impl TradeMatchReport {
    /// Create a new TradeMatchReport
    pub fn new(
        trade_report_id: String,
        match_status: MatchStatus,
        symbol: String,
        last_qty: f64,
        last_px: f64,
        transact_time: DateTime<Utc>,
    ) -> Self {
        Self {
            trade_report_id,
            trade_report_type: None,
            trd_type: None,
            trd_sub_type: None,
            match_status,
            match_type: None,
            symbol,
            security_id: None,
            security_id_source: None,
            last_qty,
            last_px,
            trade_date: None,
            transact_time,
            settl_date: None,
            text: None,
        }
    }

    /// Set the trade report type
    pub fn with_trade_report_type(mut self, trade_report_type: TradeReportType) -> Self {
        self.trade_report_type = Some(trade_report_type);
        self
    }

    /// Set the trade type
    pub fn with_trd_type(mut self, trd_type: TrdType) -> Self {
        self.trd_type = Some(trd_type);
        self
    }

    /// Set the trade sub-type
    pub fn with_trd_sub_type(mut self, trd_sub_type: TrdSubType) -> Self {
        self.trd_sub_type = Some(trd_sub_type);
        self
    }

    /// Set the match type
    pub fn with_match_type(mut self, match_type: MatchType) -> Self {
        self.match_type = Some(match_type);
        self
    }

    /// Set security ID
    pub fn with_security_id(mut self, security_id: String, source: String) -> Self {
        self.security_id = Some(security_id);
        self.security_id_source = Some(source);
        self
    }

    /// Set trade date
    pub fn with_trade_date(mut self, trade_date: String) -> Self {
        self.trade_date = Some(trade_date);
        self
    }

    /// Set settlement date
    pub fn with_settl_date(mut self, settl_date: String) -> Self {
        self.settl_date = Some(settl_date);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "DC".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)

        msg.set_field(571, self.trade_report_id.clone());

        if let Some(report_type) = self.trade_report_type {
            msg.set_field(856, report_type.to_fix().to_string());
        }
        if let Some(trd_type) = self.trd_type {
            msg.set_field(828, trd_type.to_fix().to_string());
        }
        if let Some(trd_sub_type) = self.trd_sub_type {
            msg.set_field(829, trd_sub_type.to_fix().to_string());
        }

        msg.set_field(573, self.match_status.to_fix().to_string());

        if let Some(match_type) = self.match_type {
            msg.set_field(574, match_type.to_fix().to_string());
        }

        msg.set_field(55, self.symbol.clone());

        if let Some(ref sec_id) = self.security_id {
            msg.set_field(48, sec_id.clone());
        }
        if let Some(ref sec_id_src) = self.security_id_source {
            msg.set_field(22, sec_id_src.clone());
        }

        msg.set_field(32, self.last_qty.to_string());
        msg.set_field(31, self.last_px.to_string());

        if let Some(ref trade_date) = self.trade_date {
            msg.set_field(75, trade_date.clone());
        }

        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());

        if let Some(ref settl_date) = self.settl_date {
            msg.set_field(64, settl_date.clone());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    /// Parse from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let trade_report_id = raw.get_field(571)
            .ok_or(FixParseError::MissingRequiredField(571))?
            .to_string();

        let match_status_str = raw.get_field(573)
            .ok_or(FixParseError::MissingRequiredField(573))?;
        let match_status = MatchStatus::from_fix(match_status_str)
            .ok_or(FixParseError::InvalidValue {
                tag: 573,
                value: match_status_str.to_string(),
                error: "Invalid MatchStatus".to_string(),
            })?;

        let symbol = raw.get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let last_qty: f64 = raw.get_field_as(32)
            .ok_or(FixParseError::MissingRequiredField(32))?;

        let last_px: f64 = raw.get_field_as(31)
            .ok_or(FixParseError::MissingRequiredField(31))?;

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let trade_report_type = raw.get_field(856)
            .and_then(|s| TradeReportType::from_fix(s));

        let trd_type = raw.get_field(828)
            .and_then(|s| TrdType::from_fix(s));

        let trd_sub_type = raw.get_field(829)
            .and_then(|s| TrdSubType::from_fix(s));

        let match_type = raw.get_field(574)
            .and_then(|s| MatchType::from_fix(s));

        let security_id = raw.get_field(48).map(|s| s.to_string());
        let security_id_source = raw.get_field(22).map(|s| s.to_string());
        let trade_date = raw.get_field(75).map(|s| s.to_string());
        let settl_date = raw.get_field(64).map(|s| s.to_string());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(TradeMatchReport {
            trade_report_id,
            trade_report_type,
            trd_type,
            trd_sub_type,
            match_status,
            match_type,
            symbol,
            security_id,
            security_id_source,
            last_qty,
            last_px,
            trade_date,
            transact_time,
            settl_date,
            text,
        })
    }
}

/// TradeMatchReportAck (MsgType=DD)
///
/// Acknowledgement of a Trade Match Report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeMatchReportAck {
    /// TradeReportID (571) - Reference to the trade match report being acknowledged
    pub trade_report_id: String,

    /// TradeReportType (856) - Type of trade report
    pub trade_report_type: Option<TradeReportType>,

    /// TrdType (828) - Type of trade
    pub trd_type: Option<TrdType>,

    /// TradeReportStatus (939) - Status of the trade report
    /// 0=Accepted, 1=Rejected, 2=Accepted with errors, 3=Pending
    pub trade_report_status: i32,

    /// TradeReportRejectReason (751) - Reason for rejection (if rejected)
    pub trade_report_reject_reason: Option<i32>,

    /// MatchStatus (573) - Status of trade matching
    pub match_status: Option<MatchStatus>,

    /// TransactTime (60) - Time of acknowledgement
    pub transact_time: DateTime<Utc>,

    /// Text (58) - Free format text
    pub text: Option<String>,
}

impl TradeMatchReportAck {
    /// Create a new TradeMatchReportAck
    pub fn new(
        trade_report_id: String,
        trade_report_status: i32,
        transact_time: DateTime<Utc>,
    ) -> Self {
        Self {
            trade_report_id,
            trade_report_type: None,
            trd_type: None,
            trade_report_status,
            trade_report_reject_reason: None,
            match_status: None,
            transact_time,
            text: None,
        }
    }

    /// Set the trade report type
    pub fn with_trade_report_type(mut self, trade_report_type: TradeReportType) -> Self {
        self.trade_report_type = Some(trade_report_type);
        self
    }

    /// Set the trade type
    pub fn with_trd_type(mut self, trd_type: TrdType) -> Self {
        self.trd_type = Some(trd_type);
        self
    }

    /// Set the trade report reject reason
    pub fn with_trade_report_reject_reason(mut self, reason: i32) -> Self {
        self.trade_report_reject_reason = Some(reason);
        self
    }

    /// Set the match status
    pub fn with_match_status(mut self, match_status: MatchStatus) -> Self {
        self.match_status = Some(match_status);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "DD".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)

        msg.set_field(571, self.trade_report_id.clone());

        if let Some(report_type) = self.trade_report_type {
            msg.set_field(856, report_type.to_fix().to_string());
        }
        if let Some(trd_type) = self.trd_type {
            msg.set_field(828, trd_type.to_fix().to_string());
        }

        msg.set_field(939, self.trade_report_status.to_string());

        if let Some(reject_reason) = self.trade_report_reject_reason {
            msg.set_field(751, reject_reason.to_string());
        }
        if let Some(match_status) = self.match_status {
            msg.set_field(573, match_status.to_fix().to_string());
        }

        msg.set_field(60, self.transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());

        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    /// Parse from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let trade_report_id = raw.get_field(571)
            .ok_or(FixParseError::MissingRequiredField(571))?
            .to_string();

        let trade_report_status: i32 = raw.get_field_as(939)
            .ok_or(FixParseError::MissingRequiredField(939))?;

        let transact_time_str = raw.get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
            .map_err(|e| FixParseError::InvalidValue {
                tag: 60,
                value: transact_time_str.to_string(),
                error: e.to_string(),
            })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let trade_report_type = raw.get_field(856)
            .and_then(|s| TradeReportType::from_fix(s));

        let trd_type = raw.get_field(828)
            .and_then(|s| TrdType::from_fix(s));

        let trade_report_reject_reason = raw.get_field_as(751);

        let match_status = raw.get_field(573)
            .and_then(|s| MatchStatus::from_fix(s));

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(TradeMatchReportAck {
            trade_report_id,
            trade_report_type,
            trd_type,
            trade_report_status,
            trade_report_reject_reason,
            match_status,
            transact_time,
            text,
        })
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_trade_capture_report_creation() {
        let now = Utc::now();
        let report = TradeCaptureReport::new(
            "TCR001".to_string(),
            "AAPL".to_string(),
            100.0,
            150.50,
            now,
        )
        .with_trade_report_type(TradeReportType::Submit)
        .with_trd_type(TrdType::RegularTrade)
        .with_match_status(MatchStatus::Compared);

        assert_eq!(report.trade_report_id, "TCR001");
        assert_eq!(report.symbol, "AAPL");
        assert_eq!(report.last_qty, 100.0);
        assert_eq!(report.last_px, 150.50);
        assert_eq!(report.trade_report_type, Some(TradeReportType::Submit));
        assert_eq!(report.trd_type, Some(TrdType::RegularTrade));
        assert_eq!(report.match_status, Some(MatchStatus::Compared));
    }

    #[test]
    fn test_trade_capture_report_round_trip() {
        let now = Utc::now();
        let original = TradeCaptureReport::new(
            "TCR002".to_string(),
            "MSFT".to_string(),
            200.0,
            300.75,
            now,
        )
        .with_trade_report_type(TradeReportType::Accept)
        .with_trd_type(TrdType::BlockTrade)
        .with_text("Test trade".to_string());

        let raw = original.to_raw();
        let parsed = TradeCaptureReport::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed.trade_report_id, original.trade_report_id);
        assert_eq!(parsed.symbol, original.symbol);
        assert_eq!(parsed.last_qty, original.last_qty);
        assert_eq!(parsed.last_px, original.last_px);
        assert_eq!(parsed.trade_report_type, original.trade_report_type);
        assert_eq!(parsed.trd_type, original.trd_type);
        assert_eq!(parsed.text, original.text);
    }

    #[test]
    fn test_trade_capture_report_ack_creation() {
        let now = Utc::now();
        let ack = TradeCaptureReportAck::new(
            "TCR001".to_string(),
            0, // Accepted
            now,
        )
        .with_trade_report_type(TradeReportType::Submit)
        .with_text("Accepted".to_string());

        assert_eq!(ack.trade_report_id, "TCR001");
        assert_eq!(ack.trade_report_status, 0);
        assert_eq!(ack.text, Some("Accepted".to_string()));
    }

    #[test]
    fn test_trade_capture_report_ack_round_trip() {
        let now = Utc::now();
        let original = TradeCaptureReportAck::new(
            "TCR003".to_string(),
            1, // Rejected
            now,
        )
        .with_trade_report_reject_reason(99)
        .with_text("Invalid trade".to_string());

        let raw = original.to_raw();
        let parsed = TradeCaptureReportAck::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed.trade_report_id, original.trade_report_id);
        assert_eq!(parsed.trade_report_status, original.trade_report_status);
        assert_eq!(parsed.trade_report_reject_reason, original.trade_report_reject_reason);
        assert_eq!(parsed.text, original.text);
    }

    #[test]
    fn test_trade_capture_report_request_creation() {
        let now = Utc::now();
        let request = TradeCaptureReportRequest::new(
            "REQ001".to_string(),
            TradeRequestType::AllTrades,
            now,
        )
        .with_symbol("GOOGL".to_string())
        .with_subscription_request_type('1');

        assert_eq!(request.trade_request_id, "REQ001");
        assert_eq!(request.trade_request_type, TradeRequestType::AllTrades);
        assert_eq!(request.symbol, Some("GOOGL".to_string()));
        assert_eq!(request.subscription_request_type, Some('1'));
    }

    #[test]
    fn test_trade_capture_report_request_round_trip() {
        let now = Utc::now();
        let original = TradeCaptureReportRequest::new(
            "REQ002".to_string(),
            TradeRequestType::MatchedTradesMatchingCriteria,
            now,
        )
        .with_symbol("TSLA".to_string())
        .with_trade_date("20250101".to_string());

        let raw = original.to_raw();
        let parsed = TradeCaptureReportRequest::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed.trade_request_id, original.trade_request_id);
        assert_eq!(parsed.trade_request_type, original.trade_request_type);
        assert_eq!(parsed.symbol, original.symbol);
        assert_eq!(parsed.trade_date, original.trade_date);
    }

    #[test]
    fn test_trade_capture_report_request_ack_creation() {
        let now = Utc::now();
        let ack = TradeCaptureReportRequestAck::new(
            "REQ001".to_string(),
            TradeRequestType::AllTrades,
            TradeRequestResult::Successful,
            TradeRequestStatus::Accepted,
            now,
        )
        .with_text("Request accepted".to_string());

        assert_eq!(ack.trade_request_id, "REQ001");
        assert_eq!(ack.trade_request_result, TradeRequestResult::Successful);
        assert_eq!(ack.trade_request_status, TradeRequestStatus::Accepted);
    }

    #[test]
    fn test_trade_capture_report_request_ack_round_trip() {
        let now = Utc::now();
        let original = TradeCaptureReportRequestAck::new(
            "REQ003".to_string(),
            TradeRequestType::UnmatchedTradesThatMatchCriteria,
            TradeRequestResult::InvalidParties,
            TradeRequestStatus::Rejected,
            now,
        )
        .with_text("Invalid parties".to_string());

        let raw = original.to_raw();
        let parsed = TradeCaptureReportRequestAck::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed.trade_request_id, original.trade_request_id);
        assert_eq!(parsed.trade_request_type, original.trade_request_type);
        assert_eq!(parsed.trade_request_result, original.trade_request_result);
        assert_eq!(parsed.trade_request_status, original.trade_request_status);
        assert_eq!(parsed.text, original.text);
    }

    #[test]
    fn test_trade_match_report_creation() {
        let now = Utc::now();
        let report = TradeMatchReport::new(
            "TMR001".to_string(),
            MatchStatus::Compared,
            "NFLX".to_string(),
            50.0,
            400.25,
            now,
        )
        .with_match_type(MatchType::ExactMatchOnTradeDateTime)
        .with_trd_type(TrdType::RegularTrade);

        assert_eq!(report.trade_report_id, "TMR001");
        assert_eq!(report.match_status, MatchStatus::Compared);
        assert_eq!(report.symbol, "NFLX");
        assert_eq!(report.last_qty, 50.0);
        assert_eq!(report.last_px, 400.25);
    }

    #[test]
    fn test_trade_match_report_round_trip() {
        let now = Utc::now();
        let original = TradeMatchReport::new(
            "TMR002".to_string(),
            MatchStatus::Uncompared,
            "AMZN".to_string(),
            75.0,
            3500.00,
            now,
        )
        .with_match_type(MatchType::ACTAcceptedTrade)
        .with_text("Match report".to_string());

        let raw = original.to_raw();
        let parsed = TradeMatchReport::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed.trade_report_id, original.trade_report_id);
        assert_eq!(parsed.match_status, original.match_status);
        assert_eq!(parsed.symbol, original.symbol);
        assert_eq!(parsed.last_qty, original.last_qty);
        assert_eq!(parsed.last_px, original.last_px);
        assert_eq!(parsed.match_type, original.match_type);
        assert_eq!(parsed.text, original.text);
    }

    #[test]
    fn test_trade_match_report_ack_creation() {
        let now = Utc::now();
        let ack = TradeMatchReportAck::new(
            "TMR001".to_string(),
            0, // Accepted
            now,
        )
        .with_match_status(MatchStatus::Compared)
        .with_text("Match accepted".to_string());

        assert_eq!(ack.trade_report_id, "TMR001");
        assert_eq!(ack.trade_report_status, 0);
        assert_eq!(ack.match_status, Some(MatchStatus::Compared));
    }

    #[test]
    fn test_trade_match_report_ack_round_trip() {
        let now = Utc::now();
        let original = TradeMatchReportAck::new(
            "TMR003".to_string(),
            1, // Rejected
            now,
        )
        .with_trade_report_reject_reason(5)
        .with_match_status(MatchStatus::AdvisoryOrAlert)
        .with_text("Match rejected".to_string());

        let raw = original.to_raw();
        let parsed = TradeMatchReportAck::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed.trade_report_id, original.trade_report_id);
        assert_eq!(parsed.trade_report_status, original.trade_report_status);
        assert_eq!(parsed.trade_report_reject_reason, original.trade_report_reject_reason);
        assert_eq!(parsed.match_status, original.match_status);
        assert_eq!(parsed.text, original.text);
    }

    #[test]
    fn test_enum_conversions() {
        // Test TradeRequestType
        assert_eq!(TradeRequestType::AllTrades.to_fix(), "0");
        assert_eq!(TradeRequestType::from_fix("0"), Some(TradeRequestType::AllTrades));

        // Test TradeRequestResult
        assert_eq!(TradeRequestResult::Successful.to_fix(), "0");
        assert_eq!(TradeRequestResult::from_fix("0"), Some(TradeRequestResult::Successful));

        // Test TradeRequestStatus
        assert_eq!(TradeRequestStatus::Accepted.to_fix(), "0");
        assert_eq!(TradeRequestStatus::from_fix("0"), Some(TradeRequestStatus::Accepted));

        // Test TradeReportType
        assert_eq!(TradeReportType::Submit.to_fix(), "0");
        assert_eq!(TradeReportType::from_fix("0"), Some(TradeReportType::Submit));

        // Test TrdType
        assert_eq!(TrdType::RegularTrade.to_fix(), "0");
        assert_eq!(TrdType::from_fix("0"), Some(TrdType::RegularTrade));

        // Test TrdSubType
        assert_eq!(TrdSubType::CMTA.to_fix(), "0");
        assert_eq!(TrdSubType::from_fix("0"), Some(TrdSubType::CMTA));

        // Test MatchStatus
        assert_eq!(MatchStatus::Compared.to_fix(), "0");
        assert_eq!(MatchStatus::from_fix("0"), Some(MatchStatus::Compared));

        // Test MatchType
        assert_eq!(MatchType::ExactMatchOnTradeDate.to_fix(), "M3");
        assert_eq!(MatchType::from_fix("M3"), Some(MatchType::ExactMatchOnTradeDate));
    }
}
