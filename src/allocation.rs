use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Allocation Messages
// Implementation of FIX 5.0 SP2 Post-Trade Allocation messages
// ============================================================================

// ============================================================================
// Component Structures (Repeating Groups)
// ============================================================================

/// Entry in the OrdAllocGrp repeating group (NoOrders=73)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrdAllocEntry {
    /// ClOrdID (11) - Client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// OrderID (37) - Exchange order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// SecondaryOrderID (198) - Secondary order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_order_id: Option<String>,
    /// SecondaryClOrdID (526) - Secondary client order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_cl_ord_id: Option<String>,
    /// ListID (66) - List identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// OrderQty (38) - Quantity ordered
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<f64>,
    /// OrderAvgPx (799) - Average price for order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_avg_px: Option<f64>,
    /// OrderBookingQty (800) - Quantity booked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_booking_qty: Option<f64>,
}

impl OrdAllocEntry {
    pub fn new() -> Self {
        Self {
            cl_ord_id: None,
            order_id: None,
            secondary_order_id: None,
            secondary_cl_ord_id: None,
            list_id: None,
            order_qty: None,
            order_avg_px: None,
            order_booking_qty: None,
        }
    }

    pub fn with_cl_ord_id(mut self, id: String) -> Self {
        self.cl_ord_id = Some(id);
        self
    }

    pub fn with_order_id(mut self, id: String) -> Self {
        self.order_id = Some(id);
        self
    }

    pub fn with_order_qty(mut self, qty: f64) -> Self {
        self.order_qty = Some(qty);
        self
    }

    pub fn with_order_avg_px(mut self, px: f64) -> Self {
        self.order_avg_px = Some(px);
        self
    }
}

impl Default for OrdAllocEntry {
    fn default() -> Self {
        Self::new()
    }
}

/// Entry in the ExecAllocGrp repeating group (NoExecs=124)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExecAllocEntry {
    /// LastQty (32) - Quantity executed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_qty: Option<f64>,
    /// ExecID (17) - Execution ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_id: Option<String>,
    /// SecondaryExecID (527) - Secondary execution ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_exec_id: Option<String>,
    /// LastPx (31) - Price of execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_px: Option<f64>,
    /// LastParPx (669) - Par price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_par_px: Option<f64>,
    /// LastCapacity (29) - Capacity of execution
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_capacity: Option<String>,
    /// TradeID (1003) - Trade identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    /// FirmTradeID (1041) - Firm trade identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firm_trade_id: Option<String>,
}

impl ExecAllocEntry {
    pub fn new() -> Self {
        Self {
            last_qty: None,
            exec_id: None,
            secondary_exec_id: None,
            last_px: None,
            last_par_px: None,
            last_capacity: None,
            trade_id: None,
            firm_trade_id: None,
        }
    }

    pub fn with_exec_id(mut self, id: String) -> Self {
        self.exec_id = Some(id);
        self
    }

    pub fn with_last_qty(mut self, qty: f64) -> Self {
        self.last_qty = Some(qty);
        self
    }

    pub fn with_last_px(mut self, px: f64) -> Self {
        self.last_px = Some(px);
        self
    }

    pub fn with_trade_id(mut self, id: String) -> Self {
        self.trade_id = Some(id);
        self
    }
}

impl Default for ExecAllocEntry {
    fn default() -> Self {
        Self::new()
    }
}

/// Entry in the AllocGrp repeating group (NoAllocs=78)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocEntry {
    /// AllocAccount (79) - Account for allocation
    pub alloc_account: String,
    /// AllocAcctIDSource (661) - Account ID source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_acct_id_source: Option<u32>,
    /// MatchStatus (573) - Status of match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_status: Option<MatchStatus>,
    /// AllocPrice (366) - Price for allocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_price: Option<f64>,
    /// AllocQty (80) - Quantity allocated
    pub alloc_qty: f64,
    /// IndividualAllocID (467) - Individual allocation identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_alloc_id: Option<String>,
}

impl AllocEntry {
    pub fn new(alloc_account: String, alloc_qty: f64) -> Self {
        Self {
            alloc_account,
            alloc_acct_id_source: None,
            match_status: None,
            alloc_price: None,
            alloc_qty,
            individual_alloc_id: None,
        }
    }

    pub fn with_alloc_acct_id_source(mut self, source: u32) -> Self {
        self.alloc_acct_id_source = Some(source);
        self
    }

    pub fn with_match_status(mut self, status: MatchStatus) -> Self {
        self.match_status = Some(status);
        self
    }

    pub fn with_alloc_price(mut self, price: f64) -> Self {
        self.alloc_price = Some(price);
        self
    }

    pub fn with_individual_alloc_id(mut self, id: String) -> Self {
        self.individual_alloc_id = Some(id);
        self
    }
}

