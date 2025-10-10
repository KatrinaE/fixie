//! Quotation/Negotiation message types (Pre-Trade)
//!
//! This module implements FIX 5.0 SP2 Quotation and Negotiation messages, which are
//! used for quote requests, quotes, and related workflows in pre-trade activities.
//!
//! ## Message Types
//! - Quote (S): Single or two-sided quote for a security
//! - QuoteRequest (R): Request for quote from market makers
//! - QuoteCancel (Z): Cancel previously submitted quotes
//! - QuoteStatusRequest (a): Request status of a quote
//! - QuoteResponse (AJ): Response to a quote request
//! - QuoteRequestReject (AG): Reject a quote request
//! - RFQRequest (AH): Request for quote (RFQ) request
//! - QuoteAcknowledgment (CW): Acknowledge receipt of a quote
//! - QuoteStatusReport (AI): Report status of a quote
//! - MassQuote (i): Submit multiple quotes for multiple securities
//! - MassQuoteAcknowledgement (b): Acknowledge mass quote submission

use crate::parser::{RawFixMessage, FixParseError};
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Quote (MsgType = S)
// ============================================================================

/// Quote (S) - Single or two-sided quote for a security
///
/// Used to provide bid and/or offer prices for a security. Can be in response
/// to a QuoteRequest or sent unsolicited.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quote {
    /// QuoteID (Tag 117) - Required - Unique identifier for the quote
    pub quote_id: String,

    /// QuoteReqID (Tag 131) - Optional - Reference to quote request if applicable
    pub quote_req_id: Option<String>,

    /// QuoteType (Tag 537) - Optional - Type of quote
    pub quote_type: Option<QuoteType>,

    /// QuoteResponseLevel (Tag 301) - Optional - Level of detail in quote response
    pub quote_response_level: Option<QuoteResponseLevel>,

    /// Symbol (Tag 55) - Required - Security symbol
    pub symbol: String,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// Side (Tag 54) - Optional - Side of quote
    pub side: Option<Side>,

    /// OrderQty (Tag 38) - Optional - Order quantity
    pub order_qty: Option<f64>,

    /// BidPx (Tag 132) - Optional - Bid price
    pub bid_px: Option<f64>,

    /// OfferPx (Tag 133) - Optional - Offer price
    pub offer_px: Option<f64>,

    /// BidSize (Tag 134) - Optional - Bid size
    pub bid_size: Option<f64>,

    /// OfferSize (Tag 135) - Optional - Offer size
    pub offer_size: Option<f64>,

    /// ValidUntilTime (Tag 62) - Optional - Time quote is valid until
    pub valid_until_time: Option<String>,

    /// BidSpotRate (Tag 188) - Optional - Bid spot rate
    pub bid_spot_rate: Option<f64>,

    /// OfferSpotRate (Tag 190) - Optional - Offer spot rate
    pub offer_spot_rate: Option<f64>,

    /// BidForwardPoints (Tag 189) - Optional - Bid forward points
    pub bid_forward_points: Option<f64>,

    /// OfferForwardPoints (Tag 191) - Optional - Offer forward points
    pub offer_forward_points: Option<f64>,

    /// TransactTime (Tag 60) - Optional - Transaction time
    pub transact_time: Option<String>,

    /// SettlType (Tag 63) - Optional - Settlement type
    pub settl_type: Option<String>,

    /// SettlDate (Tag 64) - Optional - Settlement date
    pub settl_date: Option<String>,

    /// OrdType (Tag 40) - Optional - Order type
    pub ord_type: Option<OrdType>,

    /// SettlDate2 (Tag 193) - Optional - Second settlement date for forex
    pub settl_date2: Option<String>,

    /// OrderQty2 (Tag 192) - Optional - Second order quantity for forex
    pub order_qty2: Option<f64>,

    /// Currency (Tag 15) - Optional - Currency
    pub currency: Option<String>,

    /// QuoteStatus (Tag 297) - Optional - Status of quote
    pub quote_status: Option<QuoteStatus>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// BidYield (Tag 632) - Optional - Bid yield
    pub bid_yield: Option<f64>,

    /// OfferYield (Tag 633) - Optional - Offer yield
    pub offer_yield: Option<f64>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: QuotQualGrp - Repeating group (Tag 735 = NoQuoteQualifiers)
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // Note: UndInstrmtGrp - Repeating group (Tag 711 = NoUnderlyings)
    // Note: LegQuotGrp - Repeating group (Tag 555 = NoLegs) for msg "S"
    // These are stored in the RawFixMessage.groups field
}

