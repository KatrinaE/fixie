//! Multileg Order message types
//!
//! This module implements FIX 5.0 SP2 Multileg Order messages, which enable
//! trading of complex securities composed of multiple legs (contracts) that
//! execute atomically.
//!
//! ## Message Types
//! - NewOrderMultileg (AB): Submit new multileg orders (spreads, strategies)
//! - MultilegOrderCancelReplace (AC): Modify existing multileg orders
//!
//! ## Use Cases
//! - Options spreads (vertical, calendar, diagonal, butterfly, condor, etc.)
//! - Futures calendar spreads and inter-commodity spreads
//! - Swaps and other multi-component derivatives
//! - Complex combination orders

use crate::parser::{RawFixMessage, FixParseError};
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// NewOrderMultileg (MsgType = AB)
// ============================================================================

/// NewOrderMultileg (AB) - Submit a new order for a multileg security
///
/// This message enables trading of securities composed of multiple legs that
/// must execute atomically. Each leg has its own instrument specification
/// and can have different sides.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewOrderMultileg {
    /// ClOrdID (Tag 11) - Required - Client order ID
    pub cl_ord_id: String,

    /// Side (Tag 54) - Required - Overall side
    pub side: Side,

    /// TransactTime (Tag 60) - Required - Time of order creation
    pub transact_time: String,

    /// OrdType (Tag 40) - Required - Order type
    pub ord_type: OrdType,

    /// Symbol (Tag 55) - Optional - Strategy symbol/name
    pub symbol: Option<String>,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// OrderQty (Tag 38) - Optional - Overall quantity
    pub order_qty: Option<f64>,

    /// Price (Tag 44) - Optional - Net price across all legs
    pub price: Option<f64>,

    /// Currency (Tag 15) - Optional - Currency
    pub currency: Option<String>,

    /// TimeInForce (Tag 59) - Optional - Time in force
    pub time_in_force: Option<String>,

    /// ExpireDate (Tag 432) - Optional - Expiration date
    pub expire_date: Option<String>,

    /// ExpireTime (Tag 126) - Optional - Expiration time
    pub expire_time: Option<String>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// MultilegReportingType (Tag 442) - Optional - Reporting type
    pub multileg_reporting_type: Option<MultilegReportingType>,

    /// MultilegPriceMethod (Tag 1378) - Optional - Pricing method
    pub multileg_price_method: Option<MultilegPriceMethod>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354) - Optional
    pub encoded_text_len: Option<u32>,

    // Note: LegOrdGrp - Repeating group of legs (Tag 555 = NoLegs)
    // This is stored in the RawFixMessage.groups field with key 555
    // Each entry contains full leg specification with nested groups
}

impl NewOrderMultileg {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(NewOrderMultileg {
            cl_ord_id: raw.get_field(11)
                .ok_or_else(|| FixParseError::MissingRequiredField(11))?
                .to_string(),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(54))?,
            transact_time: raw.get_field(60)
                .ok_or_else(|| FixParseError::MissingRequiredField(60))?
                .to_string(),
            ord_type: raw.get_field(40)
                .and_then(|s| s.chars().next())
                .and_then(OrdType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(40))?,
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            order_qty: raw.get_field(38).and_then(|s| s.parse().ok()),
            price: raw.get_field(44).and_then(|s| s.parse().ok()),
            currency: raw.get_field(15).map(|s| s.to_string()),
            time_in_force: raw.get_field(59).map(|s| s.to_string()),
            expire_date: raw.get_field(432).map(|s| s.to_string()),
            expire_time: raw.get_field(126).map(|s| s.to_string()),
            account: raw.get_field(1).map(|s| s.to_string()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            multileg_reporting_type: raw.get_field(442)
                .and_then(|s| s.chars().next())
                .and_then(MultilegReportingType::from_fix),
            multileg_price_method: raw.get_field(1378)
                .and_then(|s| s.chars().next())
                .and_then(MultilegPriceMethod::from_fix),
            text: raw.get_field(58).map(|s| s.to_string()),
            encoded_text_len: raw.get_field(354).and_then(|s| s.parse().ok()),
        })
        // Note: LegOrdGrp (555) is accessed via raw.groups.get(&555)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AB".to_string());

        // Required fields
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(60, self.transact_time.clone());
        msg.set_field(40, self.ord_type.to_fix().to_string());

