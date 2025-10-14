use crate::parser::{FixParseError, RawFixMessage};
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Confirmation Messages
// Implementation of FIX 5.0 SP2 Post-Trade Confirmation messages
// ============================================================================

// ============================================================================
// Confirmation (MsgType = AK)
// ============================================================================

/// Confirmation (AK) - Individual trade level confirmation
///
/// Used to provide individual trade level confirmations from the sell side
/// to the buy side. The Confirmation operates at an allocation account
/// (trade) level rather than block level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Confirmation {
    /// ConfirmID (Tag 664) - Required - Unique identifier for this confirmation
    pub confirm_id: String,

    /// ConfirmRefID (Tag 772) - Optional - Reference to a previous confirmation
    pub confirm_ref_id: Option<String>,

    /// ConfirmReqID (Tag 859) - Optional - Reference to confirmation request
    pub confirm_req_id: Option<String>,

    /// ConfirmTransType (Tag 666) - Required - Transaction type
    pub confirm_trans_type: ConfirmTransType,

    /// ConfirmType (Tag 773) - Required - Type of confirmation
    pub confirm_type: ConfirmType,

    /// ConfirmStatus (Tag 665) - Required - Status of confirmation
    pub confirm_status: ConfirmStatus,

    /// AllocQty (Tag 80) - Required - Quantity allocated
    pub alloc_qty: f64,

    /// AllocAccount (Tag 79) - Required - Account to which allocation is made
    pub alloc_account: String,

    /// AvgPx (Tag 6) - Required - Average price
    pub avg_px: f64,

    /// GrossTradeAmt (Tag 381) - Required - Gross trade amount
    pub gross_trade_amt: f64,

    /// NetMoney (Tag 118) - Required - Net money
    pub net_money: f64,

    /// Side (Tag 54) - Optional - Side of trade
    pub side: Option<Side>,

    /// Symbol (Tag 55) - Optional - Security symbol
    pub symbol: Option<String>,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// TradeDate (Tag 75) - Optional - Trade date
    pub trade_date: Option<String>,

    /// TransactTime (Tag 60) - Optional - Transaction time
    pub transact_time: Option<String>,

    /// SettlDate (Tag 64) - Optional - Settlement date
    pub settl_date: Option<String>,

    /// Commission (Tag 12) - Optional - Commission amount
    pub commission: Option<f64>,

    /// CommType (Tag 13) - Optional - Commission type
    pub comm_type: Option<String>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // Note: OrdAllocGrp - Repeating group for order allocation
    // Note: CpctyConfGrp - Repeating group for capacity confirmation
    // Note: MiscFeesGrp - Repeating group for miscellaneous fees
    // These are stored in the RawFixMessage.groups field
}

impl Confirmation {
    /// Create a new Confirmation message
    pub fn new(
        confirm_id: String,
        confirm_trans_type: ConfirmTransType,
        confirm_type: ConfirmType,
        confirm_status: ConfirmStatus,
        alloc_qty: f64,
        alloc_account: String,
        avg_px: f64,
        gross_trade_amt: f64,
        net_money: f64,
    ) -> Self {
        Confirmation {
            confirm_id,
            confirm_ref_id: None,
            confirm_req_id: None,
            confirm_trans_type,
            confirm_type,
            confirm_status,
            alloc_qty,
            alloc_account,
            avg_px,
            gross_trade_amt,
            net_money,
            side: None,
            symbol: None,
            security_id: None,
            security_id_source: None,
            trade_date: None,
            transact_time: None,
            settl_date: None,
            commission: None,
            comm_type: None,
            text: None,
        }
    }

    /// Set optional confirm reference ID
    pub fn with_confirm_ref_id(mut self, confirm_ref_id: String) -> Self {
        self.confirm_ref_id = Some(confirm_ref_id);
        self
    }

    /// Set optional confirm request ID
    pub fn with_confirm_req_id(mut self, confirm_req_id: String) -> Self {
        self.confirm_req_id = Some(confirm_req_id);
        self
    }

    /// Set optional side
    pub fn with_side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    /// Set optional symbol
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set optional security ID
    pub fn with_security_id(mut self, security_id: String) -> Self {
        self.security_id = Some(security_id);
        self
    }

    /// Set optional security ID source
    pub fn with_security_id_source(mut self, security_id_source: String) -> Self {
        self.security_id_source = Some(security_id_source);
        self
    }

