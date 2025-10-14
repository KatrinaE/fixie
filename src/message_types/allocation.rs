use crate::parser::{FixParseError, RawFixMessage};
use crate::types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Allocation Messages
// Implementation of FIX 5.0 SP2 Post-Trade Allocation messages
// ============================================================================

// ============================================================================
// Supporting Structs for Repeating Groups
// ============================================================================

/// Entry in the AllocGrp repeating group (NoAllocs=78)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocGroup {
    /// AllocAccount (79) - Account to which allocation is made
    pub alloc_account: String,

    /// AllocQty (80) - Quantity allocated to this account
    pub alloc_qty: f64,

    /// AllocPrice (366) - Price for this allocation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_price: Option<f64>,

    /// AllocAcctIDSource (661) - Account ID source (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_acct_id_source: Option<i32>,

    /// MatchStatus (573) - Status of match (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_status: Option<MatchStatus>,

    /// IndividualAllocID (776) - Unique identifier for this allocation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_alloc_id: Option<String>,

    /// AllocPositionEffect (1047) - Position effect (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_position_effect: Option<String>,

    /// AllocText (755) - Free text for this allocation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_text: Option<String>,
}

impl AllocGroup {
    /// Create a new allocation entry
    pub fn new(alloc_account: String, alloc_qty: f64) -> Self {
        Self {
            alloc_account,
            alloc_qty,
            alloc_price: None,
            alloc_acct_id_source: None,
            match_status: None,
            individual_alloc_id: None,
            alloc_position_effect: None,
            alloc_text: None,
        }
    }

    /// Set the allocation price
    pub fn with_price(mut self, price: f64) -> Self {
        self.alloc_price = Some(price);
        self
    }

    /// Set the match status
    pub fn with_match_status(mut self, status: MatchStatus) -> Self {
        self.match_status = Some(status);
        self
    }

    /// Set the individual allocation ID
    pub fn with_individual_alloc_id(mut self, id: String) -> Self {
        self.individual_alloc_id = Some(id);
        self
    }
}

/// Entry in the ExecAllocGrp repeating group (NoExecs=124)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecAllocGroup {
    /// LastQty (32) - Quantity executed
    pub last_qty: f64,

    /// LastPx (31) - Price of execution
    pub last_px: f64,

    /// ExecID (17) - Unique execution ID
    pub exec_id: String,

    /// LastCapacity (29) - Execution capacity (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_capacity: Option<String>,

    /// LastMkt (30) - Market of execution (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_mkt: Option<String>,
}

impl ExecAllocGroup {
    pub fn new(last_qty: f64, last_px: f64, exec_id: String) -> Self {
        Self {
            last_qty,
            last_px,
            exec_id,
            last_capacity: None,
            last_mkt: None,
        }
    }
}

// ============================================================================
// Message: AllocationInstruction (MsgType=J)
// ============================================================================

/// AllocationInstruction (MsgType=J)
///
/// Provides allocation information for a trade execution. Sent from sell-side
/// to buy-side or from executing broker to give-up broker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInstruction {
    /// AllocID (70) - Unique identifier for allocation message
    pub alloc_id: String,

    /// AllocTransType (71) - Identifies allocation transaction type
    pub alloc_trans_type: AllocTransType,

    /// AllocType (626) - Purpose of allocation message
    pub alloc_type: AllocType,

    /// Side (54) - Side of order
    pub side: Side,

    /// Symbol (55) - Instrument symbol
    pub symbol: String,

    /// Quantity (53) - Total quantity allocated
    pub quantity: f64,

    /// AvgPx (6) - Average price
    pub avg_px: f64,

    /// TradeDate (75) - Trade date
    pub trade_date: String,

    /// TransactTime (60) - Transaction time
    pub transact_time: DateTime<Utc>,

    /// SecondaryAllocID (793) - Secondary allocation ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_alloc_id: Option<String>,

    /// AllocCancReplaceReason (796) - Reason for cancel/replace (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_canc_replace_reason: Option<AllocCancReplaceReason>,

    /// RefAllocID (72) - Reference allocation ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_alloc_id: Option<String>,

    /// AllocIntermedReqType (808) - Intermediary request type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_intermed_req_type: Option<AllocIntermedReqType>,

    /// Currency (15) - Currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// AvgPxIndicator (819) - Average price indicator (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_px_indicator: Option<AvgPxIndicator>,

    /// Text (58) - Free text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// AllocGrp - Repeating group of allocation details
    pub allocations: Vec<AllocGroup>,

    /// ExecAllocGrp - Repeating group of execution details (optional)
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub executions: Vec<ExecAllocGroup>,
}

impl AllocationInstruction {
    pub fn new(
        alloc_id: String,
        alloc_trans_type: AllocTransType,
        alloc_type: AllocType,
        side: Side,
        symbol: String,
        quantity: f64,
        avg_px: f64,
        trade_date: String,
        transact_time: DateTime<Utc>,
    ) -> Self {
        Self {
            alloc_id,
            alloc_trans_type,
            alloc_type,
            side,
            symbol,
            quantity,
            avg_px,
            trade_date,
            transact_time,
            secondary_alloc_id: None,
            alloc_canc_replace_reason: None,
            ref_alloc_id: None,
            alloc_intermed_req_type: None,
            currency: None,
            avg_px_indicator: None,
            text: None,
            allocations: Vec::new(),
            executions: Vec::new(),
        }
    }

