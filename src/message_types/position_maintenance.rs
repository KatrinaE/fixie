use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Position Maintenance Messages
// Implementation of FIX 5.0 SP2 Post-Trade Position Maintenance messages
// ============================================================================

// ============================================================================
// Component Structures (Repeating Groups)
// ============================================================================

/// Entry in the PositionQty repeating group (NoPositions=702)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionQtyEntry {
    /// PosType (703) - Type of position
    pub pos_type: PosType,
    /// LongQty (704) - Long quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_qty: Option<f64>,
    /// ShortQty (705) - Short quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_qty: Option<f64>,
    /// PosQtyStatus (706) - Status of this position quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_qty_status: Option<PosQtyStatus>,
    /// QuantityDate (539) - Date associated with quantity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_date: Option<String>,
    /// PositionCurrency (1055) - Currency of position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_currency: Option<String>,
}

impl PositionQtyEntry {
    pub fn new(pos_type: PosType) -> Self {
        Self {
            pos_type,
            long_qty: None,
            short_qty: None,
            pos_qty_status: None,
            quantity_date: None,
            position_currency: None,
        }
    }

    pub fn with_long_qty(mut self, qty: f64) -> Self {
        self.long_qty = Some(qty);
        self
    }

    pub fn with_short_qty(mut self, qty: f64) -> Self {
        self.short_qty = Some(qty);
        self
    }
}

/// Entry in the PositionAmountData repeating group (NoPosAmt=753)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionAmtEntry {
    /// PosAmtType (707) - Type of position amount
    pub pos_amt_type: PosAmtType,
    /// PosAmt (708) - Position amount value
    pub pos_amt: f64,
    /// PositionCurrency (1055) - Currency of amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_currency: Option<String>,
}

impl PositionAmtEntry {
    pub fn new(pos_amt_type: PosAmtType, pos_amt: f64) -> Self {
        Self {
            pos_amt_type,
            pos_amt,
            position_currency: None,
        }
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.position_currency = Some(currency);
        self
    }
}

// ============================================================================
// Message: RequestForPositions (AN)
// ============================================================================

/// RequestForPositions (MsgType=AN)
///
/// Request position data from the holder of positions (e.g., clearing organization,
/// central counterparty, or broker).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestForPositions {
    /// PosReqID (710) - Unique identifier for position request
    pub pos_req_id: String,
    /// PosReqType (724) - Type of position request
    pub pos_req_type: PosReqType,
    /// ClearingBusinessDate (715) - Business date for which positions are requested
    pub clearing_business_date: String,
    /// TransactTime (60) - Time of request
    pub transact_time: String,
    /// Account (1) - Account for which positions are requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// AcctIDSource (660) - Source of account identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_id_source: Option<String>,
    /// AccountType (581) - Type of account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Symbol (55) - Instrument symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// Currency (15) - Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// SubscriptionRequestType (263) - Subscription request type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_request_type: Option<char>,
    /// TradingSessionID (336) - Trading session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl RequestForPositions {
    pub fn new(
        pos_req_id: String,
        pos_req_type: PosReqType,
        clearing_business_date: String,
        transact_time: String,
    ) -> Self {
        Self {
            pos_req_id,
            pos_req_type,
            clearing_business_date,
            transact_time,
            account: None,
            acct_id_source: None,
            account_type: None,
            symbol: None,
            security_id: None,
            security_id_source: None,
            currency: None,
            subscription_request_type: None,
            trading_session_id: None,
            text: None,
        }
    }

    pub fn with_account(mut self, account: String) -> Self {
        self.account = Some(account);
        self
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

// ============================================================================
// Message: RequestForPositionsAck (AO)
// ============================================================================

/// RequestForPositionsAck (MsgType=AO)
///
/// Acknowledgment of RequestForPositions message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestForPositionsAck {
    /// PosMaintRptID (721) - Unique identifier for this acknowledgment
    pub pos_maint_rpt_id: String,
    /// PosReqResult (728) - Result of position request
    pub pos_req_result: PosReqResult,
    /// PosReqStatus (729) - Status of position request
    pub pos_req_status: PosReqStatus,
    /// PosReqID (710) - Identifier from original request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_req_id: Option<String>,
    /// TotalNumPosReports (727) - Total number of position reports expected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_pos_reports: Option<i32>,
    /// Account (1) - Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// AccountType (581) - Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl RequestForPositionsAck {
    pub fn new(
        pos_maint_rpt_id: String,
        pos_req_result: PosReqResult,
        pos_req_status: PosReqStatus,
    ) -> Self {
        Self {
            pos_maint_rpt_id,
            pos_req_result,
            pos_req_status,
            pos_req_id: None,
            total_num_pos_reports: None,
            account: None,
            account_type: None,
            text: None,
        }
    }

    pub fn with_pos_req_id(mut self, id: String) -> Self {
        self.pos_req_id = Some(id);
        self
    }

    pub fn with_total_num_pos_reports(mut self, total: i32) -> Self {
        self.total_num_pos_reports = Some(total);
        self
    }
}