/// Entry in the AllocAckGrp repeating group (NoAllocs=78)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocAckEntry {
    /// AllocAccount (79) - Account for allocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_account: Option<String>,
    /// AllocAcctIDSource (661) - Account ID source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_acct_id_source: Option<u32>,
    /// AllocPrice (366) - Price for allocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_price: Option<f64>,
    /// IndividualAllocID (467) - Individual allocation identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_alloc_id: Option<String>,
    /// IndividualAllocRejCode (776) - Rejection code for individual allocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_alloc_rej_code: Option<IndividualAllocRejCode>,
    /// AllocText (161) - Text for allocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_text: Option<String>,
    /// AllocQty (80) - Quantity allocated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_qty: Option<f64>,
}

impl AllocAckEntry {
    pub fn new() -> Self {
        Self {
            alloc_account: None,
            alloc_acct_id_source: None,
            alloc_price: None,
            individual_alloc_id: None,
            individual_alloc_rej_code: None,
            alloc_text: None,
            alloc_qty: None,
        }
    }

    pub fn with_alloc_account(mut self, account: String) -> Self {
        self.alloc_account = Some(account);
        self
    }

    pub fn with_alloc_qty(mut self, qty: f64) -> Self {
        self.alloc_qty = Some(qty);
        self
    }

    pub fn with_individual_alloc_id(mut self, id: String) -> Self {
        self.individual_alloc_id = Some(id);
        self
    }

    pub fn with_individual_alloc_rej_code(mut self, code: IndividualAllocRejCode) -> Self {
        self.individual_alloc_rej_code = Some(code);
        self
    }

    pub fn with_alloc_text(mut self, text: String) -> Self {
        self.alloc_text = Some(text);
        self
    }
}

impl Default for AllocAckEntry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Message: AllocationInstruction (J)
// ============================================================================

/// AllocationInstruction (MsgType=J)
///
/// Provides allocation details for a trade execution
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocationInstruction {
    /// AllocID (70) - Unique identifier for allocation instruction
    pub alloc_id: String,
    /// AllocTransType (71) - Type of allocation transaction
    pub alloc_trans_type: AllocTransType,
    /// AllocType (626) - Purpose of allocation message
    pub alloc_type: AllocType,
    /// Side (54) - Side of order
    pub side: Side,
    /// Quantity (53) - Overall quantity
    pub quantity: f64,
    /// TradeDate (75) - Trade date
    pub trade_date: String,
    /// RefAllocID (72) - Reference allocation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_alloc_id: Option<String>,
    /// AllocCancReplaceReason (796) - Reason for cancel/replace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_canc_replace_reason: Option<AllocCancReplaceReason>,
    /// AvgPx (6) - Average price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<f64>,
    /// NetMoney (118) - Net money
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_money: Option<f64>,
    /// Currency (15) - Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// SettlDate (64) - Settlement date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_date: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// OrdAllocGrp - Repeating group of order allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_alloc_grp: Option<Vec<OrdAllocEntry>>,
    /// ExecAllocGrp - Repeating group of execution allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_alloc_grp: Option<Vec<ExecAllocEntry>>,
    /// AllocGrp - Repeating group of allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_grp: Option<Vec<AllocEntry>>,
}

impl AllocationInstruction {
    pub fn new(
        alloc_id: String,
        alloc_trans_type: AllocTransType,
        alloc_type: AllocType,
        side: Side,
        quantity: f64,
        trade_date: String,
    ) -> Self {
        Self {
            alloc_id,
            alloc_trans_type,
            alloc_type,
            side,
            quantity,
            trade_date,
            ref_alloc_id: None,
            alloc_canc_replace_reason: None,
            avg_px: None,
            net_money: None,
            currency: None,
            settl_date: None,
            text: None,
            ord_alloc_grp: None,
            exec_alloc_grp: None,
            alloc_grp: None,
        }
    }

    pub fn with_ref_alloc_id(mut self, id: String) -> Self {
        self.ref_alloc_id = Some(id);
        self
    }

    pub fn with_alloc_canc_replace_reason(mut self, reason: AllocCancReplaceReason) -> Self {
        self.alloc_canc_replace_reason = Some(reason);
        self
    }

    pub fn with_avg_px(mut self, avg_px: f64) -> Self {
        self.avg_px = Some(avg_px);
        self
    }