    /// Add an allocation to the allocations group
    pub fn add_allocation(mut self, allocation: AllocGroup) -> Self {
        self.allocations.push(allocation);
        self
    }

    /// Set all allocations at once
    pub fn with_allocations(mut self, allocations: Vec<AllocGroup>) -> Self {
        self.allocations = allocations;
        self
    }

    /// Add an execution to the executions group
    pub fn add_execution(mut self, execution: ExecAllocGroup) -> Self {
        self.executions.push(execution);
        self
    }

    /// Set the reference allocation ID
    pub fn with_ref_alloc_id(mut self, ref_alloc_id: String) -> Self {
        self.ref_alloc_id = Some(ref_alloc_id);
        self
    }

    /// Set the currency
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Set the text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "J".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)
        msg.set_field(70, self.alloc_id.clone());
        msg.set_field(71, self.alloc_trans_type.to_fix().to_string());
        msg.set_field(626, self.alloc_type.to_fix().to_string());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(55, self.symbol.clone());
        msg.set_field(53, self.quantity.to_string());
        msg.set_field(6, self.avg_px.to_string());
        msg.set_field(75, self.trade_date.clone());
        msg.set_field(
            60,
            self.transact_time
                .format("%Y%m%d-%H:%M:%S%.3f")
                .to_string(),
        );

        if let Some(ref secondary_alloc_id) = self.secondary_alloc_id {
            msg.set_field(793, secondary_alloc_id.clone());
        }
        if let Some(ref_alloc_id) = &self.ref_alloc_id {
            msg.set_field(72, ref_alloc_id.clone());
        }
        if let Some(currency) = &self.currency {
            msg.set_field(15, currency.clone());
        }
        if let Some(text) = &self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let alloc_id = raw
            .get_field(70)
            .ok_or(FixParseError::MissingRequiredField(70))?
            .to_string();

        let alloc_trans_type_str = raw
            .get_field(71)
            .ok_or(FixParseError::MissingRequiredField(71))?;
        let alloc_trans_type = AllocTransType::from_fix(alloc_trans_type_str).map_err(|e| {
            FixParseError::InvalidValue {
                tag: 71,
                value: alloc_trans_type_str.to_string(),
                error: e,
            }
        })?;

        let alloc_type_str = raw
            .get_field(626)
            .ok_or(FixParseError::MissingRequiredField(626))?;
        let alloc_type = AllocType::from_fix(alloc_type_str).map_err(|e| {
            FixParseError::InvalidValue {
                tag: 626,
                value: alloc_type_str.to_string(),
                error: e,
            }
        })?;

        let side_char = raw
            .get_field(54)
            .ok_or(FixParseError::MissingRequiredField(54))?
            .chars()
            .next()
            .ok_or(FixParseError::InvalidValue {
                tag: 54,
                value: "".to_string(),
                error: "Empty side".to_string(),
            })?;
        let side = Side::from_fix(side_char).ok_or(FixParseError::InvalidValue {
            tag: 54,
            value: side_char.to_string(),
            error: "Invalid side".to_string(),
        })?;

        let symbol = raw
            .get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let quantity: f64 = raw
            .get_field_as(53)
            .ok_or(FixParseError::MissingRequiredField(53))?;

        let avg_px: f64 = raw
            .get_field_as(6)
            .ok_or(FixParseError::MissingRequiredField(6))?;

        let trade_date = raw
            .get_field(75)
            .ok_or(FixParseError::MissingRequiredField(75))?
            .to_string();

        let transact_time_str = raw
            .get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(
            transact_time_str,
            "%Y%m%d-%H:%M:%S%.3f",
        )
        .map_err(|e| FixParseError::InvalidValue {
            tag: 60,
            value: transact_time_str.to_string(),
            error: e.to_string(),
        })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let secondary_alloc_id = raw.get_field(793).map(|s| s.to_string());
        let ref_alloc_id = raw.get_field(72).map(|s| s.to_string());
        let currency = raw.get_field(15).map(|s| s.to_string());
        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(AllocationInstruction {
            alloc_id,
            alloc_trans_type,
            alloc_type,
            side,
            symbol,
            quantity,
            avg_px,
            trade_date,
            transact_time,
            secondary_alloc_id,
            alloc_canc_replace_reason: None,
            ref_alloc_id,
            alloc_intermed_req_type: None,
            currency,
            avg_px_indicator: None,
            text,
            allocations: Vec::new(), // Would need repeating group parsing
            executions: Vec::new(),
        })
    }
}

// ============================================================================
// Message: AllocationInstructionAck (MsgType=P)
// ============================================================================

/// AllocationInstructionAck (MsgType=P)
///
/// Acknowledgement of allocation instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInstructionAck {
    /// AllocID (70) - Unique identifier for allocation message
    pub alloc_id: String,

    /// AllocStatus (87) - Status of allocation
    pub alloc_status: AllocStatus,

    /// AllocRejCode (88) - Rejection code (required if rejected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_rej_code: Option<AllocRejCode>,

    /// SecondaryAllocID (793) - Secondary allocation ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_alloc_id: Option<String>,

    /// TradeDate (75) - Trade date (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<String>,

    /// TransactTime (60) - Transaction time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<DateTime<Utc>>,

    /// Text (58) - Free text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// AllocText (161) - Additional allocation text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_text: Option<String>,
}