        // Optional fields
        if let Some(ref symbol) = self.symbol {
            msg.set_field(55, symbol.clone());
        }
        if let Some(ref security_id) = self.security_id {
            msg.set_field(48, security_id.clone());
        }
        if let Some(ref security_id_source) = self.security_id_source {
            msg.set_field(22, security_id_source.clone());
        }
        if let Some(order_qty) = self.order_qty {
            msg.set_field(38, order_qty.to_string());
        }
        if let Some(price) = self.price {
            msg.set_field(44, price.to_string());
        }
        if let Some(ref currency) = self.currency {
            msg.set_field(15, currency.clone());
        }
        if let Some(ref time_in_force) = self.time_in_force {
            msg.set_field(59, time_in_force.clone());
        }
        if let Some(ref expire_date) = self.expire_date {
            msg.set_field(432, expire_date.clone());
        }
        if let Some(ref expire_time) = self.expire_time {
            msg.set_field(126, expire_time.clone());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(multileg_reporting_type) = self.multileg_reporting_type {
            msg.set_field(442, multileg_reporting_type.to_fix().to_string());
        }
        if let Some(multileg_price_method) = self.multileg_price_method {
            msg.set_field(1378, multileg_price_method.to_fix().to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }
        if let Some(encoded_text_len) = self.encoded_text_len {
            msg.set_field(354, encoded_text_len.to_string());
        }

        // Note: LegOrdGrp (555) should be added via msg.groups
        msg
    }
}

// ============================================================================
// MultilegOrderCancelReplace (MsgType = AC)
// ============================================================================

/// MultilegOrderCancelReplace (AC) - Modify an existing multileg order
///
/// This message modifies parameters of an existing multileg order while
/// maintaining the atomic nature of the multileg security.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MultilegOrderCancelReplace {
    /// ClOrdID (Tag 11) - Required - New client order ID
    pub cl_ord_id: String,

    /// OrigClOrdID (Tag 41) - Required - Original order ID to replace
    pub orig_cl_ord_id: String,

    /// Side (Tag 54) - Required - Overall side
    pub side: Side,

    /// TransactTime (Tag 60) - Required - Time of modification
    pub transact_time: String,

    /// OrdType (Tag 40) - Required - Order type
    pub ord_type: OrdType,

    /// OrderID (Tag 37) - Optional - Order ID from exchange
    pub order_id: Option<String>,

    /// Symbol (Tag 55) - Optional - Strategy symbol/name
    pub symbol: Option<String>,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// OrderQty (Tag 38) - Optional - New quantity
    pub order_qty: Option<f64>,

    /// Price (Tag 44) - Optional - New net price
    pub price: Option<f64>,

    /// Currency (Tag 15) - Optional - Currency
    pub currency: Option<String>,

    /// TimeInForce (Tag 59) - Optional - Time in force
    pub time_in_force: Option<String>,

    /// ExpireDate (Tag 432) - Optional - Expiration date
    pub expire_date: Option<String>,

    /// ExpireTime (Tag 126) - Optional - Expiration time
    pub expire_time: Option<String>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// MultilegReportingType (Tag 442) - Optional - Reporting type
    pub multileg_reporting_type: Option<MultilegReportingType>,

    /// MultilegPriceMethod (Tag 1378) - Optional - Pricing method
    pub multileg_price_method: Option<MultilegPriceMethod>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    /// EncodedTextLen (Tag 354) - Optional
    pub encoded_text_len: Option<u32>,

    // Note: LegOrdGrp - Repeating group of legs (Tag 555 = NoLegs)
    // This is stored in the RawFixMessage.groups field with key 555
}

impl MultilegOrderCancelReplace {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(MultilegOrderCancelReplace {
            cl_ord_id: raw.get_field(11)
                .ok_or_else(|| FixParseError::MissingRequiredField(11))?
                .to_string(),
            orig_cl_ord_id: raw.get_field(41)
                .ok_or_else(|| FixParseError::MissingRequiredField(41))?
                .to_string(),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(54))?,
            transact_time: raw.get_field(60)
                .ok_or_else(|| FixParseError::MissingRequiredField(60))?
                .to_string(),
            ord_type: raw.get_field(40)
                .and_then(|s| s.chars().next())
                .and_then(OrdType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(40))?,
            order_id: raw.get_field(37).map(|s| s.to_string()),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            order_qty: raw.get_field(38).and_then(|s| s.parse().ok()),
            price: raw.get_field(44).and_then(|s| s.parse().ok()),
            currency: raw.get_field(15).map(|s| s.to_string()),
            time_in_force: raw.get_field(59).map(|s| s.to_string()),
            expire_date: raw.get_field(432).map(|s| s.to_string()),
            expire_time: raw.get_field(126).map(|s| s.to_string()),
            account: raw.get_field(1).map(|s| s.to_string()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            multileg_reporting_type: raw.get_field(442)
                .and_then(|s| s.chars().next())
                .and_then(MultilegReportingType::from_fix),
            multileg_price_method: raw.get_field(1378)
                .and_then(|s| s.chars().next())
                .and_then(MultilegPriceMethod::from_fix),
            text: raw.get_field(58).map(|s| s.to_string()),
            encoded_text_len: raw.get_field(354).and_then(|s| s.parse().ok()),
        })
        // Note: LegOrdGrp (555) is accessed via raw.groups.get(&555)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AC".to_string());

