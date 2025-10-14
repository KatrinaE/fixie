use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Settlement Instruction Messages
// Implementation of FIX 5.0 SP2 Post-Trade Settlement Instruction messages
// ============================================================================

/// SettlementInstructionRequest (MsgType=AV)
///
/// Used to request standing settlement instructions from another party.
/// The response should be a Settlement Instruction message with SettlInstTransType='N' (New)
/// containing all SSIs meeting the criteria specified in the request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementInstructionRequest {
    /// SettlInstReqID (791) - Unique identifier for settlement instruction request
    pub settl_inst_req_id: String,

    /// TransactTime (60) - Time of request
    pub transact_time: String,

    /// AllocAccount (79) - Account for which settlement instructions are requested (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_account: Option<String>,

    /// AllocAcctIDSource (661) - Alloc Account ID source (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_acct_id_source: Option<String>,

    /// Side (54) - Side of the settlement instruction (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,

    /// Product (460) - Product type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// SecurityType (167) - Type of security (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// CFICode (461) - Classification of financial instrument (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,

    /// EffectiveTime (168) - Effective time for settlement instructions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,

    /// ExpireTime (126) - Expiration time for settlement instructions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,

    /// LastUpdateTime (779) - Last update time for standing instructions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,

    /// StandInstDbType (169) - Standing instruction database type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_inst_db_type: Option<StandInstDbType>,

    /// StandInstDbName (170) - Standing instruction database name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_inst_db_name: Option<String>,

    /// StandInstDbID (171) - Standing instruction database ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_inst_db_id: Option<String>,

    /// SettlCurrency (120) - Settlement currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_currency: Option<String>,
}

impl SettlementInstructionRequest {
    /// Create a new SettlementInstructionRequest
    pub fn new(settl_inst_req_id: String, transact_time: String) -> Self {
        Self {
            settl_inst_req_id,
            transact_time,
            alloc_account: None,
            alloc_acct_id_source: None,
            side: None,
            product: None,
            security_type: None,
            cfi_code: None,
            effective_time: None,
            expire_time: None,
            last_update_time: None,
            stand_inst_db_type: None,
            stand_inst_db_name: None,
            stand_inst_db_id: None,
            settl_currency: None,
        }
    }

    /// Set alloc account
    pub fn with_alloc_account(mut self, alloc_account: String) -> Self {
        self.alloc_account = Some(alloc_account);
        self
    }

    /// Set side
    pub fn with_side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    /// Set settlement currency
    pub fn with_settl_currency(mut self, settl_currency: String) -> Self {
        self.settl_currency = Some(settl_currency);
        self
    }

    /// Set standing instruction database type
    pub fn with_stand_inst_db_type(mut self, stand_inst_db_type: StandInstDbType) -> Self {
        self.stand_inst_db_type = Some(stand_inst_db_type);
        self
    }

    /// Set standing instruction database name
    pub fn with_stand_inst_db_name(mut self, stand_inst_db_name: String) -> Self {
        self.stand_inst_db_name = Some(stand_inst_db_name);
        self
    }
}