impl AllocationInstructionAck {
    pub fn new(alloc_id: String, alloc_status: AllocStatus) -> Self {
        Self {
            alloc_id,
            alloc_status,
            alloc_rej_code: None,
            secondary_alloc_id: None,
            trade_date: None,
            transact_time: None,
            text: None,
            alloc_text: None,
        }
    }

    pub fn with_alloc_rej_code(mut self, code: AllocRejCode) -> Self {
        self.alloc_rej_code = Some(code);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn with_trade_date(mut self, trade_date: String) -> Self {
        self.trade_date = Some(trade_date);
        self
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "P".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(70, self.alloc_id.clone());
        msg.set_field(87, self.alloc_status.to_fix().to_string());

        if let Some(ref rej_code) = self.alloc_rej_code {
            msg.set_field(88, rej_code.to_fix().to_string());
        }
        if let Some(ref secondary_alloc_id) = self.secondary_alloc_id {
            msg.set_field(793, secondary_alloc_id.clone());
        }
        if let Some(ref trade_date) = self.trade_date {
            msg.set_field(75, trade_date.clone());
        }
        if let Some(transact_time) = self.transact_time {
            msg.set_field(60, transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }
        if let Some(ref alloc_text) = self.alloc_text {
            msg.set_field(161, alloc_text.clone());
        }

        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let alloc_id = raw
            .get_field(70)
            .ok_or(FixParseError::MissingRequiredField(70))?
            .to_string();

        let alloc_status_str = raw
            .get_field(87)
            .ok_or(FixParseError::MissingRequiredField(87))?;
        let alloc_status = AllocStatus::from_fix(alloc_status_str).map_err(|e| {
            FixParseError::InvalidValue {
                tag: 87,
                value: alloc_status_str.to_string(),
                error: e,
            }
        })?;

        let alloc_rej_code = if let Some(code_str) = raw.get_field(88) {
            Some(AllocRejCode::from_fix(code_str).map_err(|e| {
                FixParseError::InvalidValue {
                    tag: 88,
                    value: code_str.to_string(),
                    error: e,
                }
            })?)
        } else {
            None
        };

        let secondary_alloc_id = raw.get_field(793).map(|s| s.to_string());
        let trade_date = raw.get_field(75).map(|s| s.to_string());

        let transact_time = if let Some(time_str) = raw.get_field(60) {
            let naive_time =
                chrono::NaiveDateTime::parse_from_str(time_str, "%Y%m%d-%H:%M:%S%.3f")
                    .map_err(|e| FixParseError::InvalidValue {
                        tag: 60,
                        value: time_str.to_string(),
                        error: e.to_string(),
                    })?;
            Some(DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc))
        } else {
            None
        };

        let text = raw.get_field(58).map(|s| s.to_string());
        let alloc_text = raw.get_field(161).map(|s| s.to_string());

        Ok(AllocationInstructionAck {
            alloc_id,
            alloc_status,
            alloc_rej_code,
            secondary_alloc_id,
            trade_date,
            transact_time,
            text,
            alloc_text,
        })
    }
}

// ============================================================================
// Message: AllocationReport (MsgType=AS)
// ============================================================================

/// AllocationReport (MsgType=AS)
///
/// Report of allocation details, typically sent from buy-side to sell-side.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationReport {
    /// AllocReportID (755) - Unique identifier for allocation report
    pub alloc_report_id: String,

    /// AllocID (70) - Allocation ID
    pub alloc_id: String,

    /// AllocTransType (71) - Identifies allocation transaction type
    pub alloc_trans_type: AllocTransType,

    /// AllocReportType (794) - Type of allocation report
    pub alloc_report_type: AllocReportType,

    /// AllocStatus (87) - Status of allocation
    pub alloc_status: AllocStatus,

    /// Side (54) - Side of order
    pub side: Side,

    /// Symbol (55) - Instrument symbol
    pub symbol: String,

    /// Quantity (53) - Total quantity allocated
    pub quantity: f64,

    /// AvgPx (6) - Average price
    pub avg_px: f64,

    /// TradeDate (75) - Trade date
    pub trade_date: String,

    /// TransactTime (60) - Transaction time
    pub transact_time: DateTime<Utc>,

    /// RefAllocID (72) - Reference allocation ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_alloc_id: Option<String>,

    /// Currency (15) - Currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// AllocRejCode (88) - Rejection code (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_rej_code: Option<AllocRejCode>,

    /// Text (58) - Free text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// AllocGrp - Repeating group of allocation details
    pub allocations: Vec<AllocGroup>,
}

impl AllocationReport {
    pub fn new(
        alloc_report_id: String,
        alloc_id: String,
        alloc_trans_type: AllocTransType,
        alloc_report_type: AllocReportType,
        alloc_status: AllocStatus,
        side: Side,
        symbol: String,
        quantity: f64,
        avg_px: f64,
        trade_date: String,
        transact_time: DateTime<Utc>,
    ) -> Self {
        Self {
            alloc_report_id,
            alloc_id,
            alloc_trans_type,
            alloc_report_type,
            alloc_status,
            side,
            symbol,
            quantity,
            avg_px,
            trade_date,
            transact_time,
            ref_alloc_id: None,
            currency: None,
            alloc_rej_code: None,
            text: None,
            allocations: Vec::new(),
        }
    }