// ============================================================================
// Message: PositionMaintenanceRequest (AL)
// ============================================================================

/// PositionMaintenanceRequest (MsgType=AL)
///
/// Request to perform specific actions that will affect position quantity
/// or position amount values.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionMaintenanceRequest {
    /// PosTransType (709) - Type of position transaction
    pub pos_trans_type: PosTransType,
    /// PosMaintAction (712) - Maintenance action to be performed
    pub pos_maint_action: PosMaintAction,
    /// ClearingBusinessDate (715) - Business date for position maintenance
    pub clearing_business_date: String,
    /// OrigPosReqRefID (713) - Reference to original position request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_pos_req_ref_id: Option<String>,
    /// PosMaintRptRefID (714) - Reference to position maintenance report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_maint_rpt_ref_id: Option<String>,
    /// Account (1) - Account
    pub account: String,
    /// AcctIDSource (660) - Account ID source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_id_source: Option<String>,
    /// AccountType (581) - Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Symbol (55) - Instrument symbol
    pub symbol: String,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// TransactTime (60) - Transaction time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,
    /// AdjustmentType (718) - Type of adjustment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<AdjustmentType>,
    /// ThresholdAmount (834) - Threshold amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_amount: Option<f64>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// PositionQty - Repeating group of position quantities
    pub positions: Vec<PositionQtyEntry>,
    /// PositionAmountData - Repeating group of position amounts
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub position_amounts: Vec<PositionAmtEntry>,
}

impl PositionMaintenanceRequest {
    pub fn new(
        pos_trans_type: PosTransType,
        pos_maint_action: PosMaintAction,
        clearing_business_date: String,
        account: String,
        symbol: String,
    ) -> Self {
        Self {
            pos_trans_type,
            pos_maint_action,
            clearing_business_date,
            orig_pos_req_ref_id: None,
            pos_maint_rpt_ref_id: None,
            account,
            acct_id_source: None,
            account_type: None,
            symbol,
            security_id: None,
            security_id_source: None,
            transact_time: None,
            adjustment_type: None,
            threshold_amount: None,
            text: None,
            positions: Vec::new(),
            position_amounts: Vec::new(),
        }
    }

    pub fn add_position(mut self, position: PositionQtyEntry) -> Self {
        self.positions.push(position);
        self
    }

    pub fn add_position_amount(mut self, amount: PositionAmtEntry) -> Self {
        self.position_amounts.push(amount);
        self
    }

    pub fn with_security_id(mut self, id: String, source: String) -> Self {
        self.security_id = Some(id);
        self.security_id_source = Some(source);
        self
    }
}

// ============================================================================
// Message: PositionMaintenanceReport (AM)
// ============================================================================

