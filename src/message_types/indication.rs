//! Pre-Trade Indication Messages
//!
//! This module implements FIX 5.0 SP2 Pre-Trade Indication messages, which allow
//! market participants to advertise completed transactions and indicate trading interest
//! before formal quote requests.
//!
//! ## Message Types
//! - IOI (6): Indication of Interest - Market merchandise which the broker is buying or selling
//! - Advertisement (7): Announce completed transactions to the market
//! - CrossRequest (DS): Indicate submission of orders that may result in a crossed trade
//! - CrossRequestAck (DT): Confirm receipt of a CrossRequest

use crate::parser::{RawFixMessage, FixParseError};
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// IOI (MsgType = 6)
// ============================================================================

/// IOI (6) - Indication of Interest
///
/// Market merchandise which the broker is buying or selling. IOIs allow brokers
/// to advertise inventory or buying interest to clients before formal quotes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IOI {
    /// IOIID (Tag 23) - Required - Unique identifier for this IOI
    pub ioi_id: String,

    /// IOITransType (Tag 28) - Required - Type of IOI transaction (New/Cancel/Replace)
    pub ioi_trans_type: IOITransType,

    /// Side (Tag 54) - Required - Side of the IOI (Buy/Sell)
    pub side: Side,

    /// IOIQty (Tag 27) - Required - Quantity (e.g., "S" for small, "M" for medium, "L" for large, or numeric)
    pub ioi_qty: String,

    // Symbol/Instrument fields
    /// Symbol (Tag 55)
    pub symbol: Option<String>,

    /// SecurityID (Tag 48)
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22)
    pub security_id_source: Option<String>,

    // Optional fields
    /// IOIRefID (Tag 26) - Reference ID for Cancel/Replace
    pub ioi_ref_id: Option<String>,

    /// Currency (Tag 15)
    pub currency: Option<String>,

    /// Price (Tag 44)
    pub price: Option<f64>,

    /// ValidUntilTime (Tag 62)
    pub valid_until_time: Option<String>,

    /// IOIQltyInd (Tag 25) - Quality indicator
    pub ioi_qlty_ind: Option<IOIQltyInd>,

    /// Text (Tag 58)
    pub text: Option<String>,

    /// TransactTime (Tag 60)
    pub transact_time: Option<String>,

    /// URLLink (Tag 149)
    pub url_link: Option<String>,

    // Note: Repeating groups accessed via RawFixMessage.groups:
    // - IOIQualGrp (Tag 199 = NoIOIQualifiers)
    // - RoutingGrp (Tag 215 = NoRoutingIDs)
    // - InstrmtLegIOIGrp (Tag 555 = NoLegs)
    // - UndInstrmtGrp (Tag 711 = NoUnderlyings)
}

