//! Component blocks reused across FIX message types
//!
//! Component blocks are standardized groups of related fields that appear
//! in multiple message types. This module implements the components used
//! in Program Trading / List Trading messages.

use serde::{Deserialize, Serialize};

// ============================================================================
// Parties Component
// ============================================================================

/// Parties component - identifies the parties involved in a message
///
/// Used in many message types to specify counterparties, clearing firms,
/// executing brokers, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parties {
    /// Party ID (Tag 448)
    pub party_id: Option<String>,

    /// Party ID Source (Tag 447)
    /// Values: B = BIC, C = Generally accepted market participant identifier,
    /// D = Proprietary / Custom code, E = ISO Country Code, etc.
    pub party_id_source: Option<char>,

    /// Party Role (Tag 452)
    /// Values: 1 = Executing Firm, 2 = Broker of Credit, 3 = Client ID, etc.
    pub party_role: Option<u32>,
    // Note: Party Sub-IDs are a nested group handled separately in GroupEntry.nested_groups
}

// ============================================================================
// Instrument Component
// ============================================================================

/// Instrument component - identifies the financial instrument being traded
///
/// This is a comprehensive set of fields used across order and execution messages
/// to uniquely identify securities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Instrument {
    /// Symbol (Tag 55)
    pub symbol: Option<String>,

    /// Symbol Suffix (Tag 65)
    pub symbol_sfx: Option<String>,

    /// Security ID (Tag 48)
    pub security_id: Option<String>,

    /// Security ID Source (Tag 22)
    /// Values: 1 = CUSIP, 2 = SEDOL, 4 = ISIN, 8 = Exchange Symbol, etc.
    pub security_id_source: Option<String>,

    /// Security Type (Tag 167)
    /// Values: CS = Common Stock, PS = Preferred Stock, FUT = Future, OPT = Option, etc.
    pub security_type: Option<String>,

    /// Maturity Month Year (Tag 200)
    /// Format: YYYYMM or YYYYMMDD or YYYYMMWW
    pub maturity_month_year: Option<String>,

    /// Maturity Date (Tag 541)
    /// Format: YYYYMMDD
    pub maturity_date: Option<String>,

    /// Coupon Payment Date (Tag 224)
    pub coupon_payment_date: Option<String>,

    /// Issue Date (Tag 225)
    pub issue_date: Option<String>,

    /// Repo Collateral Security Type (Tag 239)
    pub repo_collateral_security_type: Option<String>,

    /// Repurchase Term (Tag 226)
    pub repurchase_term: Option<u32>,

    /// Repurchase Rate (Tag 227)
    pub repurchase_rate: Option<f64>,

    /// Factor (Tag 228)
    pub factor: Option<f64>,

    /// Credit Rating (Tag 255)
    pub credit_rating: Option<String>,

    /// Instrument Registry (Tag 543)
    pub instr_registry: Option<String>,

    /// Country of Issue (Tag 470)
    /// ISO 3166 two-character country code
    pub country_of_issue: Option<String>,

    /// State or Province of Issue (Tag 471)
    pub state_or_province_of_issue: Option<String>,

    /// Locale of Issue (Tag 472)
    pub locale_of_issue: Option<String>,

    /// Redemption Date (Tag 240)
    pub redemption_date: Option<String>,

    /// Strike Price (Tag 202)
    pub strike_price: Option<f64>,

    /// Strike Currency (Tag 947)
    pub strike_currency: Option<String>,

    /// Option Attribute (Tag 206)
    pub opt_attribute: Option<char>,

    /// Contract Multiplier (Tag 231)
    pub contract_multiplier: Option<f64>,

    /// Coupon Rate (Tag 223)
    pub coupon_rate: Option<f64>,

    /// Security Exchange (Tag 207)
    /// Market Identifier Code (MIC)
    pub security_exchange: Option<String>,

    /// Issuer (Tag 106)
    pub issuer: Option<String>,

    /// Security Description (Tag 107)
    pub security_desc: Option<String>,

    /// Pool (Tag 691)
    pub pool: Option<String>,
}