/// PositionMaintenanceReport (MsgType=AM)
///
/// Report sent by position holder in response to PositionMaintenanceRequest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionMaintenanceReport {
    /// PosMaintRptID (721) - Unique identifier for this report
    pub pos_maint_rpt_id: String,
    /// PosTransType (709) - Type of position transaction
    pub pos_trans_type: PosTransType,
    /// PosMaintAction (712) - Maintenance action performed
    pub pos_maint_action: PosMaintAction,
    /// PosMaintResult (723) - Result of maintenance action
    pub pos_maint_result: PosMaintResult,
    /// ClearingBusinessDate (715) - Business date
    pub clearing_business_date: String,
    /// OrigPosReqRefID (713) - Reference to original request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orig_pos_req_ref_id: Option<String>,
    /// PosMaintRptRefID (714) - Reference to position maintenance report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_maint_rpt_ref_id: Option<String>,
    /// Account (1) - Account
    pub account: String,
    /// AccountType (581) - Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Symbol (55) - Instrument symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// Currency (15) - Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// SettlPrice (730) - Settlement price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_price: Option<f64>,
    /// SettlPriceType (731) - Settlement price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_price_type: Option<SettlPriceType>,
    /// PriorSettlPrice (734) - Prior settlement price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_settl_price: Option<f64>,
    /// TransactTime (60) - Transaction time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// PositionQty - Repeating group of position quantities
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub positions: Vec<PositionQtyEntry>,
    /// PositionAmountData - Repeating group of position amounts
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub position_amounts: Vec<PositionAmtEntry>,
}

impl PositionMaintenanceReport {
    pub fn new(
        pos_maint_rpt_id: String,
        pos_trans_type: PosTransType,
        pos_maint_action: PosMaintAction,
        pos_maint_result: PosMaintResult,
        clearing_business_date: String,
        account: String,
    ) -> Self {
        Self {
            pos_maint_rpt_id,
            pos_trans_type,
            pos_maint_action,
            pos_maint_result,
            clearing_business_date,
            orig_pos_req_ref_id: None,
            pos_maint_rpt_ref_id: None,
            account,
            account_type: None,
            symbol: None,
            security_id: None,
            security_id_source: None,
            currency: None,
            settl_price: None,
            settl_price_type: None,
            prior_settl_price: None,
            transact_time: None,
            text: None,
            positions: Vec::new(),
            position_amounts: Vec::new(),
        }
    }

    pub fn add_position(mut self, position: PositionQtyEntry) -> Self {
        self.positions.push(position);
        self
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn with_settl_price(mut self, price: f64, price_type: SettlPriceType) -> Self {
        self.settl_price = Some(price);
        self.settl_price_type = Some(price_type);
        self
    }
}

// ============================================================================
// Message: PositionReport (AP)
// ============================================================================

/// PositionReport (MsgType=AP)
///
/// Report of position data sent in response to RequestForPositions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionReport {
    /// PosMaintRptID (721) - Unique identifier for this report
    pub pos_maint_rpt_id: String,
    /// ClearingBusinessDate (715) - Business date
    pub clearing_business_date: String,
    /// PosReqID (710) - Reference to position request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_req_id: Option<String>,
    /// PosReqType (724) - Type of position request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_req_type: Option<PosReqType>,
    /// Account (1) - Account
    pub account: String,
    /// AccountType (581) - Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Symbol (55) - Instrument symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// Currency (15) - Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// SettlPrice (730) - Settlement price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_price: Option<f64>,
    /// SettlPriceType (731) - Settlement price type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_price_type: Option<SettlPriceType>,
    /// PriorSettlPrice (734) - Prior settlement price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_settl_price: Option<f64>,
    /// TransactTime (60) - Transaction time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// PositionQty - Repeating group of position quantities
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub positions: Vec<PositionQtyEntry>,
    /// PositionAmountData - Repeating group of position amounts
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub position_amounts: Vec<PositionAmtEntry>,
}