impl Quote {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(Quote {
            quote_id: raw.get_field(117)
                .ok_or_else(|| FixParseError::MissingRequiredField(117))?
                .to_string(),
            quote_req_id: raw.get_field(131).map(|s| s.to_string()),
            quote_type: raw.get_field(537)
                .and_then(|s| s.chars().next())
                .and_then(QuoteType::from_fix),
            quote_response_level: raw.get_field(301)
                .and_then(|s| s.chars().next())
                .and_then(QuoteResponseLevel::from_fix),
            symbol: raw.get_field(55)
                .ok_or_else(|| FixParseError::MissingRequiredField(55))?
                .to_string(),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix),
            order_qty: raw.get_field(38).and_then(|s| s.parse().ok()),
            bid_px: raw.get_field(132).and_then(|s| s.parse().ok()),
            offer_px: raw.get_field(133).and_then(|s| s.parse().ok()),
            bid_size: raw.get_field(134).and_then(|s| s.parse().ok()),
            offer_size: raw.get_field(135).and_then(|s| s.parse().ok()),
            valid_until_time: raw.get_field(62).map(|s| s.to_string()),
            bid_spot_rate: raw.get_field(188).and_then(|s| s.parse().ok()),
            offer_spot_rate: raw.get_field(190).and_then(|s| s.parse().ok()),
            bid_forward_points: raw.get_field(189).and_then(|s| s.parse().ok()),
            offer_forward_points: raw.get_field(191).and_then(|s| s.parse().ok()),
            transact_time: raw.get_field(60).map(|s| s.to_string()),
            settl_type: raw.get_field(63).map(|s| s.to_string()),
            settl_date: raw.get_field(64).map(|s| s.to_string()),
            ord_type: raw.get_field(40)
                .and_then(|s| s.chars().next())
                .and_then(OrdType::from_fix),
            settl_date2: raw.get_field(193).map(|s| s.to_string()),
            order_qty2: raw.get_field(192).and_then(|s| s.parse().ok()),
            currency: raw.get_field(15).map(|s| s.to_string()),
            quote_status: raw.get_field(297)
                .and_then(QuoteStatus::from_fix),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            bid_yield: raw.get_field(632).and_then(|s| s.parse().ok()),
            offer_yield: raw.get_field(633).and_then(|s| s.parse().ok()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: QuotQualGrp (735) is accessed via raw.groups.get(&735)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
        // Note: UndInstrmtGrp (711) is accessed via raw.groups.get(&711)
        // Note: LegQuotGrp (555) is accessed via raw.groups.get(&555)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "S".to_string());

        // Required fields
        msg.set_field(117, self.quote_id.clone());
        msg.set_field(55, self.symbol.clone());

        // Optional fields
        if let Some(ref quote_req_id) = self.quote_req_id {
            msg.set_field(131, quote_req_id.clone());
        }
        if let Some(quote_type) = self.quote_type {
            msg.set_field(537, quote_type.to_fix().to_string());
        }
        if let Some(quote_response_level) = self.quote_response_level {
            msg.set_field(301, quote_response_level.to_fix().to_string());
        }
        if let Some(ref security_id) = self.security_id {
            msg.set_field(48, security_id.clone());
        }
        if let Some(ref security_id_source) = self.security_id_source {
            msg.set_field(22, security_id_source.clone());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(order_qty) = self.order_qty {
            msg.set_field(38, order_qty.to_string());
        }
        if let Some(bid_px) = self.bid_px {
            msg.set_field(132, bid_px.to_string());
        }
        if let Some(offer_px) = self.offer_px {
            msg.set_field(133, offer_px.to_string());
        }
        if let Some(bid_size) = self.bid_size {
            msg.set_field(134, bid_size.to_string());
        }
        if let Some(offer_size) = self.offer_size {
            msg.set_field(135, offer_size.to_string());
        }
        if let Some(ref valid_until_time) = self.valid_until_time {
            msg.set_field(62, valid_until_time.clone());
        }
        if let Some(bid_spot_rate) = self.bid_spot_rate {
            msg.set_field(188, bid_spot_rate.to_string());
        }
        if let Some(offer_spot_rate) = self.offer_spot_rate {
            msg.set_field(190, offer_spot_rate.to_string());
        }
        if let Some(bid_forward_points) = self.bid_forward_points {
            msg.set_field(189, bid_forward_points.to_string());
        }
        if let Some(offer_forward_points) = self.offer_forward_points {
            msg.set_field(191, offer_forward_points.to_string());
        }
        if let Some(ref transact_time) = self.transact_time {
            msg.set_field(60, transact_time.clone());
        }
        if let Some(ref settl_type) = self.settl_type {
            msg.set_field(63, settl_type.clone());
        }
        if let Some(ref settl_date) = self.settl_date {
            msg.set_field(64, settl_date.clone());
        }
        if let Some(ord_type) = self.ord_type {
            msg.set_field(40, ord_type.to_fix().to_string());
        }
        if let Some(ref settl_date2) = self.settl_date2 {
            msg.set_field(193, settl_date2.clone());
        }
        if let Some(order_qty2) = self.order_qty2 {
            msg.set_field(192, order_qty2.to_string());
        }
        if let Some(ref currency) = self.currency {
            msg.set_field(15, currency.clone());
        }
        if let Some(quote_status) = self.quote_status {
            msg.set_field(297, quote_status.to_fix().to_string());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(bid_yield) = self.bid_yield {
            msg.set_field(632, bid_yield.to_string());
        }
        if let Some(offer_yield) = self.offer_yield {
            msg.set_field(633, offer_yield.to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: QuotQualGrp (735) should be added via msg.groups
        // Note: Parties (453) should be added via msg.groups
        // Note: UndInstrmtGrp (711) should be added via msg.groups
        // Note: LegQuotGrp (555) should be added via msg.groups

        msg
    }
}

// ============================================================================
// QuoteRequest (MsgType = R)
// ============================================================================

/// QuoteRequest (R) - Request for quote from market makers
///
/// Used to request quotes from one or more market makers. Can request quotes
/// for multiple securities or quote sets.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuoteRequest {
    /// QuoteReqID (Tag 131) - Required - Unique identifier for the quote request
    pub quote_req_id: String,

    /// RFQReqID (Tag 644) - Optional - Reference to RFQ request
    pub rfq_req_id: Option<String>,

    /// ClOrdID (Tag 11) - Optional - Client order ID
    pub cl_ord_id: Option<String>,

    /// OrderQty (Tag 38) - Optional - Order quantity
    pub order_qty: Option<f64>,

    /// Side (Tag 54) - Optional - Side of the market
    pub side: Option<Side>,

    /// TransactTime (Tag 60) - Optional - Transaction time
    pub transact_time: Option<String>,

    /// Symbol (Tag 55) - Optional - Security symbol (when requesting for single security)
    pub symbol: Option<String>,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// SettlType (Tag 63) - Optional - Settlement type
    pub settl_type: Option<String>,

    /// SettlDate (Tag 64) - Optional - Settlement date
    pub settl_date: Option<String>,

    /// SettlDate2 (Tag 193) - Optional - Second settlement date for forex
    pub settl_date2: Option<String>,

    /// OrderQty2 (Tag 192) - Optional - Second order quantity for forex
    pub order_qty2: Option<f64>,

    /// Currency (Tag 15) - Optional - Currency
    pub currency: Option<String>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// QuoteType (Tag 537) - Optional - Type of quote requested
    pub quote_type: Option<QuoteType>,

    /// OrdType (Tag 40) - Optional - Order type
    pub ord_type: Option<OrdType>,

    /// ValidUntilTime (Tag 62) - Optional - Time quote is valid until
    pub valid_until_time: Option<String>,

    /// ExpireTime (Tag 126) - Optional - Expiration time for the quote request
    pub expire_time: Option<String>,

    /// PrevClosePx (Tag 140) - Optional - Previous close price
    pub prev_close_px: Option<f64>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: QuotReqGrp - Repeating group (Tag 146 = NoRelatedSym) for msg "R"
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // Note: LegQuotGrp - Repeating group (Tag 555 = NoLegs) for msg "R"
    // These are stored in the RawFixMessage.groups field
}

impl QuoteRequest {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(QuoteRequest {
            quote_req_id: raw.get_field(131)
                .ok_or_else(|| FixParseError::MissingRequiredField(131))?
                .to_string(),
            rfq_req_id: raw.get_field(644).map(|s| s.to_string()),
            cl_ord_id: raw.get_field(11).map(|s| s.to_string()),
            order_qty: raw.get_field(38).and_then(|s| s.parse().ok()),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix),
            transact_time: raw.get_field(60).map(|s| s.to_string()),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            settl_type: raw.get_field(63).map(|s| s.to_string()),
            settl_date: raw.get_field(64).map(|s| s.to_string()),
            settl_date2: raw.get_field(193).map(|s| s.to_string()),
            order_qty2: raw.get_field(192).and_then(|s| s.parse().ok()),
            currency: raw.get_field(15).map(|s| s.to_string()),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            quote_type: raw.get_field(537)
                .and_then(|s| s.chars().next())
                .and_then(QuoteType::from_fix),
            ord_type: raw.get_field(40)
                .and_then(|s| s.chars().next())
                .and_then(OrdType::from_fix),
            valid_until_time: raw.get_field(62).map(|s| s.to_string()),
            expire_time: raw.get_field(126).map(|s| s.to_string()),
            prev_close_px: raw.get_field(140).and_then(|s| s.parse().ok()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: QuotReqGrp (146) is accessed via raw.groups.get(&146)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
        // Note: LegQuotGrp (555) is accessed via raw.groups.get(&555)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "R".to_string());

        // Required fields
        msg.set_field(131, self.quote_req_id.clone());

        // Optional fields
        if let Some(ref rfq_req_id) = self.rfq_req_id {
            msg.set_field(644, rfq_req_id.clone());
        }
        if let Some(ref cl_ord_id) = self.cl_ord_id {
            msg.set_field(11, cl_ord_id.clone());
        }
        if let Some(order_qty) = self.order_qty {
            msg.set_field(38, order_qty.to_string());
        }
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(ref transact_time) = self.transact_time {
            msg.set_field(60, transact_time.clone());
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
        if let Some(ref settl_type) = self.settl_type {
            msg.set_field(63, settl_type.clone());
        }
        if let Some(ref settl_date) = self.settl_date {
            msg.set_field(64, settl_date.clone());
        }
        if let Some(ref settl_date2) = self.settl_date2 {
            msg.set_field(193, settl_date2.clone());
        }
        if let Some(order_qty2) = self.order_qty2 {
            msg.set_field(192, order_qty2.to_string());
        }
        if let Some(ref currency) = self.currency {
            msg.set_field(15, currency.clone());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(quote_type) = self.quote_type {
            msg.set_field(537, quote_type.to_fix().to_string());
        }
        if let Some(ord_type) = self.ord_type {
            msg.set_field(40, ord_type.to_fix().to_string());
        }
        if let Some(ref valid_until_time) = self.valid_until_time {
            msg.set_field(62, valid_until_time.clone());
        }
        if let Some(ref expire_time) = self.expire_time {
            msg.set_field(126, expire_time.clone());
        }
        if let Some(prev_close_px) = self.prev_close_px {
            msg.set_field(140, prev_close_px.to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: QuotReqGrp (146) should be added via msg.groups
        // Note: Parties (453) should be added via msg.groups
        // Note: LegQuotGrp (555) should be added via msg.groups

        msg
    }
}

// ============================================================================
// QuoteCancel (MsgType = Z)
// ============================================================================

/// QuoteCancel (Z) - Cancel previously submitted quotes
///
/// Used to cancel outstanding quotes. Can cancel all quotes, quotes for a
/// specific symbol, or specific quote IDs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuoteCancel {
    /// QuoteReqID (Tag 131) - Optional - Reference to quote request
    pub quote_req_id: Option<String>,

    /// QuoteID (Tag 117) - Required - Unique identifier for the quote cancel
    pub quote_id: String,

    /// QuoteCancelType (Tag 298) - Required - Type of quote cancellation
    pub quote_cancel_type: QuoteCancelType,

    /// QuoteResponseLevel (Tag 301) - Optional - Level of detail in response
    pub quote_response_level: Option<QuoteResponseLevel>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// TradingSessionID (Tag 336) - Optional - Trading session
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (Tag 625) - Optional - Trading session sub-ID
    pub trading_session_sub_id: Option<String>,

    /// Symbol (Tag 55) - Optional - Security symbol (when canceling for specific symbol)
    pub symbol: Option<String>,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: QuotCxlEntriesGrp - Repeating group (Tag 295 = NoQuoteEntries) for msg "Z"
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // These are stored in the RawFixMessage.groups field
}

impl QuoteCancel {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(QuoteCancel {
            quote_req_id: raw.get_field(131).map(|s| s.to_string()),
            quote_id: raw.get_field(117)
                .ok_or_else(|| FixParseError::MissingRequiredField(117))?
                .to_string(),
            quote_cancel_type: raw.get_field(298)
                .and_then(|s| s.chars().next())
                .and_then(QuoteCancelType::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(298))?,
            quote_response_level: raw.get_field(301)
                .and_then(|s| s.chars().next())
                .and_then(QuoteResponseLevel::from_fix),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            trading_session_sub_id: raw.get_field(625).map(|s| s.to_string()),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: QuotCxlEntriesGrp (295) is accessed via raw.groups.get(&295)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "Z".to_string());

        // Required fields
        msg.set_field(117, self.quote_id.clone());
        msg.set_field(298, self.quote_cancel_type.to_fix().to_string());

        // Optional fields
        if let Some(ref quote_req_id) = self.quote_req_id {
            msg.set_field(131, quote_req_id.clone());
        }
        if let Some(quote_response_level) = self.quote_response_level {
            msg.set_field(301, quote_response_level.to_fix().to_string());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(ref trading_session_id) = self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(ref trading_session_sub_id) = self.trading_session_sub_id {
            msg.set_field(625, trading_session_sub_id.clone());
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
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: QuotCxlEntriesGrp (295) should be added via msg.groups
        // Note: Parties (453) should be added via msg.groups

        msg
    }
}

// ============================================================================
// QuoteStatusRequest (MsgType = a)
// ============================================================================

/// QuoteStatusRequest (a) - Request status of a quote
///
/// Used to query the status of a quote or quotes. Can request status by
/// QuoteID, Symbol, or other criteria.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuoteStatusRequest {
    /// QuoteStatusReqID (Tag 649) - Optional - Unique identifier for this request
    pub quote_status_req_id: Option<String>,

    /// QuoteID (Tag 117) - Optional - Specific quote ID to query
    pub quote_id: Option<String>,

    /// Symbol (Tag 55) - Optional - Security symbol
    pub symbol: Option<String>,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// TradingSessionID (Tag 336) - Optional - Trading session
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (Tag 625) - Optional - Trading session sub-ID
    pub trading_session_sub_id: Option<String>,

    /// SubscriptionRequestType (Tag 263) - Optional - Subscription type
    pub subscription_request_type: Option<String>,

    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // This is stored in the RawFixMessage.groups field
}

impl QuoteStatusRequest {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(QuoteStatusRequest {
            quote_status_req_id: raw.get_field(649).map(|s| s.to_string()),
            quote_id: raw.get_field(117).map(|s| s.to_string()),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            trading_session_sub_id: raw.get_field(625).map(|s| s.to_string()),
            subscription_request_type: raw.get_field(263).map(|s| s.to_string()),
        })
        // Note: Parties (453) is accessed via raw.groups.get(&453)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "a".to_string());

        // Optional fields (all fields are optional for this message)
        if let Some(ref quote_status_req_id) = self.quote_status_req_id {
            msg.set_field(649, quote_status_req_id.clone());
        }
        if let Some(ref quote_id) = self.quote_id {
            msg.set_field(117, quote_id.clone());
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
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(ref trading_session_id) = self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(ref trading_session_sub_id) = self.trading_session_sub_id {
            msg.set_field(625, trading_session_sub_id.clone());
        }
        if let Some(ref subscription_request_type) = self.subscription_request_type {
            msg.set_field(263, subscription_request_type.clone());
        }

        // Note: Parties (453) should be added via msg.groups

        msg
    }
}

// ============================================================================
// QuoteResponse (MsgType = AJ)
// ============================================================================

/// QuoteResponse (AJ) - Response to a quote request
///
/// Used to respond to a quote request with acceptance, rejection, or other status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuoteResponse {
    /// QuoteRespID (Tag 693) - Required - Unique identifier for this response
    pub quote_resp_id: String,

    /// QuoteReqID (Tag 131) - Optional - Reference to quote request
    pub quote_req_id: Option<String>,

    /// QuoteID (Tag 117) - Optional - Quote identifier
    pub quote_id: Option<String>,

    /// QuoteRespType (Tag 694) - Optional - Type of response
    pub quote_resp_type: Option<u32>,

    /// ClOrdID (Tag 11) - Optional - Client order ID
    pub cl_ord_id: Option<String>,

    /// OrderQty (Tag 38) - Optional - Order quantity
    pub order_qty: Option<f64>,

    /// Side (Tag 54) - Optional - Side
    pub side: Option<Side>,

    /// Symbol (Tag 55) - Optional - Security symbol
    pub symbol: Option<String>,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// TradingSessionID (Tag 336) - Optional - Trading session
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (Tag 625) - Optional - Trading session sub-ID
    pub trading_session_sub_id: Option<String>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: QuotQualGrp - Repeating group (Tag 735 = NoQuoteQualifiers)
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // These are stored in the RawFixMessage.groups field
}

impl QuoteResponse {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(QuoteResponse {
            quote_resp_id: raw.get_field(693)
                .ok_or_else(|| FixParseError::MissingRequiredField(693))?
                .to_string(),
            quote_req_id: raw.get_field(131).map(|s| s.to_string()),
            quote_id: raw.get_field(117).map(|s| s.to_string()),
            quote_resp_type: raw.get_field(694).and_then(|s| s.parse().ok()),
            cl_ord_id: raw.get_field(11).map(|s| s.to_string()),
            order_qty: raw.get_field(38).and_then(|s| s.parse().ok()),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            trading_session_sub_id: raw.get_field(625).map(|s| s.to_string()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: QuotQualGrp (735) is accessed via raw.groups.get(&735)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AJ".to_string());

        // Required fields
        msg.set_field(693, self.quote_resp_id.clone());

        // Optional fields
        if let Some(ref quote_req_id) = self.quote_req_id {
            msg.set_field(131, quote_req_id.clone());
        }
        if let Some(ref quote_id) = self.quote_id {
            msg.set_field(117, quote_id.clone());
        }
        if let Some(quote_resp_type) = self.quote_resp_type {
            msg.set_field(694, quote_resp_type.to_string());
        }
        if let Some(ref cl_ord_id) = self.cl_ord_id {
            msg.set_field(11, cl_ord_id.clone());
        }
        if let Some(order_qty) = self.order_qty {
            msg.set_field(38, order_qty.to_string());
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
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(ref trading_session_id) = self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(ref trading_session_sub_id) = self.trading_session_sub_id {
            msg.set_field(625, trading_session_sub_id.clone());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: QuotQualGrp (735) should be added via msg.groups
        // Note: Parties (453) should be added via msg.groups

        msg
    }
}

// ============================================================================
// QuoteRequestReject (MsgType = AG)
// ============================================================================

/// QuoteRequestReject (AG) - Reject a quote request
///
/// Used to reject a quote request with a reason. Sent in response to a QuoteRequest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuoteRequestReject {
    /// QuoteReqID (Tag 131) - Required - Reference to quote request being rejected
    pub quote_req_id: String,

    /// RFQReqID (Tag 644) - Optional - Reference to RFQ request
    pub rfq_req_id: Option<String>,

    /// QuoteRequestRejectReason (Tag 658) - Optional - Reason for rejection
    pub quote_request_reject_reason: Option<QuoteRequestRejectReason>,

    /// Text (Tag 58) - Optional - Free-form text explanation
    pub text: Option<String>,

    // Note: QuotReqRjctGrp - Repeating group (Tag 146 = NoRelatedSym) for msg "AG"
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // These are stored in the RawFixMessage.groups field
}

impl QuoteRequestReject {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(QuoteRequestReject {
            quote_req_id: raw.get_field(131)
                .ok_or_else(|| FixParseError::MissingRequiredField(131))?
                .to_string(),
            rfq_req_id: raw.get_field(644).map(|s| s.to_string()),
            quote_request_reject_reason: raw.get_field(658)
                .and_then(QuoteRequestRejectReason::from_fix),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: QuotReqRjctGrp (146) is accessed via raw.groups.get(&146)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AG".to_string());

        // Required fields
        msg.set_field(131, self.quote_req_id.clone());

        // Optional fields
        if let Some(ref rfq_req_id) = self.rfq_req_id {
            msg.set_field(644, rfq_req_id.clone());
        }
        if let Some(reject_reason) = self.quote_request_reject_reason {
            msg.set_field(658, reject_reason.to_fix().to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: QuotReqRjctGrp (146) should be added via msg.groups
        // Note: Parties (453) should be added via msg.groups

        msg
    }
}

// ============================================================================
// RFQRequest (MsgType = AH)
// ============================================================================

/// RFQRequest (AH) - Request for quote (RFQ) request
///
/// Used to request quotes from multiple market makers for one or more securities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RFQRequest {
    /// RFQReqID (Tag 644) - Required - Unique identifier for this RFQ
    pub rfq_req_id: String,

    /// SubscriptionRequestType (Tag 263) - Optional - Subscription type
    pub subscription_request_type: Option<String>,

    // Note: RFQReqGrp - Repeating group (Tag 146 = NoRelatedSym) for msg "AH"
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // Note: LegQuotGrp - Repeating group (Tag 555 = NoLegs) for msg "AH"
    // These are stored in the RawFixMessage.groups field
}

impl RFQRequest {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(RFQRequest {
            rfq_req_id: raw.get_field(644)
                .ok_or_else(|| FixParseError::MissingRequiredField(644))?
                .to_string(),
            subscription_request_type: raw.get_field(263).map(|s| s.to_string()),
        })
        // Note: RFQReqGrp (146) is accessed via raw.groups.get(&146)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
        // Note: LegQuotGrp (555) is accessed via raw.groups.get(&555)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AH".to_string());

        // Required fields
        msg.set_field(644, self.rfq_req_id.clone());

        // Optional fields
        if let Some(ref subscription_request_type) = self.subscription_request_type {
            msg.set_field(263, subscription_request_type.clone());
        }

        // Note: RFQReqGrp (146) should be added via msg.groups
        // Note: Parties (453) should be added via msg.groups
        // Note: LegQuotGrp (555) should be added via msg.groups

        msg
    }
}

// ============================================================================
// QuoteAcknowledgment (MsgType = CW)
// ============================================================================

/// QuoteAcknowledgment (CW) - Acknowledge receipt of a quote
///
/// Used to acknowledge receipt and processing of a quote message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuoteAcknowledgment {
    /// QuoteAckStatus (Tag 297) - Required - Status of acknowledgment
    pub quote_ack_status: QuoteStatus,

    /// QuoteReqID (Tag 131) - Optional - Reference to quote request
    pub quote_req_id: Option<String>,

    /// QuoteID (Tag 117) - Optional - Quote identifier
    pub quote_id: Option<String>,

    /// QuoteType (Tag 537) - Optional - Type of quote
    pub quote_type: Option<QuoteType>,

    /// QuoteCancelType (Tag 298) - Optional - Type of cancellation if applicable
    pub quote_cancel_type: Option<QuoteCancelType>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: QuotQualGrp - Repeating group (Tag 735 = NoQuoteQualifiers)
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // These are stored in the RawFixMessage.groups field
}

impl QuoteAcknowledgment {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(QuoteAcknowledgment {
            quote_ack_status: raw.get_field(297)
                .and_then(QuoteStatus::from_fix)
                .ok_or_else(|| FixParseError::MissingRequiredField(297))?,
            quote_req_id: raw.get_field(131).map(|s| s.to_string()),
            quote_id: raw.get_field(117).map(|s| s.to_string()),
            quote_type: raw.get_field(537)
                .and_then(|s| s.chars().next())
                .and_then(QuoteType::from_fix),
            quote_cancel_type: raw.get_field(298)
                .and_then(|s| s.chars().next())
                .and_then(QuoteCancelType::from_fix),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: QuotQualGrp (735) is accessed via raw.groups.get(&735)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "CW".to_string());

        // Required fields
        msg.set_field(297, self.quote_ack_status.to_fix().to_string());

        // Optional fields
        if let Some(ref quote_req_id) = self.quote_req_id {
            msg.set_field(131, quote_req_id.clone());
        }
        if let Some(ref quote_id) = self.quote_id {
            msg.set_field(117, quote_id.clone());
        }
        if let Some(quote_type) = self.quote_type {
            msg.set_field(537, quote_type.to_fix().to_string());
        }
        if let Some(quote_cancel_type) = self.quote_cancel_type {
            msg.set_field(298, quote_cancel_type.to_fix().to_string());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: QuotQualGrp (735) should be added via msg.groups
        // Note: Parties (453) should be added via msg.groups

        msg
    }
}

// ============================================================================
// QuoteStatusReport (MsgType = AI)
// ============================================================================

/// QuoteStatusReport (AI) - Report status of a quote
///
/// Sent in response to QuoteStatusRequest or unsolicited to report quote status changes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuoteStatusReport {
    /// QuoteStatusReqID (Tag 649) - Optional - Reference to status request
    pub quote_status_req_id: Option<String>,

    /// QuoteReqID (Tag 131) - Optional - Reference to quote request
    pub quote_req_id: Option<String>,

    /// QuoteID (Tag 117) - Optional - Quote identifier
    pub quote_id: Option<String>,

    /// QuoteType (Tag 537) - Optional - Type of quote
    pub quote_type: Option<QuoteType>,

    /// QuoteStatus (Tag 297) - Optional - Current status of quote
    pub quote_status: Option<QuoteStatus>,

    /// Symbol (Tag 55) - Optional - Security symbol
    pub symbol: Option<String>,

    /// SecurityID (Tag 48) - Optional - Security identifier
    pub security_id: Option<String>,

    /// SecurityIDSource (Tag 22) - Optional - Source of security ID
    pub security_id_source: Option<String>,

    /// Side (Tag 54) - Optional - Side
    pub side: Option<Side>,

    /// OrderQty (Tag 38) - Optional - Order quantity
    pub order_qty: Option<f64>,

    /// BidPx (Tag 132) - Optional - Bid price
    pub bid_px: Option<f64>,

    /// OfferPx (Tag 133) - Optional - Offer price
    pub offer_px: Option<f64>,

    /// BidSize (Tag 134) - Optional - Bid size
    pub bid_size: Option<f64>,

    /// OfferSize (Tag 135) - Optional - Offer size
    pub offer_size: Option<f64>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// TradingSessionID (Tag 336) - Optional - Trading session
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (Tag 625) - Optional - Trading session sub-ID
    pub trading_session_sub_id: Option<String>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: QuotQualGrp - Repeating group (Tag 735 = NoQuoteQualifiers)
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // Note: LegQuotGrp - Repeating group (Tag 555 = NoLegs) for msg "AI"
    // These are stored in the RawFixMessage.groups field
}

impl QuoteStatusReport {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(QuoteStatusReport {
            quote_status_req_id: raw.get_field(649).map(|s| s.to_string()),
            quote_req_id: raw.get_field(131).map(|s| s.to_string()),
            quote_id: raw.get_field(117).map(|s| s.to_string()),
            quote_type: raw.get_field(537)
                .and_then(|s| s.chars().next())
                .and_then(QuoteType::from_fix),
            quote_status: raw.get_field(297)
                .and_then(QuoteStatus::from_fix),
            symbol: raw.get_field(55).map(|s| s.to_string()),
            security_id: raw.get_field(48).map(|s| s.to_string()),
            security_id_source: raw.get_field(22).map(|s| s.to_string()),
            side: raw.get_field(54)
                .and_then(|s| s.chars().next())
                .and_then(Side::from_fix),
            order_qty: raw.get_field(38).and_then(|s| s.parse().ok()),
            bid_px: raw.get_field(132).and_then(|s| s.parse().ok()),
            offer_px: raw.get_field(133).and_then(|s| s.parse().ok()),
            bid_size: raw.get_field(134).and_then(|s| s.parse().ok()),
            offer_size: raw.get_field(135).and_then(|s| s.parse().ok()),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            trading_session_id: raw.get_field(336).map(|s| s.to_string()),
            trading_session_sub_id: raw.get_field(625).map(|s| s.to_string()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: QuotQualGrp (735) is accessed via raw.groups.get(&735)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
        // Note: LegQuotGrp (555) is accessed via raw.groups.get(&555)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "AI".to_string());

        // Optional fields (all fields are optional for this message)
        if let Some(ref quote_status_req_id) = self.quote_status_req_id {
            msg.set_field(649, quote_status_req_id.clone());
        }
        if let Some(ref quote_req_id) = self.quote_req_id {
            msg.set_field(131, quote_req_id.clone());
        }
        if let Some(ref quote_id) = self.quote_id {
            msg.set_field(117, quote_id.clone());
        }
        if let Some(quote_type) = self.quote_type {
            msg.set_field(537, quote_type.to_fix().to_string());
        }
        if let Some(quote_status) = self.quote_status {
            msg.set_field(297, quote_status.to_fix().to_string());
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
        if let Some(side) = self.side {
            msg.set_field(54, side.to_fix().to_string());
        }
        if let Some(order_qty) = self.order_qty {
            msg.set_field(38, order_qty.to_string());
        }
        if let Some(bid_px) = self.bid_px {
            msg.set_field(132, bid_px.to_string());
        }
        if let Some(offer_px) = self.offer_px {
            msg.set_field(133, offer_px.to_string());
        }
        if let Some(bid_size) = self.bid_size {
            msg.set_field(134, bid_size.to_string());
        }
        if let Some(offer_size) = self.offer_size {
            msg.set_field(135, offer_size.to_string());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(ref trading_session_id) = self.trading_session_id {
            msg.set_field(336, trading_session_id.clone());
        }
        if let Some(ref trading_session_sub_id) = self.trading_session_sub_id {
            msg.set_field(625, trading_session_sub_id.clone());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: QuotQualGrp (735) should be added via msg.groups
        // Note: Parties (453) should be added via msg.groups
        // Note: LegQuotGrp (555) should be added via msg.groups

        msg
    }
}

// ============================================================================
// MassQuote (MsgType = i)
// ============================================================================

/// MassQuote (i) - Submit multiple quotes for multiple securities
///
/// Used to submit a batch of quotes for multiple securities or quote sets.
/// This is an efficient way to submit many quotes in a single message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MassQuote {
    /// QuoteID (Tag 117) - Optional - Unique identifier for the mass quote
    pub quote_id: Option<String>,

    /// QuoteReqID (Tag 131) - Optional - Reference to quote request
    pub quote_req_id: Option<String>,

    /// QuoteType (Tag 537) - Optional - Type of quote
    pub quote_type: Option<QuoteType>,

    /// QuoteResponseLevel (Tag 301) - Optional - Level of detail in response
    pub quote_response_level: Option<QuoteResponseLevel>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// DefBidSize (Tag 293) - Optional - Default bid size for all quotes
    pub def_bid_size: Option<f64>,

    /// DefOfferSize (Tag 294) - Optional - Default offer size for all quotes
    pub def_offer_size: Option<f64>,

    // Note: QuotSetGrp - Repeating group (Tag 296 = NoQuoteSets) for msg "i"
    //       Contains nested QuotEntryGrp (Tag 295 = NoQuoteEntries)
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // These are stored in the RawFixMessage.groups field
}

impl MassQuote {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(MassQuote {
            quote_id: raw.get_field(117).map(|s| s.to_string()),
            quote_req_id: raw.get_field(131).map(|s| s.to_string()),
            quote_type: raw.get_field(537)
                .and_then(|s| s.chars().next())
                .and_then(QuoteType::from_fix),
            quote_response_level: raw.get_field(301)
                .and_then(|s| s.chars().next())
                .and_then(QuoteResponseLevel::from_fix),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            def_bid_size: raw.get_field(293).and_then(|s| s.parse().ok()),
            def_offer_size: raw.get_field(294).and_then(|s| s.parse().ok()),
        })
        // Note: QuotSetGrp (296) is accessed via raw.groups.get(&296)
        //       Each quote set contains nested QuotEntryGrp (295)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "i".to_string());

        // Optional fields (all fields are optional for this message)
        if let Some(ref quote_id) = self.quote_id {
            msg.set_field(117, quote_id.clone());
        }
        if let Some(ref quote_req_id) = self.quote_req_id {
            msg.set_field(131, quote_req_id.clone());
        }
        if let Some(quote_type) = self.quote_type {
            msg.set_field(537, quote_type.to_fix().to_string());
        }
        if let Some(quote_response_level) = self.quote_response_level {
            msg.set_field(301, quote_response_level.to_fix().to_string());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(def_bid_size) = self.def_bid_size {
            msg.set_field(293, def_bid_size.to_string());
        }
        if let Some(def_offer_size) = self.def_offer_size {
            msg.set_field(294, def_offer_size.to_string());
        }

        // Note: QuotSetGrp (296) should be added via msg.groups
        //       Each quote set contains nested QuotEntryGrp (295)
        // Note: Parties (453) should be added via msg.groups

        msg
    }
}

// ============================================================================
// MassQuoteAcknowledgement (MsgType = b)
// ============================================================================

/// MassQuoteAcknowledgement (b) - Acknowledge mass quote submission
///
/// Sent in response to MassQuote to acknowledge receipt and processing status
/// of the submitted quotes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MassQuoteAcknowledgement {
    /// QuoteID (Tag 117) - Optional - Echo of quote ID from mass quote
    pub quote_id: Option<String>,

    /// QuoteReqID (Tag 131) - Optional - Reference to quote request
    pub quote_req_id: Option<String>,

    /// QuoteStatus (Tag 297) - Optional - Overall status of mass quote
    pub quote_status: Option<QuoteStatus>,

    /// QuoteRejectReason (Tag 300) - Optional - Reason for rejection if rejected
    pub quote_reject_reason: Option<u32>,

    /// QuoteResponseLevel (Tag 301) - Optional - Level of detail in acknowledgement
    pub quote_response_level: Option<QuoteResponseLevel>,

    /// QuoteType (Tag 537) - Optional - Type of quote
    pub quote_type: Option<QuoteType>,

    /// Account (Tag 1) - Optional - Account
    pub account: Option<String>,

    /// AcctIDSource (Tag 660) - Optional - Account ID source
    pub acct_id_source: Option<u32>,

    /// AccountType (Tag 581) - Optional - Account type
    pub account_type: Option<u32>,

    /// Text (Tag 58) - Optional - Free-form text
    pub text: Option<String>,

    // Note: QuotSetAckGrp - Repeating group (Tag 296 = NoQuoteSets) for msg "b"
    //       Contains nested QuotEntryAckGrp (Tag 295 = NoQuoteEntries)
    // Note: Parties - Repeating group (Tag 453 = NoPartyIDs)
    // These are stored in the RawFixMessage.groups field
}

impl MassQuoteAcknowledgement {
    /// Convert from RawFixMessage
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> {
        Ok(MassQuoteAcknowledgement {
            quote_id: raw.get_field(117).map(|s| s.to_string()),
            quote_req_id: raw.get_field(131).map(|s| s.to_string()),
            quote_status: raw.get_field(297)
                .and_then(QuoteStatus::from_fix),
            quote_reject_reason: raw.get_field(300).and_then(|s| s.parse().ok()),
            quote_response_level: raw.get_field(301)
                .and_then(|s| s.chars().next())
                .and_then(QuoteResponseLevel::from_fix),
            quote_type: raw.get_field(537)
                .and_then(|s| s.chars().next())
                .and_then(QuoteType::from_fix),
            account: raw.get_field(1).map(|s| s.to_string()),
            acct_id_source: raw.get_field(660).and_then(|s| s.parse().ok()),
            account_type: raw.get_field(581).and_then(|s| s.parse().ok()),
            text: raw.get_field(58).map(|s| s.to_string()),
        })
        // Note: QuotSetAckGrp (296) is accessed via raw.groups.get(&296)
        //       Each quote set contains nested QuotEntryAckGrp (295)
        // Note: Parties (453) is accessed via raw.groups.get(&453)
    }

    /// Convert to RawFixMessage
    pub fn to_raw(&self) -> RawFixMessage {
        let mut msg = RawFixMessage::new();

        // Set MsgType
        msg.set_field(35, "b".to_string());

        // Optional fields (all fields are optional for this message)
        if let Some(ref quote_id) = self.quote_id {
            msg.set_field(117, quote_id.clone());
        }
        if let Some(ref quote_req_id) = self.quote_req_id {
            msg.set_field(131, quote_req_id.clone());
        }
        if let Some(quote_status) = self.quote_status {
            msg.set_field(297, quote_status.to_fix().to_string());
        }
        if let Some(quote_reject_reason) = self.quote_reject_reason {
            msg.set_field(300, quote_reject_reason.to_string());
        }
        if let Some(quote_response_level) = self.quote_response_level {
            msg.set_field(301, quote_response_level.to_fix().to_string());
        }
        if let Some(quote_type) = self.quote_type {
            msg.set_field(537, quote_type.to_fix().to_string());
        }
        if let Some(ref account) = self.account {
            msg.set_field(1, account.clone());
        }
        if let Some(acct_id_source) = self.acct_id_source {
            msg.set_field(660, acct_id_source.to_string());
        }
        if let Some(account_type) = self.account_type {
            msg.set_field(581, account_type.to_string());
        }
        if let Some(ref text) = self.text {
            msg.set_field(58, text.clone());
        }

        // Note: QuotSetAckGrp (296) should be added via msg.groups
        //       Each quote set contains nested QuotEntryAckGrp (295)
        // Note: Parties (453) should be added via msg.groups

        msg
    }
}