    pub fn add_allocation(mut self, allocation: AllocGroup) -> Self {
        self.allocations.push(allocation);
        self
    }

    pub fn with_ref_alloc_id(mut self, ref_alloc_id: String) -> Self {
        self.ref_alloc_id = Some(ref_alloc_id);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "AS".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(755, self.alloc_report_id.clone());
        msg.set_field(70, self.alloc_id.clone());
        msg.set_field(71, self.alloc_trans_type.to_fix().to_string());
        msg.set_field(794, self.alloc_report_type.to_fix().to_string());
        msg.set_field(87, self.alloc_status.to_fix().to_string());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(55, self.symbol.clone());
        msg.set_field(53, self.quantity.to_string());
        msg.set_field(6, self.avg_px.to_string());
        msg.set_field(75, self.trade_date.clone());
        msg.set_field(
            60,
            self.transact_time
                .format("%Y%m%d-%H:%M:%S%.3f")
                .to_string(),
        );

        if let Some(ref ref_alloc_id) = self.ref_alloc_id {
            msg.set_field(72, ref_alloc_id.clone());
        }
        if let Some(ref currency) = self.currency {
            msg.set_field(15, currency.clone());
        }
        if let Some(ref rej_code) = self.alloc_rej_code {
            msg.set_field(88, rej_code.to_fix().to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let alloc_report_id = raw
            .get_field(755)
            .ok_or(FixParseError::MissingRequiredField(755))?
            .to_string();

        let alloc_id = raw
            .get_field(70)
            .ok_or(FixParseError::MissingRequiredField(70))?
            .to_string();

        let alloc_trans_type_str = raw
            .get_field(71)
            .ok_or(FixParseError::MissingRequiredField(71))?;
        let alloc_trans_type = AllocTransType::from_fix(alloc_trans_type_str).map_err(|e| {
            FixParseError::InvalidValue {
                tag: 71,
                value: alloc_trans_type_str.to_string(),
                error: e,
            }
        })?;

        let alloc_report_type_str = raw
            .get_field(794)
            .ok_or(FixParseError::MissingRequiredField(794))?;
        let alloc_report_type =
            AllocReportType::from_fix(alloc_report_type_str).map_err(|e| {
                FixParseError::InvalidValue {
                    tag: 794,
                    value: alloc_report_type_str.to_string(),
                    error: e,
                }
            })?;

        let alloc_status_str = raw
            .get_field(87)
            .ok_or(FixParseError::MissingRequiredField(87))?;
        let alloc_status = AllocStatus::from_fix(alloc_status_str).map_err(|e| {
            FixParseError::InvalidValue {
                tag: 87,
                value: alloc_status_str.to_string(),
                error: e,
            }
        })?;

        let side_char = raw
            .get_field(54)
            .ok_or(FixParseError::MissingRequiredField(54))?
            .chars()
            .next()
            .ok_or(FixParseError::InvalidValue {
                tag: 54,
                value: "".to_string(),
                error: "Empty side".to_string(),
            })?;
        let side = Side::from_fix(side_char).ok_or(FixParseError::InvalidValue {
            tag: 54,
            value: side_char.to_string(),
            error: "Invalid side".to_string(),
        })?;

        let symbol = raw
            .get_field(55)
            .ok_or(FixParseError::MissingRequiredField(55))?
            .to_string();

        let quantity: f64 = raw
            .get_field_as(53)
            .ok_or(FixParseError::MissingRequiredField(53))?;

        let avg_px: f64 = raw
            .get_field_as(6)
            .ok_or(FixParseError::MissingRequiredField(6))?;

        let trade_date = raw
            .get_field(75)
            .ok_or(FixParseError::MissingRequiredField(75))?
            .to_string();

        let transact_time_str = raw
            .get_field(60)
            .ok_or(FixParseError::MissingRequiredField(60))?;
        let naive_time = chrono::NaiveDateTime::parse_from_str(
            transact_time_str,
            "%Y%m%d-%H:%M:%S%.3f",
        )
        .map_err(|e| FixParseError::InvalidValue {
            tag: 60,
            value: transact_time_str.to_string(),
            error: e.to_string(),
        })?;
        let transact_time = DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc);

        let ref_alloc_id = raw.get_field(72).map(|s| s.to_string());
        let currency = raw.get_field(15).map(|s| s.to_string());
        let text = raw.get_field(58).map(|s| s.to_string());

        let alloc_rej_code = if let Some(code_str) = raw.get_field(88) {
            Some(AllocRejCode::from_fix(code_str).map_err(|e| {
                FixParseError::InvalidValue {
                    tag: 88,
                    value: code_str.to_string(),
                    error: e,
                }
            })?)
        } else {
            None
        };

        Ok(AllocationReport {
            alloc_report_id,
            alloc_id,
            alloc_trans_type,
            alloc_report_type,
            alloc_status,
            side,
            symbol,
            quantity,
            avg_px,
            trade_date,
            transact_time,
            ref_alloc_id,
            currency,
            alloc_rej_code,
            text,
            allocations: Vec::new(),
        })
    }
}

// ============================================================================
// Message: AllocationReportAck (MsgType=AT)
// ============================================================================

/// AllocationReportAck (MsgType=AT)
///
/// Acknowledgement of allocation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationReportAck {
    /// AllocReportID (755) - Allocation report ID being acknowledged
    pub alloc_report_id: String,