    /// Set optional trade date
    pub fn with_trade_date(mut self, trade_date: String) -> Self {
        self.trade_date = Some(trade_date);
        self
    }

    /// Set optional transaction time
    pub fn with_transact_time(mut self, transact_time: String) -> Self {
        self.transact_time = Some(transact_time);
        self
    }

    /// Set optional settlement date
    pub fn with_settl_date(mut self, settl_date: String) -> Self {
        self.settl_date = Some(settl_date);
        self
    }

    /// Set optional commission
    pub fn with_commission(mut self, commission: f64) -> Self {
        self.commission = Some(commission);
        self
    }

    /// Set optional commission type
    pub fn with_comm_type(mut self, comm_type: String) -> Self {
        self.comm_type = Some(comm_type);
        self
    }

    /// Set optional text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(Confirmation {
            confirm_id: raw
                .get_field(664)
                .ok_or_else(|| FixParseError::MissingRequiredField(664))?
                .to_string(),
            confirm_ref_id: raw.get_field(772).map(|s| s.to_string()),
            confirm_req_id: raw.get_field(859).map(|s| s.to_string()),
            confirm_trans_type: raw
                .get_field(666)
                .and_then(|s| s.parse().ok())
                .and_then(ConfirmTransType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(666))?,
            confirm_type: raw
                .get_field(773)
                .and_then(|s| s.parse().ok())
                .and_then(ConfirmType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(773))?,
            confirm_status: raw
                .get_field(665)
                .and_then(|s| s.parse().ok())
                .and_then(ConfirmStatus::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(665))?,
            alloc_qty: raw
                .get_field(80)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(80))?,
            alloc_account: raw
                .get_field(79)
                .ok_or_else(|| FixParseError::MissingRequiredField(79))?
                .to_string(),
            avg_px: raw
                .get_field(6)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(6))?,
            gross_trade_amt: raw
                .get_field(381)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(381))?,
            net_money: raw
                .get_field(118)
                .and_then(|s| s.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(118))?,
            side: raw
                .get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            trade_date: raw.get_field(75).map(|s| s.to_string()),
            transact_time: raw.get_field(60).map(|s| s.to_string()),
            settl_date: raw.get_field(64).map(|s| s.to_string()),
            commission: raw.get_field(12).and_then(|s| s.parse().ok()),
            comm_type: raw.get_field(13).map(|s| s.to_string()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AK".to_string());

        // Required fields
        msg.set_field(664, self.confirm_id.clone());
        msg.set_field(666, self.confirm_trans_type.to_fix().to_string());
        msg.set_field(773, self.confirm_type.to_fix().to_string());
        msg.set_field(665, self.confirm_status.to_fix().to_string());
        msg.set_field(80, self.alloc_qty.to_string());
        msg.set_field(79, self.alloc_account.clone());
        msg.set_field(6, self.avg_px.to_string());
        msg.set_field(381, self.gross_trade_amt.to_string());
        msg.set_field(118, self.net_money.to_string());

        // Optional fields
        if let Some(ref confirm_ref_id) = self.confirm_ref_id {
            msg.set_field(772, confirm_ref_id.clone());
        }
        if let Some(ref confirm_req_id) = self.confirm_req_id {
            msg.set_field(859, confirm_req_id.clone());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(ref security_id) = self.security_id {
            msg.set_field(48, security_id.clone());
        }
        if let Some(ref security_id_source) = self.security_id_source {
            msg.set_field(22, security_id_source.clone());
        }
        if let Some(ref trade_date) = self.trade_date {
            msg.set_field(75, trade_date.clone());
        }
        if let Some(ref transact_time) = self.transact_time {
            msg.set_field(60, transact_time.clone());
        }
        if let Some(ref settl_date) = self.settl_date {
            msg.set_field(64, settl_date.clone());
        }
        if let Some(commission) = self.commission {
            msg.set_field(12, commission.to_string());
        }
        if let Some(ref comm_type) = self.comm_type {
            msg.set_field(13, comm_type.clone());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }
}

// ============================================================================
// ConfirmationAck (MsgType = AU)
// ============================================================================

/// ConfirmationAck (AU) - Acknowledgement of a Confirmation message
///
/// Used to respond to a Confirmation message. Also known as Affirmation message.
/// Indicates whether the buy side affirms or rejects the confirmation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfirmationAck {
    /// ConfirmID (Tag 664) - Required - Reference to confirmation being acknowledged
    pub confirm_id: String,

    /// TradeDate (Tag 75) - Required - Trade date
    pub trade_date: String,

    /// TransactTime (Tag 60) - Required - Transaction time
    pub transact_time: String,

    /// AffirmStatus (Tag 940) - Required - Status of affirmation
    pub affirm_status: AffirmStatus,

    /// ConfirmRejReason (Tag 774) - Optional - Reason for rejection
    pub confirm_rej_reason: Option<ConfirmRejReason>,

    /// MatchStatus (Tag 573) - Optional - Match status
    pub match_status: Option<String>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354) - Optional - Encoded text length
    pub encoded_text_len: Option<u32>,

    /// EncodedText (Tag 355) - Optional - Encoded text
    pub encoded_text: Option<String>,
}