/// SettlementInstructions (MsgType=T)
///
/// Provides settlement instructions for trade settlement.
/// Can be sent from broker to institution, institution to broker, or to
/// an independent "standing instructions" database or matching system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementInstructions {
    /// SettlInstMsgID (777) - Unique message identifier
    pub settl_inst_msg_id: String,

    /// SettlInstMode (160) - Settlement instruction mode
    pub settl_inst_mode: SettlInstMode,

    /// TransactTime (60) - Message generation time
    pub transact_time: String,

    /// SettlInstReqID (791) - Reference to settlement instruction request (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_inst_req_id: Option<String>,

    /// SettlInstReqRejCode (792) - Rejection code if request was rejected (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_inst_req_rej_code: Option<SettlInstReqRejCode>,

    /// ClOrdID (11) - Client order ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,

    /// Side (54) - Side of the settlement instruction (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,

    /// Product (460) - Product type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// SecurityType (167) - Type of security (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// CFICode (461) - Classification of financial instrument (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,

    /// EffectiveTime (168) - Effective time for settlement instructions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,

    /// ExpireTime (126) - Expiration time for settlement instructions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,

    /// SettlCurrency (120) - Settlement currency (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_currency: Option<String>,

    /// Text (58) - Free format text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SettlementInstructions {
    /// Create a new SettlementInstructions message
    pub fn new(
        settl_inst_msg_id: String,
        settl_inst_mode: SettlInstMode,
        transact_time: String,
    ) -> Self {
        Self {
            settl_inst_msg_id,
            settl_inst_mode,
            transact_time,
            settl_inst_req_id: None,
            settl_inst_req_rej_code: None,
            cl_ord_id: None,
            side: None,
            product: None,
            security_type: None,
            cfi_code: None,
            effective_time: None,
            expire_time: None,
            settl_currency: None,
            text: None,
        }
    }

    /// Set settlement instruction request ID
    pub fn with_settl_inst_req_id(mut self, settl_inst_req_id: String) -> Self {
        self.settl_inst_req_id = Some(settl_inst_req_id);
        self
    }

    /// Set client order ID
    pub fn with_cl_ord_id(mut self, cl_ord_id: String) -> Self {
        self.cl_ord_id = Some(cl_ord_id);
        self
    }

    /// Set side
    pub fn with_side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    /// Set settlement currency
    pub fn with_settl_currency(mut self, settl_currency: String) -> Self {
        self.settl_currency = Some(settl_currency);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Set rejection code
    pub fn with_settl_inst_req_rej_code(mut self, rej_code: SettlInstReqRejCode) -> Self {
        self.settl_inst_req_rej_code = Some(rej_code);
        self
    }
}