    /// AllocID (70) - Allocation ID
    pub alloc_id: String,

    /// AllocStatus (87) - Status of allocation
    pub alloc_status: AllocStatus,

    /// AllocRejCode (88) - Rejection code (required if rejected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_rej_code: Option<AllocRejCode>,

    /// TransactTime (60) - Transaction time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<DateTime<Utc>>,

    /// Text (58) - Free text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl AllocationReportAck {
    pub fn new(
        alloc_report_id: String,
        alloc_id: String,
        alloc_status: AllocStatus,
    ) -> Self {
        Self {
            alloc_report_id,
            alloc_id,
            alloc_status,
            alloc_rej_code: None,
            transact_time: None,
            text: None,
        }
    }

    pub fn with_alloc_rej_code(mut self, code: AllocRejCode) -> Self {
        self.alloc_rej_code = Some(code);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "AT".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(755, self.alloc_report_id.clone());
        msg.set_field(70, self.alloc_id.clone());
        msg.set_field(87, self.alloc_status.to_fix().to_string());

        if let Some(ref rej_code) = self.alloc_rej_code {
            msg.set_field(88, rej_code.to_fix().to_string());
        }
        if let Some(transact_time) = self.transact_time {
            msg.set_field(60, transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let alloc_report_id = raw
            .get_field(755)
            .ok_or(FixParseError::MissingRequiredField(755))?
            .to_string();

        let alloc_id = raw
            .get_field(70)
            .ok_or(FixParseError::MissingRequiredField(70))?
            .to_string();

        let alloc_status_str = raw
            .get_field(87)
            .ok_or(FixParseError::MissingRequiredField(87))?;
        let alloc_status = AllocStatus::from_fix(alloc_status_str).map_err(|e| {
            FixParseError::InvalidValue {
                tag: 87,
                value: alloc_status_str.to_string(),
                error: e,
            }
        })?;

        let alloc_rej_code = if let Some(code_str) = raw.get_field(88) {
            Some(AllocRejCode::from_fix(code_str).map_err(|e| {
                FixParseError::InvalidValue {
                    tag: 88,
                    value: code_str.to_string(),
                    error: e,
                }
            })?)
        } else {
            None
        };

        let transact_time = if let Some(time_str) = raw.get_field(60) {
            let naive_time =
                chrono::NaiveDateTime::parse_from_str(time_str, "%Y%m%d-%H:%M:%S%.3f")
                    .map_err(|e| FixParseError::InvalidValue {
                        tag: 60,
                        value: time_str.to_string(),
                        error: e.to_string(),
                    })?;
            Some(DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc))
        } else {
            None
        };

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(AllocationReportAck {
            alloc_report_id,
            alloc_id,
            alloc_status,
            alloc_rej_code,
            transact_time,
            text,
        })
    }
}

// ============================================================================
// Message: AllocationInstructionAlert (MsgType=BM)
// ============================================================================

/// AllocationInstructionAlert (MsgType=BM)
///
/// Alert regarding an allocation instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInstructionAlert {
    /// AllocID (70) - Allocation ID being alerted
    pub alloc_id: String,

    /// MatchStatus (573) - Status of match
    pub match_status: MatchStatus,

    /// AllocTransType (71) - Identifies allocation transaction type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_trans_type: Option<AllocTransType>,

    /// AllocType (626) - Purpose of allocation message (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_type: Option<AllocType>,

    /// Side (54) - Side of order (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,

    /// Symbol (55) - Instrument symbol (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// Quantity (53) - Total quantity (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    /// AvgPx (6) - Average price (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<f64>,

    /// TradeDate (75) - Trade date (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<String>,

    /// TransactTime (60) - Transaction time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<DateTime<Utc>>,

    /// Text (58) - Free text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl AllocationInstructionAlert {
    pub fn new(alloc_id: String, match_status: MatchStatus) -> Self {
        Self {
            alloc_id,
            match_status,
            alloc_trans_type: None,
            alloc_type: None,
            side: None,
            symbol: None,
            quantity: None,
            avg_px: None,
            trade_date: None,
            transact_time: None,
            text: None,
        }
    }

    pub fn with_side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "BM".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(70, self.alloc_id.clone());
        msg.set_field(573, self.match_status.to_fix().to_string());