impl ConfirmationAck {
    /// Create a new ConfirmationAck message
    pub fn new(
        confirm_id: String,
        trade_date: String,
        transact_time: String,
        affirm_status: AffirmStatus,
    ) -> Self {
        ConfirmationAck {
            confirm_id,
            trade_date,
            transact_time,
            affirm_status,
            confirm_rej_reason: None,
            match_status: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }

    /// Set optional confirm reject reason
    pub fn with_confirm_rej_reason(mut self, confirm_rej_reason: ConfirmRejReason) -> Self {
        self.confirm_rej_reason = Some(confirm_rej_reason);
        self
    }

    /// Set optional match status
    pub fn with_match_status(mut self, match_status: String) -> Self {
        self.match_status = Some(match_status);
        self
    }

    /// Set optional text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Set optional encoded text
    pub fn with_encoded_text(mut self, encoded_text_len: u32, encoded_text: String) -> Self {
        self.encoded_text_len = Some(encoded_text_len);
        self.encoded_text = Some(encoded_text);
        self
    }

    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(ConfirmationAck {
            confirm_id: raw
                .get_field(664)
                .ok_or_else(|| FixParseError::MissingRequiredField(664))?
                .to_string(),
            trade_date: raw
                .get_field(75)
                .ok_or_else(|| FixParseError::MissingRequiredField(75))?
                .to_string(),
            transact_time: raw
                .get_field(60)
                .ok_or_else(|| FixParseError::MissingRequiredField(60))?
                .to_string(),
            affirm_status: raw
                .get_field(940)
                .and_then(|s| s.parse().ok())
                .and_then(AffirmStatus::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(940))?,
            confirm_rej_reason: raw
                .get_field(774)
                .and_then(|s| s.parse().ok())
                .and_then(ConfirmRejReason::from_fix),
            match_status: raw.get_field(573).map(|s| s.to_string()),
            text: raw.get_field(58).map(|s| s.to_string()),
            encoded_text_len: raw.get_field(354).and_then(|s| s.parse().ok()),
            encoded_text: raw.get_field(355).map(|s| s.to_string()),
        })
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AU".to_string());

        // Required fields
        msg.set_field(664, self.confirm_id.clone());
        msg.set_field(75, self.trade_date.clone());
        msg.set_field(60, self.transact_time.clone());
        msg.set_field(940, self.affirm_status.to_fix().to_string());

        // Optional fields
        if let Some(confirm_rej_reason) = self.confirm_rej_reason {
            msg.set_field(774, confirm_rej_reason.to_fix().to_string());
        }
        if let Some(ref match_status) = self.match_status {
            msg.set_field(573, match_status.clone());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }
        if let Some(encoded_text_len) = self.encoded_text_len {
            msg.set_field(354, encoded_text_len.to_string());
        }
        if let Some(ref encoded_text) = self.encoded_text {
            msg.set_field(355, encoded_text.clone());
        }

        msg
    }
}

// ============================================================================
// ConfirmationRequest (MsgType = BH)
// ============================================================================