    pub fn with_net_money(mut self, net_money: f64) -> Self {
        self.net_money = Some(net_money);
        self
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_settl_date(mut self, date: String) -> Self {
        self.settl_date = Some(date);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn add_order(mut self, order: OrdAllocEntry) -> Self {
        if let Some(ref mut grp) = self.ord_alloc_grp {
            grp.push(order);
        } else {
            self.ord_alloc_grp = Some(vec![order]);
        }
        self
    }

    pub fn with_orders(mut self, orders: Vec<OrdAllocEntry>) -> Self {
        self.ord_alloc_grp = Some(orders);
        self
    }

    pub fn add_exec(mut self, exec: ExecAllocEntry) -> Self {
        if let Some(ref mut grp) = self.exec_alloc_grp {
            grp.push(exec);
        } else {
            self.exec_alloc_grp = Some(vec![exec]);
        }
        self
    }

    pub fn with_execs(mut self, execs: Vec<ExecAllocEntry>) -> Self {
        self.exec_alloc_grp = Some(execs);
        self
    }

    pub fn add_alloc(mut self, alloc: AllocEntry) -> Self {
        if let Some(ref mut grp) = self.alloc_grp {
            grp.push(alloc);
        } else {
            self.alloc_grp = Some(vec![alloc]);
        }
        self
    }

    pub fn with_allocs(mut self, allocs: Vec<AllocEntry>) -> Self {
        self.alloc_grp = Some(allocs);
        self
    }
}

// ============================================================================
// Message: AllocationInstructionAck (P)
// ============================================================================

/// AllocationInstructionAck (MsgType=P)
///
/// Acknowledgment of AllocationInstruction message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocationInstructionAck {
    /// AllocID (70) - Unique identifier for allocation instruction
    pub alloc_id: String,
    /// AllocStatus (87) - Status of allocation
    pub alloc_status: AllocStatus,
    /// AllocRejCode (88) - Rejection code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_rej_code: Option<AllocRejCode>,
    /// AllocIntermedReqType (808) - Type of intermediary request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_intermed_req_type: Option<AllocIntermedReqType>,
    /// MatchStatus (573) - Status of match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_status: Option<MatchStatus>,
    /// SecondaryAllocID (793) - Secondary allocation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_alloc_id: Option<String>,
    /// TradeDate (75) - Trade date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<String>,
    /// TransactTime (60) - Transaction time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// AllocAckGrp - Repeating group of allocation acknowledgments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_ack_grp: Option<Vec<AllocAckEntry>>,
}

impl AllocationInstructionAck {
    pub fn new(alloc_id: String, alloc_status: AllocStatus) -> Self {
        Self {
            alloc_id,
            alloc_status,
            alloc_rej_code: None,
            alloc_intermed_req_type: None,
            match_status: None,
            secondary_alloc_id: None,
            trade_date: None,
            transact_time: None,
            text: None,
            alloc_ack_grp: None,
        }
    }

    pub fn with_alloc_rej_code(mut self, code: AllocRejCode) -> Self {
        self.alloc_rej_code = Some(code);
        self
    }

    pub fn with_alloc_intermed_req_type(mut self, req_type: AllocIntermedReqType) -> Self {
        self.alloc_intermed_req_type = Some(req_type);
        self
    }

    pub fn with_match_status(mut self, status: MatchStatus) -> Self {
        self.match_status = Some(status);
        self
    }

    pub fn with_secondary_alloc_id(mut self, id: String) -> Self {
        self.secondary_alloc_id = Some(id);
        self
    }

    pub fn with_trade_date(mut self, date: String) -> Self {
        self.trade_date = Some(date);
        self
    }

    pub fn with_transact_time(mut self, time: String) -> Self {
        self.transact_time = Some(time);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn add_alloc_ack(mut self, ack: AllocAckEntry) -> Self {
        if let Some(ref mut grp) = self.alloc_ack_grp {
            grp.push(ack);
        } else {
            self.alloc_ack_grp = Some(vec![ack]);
        }
        self
    }

    pub fn with_alloc_acks(mut self, acks: Vec<AllocAckEntry>) -> Self {
        self.alloc_ack_grp = Some(acks);
        self
    }
}

// ============================================================================
// Message: AllocationInstructionAlert (BM)
// ============================================================================

/// AllocationInstructionAlert (MsgType=BM)
///
/// Alert notification for allocation instructions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocationInstructionAlert {
    /// AllocID (70) - Unique identifier for allocation instruction
    pub alloc_id: String,
    /// AllocRequestID (2758) - Allocation request identifier
    pub alloc_request_id: String,
    /// AllocTransType (71) - Type of allocation transaction
    pub alloc_trans_type: AllocTransType,
    /// AllocType (626) - Purpose of allocation message
    pub alloc_type: AllocType,
    /// Side (54) - Side of order
    pub side: Side,
    /// Quantity (53) - Overall quantity
    pub quantity: f64,
    /// TradeDate (75) - Trade date
    pub trade_date: String,
    /// RefAllocID (72) - Reference allocation ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_alloc_id: Option<String>,
    /// AllocCancReplaceReason (796) - Reason for cancel/replace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_canc_replace_reason: Option<AllocCancReplaceReason>,
    /// AvgPx (6) - Average price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<f64>,
    /// NetMoney (118) - Net money
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_money: Option<f64>,
    /// Currency (15) - Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// SettlDate (64) - Settlement date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_date: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// OrdAllocGrp - Repeating group of order allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_alloc_grp: Option<Vec<OrdAllocEntry>>,
    /// ExecAllocGrp - Repeating group of execution allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_alloc_grp: Option<Vec<ExecAllocEntry>>,
    /// AllocGrp - Repeating group of allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_grp: Option<Vec<AllocEntry>>,
}

impl AllocationInstructionAlert {
    pub fn new(
        alloc_id: String,
        alloc_request_id: String,
        alloc_trans_type: AllocTransType,
        alloc_type: AllocType,
        side: Side,
        quantity: f64,
        trade_date: String,
    ) -> Self {
        Self {
            alloc_id,
            alloc_request_id,
            alloc_trans_type,
            alloc_type,
            side,
            quantity,
            trade_date,
            ref_alloc_id: None,
            alloc_canc_replace_reason: None,
            avg_px: None,
            net_money: None,
            currency: None,
            settl_date: None,
            text: None,
            ord_alloc_grp: None,
            exec_alloc_grp: None,
            alloc_grp: None,
        }
    }