        // Required fields
        msg.set_field(11, self.cl_ord_id.clone());
        msg.set_field(41, self.orig_cl_ord_id.clone());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(60, self.transact_time.clone());
        msg.set_field(40, self.ord_type.to_fix().to_string());

        // Optional fields
        if let Some(ref order_id) = self.order_id {
            msg.set_field(37, order_id.clone());
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
        if let Some(order_qty) = self.order_qty {
            msg.set_field(38, order_qty.to_string());
        }
        if let Some(price) = self.price {
            msg.set_field(44, price.to_string());
        }
        if let Some(ref currency) = self.currency {
            msg.set_field(15, currency.clone());
        }
        if let Some(ref time_in_force) = self.time_in_force {
            msg.set_field(59, time_in_force.clone());
        }
        if let Some(ref expire_date) = self.expire_date {
            msg.set_field(432, expire_date.clone());
        }
        if let Some(ref expire_time) = self.expire_time {
            msg.set_field(126, expire_time.clone());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(multileg_reporting_type) = self.multileg_reporting_type {
            msg.set_field(442, multileg_reporting_type.to_fix().to_string());
        }
        if let Some(multileg_price_method) = self.multileg_price_method {
            msg.set_field(1378, multileg_price_method.to_fix().to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }
        if let Some(encoded_text_len) = self.encoded_text_len {
            msg.set_field(354, encoded_text_len.to_string());
        }

        // Note: LegOrdGrp (555) should be added via msg.groups
        msg
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_order_multileg_round_trip() {
        let order = NewOrderMultileg {
            cl_ord_id: "ML001".to_string(),
            side: Side::Buy,
            transact_time: "20251008-12:00:00.000".to_string(),
            ord_type: OrdType::Limit,
            symbol: Some("AAPL_SPREAD".to_string()),
            security_id: None,
            security_id_source: None,
            order_qty: Some(10.0),
            price: Some(2.50),
            currency: Some("USD".to_string()),
            time_in_force: Some("0".to_string()),
            expire_date: None,
            expire_time: None,
            account: Some("ACCT001".to_string()),
            account_type: None,
            multileg_reporting_type: Some(MultilegReportingType::MultilegSecurity),
            multileg_price_method: Some(MultilegPriceMethod::NetPrice),
            text: Some("Vertical call spread".to_string()),
            encoded_text_len: None,
        };

        let raw = order.to_raw();
        let parsed = NewOrderMultileg::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed, order);
    }

    #[test]
    fn test_multileg_order_cancel_replace_round_trip() {
        let order = MultilegOrderCancelReplace {
            cl_ord_id: "ML002".to_string(),
            orig_cl_ord_id: "ML001".to_string(),
            side: Side::Buy,
            transact_time: "20251008-13:00:00.000".to_string(),
            ord_type: OrdType::Limit,
            order_id: Some("ORD123".to_string()),
            symbol: Some("AAPL_SPREAD".to_string()),
            security_id: None,
            security_id_source: None,
            order_qty: Some(20.0),
            price: Some(2.75),
            currency: Some("USD".to_string()),
            time_in_force: Some("0".to_string()),
            expire_date: None,
            expire_time: None,
            account: Some("ACCT001".to_string()),
            account_type: None,
            multileg_reporting_type: Some(MultilegReportingType::MultilegSecurity),
            multileg_price_method: Some(MultilegPriceMethod::NetPrice),
            text: Some("Modify quantity and price".to_string()),
            encoded_text_len: None,
        };

        let raw = order.to_raw();
        let parsed = MultilegOrderCancelReplace::from_raw(&raw).expect("Failed to parse");

        assert_eq!(parsed, order);
    }
}