impl IOI {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(IOI {
            ioi_id: raw.get_field(23)
                .ok_or_else(|| FixParseError::MissingRequiredField(23))?
                .to_string(),
            ioi_trans_type: raw.get_field(28)
                .and_then(|s| s.chars().next())
                .and_then(IOITransType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(28))?,
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(54))?,
            ioi_qty: raw.get_field(27)
                .ok_or_else(|| FixParseError::MissingRequiredField(27))?
                .to_string(),
            symbol: raw.get_field(55).map(String::from),
            security_id: raw.get_field(48).map(String::from),
            security_id_source: raw.get_field(22).map(String::from),
            ioi_ref_id: raw.get_field(26).map(String::from),
            currency: raw.get_field(15).map(String::from),
            price: raw.get_field(44).and_then(|v| v.parse().ok()),
            valid_until_time: raw.get_field(62).map(String::from),
            ioi_qlty_ind: raw.get_field(25)
                .and_then(|s| s.chars().next())
                .and_then(IOIQltyInd::from_fix),
            text: raw.get_field(58).map(String::from),
            transact_time: raw.get_field(60).map(String::from),
            url_link: raw.get_field(149).map(String::from),
        })
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "6".to_string());

        // Required fields
        msg.set_field(23, self.ioi_id.clone());
        msg.set_field(28, self.ioi_trans_type.to_fix().to_string());
        msg.set_field(54, self.side.to_fix().to_string());
        msg.set_field(27, self.ioi_qty.clone());

        // Optional fields
        if let Some(ref v) = self.symbol {
            msg.set_field(55, v.clone());
        }
        if let Some(ref v) = self.security_id {
            msg.set_field(48, v.clone());
        }
        if let Some(ref v) = self.security_id_source {
            msg.set_field(22, v.clone());
        }
        if let Some(ref v) = self.ioi_ref_id {
            msg.set_field(26, v.clone());
        }
        if let Some(ref v) = self.currency {
            msg.set_field(15, v.clone());
        }
        if let Some(v) = self.price {
            msg.set_field(44, v.to_string());
        }
        if let Some(ref v) = self.valid_until_time {
            msg.set_field(62, v.clone());
        }
        if let Some(v) = self.ioi_qlty_ind {
            msg.set_field(25, v.to_fix().to_string());
        }
        if let Some(ref v) = self.text {
            msg.set_field(58, v.clone());
        }
        if let Some(ref v) = self.transact_time {
            msg.set_field(60, v.clone());
        }
        if let Some(ref v) = self.url_link {
            msg.set_field(149, v.clone());
        }

        // Note: Repeating groups should be added via msg.groups
        msg
    }
}

// ============================================================================
// Advertisement (MsgType = 7)
// ============================================================================

/// Advertisement (7) - Announce completed transactions
///
/// Used to advertise completed transactions to the market after execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Advertisement {
    /// AdvId (Tag 2) - Required - Unique identifier for this advertisement
    pub adv_id: String,

    /// AdvTransType (Tag 5) - Required - Type of advertisement transaction (New/Cancel/Replace)
    pub adv_trans_type: AdvTransType,

    /// AdvSide (Tag 4) - Required - Side of the advertisement (Buy/Sell/Cross/Trade)
    pub adv_side: AdvSide,

    /// Quantity (Tag 53) - Required - Quantity of the trade
    pub quantity: f64,

    // Symbol/Instrument fields
    /// Symbol (Tag 55)
    pub symbol: Option<String>,

    /// SecurityID (Tag 48)
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22)
    pub security_id_source: Option<String>,

    // Optional fields
    /// AdvRefID (Tag 3) - Reference ID for Cancel/Replace
    pub adv_ref_id: Option<String>,

    /// QtyType (Tag 854) - Type of quantity
    pub qty_type: Option<QtyType>,

    /// Price (Tag 44)
    pub price: Option<f64>,

    /// Currency (Tag 15)
    pub currency: Option<String>,

    /// TradeDate (Tag 75)
    pub trade_date: Option<String>,

    /// TransactTime (Tag 60)
    pub transact_time: Option<String>,

    /// Text (Tag 58)
    pub text: Option<String>,

    /// URLLink (Tag 149)
    pub url_link: Option<String>,

    /// LastMkt (Tag 30)
    pub last_mkt: Option<String>,

    /// TradingSessionID (Tag 336)
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (Tag 625)
    pub trading_session_sub_id: Option<String>,

    // Note: Repeating groups accessed via RawFixMessage.groups:
    // - InstrmtLegGrp (Tag 555 = NoLegs)
    // - UndInstrmtGrp (Tag 711 = NoUnderlyings)
}