/// ConfirmationRequest (BH) - Request for a Confirmation message
///
/// Used to request a Confirmation message or trade status message. Can be used
/// to refer to an earlier Allocation Instruction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfirmationRequest {
    /// ConfirmReqID (Tag 859) - Required - Unique identifier for this request
    pub confirm_req_id: String,

    /// ConfirmType (Tag 773) - Required - Type of confirmation being requested
    pub confirm_type: ConfirmType,

    /// TransactTime (Tag 60) - Required - Time this message was generated
    pub transact_time: String,

    /// AllocID (Tag 70) - Optional - Reference to earlier Allocation Instruction
    pub alloc_id: Option<String>,

    /// SecondaryAllocID (Tag 793) - Optional - Secondary reference to Allocation Instruction
    pub secondary_alloc_id: Option<String>,

    /// IndividualAllocID (Tag 467) - Optional - Reference to allocation account
    pub individual_alloc_id: Option<String>,

    /// AllocAccount (Tag 79) - Optional - Allocation account
    pub alloc_account: Option<String>,

    /// AllocAcctIDSource (Tag 661) - Optional - Account ID source
    pub alloc_acct_id_source: Option<u32>,

    /// AllocAccountType (Tag 798) - Optional - Account type
    pub alloc_account_type: Option<u32>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354) - Optional - Encoded text length
    pub encoded_text_len: Option<u32>,

    /// EncodedText (Tag 355) - Optional - Encoded text
    pub encoded_text: Option<String>,

    // Note: OrdAllocGrp - Repeating group for order allocation
    // Stored in the RawFixMessage.groups field
}

impl ConfirmationRequest {
    /// Create a new ConfirmationRequest message
    pub fn new(
        confirm_req_id: String,
        confirm_type: ConfirmType,
        transact_time: String,
    ) -> Self {
        ConfirmationRequest {
            confirm_req_id,
            confirm_type,
            transact_time,
            alloc_id: None,
            secondary_alloc_id: None,
            individual_alloc_id: None,
            alloc_account: None,
            alloc_acct_id_source: None,
            alloc_account_type: None,
            text: None,
            encoded_text_len: None,
            encoded_text: None,
        }
    }

    /// Set optional alloc ID
    pub fn with_alloc_id(mut self, alloc_id: String) -> Self {
        self.alloc_id = Some(alloc_id);
        self
    }

    /// Set optional secondary alloc ID
    pub fn with_secondary_alloc_id(mut self, secondary_alloc_id: String) -> Self {
        self.secondary_alloc_id = Some(secondary_alloc_id);
        self
    }

    /// Set optional individual alloc ID
    pub fn with_individual_alloc_id(mut self, individual_alloc_id: String) -> Self {
        self.individual_alloc_id = Some(individual_alloc_id);
        self
    }

    /// Set optional alloc account
    pub fn with_alloc_account(mut self, alloc_account: String) -> Self {
        self.alloc_account = Some(alloc_account);
        self
    }

    /// Set optional alloc account ID source
    pub fn with_alloc_acct_id_source(mut self, alloc_acct_id_source: u32) -> Self {
        self.alloc_acct_id_source = Some(alloc_acct_id_source);
        self
    }

    /// Set optional alloc account type
    pub fn with_alloc_account_type(mut self, alloc_account_type: u32) -> Self {
        self.alloc_account_type = Some(alloc_account_type);
        self
    }