impl PositionReport {
    pub fn new(
        pos_maint_rpt_id: String,
        clearing_business_date: String,
        account: String,
    ) -> Self {
        Self {
            pos_maint_rpt_id,
            clearing_business_date,
            pos_req_id: None,
            pos_req_type: None,
            account,
            account_type: None,
            symbol: None,
            security_id: None,
            security_id_source: None,
            currency: None,
            settl_price: None,
            settl_price_type: None,
            prior_settl_price: None,
            transact_time: None,
            text: None,
            positions: Vec::new(),
            position_amounts: Vec::new(),
        }
    }

    pub fn with_pos_req_id(mut self, id: String, req_type: PosReqType) -> Self {
        self.pos_req_id = Some(id);
        self.pos_req_type = Some(req_type);
        self
    }

    pub fn add_position(mut self, position: PositionQtyEntry) -> Self {
        self.positions.push(position);
        self
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

// ============================================================================
// Message: AssignmentReport (AW)
// ============================================================================

/// AssignmentReport (MsgType=AW)
///
/// Report of option assignments sent by clearing organization.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssignmentReport {
    /// AsgnRptID (833) - Unique identifier for assignment report
    pub asgn_rpt_id: String,
    /// ClearingBusinessDate (715) - Business date
    pub clearing_business_date: String,
    /// Account (1) - Account assigned
    pub account: String,
    /// AccountType (581) - Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Symbol (55) - Instrument symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// Currency (15) - Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// ThresholdAmount (834) - Threshold amount for assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_amount: Option<f64>,
    /// SettlPrice (730) - Settlement price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_price: Option<f64>,
    /// AssignmentMethod (744) - Method of assignment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_method: Option<char>,
    /// OpenInterest (746) - Open interest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<f64>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// PositionQty - Repeating group of position quantities
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub positions: Vec<PositionQtyEntry>,
    /// PositionAmountData - Repeating group of position amounts
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub position_amounts: Vec<PositionAmtEntry>,
}

impl AssignmentReport {
    pub fn new(
        asgn_rpt_id: String,
        clearing_business_date: String,
        account: String,
    ) -> Self {
        Self {
            asgn_rpt_id,
            clearing_business_date,
            account,
            account_type: None,
            symbol: None,
            security_id: None,
            security_id_source: None,
            currency: None,
            threshold_amount: None,
            settl_price: None,
            assignment_method: None,
            open_interest: None,
            text: None,
            positions: Vec::new(),
            position_amounts: Vec::new(),
        }
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn add_position(mut self, position: PositionQtyEntry) -> Self {
        self.positions.push(position);
        self
    }
}

// ============================================================================
// Message: AdjustedPositionReport (BL)
// ============================================================================

/// AdjustedPositionReport (MsgType=BL)
///
/// Report of adjusted positions due to corporate actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdjustedPositionReport {
    /// PosMaintRptID (721) - Unique identifier for this report
    pub pos_maint_rpt_id: String,
    /// ClearingBusinessDate (715) - Business date
    pub clearing_business_date: String,
    /// Account (1) - Account
    pub account: String,
    /// AccountType (581) - Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Symbol (55) - Instrument symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// PosReqType (724) - Type of position
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_req_type: Option<PosReqType>,
    /// SettlPrice (730) - Settlement price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_price: Option<f64>,
    /// PriorSettlPrice (734) - Prior settlement price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_settl_price: Option<f64>,
    /// PosMaintRptRefID (714) - Reference to previous position report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_maint_rpt_ref_id: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// PositionQty - Repeating group of position quantities
    pub positions: Vec<PositionQtyEntry>,
}

impl AdjustedPositionReport {
    pub fn new(
        pos_maint_rpt_id: String,
        clearing_business_date: String,
        account: String,
        positions: Vec<PositionQtyEntry>,
    ) -> Self {
        Self {
            pos_maint_rpt_id,
            clearing_business_date,
            account,
            account_type: None,
            symbol: None,
            security_id: None,
            security_id_source: None,
            pos_req_type: None,
            settl_price: None,
            prior_settl_price: None,
            pos_maint_rpt_ref_id: None,
            text: None,
            positions,
        }
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

// ============================================================================
// Message: ContraryIntentionReport (BO)
// ============================================================================

/// ContraryIntentionReport (MsgType=BO)
///
/// Report of contrary expiration quantities for Saturday expiring options.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContraryIntentionReport {
    /// ContIntRptID (977) - Unique identifier for contrary intention report
    pub cont_int_rpt_id: String,
    /// ClearingBusinessDate (715) - Business date
    pub clearing_business_date: String,
    /// Account (1) - Account
    pub account: String,
    /// AccountType (581) - Account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Symbol (55) - Instrument symbol
    pub symbol: String,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// TransactTime (60) - Transaction time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,
    /// LateIndicator (978) - Indicates late submission
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_indicator: Option<bool>,
    /// InputSource (979) - Source of input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_source: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// ExpirationQty - Quantity subject to contrary intention
    pub expiration_qty: f64,
}

impl ContraryIntentionReport {
    pub fn new(
        cont_int_rpt_id: String,
        clearing_business_date: String,
        account: String,
        symbol: String,
        expiration_qty: f64,
    ) -> Self {
        Self {
            cont_int_rpt_id,
            clearing_business_date,
            account,
            account_type: None,
            symbol,
            security_id: None,
            security_id_source: None,
            transact_time: None,
            late_indicator: None,
            input_source: None,
            text: None,
            expiration_qty,
        }
    }