        if let Some(alloc_trans_type) = self.alloc_trans_type {
            msg.set_field(71, alloc_trans_type.to_fix().to_string());
        }
        if let Some(alloc_type) = self.alloc_type {
            msg.set_field(626, alloc_type.to_fix().to_string());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(quantity) = self.quantity {
            msg.set_field(53, quantity.to_string());
        }
        if let Some(avg_px) = self.avg_px {
            msg.set_field(6, avg_px.to_string());
        }
        if let Some(ref trade_date) = self.trade_date {
            msg.set_field(75, trade_date.clone());
        }
        if let Some(transact_time) = self.transact_time {
            msg.set_field(60, transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let alloc_id = raw
            .get_field(70)
            .ok_or(FixParseError::MissingRequiredField(70))?
            .to_string();

        let match_status_str = raw
            .get_field(573)
            .ok_or(FixParseError::MissingRequiredField(573))?;
        let match_status = MatchStatus::from_fix(match_status_str).map_err(|e| {
            FixParseError::InvalidValue {
                tag: 573,
                value: match_status_str.to_string(),
                error: e,
            }
        })?;

        let alloc_trans_type = if let Some(type_str) = raw.get_field(71) {
            Some(AllocTransType::from_fix(type_str).map_err(|e| {
                FixParseError::InvalidValue {
                    tag: 71,
                    value: type_str.to_string(),
                    error: e,
                }
            })?)
        } else {
            None
        };

        let alloc_type = if let Some(type_str) = raw.get_field(626) {
            Some(AllocType::from_fix(type_str).map_err(|e| {
                FixParseError::InvalidValue {
                    tag: 626,
                    value: type_str.to_string(),
                    error: e,
                }
            })?)
        } else {
            None
        };

        let side = if let Some(side_str) = raw.get_field(54) {
            let side_char = side_str.chars().next().ok_or(FixParseError::InvalidValue {
                tag: 54,
                value: "".to_string(),
                error: "Empty side".to_string(),
            })?;
            Some(Side::from_fix(side_char).ok_or(FixParseError::InvalidValue {
                tag: 54,
                value: side_char.to_string(),
                error: "Invalid side".to_string(),
            })?)
        } else {
            None
        };

        let symbol = raw.get_field(55).map(|s| s.to_string());
        let quantity = raw.get_field_as(53);
        let avg_px = raw.get_field_as(6);
        let trade_date = raw.get_field(75).map(|s| s.to_string());

        let transact_time = if let Some(time_str) = raw.get_field(60) {
            let naive_time =
                chrono::NaiveDateTime::parse_from_str(time_str, "%Y%m%d-%H:%M:%S%.3f")
                    .map_err(|e| FixParseError::InvalidValue {
                        tag: 60,
                        value: time_str.to_string(),
                        error: e.to_string(),
                    })?;
            Some(DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc))
        } else {
            None
        };

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(AllocationInstructionAlert {
            alloc_id,
            match_status,
            alloc_trans_type,
            alloc_type,
            side,
            symbol,
            quantity,
            avg_px,
            trade_date,
            transact_time,
            text,
        })
    }
}

// ============================================================================
// Message: AllocationInstructionAlertRequest (MsgType=DU)
// ============================================================================

/// AllocationInstructionAlertRequest (MsgType=DU)
///
/// Request for allocation instruction alerts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInstructionAlertRequest {
    /// AllocRequestID (2758) - Unique identifier for this request
    pub alloc_request_id: String,

    /// AllocID (70) - Allocation ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_id: Option<String>,

    /// Side (54) - Side of order (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,

    /// Symbol (55) - Instrument symbol (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// TradeDate (75) - Trade date (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<String>,

    /// TransactTime (60) - Transaction time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<DateTime<Utc>>,

    /// Text (58) - Free text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl AllocationInstructionAlertRequest {
    pub fn new(alloc_request_id: String) -> Self {
        Self {
            alloc_request_id,
            alloc_id: None,
            side: None,
            symbol: None,
            trade_date: None,
            transact_time: None,
            text: None,
        }
    }

    pub fn with_alloc_id(mut self, alloc_id: String) -> Self {
        self.alloc_id = Some(alloc_id);
        self
    }

    pub fn with_side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "DU".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(2758, self.alloc_request_id.clone());

        if let Some(ref alloc_id) = self.alloc_id {
            msg.set_field(70, alloc_id.clone());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(ref trade_date) = self.trade_date {
            msg.set_field(75, trade_date.clone());
        }
        if let Some(transact_time) = self.transact_time {
            msg.set_field(60, transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let alloc_request_id = raw
            .get_field(2758)
            .ok_or(FixParseError::MissingRequiredField(2758))?
            .to_string();

        let alloc_id = raw.get_field(70).map(|s| s.to_string());

        let side = if let Some(side_str) = raw.get_field(54) {
            let side_char = side_str.chars().next().ok_or(FixParseError::InvalidValue {
                tag: 54,
                value: "".to_string(),
                error: "Empty side".to_string(),
            })?;
            Some(Side::from_fix(side_char).ok_or(FixParseError::InvalidValue {
                tag: 54,
                value: side_char.to_string(),
                error: "Invalid side".to_string(),
            })?)
        } else {
            None
        };

        let symbol = raw.get_field(55).map(|s| s.to_string());
        let trade_date = raw.get_field(75).map(|s| s.to_string());

        let transact_time = if let Some(time_str) = raw.get_field(60) {
            let naive_time =
                chrono::NaiveDateTime::parse_from_str(time_str, "%Y%m%d-%H:%M:%S%.3f")
                    .map_err(|e| FixParseError::InvalidValue {
                        tag: 60,
                        value: time_str.to_string(),
                        error: e.to_string(),
                    })?;
            Some(DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc))
        } else {
            None
        };

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(AllocationInstructionAlertRequest {
            alloc_request_id,
            alloc_id,
            side,
            symbol,
            trade_date,
            transact_time,
            text,
        })
    }
}

// ============================================================================
// Message: AllocationInstructionAlertRequestAck (MsgType=DV)
// ============================================================================

/// AllocationInstructionAlertRequestAck (MsgType=DV)
///
/// Acknowledgement of allocation instruction alert request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInstructionAlertRequestAck {
    /// AllocRequestID (2758) - Request ID being acknowledged
    pub alloc_request_id: String,

    /// AllocRequestStatus (2768) - Status of request
    pub alloc_request_status: AllocRequestStatus,