    /// Set optional text
    pub fn with_text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }

    /// Set optional encoded text
    pub fn with_encoded_text(mut self, encoded_text_len: u32, encoded_text: String) -> Self {
        self.encoded_text_len = Some(encoded_text_len);
        self.encoded_text = Some(encoded_text);
        self
    }

    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(ConfirmationRequest {
            confirm_req_id: raw
                .get_field(859)
                .ok_or_else(|| FixParseError::MissingRequiredField(859))?
                .to_string(),
            confirm_type: raw
                .get_field(773)
                .and_then(|s| s.parse().ok())
                .and_then(ConfirmType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(773))?,
            transact_time: raw
                .get_field(60)
                .ok_or_else(|| FixParseError::MissingRequiredField(60))?
                .to_string(),
            alloc_id: raw.get_field(70).map(|s| s.to_string()),
            secondary_alloc_id: raw.get_field(793).map(|s| s.to_string()),
            individual_alloc_id: raw.get_field(467).map(|s| s.to_string()),
            alloc_account: raw.get_field(79).map(|s| s.to_string()),
            alloc_acct_id_source: raw.get_field(661).and_then(|s| s.parse().ok()),
            alloc_account_type: raw.get_field(798).and_then(|s| s.parse().ok()),
            text: raw.get_field(58).map(|s| s.to_string()),
            encoded_text_len: raw.get_field(354).and_then(|s| s.parse().ok()),
            encoded_text: raw.get_field(355).map(|s| s.to_string()),
        })
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "BH".to_string());

        // Required fields
        msg.set_field(859, self.confirm_req_id.clone());
        msg.set_field(773, self.confirm_type.to_fix().to_string());
        msg.set_field(60, self.transact_time.clone());

        // Optional fields
        if let Some(ref alloc_id) = self.alloc_id {
            msg.set_field(70, alloc_id.clone());
        }
        if let Some(ref secondary_alloc_id) = self.secondary_alloc_id {
            msg.set_field(793, secondary_alloc_id.clone());
        }
        if let Some(ref individual_alloc_id) = self.individual_alloc_id {
            msg.set_field(467, individual_alloc_id.clone());
        }
        if let Some(ref alloc_account) = self.alloc_account {
            msg.set_field(79, alloc_account.clone());
        }
        if let Some(alloc_acct_id_source) = self.alloc_acct_id_source {
            msg.set_field(661, alloc_acct_id_source.to_string());
        }
        if let Some(alloc_account_type) = self.alloc_account_type {
            msg.set_field(798, alloc_account_type.to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }
        if let Some(encoded_text_len) = self.encoded_text_len {
            msg.set_field(354, encoded_text_len.to_string());
        }
        if let Some(ref encoded_text) = self.encoded_text {
            msg.set_field(355, encoded_text.clone());
        }

        msg
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Confirmation Tests
    // ========================================================================

    #[test]
    fn test_confirmation_new() {
        let confirmation = Confirmation::new(
            "CONF123".to_string(),
            ConfirmTransType::New,
            ConfirmType::Confirmation,
            ConfirmStatus::Received,
            1000.0,
            "ACCT001".to_string(),
            50.25,
            50250.0,
            50200.0,
        );

        assert_eq!(confirmation.confirm_id, "CONF123");
        assert_eq!(confirmation.confirm_trans_type, ConfirmTransType::New);
        assert_eq!(confirmation.confirm_type, ConfirmType::Confirmation);
        assert_eq!(confirmation.confirm_status, ConfirmStatus::Received);
        assert_eq!(confirmation.alloc_qty, 1000.0);
        assert_eq!(confirmation.alloc_account, "ACCT001");
        assert_eq!(confirmation.avg_px, 50.25);
        assert_eq!(confirmation.gross_trade_amt, 50250.0);
        assert_eq!(confirmation.net_money, 50200.0);
        assert_eq!(confirmation.confirm_ref_id, None);
        assert_eq!(confirmation.side, None);
        assert_eq!(confirmation.symbol, None);
    }

    #[test]
    fn test_confirmation_with_optional_fields() {
        let confirmation = Confirmation::new(
            "CONF456".to_string(),
            ConfirmTransType::Replace,
            ConfirmType::Status,
            ConfirmStatus::Confirmed,
            500.0,
            "ACCT002".to_string(),
            100.50,
            50250.0,
            50150.0,
        )
        .with_confirm_ref_id("CONF123".to_string())
        .with_confirm_req_id("REQ001".to_string())
        .with_side(Side::Buy)
        .with_symbol("AAPL".to_string())
        .with_security_id("US0378331005".to_string())
        .with_security_id_source("ISIN".to_string())
        .with_trade_date("20250109".to_string())
        .with_transact_time("20250109-11:00:00".to_string())
        .with_settl_date("20250112".to_string())
        .with_commission(10.0)
        .with_comm_type("1".to_string())
        .with_text("Trade confirmed".to_string());

        assert_eq!(confirmation.confirm_id, "CONF456");
        assert_eq!(confirmation.confirm_ref_id, Some("CONF123".to_string()));
        assert_eq!(confirmation.confirm_req_id, Some("REQ001".to_string()));
        assert_eq!(confirmation.side, Some(Side::Buy));
        assert_eq!(confirmation.symbol, Some("AAPL".to_string()));
        assert_eq!(confirmation.security_id, Some("US0378331005".to_string()));
        assert_eq!(confirmation.security_id_source, Some("ISIN".to_string()));
        assert_eq!(confirmation.trade_date, Some("20250109".to_string()));
        assert_eq!(confirmation.transact_time, Some("20250109-11:00:00".to_string()));
        assert_eq!(confirmation.settl_date, Some("20250112".to_string()));
        assert_eq!(confirmation.commission, Some(10.0));
        assert_eq!(confirmation.comm_type, Some("1".to_string()));
        assert_eq!(confirmation.text, Some("Trade confirmed".to_string()));
    }

    #[test]
    fn test_confirmation_round_trip() {
        let confirmation = Confirmation::new(
            "CONF789".to_string(),
            ConfirmTransType::New,
            ConfirmType::Confirmation,
            ConfirmStatus::Confirmed,
            2000.0,
            "ACCT003".to_string(),
            75.00,
            150000.0,
            149900.0,
        )
        .with_side(Side::Sell)
        .with_symbol("MSFT".to_string())
        .with_trade_date("20250109".to_string())
        .with_commission(100.0);

        let raw = confirmation.to_raw();
        let parsed = Confirmation::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.confirm_id, confirmation.confirm_id);
        assert_eq!(parsed.confirm_trans_type, confirmation.confirm_trans_type);
        assert_eq!(parsed.confirm_type, confirmation.confirm_type);
        assert_eq!(parsed.confirm_status, confirmation.confirm_status);
        assert_eq!(parsed.alloc_qty, confirmation.alloc_qty);
        assert_eq!(parsed.alloc_account, confirmation.alloc_account);
        assert_eq!(parsed.avg_px, confirmation.avg_px);
        assert_eq!(parsed.gross_trade_amt, confirmation.gross_trade_amt);
        assert_eq!(parsed.net_money, confirmation.net_money);
        assert_eq!(parsed.side, confirmation.side);
        assert_eq!(parsed.symbol, confirmation.symbol);
        assert_eq!(parsed.trade_date, confirmation.trade_date);
        assert_eq!(parsed.commission, confirmation.commission);
    }

    #[test]
    fn test_confirmation_to_raw_msg_type() {
        let confirmation = Confirmation::new(
            "CONF001".to_string(),
            ConfirmTransType::New,
            ConfirmType::Confirmation,
            ConfirmStatus::Received,
            100.0,
            "ACCT001".to_string(),
            10.0,
            1000.0,
            990.0,
        );

        let raw = confirmation.to_raw();
        assert_eq!(raw.get_field(35), Some("AK"));
    }

    #[test]
    fn test_confirmation_all_statuses() {
        let statuses = vec![
            ConfirmStatus::Received,
            ConfirmStatus::MismatchedAccount,
            ConfirmStatus::MissingSettlementInstructions,
            ConfirmStatus::Confirmed,
            ConfirmStatus::RequestRejected,
        ];

        for status in statuses {
            let confirmation = Confirmation::new(
                "CONF".to_string(),
                ConfirmTransType::New,
                ConfirmType::Confirmation,
                status,
                100.0,
                "ACCT".to_string(),
                10.0,
                1000.0,
                990.0,
            );

            let raw = confirmation.to_raw();
            let parsed = Confirmation::from_raw(&raw).expect("Should parse");
            assert_eq!(parsed.confirm_status, status);
        }
    }

    #[test]
    fn test_confirmation_all_trans_types() {
        let trans_types = vec![
            ConfirmTransType::New,
            ConfirmTransType::Replace,
            ConfirmTransType::Cancel,
        ];

        for trans_type in trans_types {
            let confirmation = Confirmation::new(
                "CONF".to_string(),
                trans_type,
                ConfirmType::Confirmation,
                ConfirmStatus::Received,
                100.0,
                "ACCT".to_string(),
                10.0,
                1000.0,
                990.0,
            );

            let raw = confirmation.to_raw();
            let parsed = Confirmation::from_raw(&raw).expect("Should parse");
            assert_eq!(parsed.confirm_trans_type, trans_type);
        }
    }

    // ========================================================================
    // ConfirmationAck Tests
    // ========================================================================

    #[test]
    fn test_confirmation_ack_new() {
        let ack = ConfirmationAck::new(
            "CONF123".to_string(),
            "20250109".to_string(),
            "20250109-11:00:00".to_string(),
            AffirmStatus::Affirmed,
        );

        assert_eq!(ack.confirm_id, "CONF123");
        assert_eq!(ack.trade_date, "20250109");
        assert_eq!(ack.transact_time, "20250109-11:00:00");
        assert_eq!(ack.affirm_status, AffirmStatus::Affirmed);
        assert_eq!(ack.confirm_rej_reason, None);
        assert_eq!(ack.match_status, None);
        assert_eq!(ack.text, None);
    }

    #[test]
    fn test_confirmation_ack_with_rejection() {
        let ack = ConfirmationAck::new(
            "CONF456".to_string(),
            "20250109".to_string(),
            "20250109-12:00:00".to_string(),
            AffirmStatus::ConfirmRejected,
        )
        .with_confirm_rej_reason(ConfirmRejReason::MismatchedAccount)
        .with_match_status("0".to_string())
        .with_text("Account mismatch detected".to_string());

        assert_eq!(ack.affirm_status, AffirmStatus::ConfirmRejected);
        assert_eq!(ack.confirm_rej_reason, Some(ConfirmRejReason::MismatchedAccount));
        assert_eq!(ack.match_status, Some("0".to_string()));
        assert_eq!(ack.text, Some("Account mismatch detected".to_string()));
    }

    #[test]
    fn test_confirmation_ack_with_encoded_text() {
        let ack = ConfirmationAck::new(
            "CONF789".to_string(),
            "20250109".to_string(),
            "20250109-13:00:00".to_string(),
            AffirmStatus::Received,
        )
        .with_encoded_text(10, "EncodedMsg".to_string());

        assert_eq!(ack.encoded_text_len, Some(10));
        assert_eq!(ack.encoded_text, Some("EncodedMsg".to_string()));
    }

    #[test]
    fn test_confirmation_ack_round_trip() {
        let ack = ConfirmationAck::new(
            "CONF001".to_string(),
            "20250110".to_string(),
            "20250110-10:30:00".to_string(),
            AffirmStatus::Affirmed,
        )
        .with_match_status("1".to_string())
        .with_text("Affirmation successful".to_string());

        let raw = ack.to_raw();
        let parsed = ConfirmationAck::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.confirm_id, ack.confirm_id);
        assert_eq!(parsed.trade_date, ack.trade_date);
        assert_eq!(parsed.transact_time, ack.transact_time);
        assert_eq!(parsed.affirm_status, ack.affirm_status);
        assert_eq!(parsed.match_status, ack.match_status);
        assert_eq!(parsed.text, ack.text);
    }

    #[test]
    fn test_confirmation_ack_to_raw_msg_type() {
        let ack = ConfirmationAck::new(
            "CONF001".to_string(),
            "20250109".to_string(),
            "20250109-11:00:00".to_string(),
            AffirmStatus::Affirmed,
        );

        let raw = ack.to_raw();
        assert_eq!(raw.get_field(35), Some("AU"));
    }

    #[test]
    fn test_confirmation_ack_all_affirm_statuses() {
        let statuses = vec![
            AffirmStatus::Received,
            AffirmStatus::ConfirmRejected,
            AffirmStatus::Affirmed,
        ];

        for status in statuses {
            let ack = ConfirmationAck::new(
                "CONF".to_string(),
                "20250109".to_string(),
                "20250109-11:00:00".to_string(),
                status,
            );

            let raw = ack.to_raw();
            let parsed = ConfirmationAck::from_raw(&raw).expect("Should parse");
            assert_eq!(parsed.affirm_status, status);
        }
    }

    #[test]
    fn test_confirmation_ack_all_reject_reasons() {
        let reasons = vec![
            ConfirmRejReason::MismatchedAccount,
            ConfirmRejReason::MissingSettlementInstructions,
            ConfirmRejReason::Other,
        ];

        for reason in reasons {
            let ack = ConfirmationAck::new(
                "CONF".to_string(),
                "20250109".to_string(),
                "20250109-11:00:00".to_string(),
                AffirmStatus::ConfirmRejected,
            )
            .with_confirm_rej_reason(reason);

            let raw = ack.to_raw();
            let parsed = ConfirmationAck::from_raw(&raw).expect("Should parse");
            assert_eq!(parsed.confirm_rej_reason, Some(reason));
        }
    }

    // ========================================================================
    // ConfirmationRequest Tests
    // ========================================================================

    #[test]
    fn test_confirmation_request_new() {
        let request = ConfirmationRequest::new(
            "REQ123".to_string(),
            ConfirmType::Confirmation,
            "20250109-11:00:00".to_string(),
        );

        assert_eq!(request.confirm_req_id, "REQ123");
        assert_eq!(request.confirm_type, ConfirmType::Confirmation);
        assert_eq!(request.transact_time, "20250109-11:00:00");
        assert_eq!(request.alloc_id, None);
        assert_eq!(request.alloc_account, None);
        assert_eq!(request.text, None);
    }

    #[test]
    fn test_confirmation_request_with_allocation_reference() {
        let request = ConfirmationRequest::new(
            "REQ456".to_string(),
            ConfirmType::Status,
            "20250109-12:00:00".to_string(),
        )
        .with_alloc_id("ALLOC001".to_string())
        .with_secondary_alloc_id("SECALLOC001".to_string())
        .with_individual_alloc_id("INDALLOC001".to_string())
        .with_alloc_account("ACCT123".to_string())
        .with_alloc_acct_id_source(1)
        .with_alloc_account_type(2);

        assert_eq!(request.alloc_id, Some("ALLOC001".to_string()));
        assert_eq!(request.secondary_alloc_id, Some("SECALLOC001".to_string()));
        assert_eq!(request.individual_alloc_id, Some("INDALLOC001".to_string()));
        assert_eq!(request.alloc_account, Some("ACCT123".to_string()));
        assert_eq!(request.alloc_acct_id_source, Some(1));
        assert_eq!(request.alloc_account_type, Some(2));
    }

    #[test]
    fn test_confirmation_request_with_text() {
        let request = ConfirmationRequest::new(
            "REQ789".to_string(),
            ConfirmType::ConfirmationRequestRejected,
            "20250109-13:00:00".to_string(),
        )
        .with_text("Requesting confirmation status".to_string())
        .with_encoded_text(20, "EncodedRequest".to_string());

        assert_eq!(request.text, Some("Requesting confirmation status".to_string()));
        assert_eq!(request.encoded_text_len, Some(20));
        assert_eq!(request.encoded_text, Some("EncodedRequest".to_string()));
    }

    #[test]
    fn test_confirmation_request_round_trip() {
        let request = ConfirmationRequest::new(
            "REQ001".to_string(),
            ConfirmType::Confirmation,
            "20250110-10:00:00".to_string(),
        )
        .with_alloc_id("ALLOC123".to_string())
        .with_alloc_account("ACCT456".to_string())
        .with_text("Status check".to_string());

        let raw = request.to_raw();
        let parsed = ConfirmationRequest::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.confirm_req_id, request.confirm_req_id);
        assert_eq!(parsed.confirm_type, request.confirm_type);
        assert_eq!(parsed.transact_time, request.transact_time);
        assert_eq!(parsed.alloc_id, request.alloc_id);
        assert_eq!(parsed.alloc_account, request.alloc_account);
        assert_eq!(parsed.text, request.text);
    }

    #[test]
    fn test_confirmation_request_to_raw_msg_type() {
        let request = ConfirmationRequest::new(
            "REQ001".to_string(),
            ConfirmType::Confirmation,
            "20250109-11:00:00".to_string(),
        );

        let raw = request.to_raw();
        assert_eq!(raw.get_field(35), Some("BH"));
    }

    #[test]
    fn test_confirmation_request_all_confirm_types() {
        let confirm_types = vec![
            ConfirmType::Status,
            ConfirmType::Confirmation,
            ConfirmType::ConfirmationRequestRejected,
        ];

        for confirm_type in confirm_types {
            let request = ConfirmationRequest::new(
                "REQ".to_string(),
                confirm_type,
                "20250109-11:00:00".to_string(),
            );

            let raw = request.to_raw();
            let parsed = ConfirmationRequest::from_raw(&raw).expect("Should parse");
            assert_eq!(parsed.confirm_type, confirm_type);
        }
    }

    #[test]
    fn test_confirmation_request_minimal() {
        let request = ConfirmationRequest::new(
            "REQ_MIN".to_string(),
            ConfirmType::Status,
            "20250109-14:00:00".to_string(),
        );

        let raw = request.to_raw();
        let parsed = ConfirmationRequest::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.confirm_req_id, "REQ_MIN");
        assert_eq!(parsed.confirm_type, ConfirmType::Status);
        assert_eq!(parsed.transact_time, "20250109-14:00:00");
        assert_eq!(parsed.alloc_id, None);
        assert_eq!(parsed.secondary_alloc_id, None);
        assert_eq!(parsed.individual_alloc_id, None);
        assert_eq!(parsed.alloc_account, None);
        assert_eq!(parsed.text, None);
    }
}