    pub fn with_security_id(mut self, id: String, source: String) -> Self {
        self.security_id = Some(id);
        self.security_id_source = Some(source);
        self
    }
}

// ============================================================================
// Message: PositionTransferInstruction (DL)
// ============================================================================

/// PositionTransferInstruction (MsgType=DL)
///
/// Initiate, cancel, change, accept, or decline position transfers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionTransferInstruction {
    /// TransferInstructionID (2436) - Unique identifier for transfer instruction
    pub transfer_instruction_id: String,
    /// TransferID (2437) - Transfer identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    /// TransferTransType (2439) - Transfer transaction type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_trans_type: Option<String>,
    /// TransferType (2440) - Type of transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
    /// TransferScope (2441) - Scope of transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_scope: Option<String>,
    /// ClearingBusinessDate (715) - Business date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_business_date: Option<String>,
    /// Account (1) - Source account
    pub account: String,
    /// Symbol (55) - Instrument symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// TransactTime (60) - Transaction time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// PositionQty - Repeating group of position quantities to transfer
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub positions: Vec<PositionQtyEntry>,
}

impl PositionTransferInstruction {
    pub fn new(transfer_instruction_id: String, account: String) -> Self {
        Self {
            transfer_instruction_id,
            transfer_id: None,
            transfer_trans_type: None,
            transfer_type: None,
            transfer_scope: None,
            clearing_business_date: None,
            account,
            symbol: None,
            security_id: None,
            security_id_source: None,
            transact_time: None,
            text: None,
            positions: Vec::new(),
        }
    }

    pub fn with_transfer_id(mut self, id: String) -> Self {
        self.transfer_id = Some(id);
        self
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn add_position(mut self, position: PositionQtyEntry) -> Self {
        self.positions.push(position);
        self
    }
}

// ============================================================================
// Message: PositionTransferInstructionAck (DM)
// ============================================================================

/// PositionTransferInstructionAck (MsgType=DM)
///
/// Technical acknowledgment of PositionTransferInstruction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionTransferInstructionAck {
    /// TransferInstructionID (2436) - Reference to transfer instruction
    pub transfer_instruction_id: String,
    /// TransferID (2437) - Transfer identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    /// TransferTransType (2439) - Transfer transaction type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_trans_type: Option<String>,
    /// TransferType (2440) - Type of transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
    /// TransferStatus (2442) - Status of transfer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<String>,
    /// TransferRejectReason (2443) - Reason for rejection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_reject_reason: Option<String>,
    /// TransactTime (60) - Transaction time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl PositionTransferInstructionAck {
    pub fn new(transfer_instruction_id: String) -> Self {
        Self {
            transfer_instruction_id,
            transfer_id: None,
            transfer_trans_type: None,
            transfer_type: None,
            transfer_status: None,
            transfer_reject_reason: None,
            transact_time: None,
            text: None,
        }
    }