    pub fn with_ref_alloc_id(mut self, id: String) -> Self {
        self.ref_alloc_id = Some(id);
        self
    }

    pub fn with_avg_px(mut self, avg_px: f64) -> Self {
        self.avg_px = Some(avg_px);
        self
    }

    pub fn with_net_money(mut self, net_money: f64) -> Self {
        self.net_money = Some(net_money);
        self
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn add_order(mut self, order: OrdAllocEntry) -> Self {
        if let Some(ref mut grp) = self.ord_alloc_grp {
            grp.push(order);
        } else {
            self.ord_alloc_grp = Some(vec![order]);
        }
        self
    }

    pub fn add_exec(mut self, exec: ExecAllocEntry) -> Self {
        if let Some(ref mut grp) = self.exec_alloc_grp {
            grp.push(exec);
        } else {
            self.exec_alloc_grp = Some(vec![exec]);
        }
        self
    }

    pub fn add_alloc(mut self, alloc: AllocEntry) -> Self {
        if let Some(ref mut grp) = self.alloc_grp {
            grp.push(alloc);
        } else {
            self.alloc_grp = Some(vec![alloc]);
        }
        self
    }
}

// ============================================================================
// Message: AllocationInstructionAlertRequest (DU)
// ============================================================================

/// AllocationInstructionAlertRequest (MsgType=DU)
///
/// Request for allocation instruction alerts
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocationInstructionAlertRequest {
    /// AllocRequestID (2758) - Unique identifier for allocation request
    pub alloc_request_id: String,
    /// AllocID (70) - Allocation instruction identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_id: Option<String>,
    /// TradeDate (75) - Trade date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<String>,
}

impl AllocationInstructionAlertRequest {
    pub fn new(alloc_request_id: String) -> Self {
        Self {
            alloc_request_id,
            alloc_id: None,
            trade_date: None,
        }
    }

    pub fn with_alloc_id(mut self, id: String) -> Self {
        self.alloc_id = Some(id);
        self
    }

    pub fn with_trade_date(mut self, date: String) -> Self {
        self.trade_date = Some(date);
        self
    }
}

// ============================================================================
// Message: AllocationInstructionAlertRequestAck (DV)
// ============================================================================

/// AllocationInstructionAlertRequestAck (MsgType=DV)
///
/// Acknowledgment of AllocationInstructionAlertRequest
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocationInstructionAlertRequestAck {
    /// AllocRequestID (2758) - Reference to allocation request
    pub alloc_request_id: String,
    /// AllocRequestStatus (2768) - Status of allocation request
    pub alloc_request_status: AllocRequestStatus,
    /// RejectText (1328) - Reason for rejection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_text: Option<String>,
}

impl AllocationInstructionAlertRequestAck {
    pub fn new(alloc_request_id: String, alloc_request_status: AllocRequestStatus) -> Self {
        Self {
            alloc_request_id,
            alloc_request_status,
            reject_text: None,
        }
    }

    pub fn with_reject_text(mut self, text: String) -> Self {
        self.reject_text = Some(text);
        self
    }
}

// ============================================================================
// Message: AllocationReport (AS)
// ============================================================================

/// AllocationReport (MsgType=AS)
///
/// Report of allocation information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocationReport {
    /// AllocReportID (755) - Unique identifier for allocation report
    pub alloc_report_id: String,
    /// AllocTransType (71) - Type of allocation transaction
    pub alloc_trans_type: AllocTransType,
    /// AllocReportType (794) - Type of allocation report
    pub alloc_report_type: AllocReportType,
    /// AllocStatus (87) - Status of allocation
    pub alloc_status: AllocStatus,
    /// Side (54) - Side of order
    pub side: Side,
    /// Quantity (53) - Overall quantity
    pub quantity: f64,
    /// AvgPx (6) - Average price
    pub avg_px: f64,
    /// TradeDate (75) - Trade date
    pub trade_date: String,
    /// AllocID (70) - Allocation instruction identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_id: Option<String>,
    /// AllocReportRefID (795) - Reference to previous allocation report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_report_ref_id: Option<String>,
    /// AllocCancReplaceReason (796) - Reason for cancel/replace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_canc_replace_reason: Option<AllocCancReplaceReason>,
    /// AllocRejCode (88) - Rejection code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_rej_code: Option<AllocRejCode>,
    /// NetMoney (118) - Net money
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_money: Option<f64>,
    /// Currency (15) - Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// OrdAllocGrp - Repeating group of order allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_alloc_grp: Option<Vec<OrdAllocEntry>>,
    /// ExecAllocGrp - Repeating group of execution allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_alloc_grp: Option<Vec<ExecAllocEntry>>,
    /// AllocGrp - Repeating group of allocations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_grp: Option<Vec<AllocEntry>>,
}

impl AllocationReport {
    pub fn new(
        alloc_report_id: String,
        alloc_trans_type: AllocTransType,
        alloc_report_type: AllocReportType,
        alloc_status: AllocStatus,
        side: Side,
        quantity: f64,
        avg_px: f64,
        trade_date: String,
    ) -> Self {
        Self {
            alloc_report_id,
            alloc_trans_type,
            alloc_report_type,
            alloc_status,
            side,
            quantity,
            avg_px,
            trade_date,
            alloc_id: None,
            alloc_report_ref_id: None,
            alloc_canc_replace_reason: None,
            alloc_rej_code: None,
            net_money: None,
            currency: None,
            text: None,
            ord_alloc_grp: None,
            exec_alloc_grp: None,
            alloc_grp: None,
        }
    }