// ============================================================================
// OrderQtyData Component
// ============================================================================

/// OrderQtyData component - specifies order quantity in various formats
///
/// Allows quantity to be expressed as shares, cash value, or percentage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrderQtyData {
    /// Order Quantity (Tag 38)
    /// Number of shares/contracts
    pub order_qty: Option<f64>,

    /// Cash Order Quantity (Tag 152)
    /// Order quantity expressed in monetary units
    pub cash_order_qty: Option<f64>,

    /// Order Percent (Tag 516)
    /// Order quantity as percentage of total
    pub order_percent: Option<f64>,

    /// Rounding Direction (Tag 468)
    /// Values: 0 = Round to nearest, 1 = Round down, 2 = Round up
    pub rounding_direction: Option<char>,

    /// Rounding Modulus (Tag 469)
    /// Specifies the rounding unit
    pub rounding_modulus: Option<f64>,
}

// ============================================================================
// CommissionData Component
// ============================================================================

/// CommissionData component - specifies commission details
///
/// Used to communicate commission charges on executions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommissionData {
    /// Commission (Tag 12)
    /// Commission amount
    pub commission: Option<f64>,

    /// Commission Type (Tag 13)
    /// Values: 1 = Per Unit, 2 = Percent, 3 = Absolute, etc.
    pub comm_type: Option<char>,

    /// Commission Currency (Tag 479)
    pub comm_currency: Option<String>,

    /// Fund Renew Waiv (Tag 497)
    /// Values: Y = Broker to receive waiver, N = Broker not to receive waiver
    pub fund_renew_waiv: Option<char>,
}

// ============================================================================
// Helper Methods
// ============================================================================

impl Parties {
    /// Create a new empty Parties component
    pub fn new() -> Self {
        Self {
            party_id: None,
            party_id_source: None,
            party_role: None,
        }
    }
}

impl Default for Parties {
    fn default() -> Self {
        Self::new()
    }
}

impl Instrument {
    /// Create a new empty Instrument component
    pub fn new() -> Self {
        Self {
            symbol: None,
            symbol_sfx: None,
            security_id: None,
            security_id_source: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            coupon_payment_date: None,
            issue_date: None,
            repo_collateral_security_type: None,
            repurchase_term: None,
            repurchase_rate: None,
            factor: None,
            credit_rating: None,
            instr_registry: None,
            country_of_issue: None,
            state_or_province_of_issue: None,
            locale_of_issue: None,
            redemption_date: None,
            strike_price: None,
            strike_currency: None,
            opt_attribute: None,
            contract_multiplier: None,
            coupon_rate: None,
            security_exchange: None,
            issuer: None,
            security_desc: None,
            pool: None,
        }
    }