impl Advertisement {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(Advertisement {
            adv_id: raw.get_field(2)
                .ok_or_else(|| FixParseError::MissingRequiredField(2))?
                .to_string(),
            adv_trans_type: raw.get_field(5)
                .and_then(|s| s.chars().next())
                .and_then(AdvTransType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(5))?,
            adv_side: raw.get_field(4)
                .and_then(|s| s.chars().next())
                .and_then(AdvSide::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(4))?,
            quantity: raw.get_field(53)
                .and_then(|v| v.parse().ok())
                .ok_or_else(|| FixParseError::MissingRequiredField(53))?,
            symbol: raw.get_field(55).map(String::from),
            security_id: raw.get_field(48).map(String::from),
            security_id_source: raw.get_field(22).map(String::from),
            adv_ref_id: raw.get_field(3).map(String::from),
            qty_type: raw.get_field(854)
                .and_then(|s| s.chars().next())
                .and_then(QtyType::from_fix),
            price: raw.get_field(44).and_then(|v| v.parse().ok()),
            currency: raw.get_field(15).map(String::from),
            trade_date: raw.get_field(75).map(String::from),
            transact_time: raw.get_field(60).map(String::from),
            text: raw.get_field(58).map(String::from),
            url_link: raw.get_field(149).map(String::from),
            last_mkt: raw.get_field(30).map(String::from),
            trading_session_id: raw.get_field(336).map(String::from),
            trading_session_sub_id: raw.get_field(625).map(String::from),
        })
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "7".to_string());

        // Required fields
        msg.set_field(2, self.adv_id.clone());
        msg.set_field(5, self.adv_trans_type.to_fix().to_string());
        msg.set_field(4, self.adv_side.to_fix().to_string());
        msg.set_field(53, self.quantity.to_string());

        // Optional fields
        if let Some(ref v) = self.symbol {
            msg.set_field(55, v.clone());
        }
        if let Some(ref v) = self.security_id {
            msg.set_field(48, v.clone());
        }
        if let Some(ref v) = self.security_id_source {
            msg.set_field(22, v.clone());
        }
        if let Some(ref v) = self.adv_ref_id {
            msg.set_field(3, v.clone());
        }
        if let Some(v) = self.qty_type {
            msg.set_field(854, v.to_fix().to_string());
        }
        if let Some(v) = self.price {
            msg.set_field(44, v.to_string());
        }
        if let Some(ref v) = self.currency {
            msg.set_field(15, v.clone());
        }
        if let Some(ref v) = self.trade_date {
            msg.set_field(75, v.clone());
        }
        if let Some(ref v) = self.transact_time {
            msg.set_field(60, v.clone());
        }
        if let Some(ref v) = self.text {
            msg.set_field(58, v.clone());
        }
        if let Some(ref v) = self.url_link {
            msg.set_field(149, v.clone());
        }
        if let Some(ref v) = self.last_mkt {
            msg.set_field(30, v.clone());
        }
        if let Some(ref v) = self.trading_session_id {
            msg.set_field(336, v.clone());
        }
        if let Some(ref v) = self.trading_session_sub_id {
            msg.set_field(625, v.clone());
        }

        msg
    }
}

// ============================================================================
// CrossRequest (MsgType = DS)
// ============================================================================

/// CrossRequest (DS) - Indicate submission of orders that may result in a crossed trade
///
/// Used to indicate the submission of orders that may result in a cross between
/// two parties, often required for regulatory compliance before crossing orders.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CrossRequest {
    /// CrossRequestID (Tag 2672) - Required - Unique identifier for this cross request
    pub cross_request_id: String,

    // Symbol/Instrument fields
    /// Symbol (Tag 55)
    pub symbol: Option<String>,

    /// SecurityID (Tag 48)
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22)
    pub security_id_source: Option<String>,

    // Optional fields
    /// MarketID (Tag 1301)
    pub market_id: Option<String>,

    /// MarketSegmentID (Tag 1300)
    pub market_segment_id: Option<String>,

    /// OrderQty (Tag 38)
    pub order_qty: Option<f64>,

    /// ComplianceID (Tag 376)
    pub compliance_id: Option<String>,

    /// ComplianceText (Tag 2404)
    pub compliance_text: Option<String>,

    // Note: Repeating groups accessed via RawFixMessage.groups:
    // - NoHops, NoSecurityAltID, NoInstrumentParties
}