    /// AllocRejCode (88) - Rejection code (required if rejected)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_rej_code: Option<AllocRejCode>,

    /// TransactTime (60) - Transaction time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<DateTime<Utc>>,

    /// Text (58) - Free text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl AllocationInstructionAlertRequestAck {
    pub fn new(
        alloc_request_id: String,
        alloc_request_status: AllocRequestStatus,
    ) -> Self {
        Self {
            alloc_request_id,
            alloc_request_status,
            alloc_rej_code: None,
            transact_time: None,
            text: None,
        }
    }

    pub fn with_alloc_rej_code(mut self, code: AllocRejCode) -> Self {
        self.alloc_rej_code = Some(code);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "DV".to_string());
        msg.set_field(1128, "9".to_string());
        msg.set_field(2758, self.alloc_request_id.clone());
        msg.set_field(2768, self.alloc_request_status.to_fix().to_string());

        if let Some(ref rej_code) = self.alloc_rej_code {
            msg.set_field(88, rej_code.to_fix().to_string());
        }
        if let Some(transact_time) = self.transact_time {
            msg.set_field(60, transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let alloc_request_id = raw
            .get_field(2758)
            .ok_or(FixParseError::MissingRequiredField(2758))?
            .to_string();

        let alloc_request_status_str = raw
            .get_field(2768)
            .ok_or(FixParseError::MissingRequiredField(2768))?;
        let alloc_request_status =
            AllocRequestStatus::from_fix(alloc_request_status_str).map_err(|e| {
                FixParseError::InvalidValue {
                    tag: 2768,
                    value: alloc_request_status_str.to_string(),
                    error: e,
                }
            })?;

        let alloc_rej_code = if let Some(code_str) = raw.get_field(88) {
            Some(AllocRejCode::from_fix(code_str).map_err(|e| {
                FixParseError::InvalidValue {
                    tag: 88,
                    value: code_str.to_string(),
                    error: e,
                }
            })?)
        } else {
            None
        };

        let transact_time = if let Some(time_str) = raw.get_field(60) {
            let naive_time =
                chrono::NaiveDateTime::parse_from_str(time_str, "%Y%m%d-%H:%M:%S%.3f")
                    .map_err(|e| FixParseError::InvalidValue {
                        tag: 60,
                        value: time_str.to_string(),
                        error: e.to_string(),
                    })?;
            Some(DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc))
        } else {
            None
        };

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(AllocationInstructionAlertRequestAck {
            alloc_request_id,
            alloc_request_status,
            alloc_rej_code,
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

    #[test]
    fn test_alloc_group_creation() {
        let alloc = AllocGroup::new("ACCT1".to_string(), 100.0).with_price(50.25);

        assert_eq!(alloc.alloc_account, "ACCT1");
        assert_eq!(alloc.alloc_qty, 100.0);
        assert_eq!(alloc.alloc_price, Some(50.25));
    }

    #[test]
    fn test_allocation_instruction_creation() {
        let alloc1 = AllocGroup::new("ACCT1".to_string(), 100.0).with_price(50.25);
        let alloc2 = AllocGroup::new("ACCT2".to_string(), 200.0).with_price(50.26);

        let msg = AllocationInstruction::new(
            "ALLOC123".to_string(),
            AllocTransType::New,
            AllocType::Calculated,
            Side::Buy,
            "AAPL".to_string(),
            300.0,
            50.255,
            "20250114".to_string(),
            Utc::now(),
        )
        .add_allocation(alloc1)
        .add_allocation(alloc2)
        .with_currency("USD".to_string());

        assert_eq!(msg.alloc_id, "ALLOC123");
        assert_eq!(msg.symbol, "AAPL");
        assert_eq!(msg.allocations.len(), 2);
        assert_eq!(msg.allocations[0].alloc_account, "ACCT1");
        assert_eq!(msg.allocations[1].alloc_qty, 200.0);
        assert_eq!(msg.currency, Some("USD".to_string()));
    }

    #[test]
    fn test_allocation_instruction_to_raw() {
        let msg = AllocationInstruction::new(
            "ALLOC456".to_string(),
            AllocTransType::New,
            AllocType::Calculated,
            Side::Sell,
            "MSFT".to_string(),
            500.0,
            100.0,
            "20250114".to_string(),
            DateTime::from_naive_utc_and_offset(
                chrono::NaiveDateTime::parse_from_str(
                    "20250114-10:30:00.000",
                    "%Y%m%d-%H:%M:%S%.3f",
                )
                .unwrap(),
                Utc,
            ),
        )
        .with_text("Test allocation".to_string());

        let raw = msg.to_raw();
        assert_eq!(raw.get_field(35), Some("J"));
        assert_eq!(raw.get_field(70), Some("ALLOC456"));
        assert_eq!(raw.get_field(71), Some("0")); // AllocTransType::New
        assert_eq!(raw.get_field(626), Some("1")); // AllocType::Calculated
        assert_eq!(raw.get_field(54), Some("2")); // Side::Sell
        assert_eq!(raw.get_field(55), Some("MSFT"));
        assert_eq!(raw.get_field(58), Some("Test allocation"));
    }

    #[test]
    fn test_allocation_instruction_ack_creation() {
        let msg = AllocationInstructionAck::new(
            "ALLOC123".to_string(),
            AllocStatus::Accepted,
        )
        .with_text("Allocation accepted".to_string());

        assert_eq!(msg.alloc_id, "ALLOC123");
        assert_eq!(msg.alloc_status, AllocStatus::Accepted);
        assert_eq!(msg.text, Some("Allocation accepted".to_string()));
    }

    #[test]
    fn test_allocation_instruction_ack_rejected() {
        let msg = AllocationInstructionAck::new(
            "ALLOC789".to_string(),
            AllocStatus::BlockLevelReject,
        )
        .with_alloc_rej_code(AllocRejCode::UnknownAccount)
        .with_text("Unknown account".to_string());

        assert_eq!(msg.alloc_status, AllocStatus::BlockLevelReject);
        assert_eq!(msg.alloc_rej_code, Some(AllocRejCode::UnknownAccount));
    }

    #[test]
    fn test_allocation_report_creation() {
        let msg = AllocationReport::new(
            "REPT001".to_string(),
            "ALLOC123".to_string(),
            AllocTransType::New,
            AllocReportType::Accept,
            AllocStatus::Accepted,
            Side::Buy,
            "GOOGL".to_string(),
            1000.0,
            150.50,
            "20250114".to_string(),
            Utc::now(),
        )
        .with_ref_alloc_id("ORIGALLOC".to_string());

        assert_eq!(msg.alloc_report_id, "REPT001");
        assert_eq!(msg.alloc_id, "ALLOC123");
        assert_eq!(msg.ref_alloc_id, Some("ORIGALLOC".to_string()));
    }

    #[test]
    fn test_allocation_report_ack_creation() {
        let msg = AllocationReportAck::new(
            "REPT001".to_string(),
            "ALLOC123".to_string(),
            AllocStatus::Accepted,
        )
        .with_text("Report acknowledged".to_string());

        assert_eq!(msg.alloc_report_id, "REPT001");
        assert_eq!(msg.alloc_id, "ALLOC123");
        assert_eq!(msg.alloc_status, AllocStatus::Accepted);
    }

    #[test]
    fn test_allocation_instruction_alert_creation() {
        let msg = AllocationInstructionAlert::new(
            "ALLOC123".to_string(),
            MatchStatus::ComparedMatchedOrAffirmed,
        )
        .with_side(Side::Buy)
        .with_symbol("TSLA".to_string())
        .with_text("Allocation alert".to_string());

        assert_eq!(msg.alloc_id, "ALLOC123");
        assert_eq!(msg.match_status, MatchStatus::ComparedMatchedOrAffirmed);
        assert_eq!(msg.side, Some(Side::Buy));
        assert_eq!(msg.symbol, Some("TSLA".to_string()));
    }

    #[test]
    fn test_allocation_instruction_alert_request_creation() {
        let msg = AllocationInstructionAlertRequest::new("REQ001".to_string())
            .with_alloc_id("ALLOC123".to_string())
            .with_symbol("NVDA".to_string())
            .with_text("Request for alerts".to_string());

        assert_eq!(msg.alloc_request_id, "REQ001");
        assert_eq!(msg.alloc_id, Some("ALLOC123".to_string()));
        assert_eq!(msg.symbol, Some("NVDA".to_string()));
    }

    #[test]
    fn test_allocation_instruction_alert_request_ack_creation() {
        let msg = AllocationInstructionAlertRequestAck::new(
            "REQ001".to_string(),
            AllocRequestStatus::Accepted,
        )
        .with_text("Request accepted".to_string());

        assert_eq!(msg.alloc_request_id, "REQ001");
        assert_eq!(msg.alloc_request_status, AllocRequestStatus::Accepted);
        assert_eq!(msg.text, Some("Request accepted".to_string()));
    }

    #[test]
    fn test_allocation_instruction_alert_request_ack_rejected() {
        let msg = AllocationInstructionAlertRequestAck::new(
            "REQ002".to_string(),
            AllocRequestStatus::Rejected,
        )
        .with_alloc_rej_code(AllocRejCode::Other)
        .with_text("Invalid request".to_string());

        assert_eq!(msg.alloc_request_status, AllocRequestStatus::Rejected);
        assert_eq!(msg.alloc_rej_code, Some(AllocRejCode::Other));
    }

    #[test]
    fn test_exec_alloc_group_creation() {
        let exec = ExecAllocGroup::new(100.0, 50.25, "EXEC123".to_string());

        assert_eq!(exec.last_qty, 100.0);
        assert_eq!(exec.last_px, 50.25);
        assert_eq!(exec.exec_id, "EXEC123");
    }

    #[test]
    fn test_allocation_instruction_with_executions() {
        let exec1 = ExecAllocGroup::new(150.0, 50.25, "EXEC001".to_string());
        let exec2 = ExecAllocGroup::new(150.0, 50.30, "EXEC002".to_string());

        let msg = AllocationInstruction::new(
            "ALLOC999".to_string(),
            AllocTransType::New,
            AllocType::Calculated,
            Side::Buy,
            "IBM".to_string(),
            300.0,
            50.275,
            "20250114".to_string(),
            Utc::now(),
        )
        .add_execution(exec1)
        .add_execution(exec2);

        assert_eq!(msg.executions.len(), 2);
        assert_eq!(msg.executions[0].exec_id, "EXEC001");
        assert_eq!(msg.executions[1].last_px, 50.30);
    }
}