    pub fn with_alloc_id(mut self, id: String) -> Self {
        self.alloc_id = Some(id);
        self
    }

    pub fn with_alloc_report_ref_id(mut self, id: String) -> Self {
        self.alloc_report_ref_id = Some(id);
        self
    }

    pub fn with_alloc_canc_replace_reason(mut self, reason: AllocCancReplaceReason) -> Self {
        self.alloc_canc_replace_reason = Some(reason);
        self
    }

    pub fn with_alloc_rej_code(mut self, code: AllocRejCode) -> Self {
        self.alloc_rej_code = Some(code);
        self
    }

    pub fn with_net_money(mut self, net_money: f64) -> Self {
        self.net_money = Some(net_money);
        self
    }

    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn add_order(mut self, order: OrdAllocEntry) -> Self {
        if let Some(ref mut grp) = self.ord_alloc_grp {
            grp.push(order);
        } else {
            self.ord_alloc_grp = Some(vec![order]);
        }
        self
    }

    pub fn with_orders(mut self, orders: Vec<OrdAllocEntry>) -> Self {
        self.ord_alloc_grp = Some(orders);
        self
    }

    pub fn add_exec(mut self, exec: ExecAllocEntry) -> Self {
        if let Some(ref mut grp) = self.exec_alloc_grp {
            grp.push(exec);
        } else {
            self.exec_alloc_grp = Some(vec![exec]);
        }
        self
    }

    pub fn with_execs(mut self, execs: Vec<ExecAllocEntry>) -> Self {
        self.exec_alloc_grp = Some(execs);
        self
    }

    pub fn add_alloc(mut self, alloc: AllocEntry) -> Self {
        if let Some(ref mut grp) = self.alloc_grp {
            grp.push(alloc);
        } else {
            self.alloc_grp = Some(vec![alloc]);
        }
        self
    }

    pub fn with_allocs(mut self, allocs: Vec<AllocEntry>) -> Self {
        self.alloc_grp = Some(allocs);
        self
    }
}

// ============================================================================
// Message: AllocationReportAck (AT)
// ============================================================================

/// AllocationReportAck (MsgType=AT)
///
/// Acknowledgment of AllocationReport message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AllocationReportAck {
    /// AllocReportID (755) - Reference to allocation report
    pub alloc_report_id: String,
    /// AllocID (70) - Allocation instruction identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_id: Option<String>,
    /// AllocStatus (87) - Status of allocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_status: Option<AllocStatus>,
    /// AllocRejCode (88) - Rejection code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_rej_code: Option<AllocRejCode>,
    /// AllocReportType (794) - Type of allocation report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_report_type: Option<AllocReportType>,
    /// MatchStatus (573) - Status of match
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_status: Option<MatchStatus>,
    /// Text (58) - Free form text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// AllocAckGrp - Repeating group of allocation acknowledgments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_ack_grp: Option<Vec<AllocAckEntry>>,
}

impl AllocationReportAck {
    pub fn new(alloc_report_id: String) -> Self {
        Self {
            alloc_report_id,
            alloc_id: None,
            alloc_status: None,
            alloc_rej_code: None,
            alloc_report_type: None,
            match_status: None,
            text: None,
            alloc_ack_grp: None,
        }
    }

    pub fn with_alloc_id(mut self, id: String) -> Self {
        self.alloc_id = Some(id);
        self
    }

    pub fn with_alloc_status(mut self, status: AllocStatus) -> Self {
        self.alloc_status = Some(status);
        self
    }

    pub fn with_alloc_rej_code(mut self, code: AllocRejCode) -> Self {
        self.alloc_rej_code = Some(code);
        self
    }

    pub fn with_alloc_report_type(mut self, report_type: AllocReportType) -> Self {
        self.alloc_report_type = Some(report_type);
        self
    }