impl CrossRequest {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(CrossRequest {
            cross_request_id: raw.get_field(2672)
                .ok_or_else(|| FixParseError::MissingRequiredField(2672))?
                .to_string(),
            symbol: raw.get_field(55).map(String::from),
            security_id: raw.get_field(48).map(String::from),
            security_id_source: raw.get_field(22).map(String::from),
            market_id: raw.get_field(1301).map(String::from),
            market_segment_id: raw.get_field(1300).map(String::from),
            order_qty: raw.get_field(38).and_then(|v| v.parse().ok()),
            compliance_id: raw.get_field(376).map(String::from),
            compliance_text: raw.get_field(2404).map(String::from),
        })
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "DS".to_string());

        // Required fields
        msg.set_field(2672, self.cross_request_id.clone());

        // Optional fields
        if let Some(ref v) = self.symbol {
            msg.set_field(55, v.clone());
        }
        if let Some(ref v) = self.security_id {
            msg.set_field(48, v.clone());
        }
        if let Some(ref v) = self.security_id_source {
            msg.set_field(22, v.clone());
        }
        if let Some(ref v) = self.market_id {
            msg.set_field(1301, v.clone());
        }
        if let Some(ref v) = self.market_segment_id {
            msg.set_field(1300, v.clone());
        }
        if let Some(v) = self.order_qty {
            msg.set_field(38, v.to_string());
        }
        if let Some(ref v) = self.compliance_id {
            msg.set_field(376, v.clone());
        }
        if let Some(ref v) = self.compliance_text {
            msg.set_field(2404, v.clone());
        }

        msg
    }
}

// ============================================================================
// CrossRequestAck (MsgType = DT)
// ============================================================================

/// CrossRequestAck (DT) - Confirm receipt of a CrossRequest
///
/// Acknowledgment message sent in response to a CrossRequest to confirm receipt
/// and processing status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CrossRequestAck {
    /// CrossRequestID (Tag 2672) - Required - Identifier from the original CrossRequest
    pub cross_request_id: String,

    // Symbol/Instrument fields
    /// Symbol (Tag 55)
    pub symbol: Option<String>,

    /// SecurityID (Tag 48)
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22)
    pub security_id_source: Option<String>,

    // Optional fields
    /// MarketID (Tag 1301)
    pub market_id: Option<String>,

    /// MarketSegmentID (Tag 1300)
    pub market_segment_id: Option<String>,

    // Note: Repeating groups accessed via RawFixMessage.groups:
    // - NoHops, NoSecurityAltID, NoInstrumentParties
}