    /// Create from just a symbol (most common case)
    pub fn from_symbol(symbol: impl Into<String>) -> Self {
        Self {
            symbol: Some(symbol.into()),
            ..Self::new()
        }
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderQtyData {
    /// Create a new empty OrderQtyData component
    pub fn new() -> Self {
        Self {
            order_qty: None,
            cash_order_qty: None,
            order_percent: None,
            rounding_direction: None,
            rounding_modulus: None,
        }
    }

    /// Create from order quantity in shares
    pub fn from_qty(qty: f64) -> Self {
        Self {
            order_qty: Some(qty),
            ..Self::new()
        }
    }

    /// Create from cash order quantity
    pub fn from_cash_qty(cash_qty: f64) -> Self {
        Self {
            cash_order_qty: Some(cash_qty),
            ..Self::new()
        }
    }
}

impl Default for OrderQtyData {
    fn default() -> Self {
        Self::new()
    }
}

impl CommissionData {
    /// Create a new empty CommissionData component
    pub fn new() -> Self {
        Self {
            commission: None,
            comm_type: None,
            comm_currency: None,
            fund_renew_waiv: None,
        }
    }

    /// Create with commission amount and type
    pub fn new_with_amount(amount: f64, comm_type: char) -> Self {
        Self {
            commission: Some(amount),
            comm_type: Some(comm_type),
            comm_currency: None,
            fund_renew_waiv: None,
        }
    }
}

impl Default for CommissionData {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parties_new() {
        let parties = Parties::new();
        assert_eq!(parties.party_id, None);
        assert_eq!(parties.party_id_source, None);
        assert_eq!(parties.party_role, None);
    }

    #[test]
    fn test_parties_with_data() {
        let parties = Parties {
            party_id: Some("TRADER1".to_string()),
            party_id_source: Some('D'),
            party_role: Some(1),
        };
        assert_eq!(parties.party_id, Some("TRADER1".to_string()));
        assert_eq!(parties.party_id_source, Some('D'));
        assert_eq!(parties.party_role, Some(1));
    }

    #[test]
    fn test_instrument_new() {
        let inst = Instrument::new();
        assert_eq!(inst.symbol, None);
        assert_eq!(inst.security_id, None);
    }

    #[test]
    fn test_instrument_from_symbol() {
        let inst = Instrument::from_symbol("MSFT");
        assert_eq!(inst.symbol, Some("MSFT".to_string()));
        assert_eq!(inst.security_id, None);
    }

    #[test]
    fn test_instrument_with_full_data() {
        let inst = Instrument {
            symbol: Some("AAPL".to_string()),
            security_id: Some("037833100".to_string()),
            security_id_source: Some("1".to_string()), // CUSIP
            security_type: Some("CS".to_string()), // Common Stock
            security_exchange: Some("XNAS".to_string()), // NASDAQ
            ..Instrument::new()
        };
        assert_eq!(inst.symbol, Some("AAPL".to_string()));
        assert_eq!(inst.security_id, Some("037833100".to_string()));
    }

    #[test]
    fn test_order_qty_data_new() {
        let qty = OrderQtyData::new();
        assert_eq!(qty.order_qty, None);
        assert_eq!(qty.cash_order_qty, None);
    }

    #[test]
    fn test_order_qty_data_from_qty() {
        let qty = OrderQtyData::from_qty(100.0);
        assert_eq!(qty.order_qty, Some(100.0));
        assert_eq!(qty.cash_order_qty, None);
    }

    #[test]
    fn test_order_qty_data_from_cash_qty() {
        let qty = OrderQtyData::from_cash_qty(10000.0);
        assert_eq!(qty.cash_order_qty, Some(10000.0));
        assert_eq!(qty.order_qty, None);
    }

    #[test]
    fn test_order_qty_data_with_rounding() {
        let qty = OrderQtyData {
            order_qty: Some(100.0),
            rounding_direction: Some('0'), // Round to nearest
            rounding_modulus: Some(10.0),
            ..OrderQtyData::new()
        };
        assert_eq!(qty.order_qty, Some(100.0));
        assert_eq!(qty.rounding_direction, Some('0'));
        assert_eq!(qty.rounding_modulus, Some(10.0));
    }

    #[test]
    fn test_commission_data_new() {
        let comm = CommissionData::new();
        assert_eq!(comm.commission, None);
        assert_eq!(comm.comm_type, None);
    }

    #[test]
    fn test_commission_data_with_amount() {
        let comm = CommissionData::new_with_amount(5.0, '1');
        assert_eq!(comm.commission, Some(5.0));
        assert_eq!(comm.comm_type, Some('1'));
        assert_eq!(comm.comm_currency, None);
    }

    #[test]
    fn test_commission_data_full() {
        let comm = CommissionData {
            commission: Some(10.50),
            comm_type: Some('2'), // Percent
            comm_currency: Some("USD".to_string()),
            fund_renew_waiv: Some('N'),
        };
        assert_eq!(comm.commission, Some(10.50));
        assert_eq!(comm.comm_type, Some('2'));
        assert_eq!(comm.comm_currency, Some("USD".to_string()));
        assert_eq!(comm.fund_renew_waiv, Some('N'));
    }
}