    pub fn with_transfer_id(mut self, id: String) -> Self {
        self.transfer_id = Some(id);
        self
    }

    pub fn with_transfer_status(mut self, status: String) -> Self {
        self.transfer_status = Some(status);
        self
    }
}

// ============================================================================
// Message: PositionTransferReport (DN)
// ============================================================================

/// PositionTransferReport (MsgType=DN)
///
/// Report of positions to be transferred and transfer status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionTransferReport {
    /// TransferReportID (2447) - Unique identifier for transfer report
    pub transfer_report_id: String,
    /// TransferID (2437) - Transfer identifier
    pub transfer_id: String,
    /// TransferTransType (2439) - Transfer transaction type
    pub transfer_trans_type: String,
    /// TransferReportType (2444) - Type of transfer report
    pub transfer_report_type: String,
    /// TransferStatus (2442) - Status of transfer
    pub transfer_status: String,
    /// TransferInstructionID (2436) - Reference to transfer instruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_instruction_id: Option<String>,
    /// TransferRejectReason (2443) - Reason for rejection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_reject_reason: Option<String>,
    /// ClearingBusinessDate (715) - Business date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_business_date: Option<String>,
    /// Account (1) - Source account
    pub account: String,
    /// Symbol (55) - Instrument symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,
    /// SecurityIDSource (22) - Security identifier source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,
    /// TransactTime (60) - Transaction time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// PositionQty - Repeating group of position quantities
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub positions: Vec<PositionQtyEntry>,
    /// PositionAmountData - Repeating group of position amounts
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub position_amounts: Vec<PositionAmtEntry>,
}

