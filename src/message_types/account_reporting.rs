use crate::components::Parties;
use crate::parser::{FixParseError, RawFixMessage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Account Reporting Messages
// Implementation of FIX 5.0 SP2 Post-Trade Account Reporting messages
// ============================================================================

/// AccountSummaryReport (MsgType=CQ)
///
/// Provided by the clearinghouse to its clearing members on a daily basis.
/// Contains margin, settlement, collateral and pay/collect data for each
/// clearing member level account type. Clearing member account types are
/// described through use of the Parties component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSummaryReport {
    /// AccountSummaryReportID (Tag 1699) - Unique identifier for this report
    pub account_summary_report_id: String,

    /// ClearingBusinessDate (Tag 715) - The business date for which the report is generated
    pub clearing_business_date: String,

    /// Parties - Identifies the account parties (clearing member, customer, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parties: Option<Vec<Parties>>,

    /// Currency (Tag 15) - Currency used for monetary amounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// TotalNetValue (Tag 900) - Total net value of the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_value: Option<f64>,

    /// MarginExcess (Tag 899) - Excess margin amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_excess: Option<f64>,

    /// SettlSessID (Tag 716) - Settlement session identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_sess_id: Option<String>,

    /// SettlSessSubID (Tag 717) - Settlement session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settl_sess_sub_id: Option<String>,

    /// TransactTime (Tag 60) - Time this report was generated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<DateTime<Utc>>,

    /// Text (Tag 58) - Free format text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl AccountSummaryReport {
    /// Create a new AccountSummaryReport with required fields
    pub fn new(
        account_summary_report_id: impl Into<String>,
        clearing_business_date: impl Into<String>,
    ) -> Self {
        Self {
            account_summary_report_id: account_summary_report_id.into(),
            clearing_business_date: clearing_business_date.into(),
            parties: None,
            currency: None,
            total_net_value: None,
            margin_excess: None,
            settl_sess_id: None,
            settl_sess_sub_id: None,
            transact_time: None,
            text: None,
        }
    }

    /// Set the parties component
    pub fn with_parties(mut self, parties: Vec<Parties>) -> Self {
        self.parties = Some(parties);
        self
    }

    /// Set the currency
    pub fn with_currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Set the total net value
    pub fn with_total_net_value(mut self, total_net_value: f64) -> Self {
        self.total_net_value = Some(total_net_value);
        self
    }

    /// Set the margin excess
    pub fn with_margin_excess(mut self, margin_excess: f64) -> Self {
        self.margin_excess = Some(margin_excess);
        self
    }

    /// Set the settlement session ID
    pub fn with_settl_sess_id(mut self, settl_sess_id: impl Into<String>) -> Self {
        self.settl_sess_id = Some(settl_sess_id.into());
        self
    }

    /// Set the settlement session sub-ID
    pub fn with_settl_sess_sub_id(mut self, settl_sess_sub_id: impl Into<String>) -> Self {
        self.settl_sess_sub_id = Some(settl_sess_sub_id.into());
        self
    }

    /// Set the transaction time
    pub fn with_transact_time(mut self, transact_time: DateTime<Utc>) -> Self {
        self.transact_time = Some(transact_time);
        self
    }

    /// Set the text field
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Convert to RawFixMessage for encoding
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();
        msg.set_field(8, "FIXT.1.1".to_string());
        msg.set_field(35, "CQ".to_string());
        msg.set_field(1128, "9".to_string()); // ApplVerID = 9 (FIX 5.0 SP2)

        msg.set_field(1699, self.account_summary_report_id.clone());
        msg.set_field(715, self.clearing_business_date.clone());

        if let Some(currency) = &self.currency {
            msg.set_field(15, currency.clone());
        }

        if let Some(total_net_value) = self.total_net_value {
            msg.set_field(900, total_net_value.to_string());
        }

        if let Some(margin_excess) = self.margin_excess {
            msg.set_field(899, margin_excess.to_string());
        }

        if let Some(settl_sess_id) = &self.settl_sess_id {
            msg.set_field(716, settl_sess_id.clone());
        }

        if let Some(settl_sess_sub_id) = &self.settl_sess_sub_id {
            msg.set_field(717, settl_sess_sub_id.clone());
        }

        if let Some(transact_time) = self.transact_time {
            msg.set_field(
                60,
                transact_time.format("%Y%m%d-%H:%M:%S%.3f").to_string(),
            );
        }

        if let Some(text) = &self.text {
            msg.set_field(58, text.clone());
        }

        msg
    }

    /// Parse from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        let account_summary_report_id = raw
            .get_field(1699)
            .ok_or(FixParseError::MissingRequiredField(1699))?
            .to_string();

        let clearing_business_date = raw
            .get_field(715)
            .ok_or(FixParseError::MissingRequiredField(715))?
            .to_string();

        let currency = raw.get_field(15).map(|s| s.to_string());

        let total_net_value: Option<f64> = raw.get_field_as(900);
        let margin_excess: Option<f64> = raw.get_field_as(899);

        let settl_sess_id = raw.get_field(716).map(|s| s.to_string());
        let settl_sess_sub_id = raw.get_field(717).map(|s| s.to_string());

        let transact_time = if let Some(transact_time_str) = raw.get_field(60) {
            let naive_time =
                chrono::NaiveDateTime::parse_from_str(transact_time_str, "%Y%m%d-%H:%M:%S%.3f")
                    .map_err(|e| FixParseError::InvalidValue {
                        tag: 60,
                        value: transact_time_str.to_string(),
                        error: e.to_string(),
                    })?;
            Some(DateTime::<Utc>::from_naive_utc_and_offset(naive_time, Utc))
        } else {
            None
        };

        let text = raw.get_field(58).map(|s| s.to_string());

        Ok(AccountSummaryReport {
            account_summary_report_id,
            clearing_business_date,
            parties: None, // TODO: Parse parties from repeating groups
            currency,
            total_net_value,
            margin_excess,
            settl_sess_id,
            settl_sess_sub_id,
            transact_time,
            text,
        })
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_account_summary_report_new() {
        let report = AccountSummaryReport::new("ASR123", "20250114");

        assert_eq!(report.account_summary_report_id, "ASR123");
        assert_eq!(report.clearing_business_date, "20250114");
        assert_eq!(report.currency, None);
        assert_eq!(report.total_net_value, None);
    }

    #[test]
    fn test_account_summary_report_with_currency() {
        let report = AccountSummaryReport::new("ASR123", "20250114").with_currency("USD");

        assert_eq!(report.currency, Some("USD".to_string()));
    }

    #[test]
    fn test_account_summary_report_with_values() {
        let report = AccountSummaryReport::new("ASR123", "20250114")
            .with_currency("USD")
            .with_total_net_value(1000000.50)
            .with_margin_excess(50000.25);

        assert_eq!(report.currency, Some("USD".to_string()));
        assert_eq!(report.total_net_value, Some(1000000.50));
        assert_eq!(report.margin_excess, Some(50000.25));
    }

    #[test]
    fn test_account_summary_report_with_settlement_ids() {
        let report = AccountSummaryReport::new("ASR123", "20250114")
            .with_settl_sess_id("EOD")
            .with_settl_sess_sub_id("MAIN");

        assert_eq!(report.settl_sess_id, Some("EOD".to_string()));
        assert_eq!(report.settl_sess_sub_id, Some("MAIN".to_string()));
    }

    #[test]
    fn test_account_summary_report_with_transact_time() {
        let now = Utc::now();
        let report = AccountSummaryReport::new("ASR123", "20250114").with_transact_time(now);

        assert_eq!(report.transact_time, Some(now));
    }

    #[test]
    fn test_account_summary_report_with_text() {
        let report =
            AccountSummaryReport::new("ASR123", "20250114").with_text("Daily settlement report");

        assert_eq!(report.text, Some("Daily settlement report".to_string()));
    }

    #[test]
    fn test_account_summary_report_to_raw() {
        let report = AccountSummaryReport::new("ASR123", "20250114")
            .with_currency("USD")
            .with_total_net_value(1000000.50);

        let raw = report.to_raw();

        assert_eq!(raw.get_field(35), Some("CQ"));
        assert_eq!(raw.get_field(1699), Some("ASR123"));
        assert_eq!(raw.get_field(715), Some("20250114"));
        assert_eq!(raw.get_field(15), Some("USD"));
        assert_eq!(raw.get_field(900), Some("1000000.5"));
    }

    #[test]
    fn test_account_summary_report_from_raw() {
        let mut raw = RawFixMessage::new();
        raw.set_field(35, "CQ".to_string());
        raw.set_field(1699, "ASR123".to_string());
        raw.set_field(715, "20250114".to_string());
        raw.set_field(15, "USD".to_string());
        raw.set_field(900, "1000000.5".to_string());
        raw.set_field(899, "50000.25".to_string());

        let report = AccountSummaryReport::from_raw(&raw).unwrap();

        assert_eq!(report.account_summary_report_id, "ASR123");
        assert_eq!(report.clearing_business_date, "20250114");
        assert_eq!(report.currency, Some("USD".to_string()));
        assert_eq!(report.total_net_value, Some(1000000.5));
        assert_eq!(report.margin_excess, Some(50000.25));
    }

    #[test]
    fn test_account_summary_report_round_trip() {
        let original = AccountSummaryReport::new("ASR456", "20250114")
            .with_currency("EUR")
            .with_total_net_value(2500000.75)
            .with_margin_excess(125000.50)
            .with_settl_sess_id("EOD")
            .with_text("End of day report");

        let raw = original.to_raw();
        let parsed = AccountSummaryReport::from_raw(&raw).unwrap();

        assert_eq!(parsed.account_summary_report_id, "ASR456");
        assert_eq!(parsed.clearing_business_date, "20250114");
        assert_eq!(parsed.currency, Some("EUR".to_string()));
        assert_eq!(parsed.total_net_value, Some(2500000.75));
        assert_eq!(parsed.margin_excess, Some(125000.50));
        assert_eq!(parsed.settl_sess_id, Some("EOD".to_string()));
        assert_eq!(parsed.text, Some("End of day report".to_string()));
    }

    #[test]
    fn test_account_summary_report_missing_required_field() {
        let mut raw = RawFixMessage::new();
        raw.set_field(35, "CQ".to_string());
        raw.set_field(1699, "ASR123".to_string());
        // Missing ClearingBusinessDate (715)

        let result = AccountSummaryReport::from_raw(&raw);
        assert!(result.is_err());
        match result {
            Err(FixParseError::MissingRequiredField(tag)) => assert_eq!(tag, 715),
            _ => panic!("Expected MissingRequiredField error"),
        }
    }
}