    pub fn with_match_status(mut self, status: MatchStatus) -> Self {
        self.match_status = Some(status);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    pub fn add_alloc_ack(mut self, ack: AllocAckEntry) -> Self {
        if let Some(ref mut grp) = self.alloc_ack_grp {
            grp.push(ack);
        } else {
            self.alloc_ack_grp = Some(vec![ack]);
        }
        self
    }

    pub fn with_alloc_acks(mut self, acks: Vec<AllocAckEntry>) -> Self {
        self.alloc_ack_grp = Some(acks);
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
    fn test_allocation_instruction_creation() {
        let msg = AllocationInstruction::new(
            "ALLOC123".to_string(),
            AllocTransType::New,
            AllocType::Calculated,
            Side::Buy,
            1000.0,
            "20231015".to_string(),
        );

        assert_eq!(msg.alloc_id, "ALLOC123");
        assert_eq!(msg.alloc_trans_type, AllocTransType::New);
        assert_eq!(msg.alloc_type, AllocType::Calculated);
        assert_eq!(msg.side, Side::Buy);
        assert_eq!(msg.quantity, 1000.0);
        assert_eq!(msg.trade_date, "20231015");
        assert!(msg.avg_px.is_none());
        assert!(msg.net_money.is_none());
    }

    #[test]
    fn test_allocation_instruction_with_optional_fields() {
        let msg = AllocationInstruction::new(
            "ALLOC123".to_string(),
            AllocTransType::New,
            AllocType::Calculated,
            Side::Buy,
            1000.0,
            "20231015".to_string(),
        )
        .with_avg_px(150.25)
        .with_net_money(150250.0)
        .with_currency("USD".to_string())
        .with_settl_date("20231017".to_string())
        .with_text("Test allocation".to_string());

        assert_eq!(msg.avg_px, Some(150.25));
        assert_eq!(msg.net_money, Some(150250.0));
        assert_eq!(msg.currency, Some("USD".to_string()));
        assert_eq!(msg.settl_date, Some("20231017".to_string()));
        assert_eq!(msg.text, Some("Test allocation".to_string()));
    }

    #[test]
    fn test_allocation_instruction_with_groups() {
        let order = OrdAllocEntry::new()
            .with_cl_ord_id("ORD123".to_string())
            .with_order_qty(1000.0);

        let exec = ExecAllocEntry::new()
            .with_exec_id("EXEC456".to_string())
            .with_last_qty(500.0)
            .with_last_px(150.25);

        let alloc = AllocEntry::new("ACCT001".to_string(), 300.0)
            .with_alloc_price(150.25);

        let msg = AllocationInstruction::new(
            "ALLOC123".to_string(),
            AllocTransType::New,
            AllocType::Calculated,
            Side::Buy,
            1000.0,
            "20231015".to_string(),
        )
        .add_order(order)
        .add_exec(exec)
        .add_alloc(alloc);

        assert_eq!(msg.ord_alloc_grp.as_ref().unwrap().len(), 1);
        assert_eq!(msg.exec_alloc_grp.as_ref().unwrap().len(), 1);
        assert_eq!(msg.alloc_grp.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_allocation_instruction_ack_creation() {
        let msg = AllocationInstructionAck::new(
            "ALLOC123".to_string(),
            AllocStatus::Accepted,
        );

        assert_eq!(msg.alloc_id, "ALLOC123");
        assert_eq!(msg.alloc_status, AllocStatus::Accepted);
        assert!(msg.alloc_rej_code.is_none());
        assert!(msg.match_status.is_none());
    }

    #[test]
    fn test_allocation_instruction_ack_with_optional_fields() {
        let msg = AllocationInstructionAck::new(
            "ALLOC123".to_string(),
            AllocStatus::Rejected,
        )
        .with_alloc_rej_code(AllocRejCode::UnknownAccount)
        .with_match_status(MatchStatus::Uncompared)
        .with_trade_date("20231015".to_string())
        .with_text("Invalid account".to_string());

        assert_eq!(msg.alloc_rej_code, Some(AllocRejCode::UnknownAccount));
        assert_eq!(msg.match_status, Some(MatchStatus::Uncompared));
        assert_eq!(msg.trade_date, Some("20231015".to_string()));
        assert_eq!(msg.text, Some("Invalid account".to_string()));
    }

    #[test]
    fn test_allocation_instruction_ack_with_ack_group() {
        let ack_entry = AllocAckEntry::new()
            .with_alloc_account("ACCT001".to_string())
            .with_alloc_qty(300.0)
            .with_individual_alloc_id("IND123".to_string());

        let msg = AllocationInstructionAck::new(
            "ALLOC123".to_string(),
            AllocStatus::Accepted,
        )
        .add_alloc_ack(ack_entry);

        assert_eq!(msg.alloc_ack_grp.as_ref().unwrap().len(), 1);
        assert_eq!(
            msg.alloc_ack_grp.as_ref().unwrap()[0].alloc_account,
            Some("ACCT001".to_string())
        );
    }

    #[test]
    fn test_allocation_instruction_alert_creation() {
        let msg = AllocationInstructionAlert::new(
            "ALLOC123".to_string(),
            "REQ456".to_string(),
            AllocTransType::New,
            AllocType::Calculated,
            Side::Sell,
            2000.0,
            "20231015".to_string(),
        );

        assert_eq!(msg.alloc_id, "ALLOC123");
        assert_eq!(msg.alloc_request_id, "REQ456");
        assert_eq!(msg.alloc_trans_type, AllocTransType::New);
        assert_eq!(msg.side, Side::Sell);
        assert_eq!(msg.quantity, 2000.0);
    }

    #[test]
    fn test_allocation_instruction_alert_request_creation() {
        let msg = AllocationInstructionAlertRequest::new("REQ123".to_string());

        assert_eq!(msg.alloc_request_id, "REQ123");
        assert!(msg.alloc_id.is_none());
        assert!(msg.trade_date.is_none());
    }

    #[test]
    fn test_allocation_instruction_alert_request_with_optional_fields() {
        let msg = AllocationInstructionAlertRequest::new("REQ123".to_string())
            .with_alloc_id("ALLOC456".to_string())
            .with_trade_date("20231015".to_string());

        assert_eq!(msg.alloc_id, Some("ALLOC456".to_string()));
        assert_eq!(msg.trade_date, Some("20231015".to_string()));
    }

    #[test]
    fn test_allocation_instruction_alert_request_ack_creation() {
        let msg = AllocationInstructionAlertRequestAck::new(
            "REQ123".to_string(),
            AllocRequestStatus::Accepted,
        );

        assert_eq!(msg.alloc_request_id, "REQ123");
        assert_eq!(msg.alloc_request_status, AllocRequestStatus::Accepted);
        assert!(msg.reject_text.is_none());
    }

    #[test]
    fn test_allocation_instruction_alert_request_ack_with_reject() {
        let msg = AllocationInstructionAlertRequestAck::new(
            "REQ123".to_string(),
            AllocRequestStatus::Rejected,
        )
        .with_reject_text("Invalid request parameters".to_string());

        assert_eq!(msg.alloc_request_status, AllocRequestStatus::Rejected);
        assert_eq!(
            msg.reject_text,
            Some("Invalid request parameters".to_string())
        );
    }

    #[test]
    fn test_allocation_report_creation() {
        let msg = AllocationReport::new(
            "RPT123".to_string(),
            AllocTransType::New,
            AllocReportType::Complete,
            AllocStatus::Accepted,
            Side::Buy,
            1000.0,
            150.50,
            "20231015".to_string(),
        );

        assert_eq!(msg.alloc_report_id, "RPT123");
        assert_eq!(msg.alloc_trans_type, AllocTransType::New);
        assert_eq!(msg.alloc_report_type, AllocReportType::Complete);
        assert_eq!(msg.alloc_status, AllocStatus::Accepted);
        assert_eq!(msg.side, Side::Buy);
        assert_eq!(msg.quantity, 1000.0);
        assert_eq!(msg.avg_px, 150.50);
        assert_eq!(msg.trade_date, "20231015");
    }

    #[test]
    fn test_allocation_report_with_optional_fields() {
        let msg = AllocationReport::new(
            "RPT123".to_string(),
            AllocTransType::Replace,
            AllocReportType::Complete,
            AllocStatus::Accepted,
            Side::Buy,
            1000.0,
            150.50,
            "20231015".to_string(),
        )
        .with_alloc_id("ALLOC123".to_string())
        .with_alloc_report_ref_id("PREV_RPT".to_string())
        .with_net_money(150500.0)
        .with_currency("USD".to_string())
        .with_text("Allocation report".to_string());

        assert_eq!(msg.alloc_id, Some("ALLOC123".to_string()));
        assert_eq!(msg.alloc_report_ref_id, Some("PREV_RPT".to_string()));
        assert_eq!(msg.net_money, Some(150500.0));
        assert_eq!(msg.currency, Some("USD".to_string()));
    }

    #[test]
    fn test_allocation_report_with_groups() {
        let order = OrdAllocEntry::new()
            .with_order_id("ORD123".to_string())
            .with_order_qty(1000.0);

        let exec = ExecAllocEntry::new()
            .with_exec_id("EXEC456".to_string())
            .with_last_qty(1000.0);

        let alloc = AllocEntry::new("ACCT001".to_string(), 500.0);

        let msg = AllocationReport::new(
            "RPT123".to_string(),
            AllocTransType::New,
            AllocReportType::Complete,
            AllocStatus::Accepted,
            Side::Buy,
            1000.0,
            150.50,
            "20231015".to_string(),
        )
        .add_order(order)
        .add_exec(exec)
        .add_alloc(alloc);

        assert_eq!(msg.ord_alloc_grp.as_ref().unwrap().len(), 1);
        assert_eq!(msg.exec_alloc_grp.as_ref().unwrap().len(), 1);
        assert_eq!(msg.alloc_grp.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_allocation_report_ack_creation() {
        let msg = AllocationReportAck::new("RPT123".to_string());

        assert_eq!(msg.alloc_report_id, "RPT123");
        assert!(msg.alloc_id.is_none());
        assert!(msg.alloc_status.is_none());
        assert!(msg.alloc_rej_code.is_none());
    }

    #[test]
    fn test_allocation_report_ack_with_optional_fields() {
        let msg = AllocationReportAck::new("RPT123".to_string())
            .with_alloc_id("ALLOC123".to_string())
            .with_alloc_status(AllocStatus::Accepted)
            .with_alloc_report_type(AllocReportType::Complete)
            .with_match_status(MatchStatus::Compared)
            .with_text("Acknowledged".to_string());

        assert_eq!(msg.alloc_id, Some("ALLOC123".to_string()));
        assert_eq!(msg.alloc_status, Some(AllocStatus::Accepted));
        assert_eq!(msg.alloc_report_type, Some(AllocReportType::Complete));
        assert_eq!(msg.match_status, Some(MatchStatus::Compared));
        assert_eq!(msg.text, Some("Acknowledged".to_string()));
    }

    #[test]
    fn test_ord_alloc_entry_builder() {
        let entry = OrdAllocEntry::new()
            .with_cl_ord_id("CLO123".to_string())
            .with_order_id("ORD456".to_string())
            .with_order_qty(1000.0)
            .with_order_avg_px(150.25);

        assert_eq!(entry.cl_ord_id, Some("CLO123".to_string()));
        assert_eq!(entry.order_id, Some("ORD456".to_string()));
        assert_eq!(entry.order_qty, Some(1000.0));
        assert_eq!(entry.order_avg_px, Some(150.25));
    }

    #[test]
    fn test_exec_alloc_entry_builder() {
        let entry = ExecAllocEntry::new()
            .with_exec_id("EXEC123".to_string())
            .with_last_qty(500.0)
            .with_last_px(150.50)
            .with_trade_id("TRADE789".to_string());

        assert_eq!(entry.exec_id, Some("EXEC123".to_string()));
        assert_eq!(entry.last_qty, Some(500.0));
        assert_eq!(entry.last_px, Some(150.50));
        assert_eq!(entry.trade_id, Some("TRADE789".to_string()));
    }

    #[test]
    fn test_alloc_entry_builder() {
        let entry = AllocEntry::new("ACCT001".to_string(), 300.0)
            .with_alloc_acct_id_source(1)
            .with_match_status(MatchStatus::Compared)
            .with_alloc_price(150.25)
            .with_individual_alloc_id("IND123".to_string());

        assert_eq!(entry.alloc_account, "ACCT001");
        assert_eq!(entry.alloc_qty, 300.0);
        assert_eq!(entry.alloc_acct_id_source, Some(1));
        assert_eq!(entry.match_status, Some(MatchStatus::Compared));
        assert_eq!(entry.alloc_price, Some(150.25));
        assert_eq!(entry.individual_alloc_id, Some("IND123".to_string()));
    }

    #[test]
    fn test_alloc_ack_entry_builder() {
        let entry = AllocAckEntry::new()
            .with_alloc_account("ACCT001".to_string())
            .with_alloc_qty(300.0)
            .with_individual_alloc_id("IND123".to_string())
            .with_individual_alloc_rej_code(IndividualAllocRejCode::UnknownAccount)
            .with_alloc_text("Account validation failed".to_string());

        assert_eq!(entry.alloc_account, Some("ACCT001".to_string()));
        assert_eq!(entry.alloc_qty, Some(300.0));
        assert_eq!(entry.individual_alloc_id, Some("IND123".to_string()));
        assert_eq!(
            entry.individual_alloc_rej_code,
            Some(IndividualAllocRejCode::UnknownAccount)
        );
        assert_eq!(
            entry.alloc_text,
            Some("Account validation failed".to_string())
        );
    }

    #[test]
    fn test_allocation_instruction_multiple_allocs() {
        let alloc1 = AllocEntry::new("ACCT001".to_string(), 300.0);
        let alloc2 = AllocEntry::new("ACCT002".to_string(), 700.0);

        let msg = AllocationInstruction::new(
            "ALLOC123".to_string(),
            AllocTransType::New,
            AllocType::Calculated,
            Side::Buy,
            1000.0,
            "20231015".to_string(),
        )
        .add_alloc(alloc1)
        .add_alloc(alloc2);

        assert_eq!(msg.alloc_grp.as_ref().unwrap().len(), 2);
        assert_eq!(msg.alloc_grp.as_ref().unwrap()[0].alloc_qty, 300.0);
        assert_eq!(msg.alloc_grp.as_ref().unwrap()[1].alloc_qty, 700.0);
    }

    #[test]
    fn test_allocation_report_ack_with_ack_group() {
        let ack1 = AllocAckEntry::new()
            .with_alloc_account("ACCT001".to_string())
            .with_alloc_qty(300.0);

        let ack2 = AllocAckEntry::new()
            .with_alloc_account("ACCT002".to_string())
            .with_alloc_qty(700.0);

        let msg = AllocationReportAck::new("RPT123".to_string())
            .add_alloc_ack(ack1)
            .add_alloc_ack(ack2);

        assert_eq!(msg.alloc_ack_grp.as_ref().unwrap().len(), 2);
    }
}