impl PositionTransferReport {
    pub fn new(
        transfer_report_id: String,
        transfer_id: String,
        transfer_trans_type: String,
        transfer_report_type: String,
        transfer_status: String,
        account: String,
    ) -> Self {
        Self {
            transfer_report_id,
            transfer_id,
            transfer_trans_type,
            transfer_report_type,
            transfer_status,
            transfer_instruction_id: None,
            transfer_reject_reason: None,
            clearing_business_date: None,
            account,
            symbol: None,
            security_id: None,
            security_id_source: None,
            transact_time: None,
            text: None,
            positions: Vec::new(),
            position_amounts: Vec::new(),
        }
    }

    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn add_position(mut self, position: PositionQtyEntry) -> Self {
        self.positions.push(position);
        self
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_for_positions_creation() {
        let msg = RequestForPositions::new(
            "REQ123".to_string(),
            PosReqType::Positions,
            "2024-01-15".to_string(),
            "2024-01-15T10:30:00Z".to_string(),
        );

        assert_eq!(msg.pos_req_id, "REQ123");
        assert_eq!(msg.pos_req_type, PosReqType::Positions);
        assert_eq!(msg.clearing_business_date, "2024-01-15");
    }

    #[test]
    fn test_request_for_positions_with_account() {
        let msg = RequestForPositions::new(
            "REQ123".to_string(),
            PosReqType::Positions,
            "2024-01-15".to_string(),
            "2024-01-15T10:30:00Z".to_string(),
        )
        .with_account("ACCT001".to_string())
        .with_symbol("MSFT".to_string());

        assert_eq!(msg.account, Some("ACCT001".to_string()));
        assert_eq!(msg.symbol, Some("MSFT".to_string()));
    }

    #[test]
    fn test_request_for_positions_ack_creation() {
        let msg = RequestForPositionsAck::new(
            "RPT001".to_string(),
            PosReqResult::ValidRequest,
            PosReqStatus::Completed,
        )
        .with_pos_req_id("REQ123".to_string())
        .with_total_num_pos_reports(5);

        assert_eq!(msg.pos_maint_rpt_id, "RPT001");
        assert_eq!(msg.pos_req_result, PosReqResult::ValidRequest);
        assert_eq!(msg.pos_req_status, PosReqStatus::Completed);
        assert_eq!(msg.total_num_pos_reports, Some(5));
    }

    #[test]
    fn test_position_maintenance_request_creation() {
        let position = PositionQtyEntry::new(PosType::EndOfDayQty)
            .with_long_qty(1000.0)
            .with_short_qty(500.0);

        let msg = PositionMaintenanceRequest::new(
            PosTransType::PositionAdjustment,
            PosMaintAction::New,
            "2024-01-15".to_string(),
            "ACCT001".to_string(),
            "AAPL".to_string(),
        )
        .add_position(position)
        .with_security_id("US0378331005".to_string(), "ISIN".to_string());

        assert_eq!(msg.pos_trans_type, PosTransType::PositionAdjustment);
        assert_eq!(msg.pos_maint_action, PosMaintAction::New);
        assert_eq!(msg.account, "ACCT001");
        assert_eq!(msg.symbol, "AAPL");
        assert_eq!(msg.positions.len(), 1);
        assert_eq!(msg.positions[0].pos_type, PosType::EndOfDayQty);
        assert_eq!(msg.positions[0].long_qty, Some(1000.0));
    }

    #[test]
    fn test_position_maintenance_report_creation() {
        let msg = PositionMaintenanceReport::new(
            "RPTID123".to_string(),
            PosTransType::PositionAdjustment,
            PosMaintAction::New,
            PosMaintResult::SuccessfulCompletion,
            "2024-01-15".to_string(),
            "ACCT001".to_string(),
        )
        .with_symbol("AAPL".to_string())
        .with_settl_price(150.25, SettlPriceType::Final);

        assert_eq!(msg.pos_maint_rpt_id, "RPTID123");
        assert_eq!(msg.pos_maint_result, PosMaintResult::SuccessfulCompletion);
        assert_eq!(msg.symbol, Some("AAPL".to_string()));
        assert_eq!(msg.settl_price, Some(150.25));
        assert_eq!(msg.settl_price_type, Some(SettlPriceType::Final));
    }

    #[test]
    fn test_position_report_creation() {
        let position1 = PositionQtyEntry::new(PosType::StartOfDayQty)
            .with_long_qty(5000.0);
        let position2 = PositionQtyEntry::new(PosType::EndOfDayQty)
            .with_long_qty(5200.0);

        let msg = PositionReport::new(
            "POS001".to_string(),
            "2024-01-15".to_string(),
            "ACCT001".to_string(),
        )
        .with_pos_req_id("REQ123".to_string(), PosReqType::Positions)
        .add_position(position1)
        .add_position(position2)
        .with_symbol("MSFT".to_string());

        assert_eq!(msg.pos_maint_rpt_id, "POS001");
        assert_eq!(msg.positions.len(), 2);
        assert_eq!(msg.pos_req_id, Some("REQ123".to_string()));
        assert_eq!(msg.pos_req_type, Some(PosReqType::Positions));
    }

    #[test]
    fn test_assignment_report_creation() {
        let position = PositionQtyEntry::new(PosType::OptionAssignment)
            .with_long_qty(100.0);

        let msg = AssignmentReport::new(
            "ASGN001".to_string(),
            "2024-01-15".to_string(),
            "ACCT001".to_string(),
        )
        .with_symbol("SPY".to_string())
        .add_position(position);

        assert_eq!(msg.asgn_rpt_id, "ASGN001");
        assert_eq!(msg.account, "ACCT001");
        assert_eq!(msg.positions.len(), 1);
        assert_eq!(msg.positions[0].pos_type, PosType::OptionAssignment);
    }

    #[test]
    fn test_adjusted_position_report_creation() {
        let position = PositionQtyEntry::new(PosType::CorporateActionAdjustment)
            .with_long_qty(1200.0);

        let msg = AdjustedPositionReport::new(
            "ADJ001".to_string(),
            "2024-01-15".to_string(),
            "ACCT001".to_string(),
            vec![position],
        )
        .with_symbol("AAPL".to_string());

        assert_eq!(msg.pos_maint_rpt_id, "ADJ001");
        assert_eq!(msg.positions.len(), 1);
        assert_eq!(msg.positions[0].pos_type, PosType::CorporateActionAdjustment);
    }

    #[test]
    fn test_contrary_intention_report_creation() {
        let msg = ContraryIntentionReport::new(
            "CNT001".to_string(),
            "2024-01-15".to_string(),
            "ACCT001".to_string(),
            "SPY".to_string(),
            500.0,
        )
        .with_security_id("US78462F1030".to_string(), "ISIN".to_string());

        assert_eq!(msg.cont_int_rpt_id, "CNT001");
        assert_eq!(msg.symbol, "SPY");
        assert_eq!(msg.expiration_qty, 500.0);
        assert_eq!(msg.security_id, Some("US78462F1030".to_string()));
    }

    #[test]
    fn test_position_transfer_instruction_creation() {
        let position = PositionQtyEntry::new(PosType::TotalTransactionQty)
            .with_long_qty(1000.0);

        let msg = PositionTransferInstruction::new(
            "XFER001".to_string(),
            "SOURCE_ACCT".to_string(),
        )
        .with_transfer_id("XFER_ID123".to_string())
        .with_symbol("AAPL".to_string())
        .add_position(position);

        assert_eq!(msg.transfer_instruction_id, "XFER001");
        assert_eq!(msg.account, "SOURCE_ACCT");
        assert_eq!(msg.transfer_id, Some("XFER_ID123".to_string()));
        assert_eq!(msg.positions.len(), 1);
    }

    #[test]
    fn test_position_transfer_instruction_ack_creation() {
        let msg = PositionTransferInstructionAck::new("XFER001".to_string())
            .with_transfer_id("XFER_ID123".to_string())
            .with_transfer_status("ACCEPTED".to_string());

        assert_eq!(msg.transfer_instruction_id, "XFER001");
        assert_eq!(msg.transfer_id, Some("XFER_ID123".to_string()));
        assert_eq!(msg.transfer_status, Some("ACCEPTED".to_string()));
    }

    #[test]
    fn test_position_transfer_report_creation() {
        let position = PositionQtyEntry::new(PosType::TransferTradeQty)
            .with_long_qty(2000.0);

        let msg = PositionTransferReport::new(
            "XFER_RPT001".to_string(),
            "XFER_ID123".to_string(),
            "NEW".to_string(),
            "STATUS".to_string(),
            "COMPLETED".to_string(),
            "SOURCE_ACCT".to_string(),
        )
        .with_symbol("MSFT".to_string())
        .add_position(position);

        assert_eq!(msg.transfer_report_id, "XFER_RPT001");
        assert_eq!(msg.transfer_id, "XFER_ID123");
        assert_eq!(msg.transfer_status, "COMPLETED");
        assert_eq!(msg.positions.len(), 1);
    }

    #[test]
    fn test_position_qty_entry_builder() {
        let entry = PositionQtyEntry::new(PosType::StartOfDayQty)
            .with_long_qty(1000.0)
            .with_short_qty(200.0);

        assert_eq!(entry.pos_type, PosType::StartOfDayQty);
        assert_eq!(entry.long_qty, Some(1000.0));
        assert_eq!(entry.short_qty, Some(200.0));
    }

    #[test]
    fn test_position_amt_entry_builder() {
        let entry = PositionAmtEntry::new(PosAmtType::FinalMarkToMarket, 50000.0)
            .with_currency("USD".to_string());

        assert_eq!(entry.pos_amt_type, PosAmtType::FinalMarkToMarket);
        assert_eq!(entry.pos_amt, 50000.0);
        assert_eq!(entry.position_currency, Some("USD".to_string()));
    }
}