/// SettlementObligationReport (MsgType=BQ)
///
/// Provides a central counterparty, institution, or individual counterparty
/// with a capacity for reporting the final details of a currency settlement obligation.
/// Designed to allow multiple FX deals to be aggregated and netted into a single instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettlementObligationReport {
    /// SettlObligMsgID (1160) - Unique settlement obligation message ID
    pub settl_oblig_msg_id: String,

    /// SettlObligMode (1159) - Settlement obligation mode (Preliminary or Final)
    pub settl_oblig_mode: SettlObligMode,

    /// ClearingBusinessDate (715) - Clearing business date (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_business_date: Option<String>,

    /// SettlementCycleNo (1153) - Settlement cycle number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_cycle_no: Option<u32>,

    /// TransactTime (60) - Transaction time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,

    /// EffectiveTime (168) - Effective time for settlement obligation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,

    /// ExpireTime (126) - Expiration time for settlement obligation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,

    /// Text (58) - Free format text (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SettlementObligationReport {
    /// Create a new SettlementObligationReport
    pub fn new(settl_oblig_msg_id: String, settl_oblig_mode: SettlObligMode) -> Self {
        Self {
            settl_oblig_msg_id,
            settl_oblig_mode,
            clearing_business_date: None,
            settlement_cycle_no: None,
            transact_time: None,
            effective_time: None,
            expire_time: None,
            text: None,
        }
    }

    /// Set clearing business date
    pub fn with_clearing_business_date(mut self, clearing_business_date: String) -> Self {
        self.clearing_business_date = Some(clearing_business_date);
        self
    }

    /// Set settlement cycle number
    pub fn with_settlement_cycle_no(mut self, settlement_cycle_no: u32) -> Self {
        self.settlement_cycle_no = Some(settlement_cycle_no);
        self
    }

    /// Set transaction time
    pub fn with_transact_time(mut self, transact_time: String) -> Self {
        self.transact_time = Some(transact_time);
        self
    }

    /// Set text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_instruction_request_creation() {
        let msg = SettlementInstructionRequest::new(
            "REQ123".to_string(),
            "20250101-12:00:00".to_string(),
        );

        assert_eq!(msg.settl_inst_req_id, "REQ123");
        assert_eq!(msg.transact_time, "20250101-12:00:00");
        assert!(msg.alloc_account.is_none());
        assert!(msg.side.is_none());
    }

    #[test]
    fn test_settlement_instruction_request_with_fields() {
        let msg = SettlementInstructionRequest::new(
            "REQ123".to_string(),
            "20250101-12:00:00".to_string(),
        )
        .with_alloc_account("ACC001".to_string())
        .with_side(Side::Buy)
        .with_settl_currency("USD".to_string())
        .with_stand_inst_db_type(StandInstDbType::DtcSid)
        .with_stand_inst_db_name("DTC".to_string());

        assert_eq!(msg.alloc_account, Some("ACC001".to_string()));
        assert_eq!(msg.side, Some(Side::Buy));
        assert_eq!(msg.settl_currency, Some("USD".to_string()));
        assert_eq!(msg.stand_inst_db_type, Some(StandInstDbType::DtcSid));
        assert_eq!(msg.stand_inst_db_name, Some("DTC".to_string()));
    }

    #[test]
    fn test_settlement_instructions_creation() {
        let msg = SettlementInstructions::new(
            "MSG456".to_string(),
            SettlInstMode::StandingInstructionsProvided,
            "20250101-12:00:00".to_string(),
        );

        assert_eq!(msg.settl_inst_msg_id, "MSG456");
        assert_eq!(
            msg.settl_inst_mode,
            SettlInstMode::StandingInstructionsProvided
        );
        assert_eq!(msg.transact_time, "20250101-12:00:00");
        assert!(msg.settl_inst_req_id.is_none());
        assert!(msg.cl_ord_id.is_none());
    }

    #[test]
    fn test_settlement_instructions_with_fields() {
        let msg = SettlementInstructions::new(
            "MSG456".to_string(),
            SettlInstMode::StandingInstructionsProvided,
            "20250101-12:00:00".to_string(),
        )
        .with_settl_inst_req_id("REQ123".to_string())
        .with_cl_ord_id("ORD789".to_string())
        .with_side(Side::Sell)
        .with_settl_currency("EUR".to_string())
        .with_text("Standing instructions".to_string());

        assert_eq!(msg.settl_inst_req_id, Some("REQ123".to_string()));
        assert_eq!(msg.cl_ord_id, Some("ORD789".to_string()));
        assert_eq!(msg.side, Some(Side::Sell));
        assert_eq!(msg.settl_currency, Some("EUR".to_string()));
        assert_eq!(msg.text, Some("Standing instructions".to_string()));
    }

    #[test]
    fn test_settlement_instructions_with_rejection() {
        let msg = SettlementInstructions::new(
            "MSG456".to_string(),
            SettlInstMode::RequestReject,
            "20250101-12:00:00".to_string(),
        )
        .with_settl_inst_req_id("REQ123".to_string())
        .with_settl_inst_req_rej_code(SettlInstReqRejCode::UnknownAccount);

        assert_eq!(msg.settl_inst_mode, SettlInstMode::RequestReject);
        assert_eq!(
            msg.settl_inst_req_rej_code,
            Some(SettlInstReqRejCode::UnknownAccount)
        );
    }

    #[test]
    fn test_settlement_obligation_report_creation() {
        let msg = SettlementObligationReport::new(
            "OBLIG123".to_string(),
            SettlObligMode::Preliminary,
        );

        assert_eq!(msg.settl_oblig_msg_id, "OBLIG123");
        assert_eq!(msg.settl_oblig_mode, SettlObligMode::Preliminary);
        assert!(msg.clearing_business_date.is_none());
        assert!(msg.settlement_cycle_no.is_none());
    }

    #[test]
    fn test_settlement_obligation_report_with_fields() {
        let msg = SettlementObligationReport::new(
            "OBLIG123".to_string(),
            SettlObligMode::Final,
        )
        .with_clearing_business_date("20250101".to_string())
        .with_settlement_cycle_no(5)
        .with_transact_time("20250101-12:00:00".to_string())
        .with_text("Final settlement obligation".to_string());

        assert_eq!(msg.settl_oblig_mode, SettlObligMode::Final);
        assert_eq!(
            msg.clearing_business_date,
            Some("20250101".to_string())
        );
        assert_eq!(msg.settlement_cycle_no, Some(5));
        assert_eq!(msg.transact_time, Some("20250101-12:00:00".to_string()));
        assert_eq!(
            msg.text,
            Some("Final settlement obligation".to_string())
        );
    }

    #[test]
    fn test_settlement_obligation_modes() {
        let preliminary = SettlementObligationReport::new(
            "OBLIG1".to_string(),
            SettlObligMode::Preliminary,
        );
        let final_report = SettlementObligationReport::new(
            "OBLIG2".to_string(),
            SettlObligMode::Final,
        );

        assert_eq!(preliminary.settl_oblig_mode, SettlObligMode::Preliminary);
        assert_eq!(final_report.settl_oblig_mode, SettlObligMode::Final);
    }
}