impl CrossRequestAck {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(CrossRequestAck {
            cross_request_id: raw.get_field(2672)
                .ok_or_else(|| FixParseError::MissingRequiredField(2672))?
                .to_string(),
            symbol: raw.get_field(55).map(String::from),
            security_id: raw.get_field(48).map(String::from),
            security_id_source: raw.get_field(22).map(String::from),
            market_id: raw.get_field(1301).map(String::from),
            market_segment_id: raw.get_field(1300).map(String::from),
        })
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "DT".to_string());

        // Required fields
        msg.set_field(2672, self.cross_request_id.clone());

        // Optional fields
        if let Some(ref v) = self.symbol {
            msg.set_field(55, v.clone());
        }
        if let Some(ref v) = self.security_id {
            msg.set_field(48, v.clone());
        }
        if let Some(ref v) = self.security_id_source {
            msg.set_field(22, v.clone());
        }
        if let Some(ref v) = self.market_id {
            msg.set_field(1301, v.clone());
        }
        if let Some(ref v) = self.market_segment_id {
            msg.set_field(1300, v.clone());
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

    #[test]
    fn test_ioi_round_trip() {
        let msg = IOI {
            ioi_id: "IOI123".to_string(),
            ioi_trans_type: IOITransType::New,
            side: Side::Buy,
            ioi_qty: "L".to_string(),
            symbol: Some("AAPL".to_string()),
            security_id: Some("037833100".to_string()),
            security_id_source: Some("1".to_string()),
            ioi_ref_id: None,
            currency: Some("USD".to_string()),
            price: Some(150.25),
            valid_until_time: Some("20250108-17:00:00".to_string()),
            ioi_qlty_ind: Some(IOIQltyInd::High),
            text: Some("Interested in buying".to_string()),
            transact_time: Some("20250108-12:00:00".to_string()),
            url_link: None,
        };

        let raw = msg.to_raw();
        let parsed = IOI::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.ioi_id, "IOI123");
        assert_eq!(parsed.ioi_trans_type, IOITransType::New);
        assert_eq!(parsed.side, Side::Buy);
        assert_eq!(parsed.ioi_qty, "L");
        assert_eq!(parsed.symbol, Some("AAPL".to_string()));
        assert_eq!(parsed.price, Some(150.25));
    }

    #[test]
    fn test_advertisement_round_trip() {
        let msg = Advertisement {
            adv_id: "ADV456".to_string(),
            adv_trans_type: AdvTransType::New,
            adv_side: AdvSide::Sell,
            quantity: 1000.0,
            symbol: Some("MSFT".to_string()),
            security_id: None,
            security_id_source: None,
            adv_ref_id: None,
            qty_type: Some(QtyType::Units),
            price: Some(350.75),
            currency: Some("USD".to_string()),
            trade_date: Some("20250108".to_string()),
            transact_time: Some("20250108-14:30:00".to_string()),
            text: Some("Block trade completed".to_string()),
            url_link: None,
            last_mkt: Some("NASDAQ".to_string()),
            trading_session_id: None,
            trading_session_sub_id: None,
        };

        let raw = msg.to_raw();
        let parsed = Advertisement::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.adv_id, "ADV456");
        assert_eq!(parsed.adv_trans_type, AdvTransType::New);
        assert_eq!(parsed.adv_side, AdvSide::Sell);
        assert_eq!(parsed.quantity, 1000.0);
        assert_eq!(parsed.symbol, Some("MSFT".to_string()));
    }

    #[test]
    fn test_cross_request_round_trip() {
        let msg = CrossRequest {
            cross_request_id: "CROSS789".to_string(),
            symbol: Some("GOOGL".to_string()),
            security_id: Some("02079K305".to_string()),
            security_id_source: Some("1".to_string()),
            market_id: Some("XNAS".to_string()),
            market_segment_id: Some("MAIN".to_string()),
            order_qty: Some(500.0),
            compliance_id: Some("COMP123".to_string()),
            compliance_text: Some("Pre-cross notification".to_string()),
        };

        let raw = msg.to_raw();
        let parsed = CrossRequest::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.cross_request_id, "CROSS789");
        assert_eq!(parsed.symbol, Some("GOOGL".to_string()));
        assert_eq!(parsed.order_qty, Some(500.0));
    }

    #[test]
    fn test_cross_request_ack_round_trip() {
        let msg = CrossRequestAck {
            cross_request_id: "CROSS789".to_string(),
            symbol: Some("GOOGL".to_string()),
            security_id: Some("02079K305".to_string()),
            security_id_source: Some("1".to_string()),
            market_id: Some("XNAS".to_string()),
            market_segment_id: Some("MAIN".to_string()),
        };

        let raw = msg.to_raw();
        let parsed = CrossRequestAck::from_raw(&raw).expect("Should parse");

        assert_eq!(parsed.cross_request_id, "CROSS789");
        assert_eq!(parsed.symbol, Some("GOOGL".to_string()));
        assert_eq!(parsed.market_id, Some("XNAS".to_string()));
    }
}
