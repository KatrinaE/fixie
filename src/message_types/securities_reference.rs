use crate::types::{SecurityRequestType, SecurityRequestResult, SecurityUpdateAction, SecurityListRequestType, SecurityTradingStatus, HaltReason};
use serde::{Deserialize, Serialize};

// ============================================================================
// Securities Reference Data Messages
// Implementation of FIX 5.0 SP2 Securities Reference Data messages
// ============================================================================

/// SecurityDefinitionRequest (MsgType=c)
///
/// Used to request security definitions from a counterparty.
/// The request can be for a specific security or for a list of securities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityDefinitionRequest {
    /// SecurityReqID (320) - Unique identifier for the security definition request
    pub security_req_id: String,

    /// SecurityRequestType (321) - Type of security definition request
    pub security_request_type: SecurityRequestType,

    /// Symbol (55) - Ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// Product (460) - Product classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// CFICode (461) - Classification of Financial Instruments code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,

    /// SecurityType (167) - Type of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// SecuritySubType (762) - Sub-type qualification/classification of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_sub_type: Option<String>,

    /// MaturityMonthYear (200) - Month and year of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_month_year: Option<String>,

    /// MaturityDate (541) - Date of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<String>,

    /// StrikePrice (202) - Strike price for options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,

    /// TradingSessionID (336) - Trading session for which security definition is requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SecurityDefinitionRequest {
    /// Create a new SecurityDefinitionRequest with required fields
    pub fn new(
        security_req_id: String,
        security_request_type: SecurityRequestType,
    ) -> Self {
        Self {
            security_req_id,
            security_request_type,
            symbol: None,
            security_id: None,
            security_id_source: None,
            product: None,
            cfi_code: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            strike_price: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
        }
    }

    /// Set the symbol
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set the security ID
    pub fn with_security_id(mut self, security_id: String, source: String) -> Self {
        self.security_id = Some(security_id);
        self.security_id_source = Some(source);
        self
    }

    /// Set the security type
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }
}

/// SecurityDefinition (MsgType=d)
///
/// Response to a SecurityDefinitionRequest message.
/// Contains the security definition details or indicates the result of the request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityDefinition {
    /// SecurityReqID (320) - Unique identifier echoed from the request
    pub security_req_id: String,

    /// SecurityResponseID (322) - Unique identifier for this response
    pub security_response_id: String,

    /// SecurityRequestResult (560) - Result of the security definition request
    pub security_request_result: SecurityRequestResult,

    /// Symbol (55) - Ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// SecurityDesc (107) - Description of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_desc: Option<String>,

    /// SecurityType (167) - Type of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// MaturityMonthYear (200) - Month and year of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_month_year: Option<String>,

    /// MaturityDate (541) - Date of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<String>,

    /// StrikePrice (202) - Strike price for options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,

    /// Currency (15) - Currency of the security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// TotalNumSecurities (393) - Total number of securities in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_securities: Option<u32>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SecurityDefinition {
    /// Create a new SecurityDefinition with required fields
    pub fn new(
        security_req_id: String,
        security_response_id: String,
        security_request_result: SecurityRequestResult,
    ) -> Self {
        Self {
            security_req_id,
            security_response_id,
            security_request_result,
            symbol: None,
            security_id: None,
            security_id_source: None,
            security_desc: None,
            security_type: None,
            maturity_month_year: None,
            maturity_date: None,
            strike_price: None,
            currency: None,
            total_num_securities: None,
            text: None,
        }
    }

    /// Set the symbol and description
    pub fn with_symbol(mut self, symbol: String, desc: Option<String>) -> Self {
        self.symbol = Some(symbol);
        self.security_desc = desc;
        self
    }

    /// Set the security ID
    pub fn with_security_id(mut self, security_id: String, source: String) -> Self {
        self.security_id = Some(security_id);
        self.security_id_source = Some(source);
        self
    }

    /// Set the security type
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Set the currency
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
}

/// SecurityDefinitionUpdateReport (MsgType=BP)
///
/// Broadcast message providing unsolicited updates to security definitions.
/// Typically sent by exchanges to notify participants of security changes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityDefinitionUpdateReport {
    /// SecurityUpdateAction (980) - Type of update action
    pub security_update_action: SecurityUpdateAction,

    /// SecurityReportID (964) - Unique identifier for this report
    pub security_report_id: String,

    /// ClearingBusinessDate (715) - Business date for which update applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_business_date: Option<String>,

    /// Symbol (55) - Ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// SecurityDesc (107) - Description of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_desc: Option<String>,

    /// SecurityType (167) - Type of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// SecuritySubType (762) - Sub-type qualification/classification of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_sub_type: Option<String>,

    /// MaturityMonthYear (200) - Month and year of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_month_year: Option<String>,

    /// MaturityDate (541) - Date of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<String>,

    /// StrikePrice (202) - Strike price for options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,

    /// Currency (15) - Currency of the security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// CFICode (461) - Classification of Financial Instruments code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,

    /// Product (460) - Product classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SecurityDefinitionUpdateReport {
    /// Create a new SecurityDefinitionUpdateReport with required fields
    pub fn new(
        security_update_action: SecurityUpdateAction,
        security_report_id: String,
    ) -> Self {
        Self {
            security_update_action,
            security_report_id,
            clearing_business_date: None,
            symbol: None,
            security_id: None,
            security_id_source: None,
            security_desc: None,
            security_type: None,
            security_sub_type: None,
            maturity_month_year: None,
            maturity_date: None,
            strike_price: None,
            currency: None,
            cfi_code: None,
            product: None,
            text: None,
        }
    }

    /// Set the clearing business date
    pub fn with_clearing_business_date(mut self, date: String) -> Self {
        self.clearing_business_date = Some(date);
        self
    }

    /// Set the symbol and description
    pub fn with_symbol(mut self, symbol: String, desc: Option<String>) -> Self {
        self.symbol = Some(symbol);
        self.security_desc = desc;
        self
    }

    /// Set the security ID
    pub fn with_security_id(mut self, security_id: String, source: String) -> Self {
        self.security_id = Some(security_id);
        self.security_id_source = Some(source);
        self
    }

    /// Set the security type
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Set the currency
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
}

/// SecurityListRequest (MsgType=x)
///
/// Request for a list of securities matching specified criteria.
/// Can be used to retrieve multiple securities in a single request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityListRequest {
    /// SecurityReqID (320) - Unique identifier for the security list request
    pub security_req_id: String,

    /// SecurityListRequestType (559) - Type of security list request
    pub security_list_request_type: SecurityListRequestType,

    /// Symbol (55) - Ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// SecurityType (167) - Type of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// CFICode (461) - Classification of Financial Instruments code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,

    /// Product (460) - Product classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// TradingSessionID (336) - Trading session identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// Currency (15) - Currency of the securities requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SecurityListRequest {
    /// Create a new SecurityListRequest with required fields
    pub fn new(
        security_req_id: String,
        security_list_request_type: SecurityListRequestType,
    ) -> Self {
        Self {
            security_req_id,
            security_list_request_type,
            symbol: None,
            security_type: None,
            cfi_code: None,
            product: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            currency: None,
            text: None,
        }
    }

    /// Set the symbol
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set the security type
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Set the product classification
    pub fn with_product(mut self, product: u32) -> Self {
        self.product = Some(product);
        self
    }

    /// Set the trading session
    pub fn with_trading_session(mut self, session_id: String, sub_id: Option<String>) -> Self {
        self.trading_session_id = Some(session_id);
        self.trading_session_sub_id = sub_id;
        self
    }
}

/// SecurityList (MsgType=y)
///
/// Response to a SecurityListRequest containing a list of securities.
/// May be sent in multiple fragments for large lists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityList {
    /// SecurityReqID (320) - Unique identifier echoed from the request
    pub security_req_id: String,

    /// SecurityResponseID (322) - Unique identifier for this response
    pub security_response_id: String,

    /// SecurityRequestResult (560) - Result of the security list request
    pub security_request_result: SecurityRequestResult,

    /// TotalNumSecurities (393) - Total number of securities in the complete response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_securities: Option<u32>,

    /// LastFragment (893) - Indicates whether this is the last fragment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fragment: Option<bool>,

    /// SecurityReportID (964) - Unique identifier for the report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_id: Option<String>,

    /// ClearingBusinessDate (715) - Business date for which list applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_business_date: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    // Note: In a full implementation, this would include a repeating group
    // of security details (NoRelatedSym group). For simplicity, we're
    // representing basic fields here. A complete implementation would add:
    // pub securities: Vec<SecurityDetails>
}

impl SecurityList {
    /// Create a new SecurityList with required fields
    pub fn new(
        security_req_id: String,
        security_response_id: String,
        security_request_result: SecurityRequestResult,
    ) -> Self {
        Self {
            security_req_id,
            security_response_id,
            security_request_result,
            total_num_securities: None,
            last_fragment: None,
            security_report_id: None,
            clearing_business_date: None,
            text: None,
        }
    }

    /// Set the total number of securities
    pub fn with_total_num_securities(mut self, total: u32) -> Self {
        self.total_num_securities = Some(total);
        self
    }

    /// Set whether this is the last fragment
    pub fn with_last_fragment(mut self, is_last: bool) -> Self {
        self.last_fragment = Some(is_last);
        self
    }

    /// Set the security report ID
    pub fn with_security_report_id(mut self, report_id: String) -> Self {
        self.security_report_id = Some(report_id);
        self
    }

    /// Set the clearing business date
    pub fn with_clearing_business_date(mut self, date: String) -> Self {
        self.clearing_business_date = Some(date);
        self
    }
}

/// SecurityListUpdateReport (MsgType=BK)
///
/// Broadcast message providing unsolicited updates to security lists.
/// Used to notify participants of changes to multiple securities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityListUpdateReport {
    /// SecurityUpdateAction (980) - Type of update action
    pub security_update_action: SecurityUpdateAction,

    /// SecurityReportID (964) - Unique identifier for this report
    pub security_report_id: String,

    /// SecurityRequestResult (560) - Result of the security list update
    pub security_request_result: SecurityRequestResult,

    /// ClearingBusinessDate (715) - Business date for which update applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_business_date: Option<String>,

    /// TotalNumSecurities (393) - Total number of securities in the complete report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_securities: Option<u32>,

    /// LastFragment (893) - Indicates whether this is the last fragment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fragment: Option<bool>,

    /// SecurityRequestType (321) - Type of security data update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_request_type: Option<SecurityRequestType>,

    /// TradingSessionID (336) - Trading session for which update applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    // Note: In a full implementation, this would include a repeating group
    // of security update details (NoRelatedSym group). For simplicity, we're
    // representing basic fields here.
}

impl SecurityListUpdateReport {
    /// Create a new SecurityListUpdateReport with required fields
    pub fn new(
        security_update_action: SecurityUpdateAction,
        security_report_id: String,
        security_request_result: SecurityRequestResult,
    ) -> Self {
        Self {
            security_update_action,
            security_report_id,
            security_request_result,
            clearing_business_date: None,
            total_num_securities: None,
            last_fragment: None,
            security_request_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
        }
    }

    /// Set the clearing business date
    pub fn with_clearing_business_date(mut self, date: String) -> Self {
        self.clearing_business_date = Some(date);
        self
    }

    /// Set the total number of securities and fragment status
    pub fn with_fragment_info(mut self, total: u32, is_last: bool) -> Self {
        self.total_num_securities = Some(total);
        self.last_fragment = Some(is_last);
        self
    }

    /// Set the security request type
    pub fn with_security_request_type(mut self, request_type: SecurityRequestType) -> Self {
        self.security_request_type = Some(request_type);
        self
    }

    /// Set the trading session
    pub fn with_trading_session(mut self, session_id: String, sub_id: Option<String>) -> Self {
        self.trading_session_id = Some(session_id);
        self.trading_session_sub_id = sub_id;
        self
    }
}

/// SecurityStatusRequest (MsgType=e)
///
/// Request for the trading status of a security.
/// Used to query the current trading state, halt reasons, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityStatusRequest {
    /// SecurityStatusReqID (324) - Unique identifier for the security status request
    pub security_status_req_id: String,

    /// Symbol (55) - Ticker symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// SecurityType (167) - Type of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// TradingSessionID (336) - Trading session for which status is requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// SubscriptionRequestType (263) - Type of subscription (0=snapshot, 1=subscribe, 2=unsubscribe)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_request_type: Option<char>,
}

impl SecurityStatusRequest {
    /// Create a new SecurityStatusRequest with required fields
    pub fn new(security_status_req_id: String) -> Self {
        Self {
            security_status_req_id,
            symbol: None,
            security_id: None,
            security_id_source: None,
            security_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            subscription_request_type: None,
        }
    }

    /// Set the symbol
    pub fn with_symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    /// Set the security ID
    pub fn with_security_id(mut self, security_id: String, source: String) -> Self {
        self.security_id = Some(security_id);
        self.security_id_source = Some(source);
        self
    }

    /// Set the subscription request type (0=snapshot, 1=subscribe, 2=unsubscribe)
    pub fn with_subscription_type(mut self, subscription_type: char) -> Self {
        self.subscription_request_type = Some(subscription_type);
        self
    }
}

/// SecurityStatus (MsgType=f)
///
/// Response to a SecurityStatusRequest or unsolicited update of security trading status.
/// Provides information about trading state, halt reasons, and other status details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityStatus {
    /// SecurityStatusReqID (324) - Unique identifier echoed from the request (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_status_req_id: Option<String>,

    /// Symbol (55) - Ticker symbol
    pub symbol: String,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// SecurityTradingStatus (326) - Trading status of the security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_trading_status: Option<SecurityTradingStatus>,

    /// HaltReason (327) - Reason for trading halt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub halt_reason: Option<HaltReason>,

    /// TradingSessionID (336) - Trading session for which status applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// TradeDate (75) - Trade date for which status applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<String>,

    /// TransactTime (60) - Time of status update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SecurityStatus {
    /// Create a new SecurityStatus with required fields
    pub fn new(symbol: String) -> Self {
        Self {
            security_status_req_id: None,
            symbol,
            security_id: None,
            security_id_source: None,
            security_trading_status: None,
            halt_reason: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            trade_date: None,
            transact_time: None,
            text: None,
        }
    }

    /// Set the request ID (for responses to requests)
    pub fn with_request_id(mut self, req_id: String) -> Self {
        self.security_status_req_id = Some(req_id);
        self
    }

    /// Set the security ID
    pub fn with_security_id(mut self, security_id: String, source: String) -> Self {
        self.security_id = Some(security_id);
        self.security_id_source = Some(source);
        self
    }

    /// Set the trading status
    pub fn with_trading_status(mut self, status: SecurityTradingStatus) -> Self {
        self.security_trading_status = Some(status);
        self
    }

    /// Set the halt reason
    pub fn with_halt_reason(mut self, reason: HaltReason) -> Self {
        self.halt_reason = Some(reason);
        self
    }

    /// Set the transaction time
    pub fn with_transact_time(mut self, time: String) -> Self {
        self.transact_time = Some(time);
        self
    }
}

/// SecurityTypeRequest (MsgType=v)
///
/// Request for a list of security types available on the system.
/// Used to discover what types of securities are supported.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityTypeRequest {
    /// SecurityReqID (320) - Unique identifier for the security type request
    pub security_req_id: String,

    /// SecurityType (167) - Type of security (optional - if omitted, returns all types)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// SecuritySubType (762) - Sub-type qualification/classification of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_sub_type: Option<String>,

    /// Product (460) - Product classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// TradingSessionID (336) - Trading session for which types are requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SecurityTypeRequest {
    /// Create a new SecurityTypeRequest with required fields
    pub fn new(security_req_id: String) -> Self {
        Self {
            security_req_id,
            security_type: None,
            security_sub_type: None,
            product: None,
            trading_session_id: None,
            text: None,
        }
    }

    /// Set the security type filter
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Set the product filter
    pub fn with_product(mut self, product: u32) -> Self {
        self.product = Some(product);
        self
    }
}

/// SecurityTypes (MsgType=w)
///
/// Response to a SecurityTypeRequest providing a list of security types.
/// Contains information about available security types on the system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityTypes {
    /// SecurityReqID (320) - Unique identifier echoed from the request
    pub security_req_id: String,

    /// SecurityResponseID (322) - Unique identifier for this response
    pub security_response_id: String,

    /// SecurityRequestResult (560) - Result of the security type request
    pub security_request_result: SecurityRequestResult,

    /// TotalNumSecurityTypes (557) - Total number of security types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_security_types: Option<u32>,

    /// LastFragment (893) - Indicates whether this is the last fragment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fragment: Option<bool>,

    /// TradingSessionID (336) - Trading session for which types apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    // Note: In a full implementation, this would include a repeating group
    // of security type details (NoSecurityTypes group). For simplicity, we're
    // representing basic fields here.
}

impl SecurityTypes {
    /// Create a new SecurityTypes with required fields
    pub fn new(
        security_req_id: String,
        security_response_id: String,
        security_request_result: SecurityRequestResult,
    ) -> Self {
        Self {
            security_req_id,
            security_response_id,
            security_request_result,
            total_num_security_types: None,
            last_fragment: None,
            trading_session_id: None,
            text: None,
        }
    }

    /// Set the total number of security types
    pub fn with_total_num_types(mut self, total: u32) -> Self {
        self.total_num_security_types = Some(total);
        self
    }

    /// Set whether this is the last fragment
    pub fn with_last_fragment(mut self, is_last: bool) -> Self {
        self.last_fragment = Some(is_last);
        self
    }

    /// Set the trading session
    pub fn with_trading_session(mut self, session_id: String) -> Self {
        self.trading_session_id = Some(session_id);
        self
    }
}

/// DerivativeSecurityListRequest (MsgType=z)
///
/// Request for a list of derivative securities matching specified criteria.
/// Similar to SecurityListRequest but specifically for derivative instruments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DerivativeSecurityListRequest {
    /// SecurityReqID (320) - Unique identifier for the derivative security list request
    pub security_req_id: String,

    /// SecurityListRequestType (559) - Type of security list request
    pub security_list_request_type: SecurityListRequestType,

    /// UnderlyingSymbol (311) - Symbol of the underlying security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_symbol: Option<String>,

    /// UnderlyingSecurityID (309) - Security identifier of the underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_security_id: Option<String>,

    /// UnderlyingSecurityIDSource (305) - Identifies the type of UnderlyingSecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_security_id_source: Option<String>,

    /// SecurityType (167) - Type of derivative security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// SecuritySubType (762) - Sub-type qualification/classification of derivative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_sub_type: Option<String>,

    /// CFICode (461) - Classification of Financial Instruments code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,

    /// Product (460) - Product classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// MaturityMonthYear (200) - Month and year of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_month_year: Option<String>,

    /// MaturityDate (541) - Date of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<String>,

    /// StrikePrice (202) - Strike price for options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,

    /// PutOrCall (201) - Indicates whether an option is a put or call (0=Put, 1=Call)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_or_call: Option<u8>,

    /// TradingSessionID (336) - Trading session identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// Currency (15) - Currency of the derivative securities requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl DerivativeSecurityListRequest {
    /// Create a new DerivativeSecurityListRequest with required fields
    pub fn new(
        security_req_id: String,
        security_list_request_type: SecurityListRequestType,
    ) -> Self {
        Self {
            security_req_id,
            security_list_request_type,
            underlying_symbol: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            security_type: None,
            security_sub_type: None,
            cfi_code: None,
            product: None,
            maturity_month_year: None,
            maturity_date: None,
            strike_price: None,
            put_or_call: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            currency: None,
            text: None,
        }
    }

    /// Set the underlying security symbol
    pub fn with_underlying_symbol(mut self, symbol: String) -> Self {
        self.underlying_symbol = Some(symbol);
        self
    }

    /// Set the underlying security ID
    pub fn with_underlying_security_id(mut self, security_id: String, source: String) -> Self {
        self.underlying_security_id = Some(security_id);
        self.underlying_security_id_source = Some(source);
        self
    }

    /// Set the derivative security type
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Set the product classification
    pub fn with_product(mut self, product: u32) -> Self {
        self.product = Some(product);
        self
    }

    /// Set option details (strike price and put/call indicator)
    pub fn with_option_details(mut self, strike_price: f64, put_or_call: u8) -> Self {
        self.strike_price = Some(strike_price);
        self.put_or_call = Some(put_or_call);
        self
    }

    /// Set maturity information
    pub fn with_maturity(mut self, maturity: String) -> Self {
        self.maturity_month_year = Some(maturity);
        self
    }

    /// Set the trading session
    pub fn with_trading_session(mut self, session_id: String, sub_id: Option<String>) -> Self {
        self.trading_session_id = Some(session_id);
        self.trading_session_sub_id = sub_id;
        self
    }
}

/// DerivativeSecurityList (MsgType=AA)
///
/// Response to a DerivativeSecurityListRequest containing a list of derivative securities.
/// May be sent in multiple fragments for large lists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DerivativeSecurityList {
    /// SecurityReqID (320) - Unique identifier echoed from the request
    pub security_req_id: String,

    /// SecurityResponseID (322) - Unique identifier for this response
    pub security_response_id: String,

    /// SecurityRequestResult (560) - Result of the derivative security list request
    pub security_request_result: SecurityRequestResult,

    /// UnderlyingSymbol (311) - Symbol of the underlying security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_symbol: Option<String>,

    /// UnderlyingSecurityID (309) - Security identifier of the underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_security_id: Option<String>,

    /// UnderlyingSecurityIDSource (305) - Identifies the type of UnderlyingSecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_security_id_source: Option<String>,

    /// TotalNumSecurities (393) - Total number of derivative securities in the complete response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_securities: Option<u32>,

    /// LastFragment (893) - Indicates whether this is the last fragment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fragment: Option<bool>,

    /// SecurityReportID (964) - Unique identifier for the report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_id: Option<String>,

    /// ClearingBusinessDate (715) - Business date for which list applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clearing_business_date: Option<String>,

    /// SecurityType (167) - Type of derivative security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// TradingSessionID (336) - Trading session for which list applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    // Note: In a full implementation, this would include a repeating group
    // of derivative security details (NoRelatedSym group). For simplicity, we're
    // representing basic fields here. A complete implementation would add:
    // pub derivatives: Vec<DerivativeSecurityDetails>
}

impl DerivativeSecurityList {
    /// Create a new DerivativeSecurityList with required fields
    pub fn new(
        security_req_id: String,
        security_response_id: String,
        security_request_result: SecurityRequestResult,
    ) -> Self {
        Self {
            security_req_id,
            security_response_id,
            security_request_result,
            underlying_symbol: None,
            underlying_security_id: None,
            underlying_security_id_source: None,
            total_num_securities: None,
            last_fragment: None,
            security_report_id: None,
            clearing_business_date: None,
            security_type: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            text: None,
        }
    }

    /// Set the underlying security details
    pub fn with_underlying_symbol(mut self, symbol: String) -> Self {
        self.underlying_symbol = Some(symbol);
        self
    }

    /// Set the underlying security ID
    pub fn with_underlying_security_id(mut self, security_id: String, source: String) -> Self {
        self.underlying_security_id = Some(security_id);
        self.underlying_security_id_source = Some(source);
        self
    }

    /// Set the total number of derivative securities
    pub fn with_total_num_securities(mut self, total: u32) -> Self {
        self.total_num_securities = Some(total);
        self
    }

    /// Set whether this is the last fragment
    pub fn with_last_fragment(mut self, is_last: bool) -> Self {
        self.last_fragment = Some(is_last);
        self
    }

    /// Set the security report ID
    pub fn with_security_report_id(mut self, report_id: String) -> Self {
        self.security_report_id = Some(report_id);
        self
    }

    /// Set the clearing business date
    pub fn with_clearing_business_date(mut self, date: String) -> Self {
        self.clearing_business_date = Some(date);
        self
    }

    /// Set the derivative security type
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Set the trading session
    pub fn with_trading_session(mut self, session_id: String, sub_id: Option<String>) -> Self {
        self.trading_session_id = Some(session_id);
        self.trading_session_sub_id = sub_id;
        self
    }
}

/// SecurityMassStatusRequest (MsgType=CN)
///
/// Request for the status of multiple securities matching specified criteria.
/// Used for bulk status queries rather than individual security status requests.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityMassStatusRequest {
    /// SecurityStatusReqID (324) - Unique identifier for the mass status request
    pub security_status_req_id: String,

    /// MassStatusReqType (585) - Type of mass status request (1=Status for securities, 2=Status for underlying, etc.)
    pub mass_status_req_type: u8,

    /// SecurityType (167) - Type of security for which status is requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// SecuritySubType (762) - Sub-type qualification/classification of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_sub_type: Option<String>,

    /// Product (460) - Product classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// CFICode (461) - Classification of Financial Instruments code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,

    /// TradingSessionID (336) - Trading session for which status is requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// MarketID (1301) - Market identifier for which status is requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,

    /// MarketSegmentID (1300) - Market segment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_segment_id: Option<String>,

    /// Currency (15) - Currency of the securities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// UnderlyingSymbol (311) - Symbol of underlying security for derivatives
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying_symbol: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl SecurityMassStatusRequest {
    /// Create a new SecurityMassStatusRequest with required fields
    pub fn new(security_status_req_id: String, mass_status_req_type: u8) -> Self {
        Self {
            security_status_req_id,
            mass_status_req_type,
            security_type: None,
            security_sub_type: None,
            product: None,
            cfi_code: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            market_id: None,
            market_segment_id: None,
            currency: None,
            underlying_symbol: None,
            text: None,
        }
    }

    /// Set the security type filter
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Set the product filter
    pub fn with_product(mut self, product: u32) -> Self {
        self.product = Some(product);
        self
    }

    /// Set the trading session
    pub fn with_trading_session(mut self, session_id: String, sub_id: Option<String>) -> Self {
        self.trading_session_id = Some(session_id);
        self.trading_session_sub_id = sub_id;
        self
    }

    /// Set the market identifiers
    pub fn with_market(mut self, market_id: String, segment_id: Option<String>) -> Self {
        self.market_id = Some(market_id);
        self.market_segment_id = segment_id;
        self
    }

    /// Set the underlying symbol (for derivative queries)
    pub fn with_underlying_symbol(mut self, symbol: String) -> Self {
        self.underlying_symbol = Some(symbol);
        self
    }

    /// Set the currency filter
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }
}

/// SecurityMassStatus (MsgType=CO)
///
/// Response to a SecurityMassStatusRequest providing status for multiple securities.
/// May be sent in multiple messages for large result sets.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityMassStatus {
    /// SecurityStatusReqID (324) - Unique identifier echoed from the request
    pub security_status_req_id: String,

    /// SecurityMassStatusReqID (1654) - Unique identifier for this mass status report
    pub security_mass_status_req_id: String,

    /// SecurityRequestResult (560) - Result of the mass status request
    pub security_request_result: SecurityRequestResult,

    /// MassStatusReqType (585) - Type of mass status request echoed from request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass_status_req_type: Option<u8>,

    /// TotalNumSecurities (393) - Total number of securities in the complete response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num_securities: Option<u32>,

    /// LastFragment (893) - Indicates whether this is the last fragment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fragment: Option<bool>,

    /// MarketID (1301) - Market identifier for the status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<String>,

    /// MarketSegmentID (1300) - Market segment identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_segment_id: Option<String>,

    /// TradingSessionID (336) - Trading session for which status applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_id: Option<String>,

    /// TradingSessionSubID (625) - Trading session sub-identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_session_sub_id: Option<String>,

    /// SecurityType (167) - Type of securities in this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// Currency (15) - Currency of the securities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// TransactTime (60) - Time of status report
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    // Note: In a full implementation, this would include a repeating group
    // of security status details (NoSecurityStatus group). For simplicity, we're
    // representing basic fields here. A complete implementation would add:
    // pub security_statuses: Vec<SecurityStatusDetails>
}

impl SecurityMassStatus {
    /// Create a new SecurityMassStatus with required fields
    pub fn new(
        security_status_req_id: String,
        security_mass_status_req_id: String,
        security_request_result: SecurityRequestResult,
    ) -> Self {
        Self {
            security_status_req_id,
            security_mass_status_req_id,
            security_request_result,
            mass_status_req_type: None,
            total_num_securities: None,
            last_fragment: None,
            market_id: None,
            market_segment_id: None,
            trading_session_id: None,
            trading_session_sub_id: None,
            security_type: None,
            currency: None,
            transact_time: None,
            text: None,
        }
    }

    /// Set the mass status request type
    pub fn with_mass_status_req_type(mut self, req_type: u8) -> Self {
        self.mass_status_req_type = Some(req_type);
        self
    }

    /// Set the total number of securities and fragment status
    pub fn with_fragment_info(mut self, total: u32, is_last: bool) -> Self {
        self.total_num_securities = Some(total);
        self.last_fragment = Some(is_last);
        self
    }

    /// Set the market identifiers
    pub fn with_market(mut self, market_id: String, segment_id: Option<String>) -> Self {
        self.market_id = Some(market_id);
        self.market_segment_id = segment_id;
        self
    }

    /// Set the trading session
    pub fn with_trading_session(mut self, session_id: String, sub_id: Option<String>) -> Self {
        self.trading_session_id = Some(session_id);
        self.trading_session_sub_id = sub_id;
        self
    }

    /// Set the security type
    pub fn with_security_type(mut self, security_type: String) -> Self {
        self.security_type = Some(security_type);
        self
    }

    /// Set the currency
    pub fn with_currency(mut self, currency: String) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Set the transaction time
    pub fn with_transact_time(mut self, time: String) -> Self {
        self.transact_time = Some(time);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_definition_request_creation() {
        let req = SecurityDefinitionRequest::new(
            "REQ123".to_string(),
            SecurityRequestType::RequestSecurityIdentityAndSpecifications,
        );

        assert_eq!(req.security_req_id, "REQ123");
        assert_eq!(req.security_request_type, SecurityRequestType::RequestSecurityIdentityAndSpecifications);
        assert_eq!(req.symbol, None);
    }

    #[test]
    fn test_security_definition_request_with_symbol() {
        let req = SecurityDefinitionRequest::new(
            "REQ123".to_string(),
            SecurityRequestType::Symbol,
        )
        .with_symbol("AAPL".to_string())
        .with_security_type("CS".to_string());

        assert_eq!(req.security_req_id, "REQ123");
        assert_eq!(req.symbol, Some("AAPL".to_string()));
        assert_eq!(req.security_type, Some("CS".to_string()));
    }

    #[test]
    fn test_security_definition_creation() {
        let def = SecurityDefinition::new(
            "REQ123".to_string(),
            "RESP456".to_string(),
            SecurityRequestResult::ValidRequest,
        );

        assert_eq!(def.security_req_id, "REQ123");
        assert_eq!(def.security_response_id, "RESP456");
        assert_eq!(def.security_request_result, SecurityRequestResult::ValidRequest);
    }

    #[test]
    fn test_security_definition_with_details() {
        let def = SecurityDefinition::new(
            "REQ123".to_string(),
            "RESP456".to_string(),
            SecurityRequestResult::ValidRequest,
        )
        .with_symbol("AAPL".to_string(), Some("Apple Inc".to_string()))
        .with_security_id("037833100".to_string(), "1".to_string())
        .with_currency("USD".to_string());

        assert_eq!(def.symbol, Some("AAPL".to_string()));
        assert_eq!(def.security_desc, Some("Apple Inc".to_string()));
        assert_eq!(def.security_id, Some("037833100".to_string()));
        assert_eq!(def.currency, Some("USD".to_string()));
    }

    #[test]
    fn test_security_definition_error_result() {
        let def = SecurityDefinition::new(
            "REQ123".to_string(),
            "RESP456".to_string(),
            SecurityRequestResult::NoInstrumentsFound,
        );

        assert_eq!(def.security_request_result, SecurityRequestResult::NoInstrumentsFound);
    }

    #[test]
    fn test_security_definition_update_report_creation() {
        let report = SecurityDefinitionUpdateReport::new(
            SecurityUpdateAction::Add,
            "RPT789".to_string(),
        );

        assert_eq!(report.security_update_action, SecurityUpdateAction::Add);
        assert_eq!(report.security_report_id, "RPT789");
        assert_eq!(report.symbol, None);
    }

    #[test]
    fn test_security_definition_update_report_with_details() {
        let report = SecurityDefinitionUpdateReport::new(
            SecurityUpdateAction::Modify,
            "RPT789".to_string(),
        )
        .with_clearing_business_date("20250110".to_string())
        .with_symbol("MSFT".to_string(), Some("Microsoft Corporation".to_string()))
        .with_security_id("594918104".to_string(), "1".to_string())
        .with_security_type("CS".to_string())
        .with_currency("USD".to_string());

        assert_eq!(report.security_update_action, SecurityUpdateAction::Modify);
        assert_eq!(report.clearing_business_date, Some("20250110".to_string()));
        assert_eq!(report.symbol, Some("MSFT".to_string()));
        assert_eq!(report.security_desc, Some("Microsoft Corporation".to_string()));
        assert_eq!(report.security_id, Some("594918104".to_string()));
        assert_eq!(report.currency, Some("USD".to_string()));
    }

    #[test]
    fn test_security_definition_update_report_delete_action() {
        let report = SecurityDefinitionUpdateReport::new(
            SecurityUpdateAction::Delete,
            "RPT999".to_string(),
        )
        .with_symbol("OLDTK".to_string(), None);

        assert_eq!(report.security_update_action, SecurityUpdateAction::Delete);
        assert_eq!(report.symbol, Some("OLDTK".to_string()));
    }

    #[test]
    fn test_security_list_request_creation() {
        let req = SecurityListRequest::new(
            "LISTREQ123".to_string(),
            SecurityListRequestType::AllSecurities,
        );

        assert_eq!(req.security_req_id, "LISTREQ123");
        assert_eq!(req.security_list_request_type, SecurityListRequestType::AllSecurities);
        assert_eq!(req.symbol, None);
    }

    #[test]
    fn test_security_list_request_with_criteria() {
        let req = SecurityListRequest::new(
            "LISTREQ456".to_string(),
            SecurityListRequestType::SecurityTypeAndOrCFICode,
        )
        .with_security_type("CS".to_string())
        .with_product(4)
        .with_trading_session("PRE".to_string(), Some("PRE1".to_string()));

        assert_eq!(req.security_list_request_type, SecurityListRequestType::SecurityTypeAndOrCFICode);
        assert_eq!(req.security_type, Some("CS".to_string()));
        assert_eq!(req.product, Some(4));
        assert_eq!(req.trading_session_id, Some("PRE".to_string()));
        assert_eq!(req.trading_session_sub_id, Some("PRE1".to_string()));
    }

    #[test]
    fn test_security_list_creation() {
        let list = SecurityList::new(
            "LISTREQ123".to_string(),
            "LISTRESP789".to_string(),
            SecurityRequestResult::ValidRequest,
        );

        assert_eq!(list.security_req_id, "LISTREQ123");
        assert_eq!(list.security_response_id, "LISTRESP789");
        assert_eq!(list.security_request_result, SecurityRequestResult::ValidRequest);
    }

    #[test]
    fn test_security_list_with_details() {
        let list = SecurityList::new(
            "LISTREQ123".to_string(),
            "LISTRESP789".to_string(),
            SecurityRequestResult::ValidRequest,
        )
        .with_total_num_securities(150)
        .with_last_fragment(true)
        .with_security_report_id("SECRPT001".to_string())
        .with_clearing_business_date("20250110".to_string());

        assert_eq!(list.total_num_securities, Some(150));
        assert_eq!(list.last_fragment, Some(true));
        assert_eq!(list.security_report_id, Some("SECRPT001".to_string()));
        assert_eq!(list.clearing_business_date, Some("20250110".to_string()));
    }

    #[test]
    fn test_security_list_error_response() {
        let list = SecurityList::new(
            "LISTREQ999".to_string(),
            "LISTRESP999".to_string(),
            SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData,
        );

        assert_eq!(list.security_request_result, SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData);
        assert_eq!(list.total_num_securities, None);
    }

    #[test]
    fn test_security_list_update_report_creation() {
        let report = SecurityListUpdateReport::new(
            SecurityUpdateAction::Add,
            "LSTUPD123".to_string(),
            SecurityRequestResult::ValidRequest,
        );

        assert_eq!(report.security_update_action, SecurityUpdateAction::Add);
        assert_eq!(report.security_report_id, "LSTUPD123");
        assert_eq!(report.security_request_result, SecurityRequestResult::ValidRequest);
    }

    #[test]
    fn test_security_list_update_report_with_fragments() {
        let report = SecurityListUpdateReport::new(
            SecurityUpdateAction::Modify,
            "LSTUPD456".to_string(),
            SecurityRequestResult::ValidRequest,
        )
        .with_clearing_business_date("20250110".to_string())
        .with_fragment_info(250, false)
        .with_security_request_type(SecurityRequestType::AllSecurities)
        .with_trading_session("REG".to_string(), Some("REG1".to_string()));

        assert_eq!(report.clearing_business_date, Some("20250110".to_string()));
        assert_eq!(report.total_num_securities, Some(250));
        assert_eq!(report.last_fragment, Some(false));
        assert_eq!(report.security_request_type, Some(SecurityRequestType::AllSecurities));
        assert_eq!(report.trading_session_id, Some("REG".to_string()));
    }

    #[test]
    fn test_security_list_update_report_delete() {
        let report = SecurityListUpdateReport::new(
            SecurityUpdateAction::Delete,
            "LSTUPD789".to_string(),
            SecurityRequestResult::ValidRequest,
        )
        .with_fragment_info(10, true);

        assert_eq!(report.security_update_action, SecurityUpdateAction::Delete);
        assert_eq!(report.total_num_securities, Some(10));
        assert_eq!(report.last_fragment, Some(true));
    }

    #[test]
    fn test_security_status_request_creation() {
        let req = SecurityStatusRequest::new("STATREQ123".to_string());

        assert_eq!(req.security_status_req_id, "STATREQ123");
        assert_eq!(req.symbol, None);
    }

    #[test]
    fn test_security_status_request_with_details() {
        let req = SecurityStatusRequest::new("STATREQ456".to_string())
            .with_symbol("TSLA".to_string())
            .with_security_id("88160R101".to_string(), "1".to_string())
            .with_subscription_type('1');

        assert_eq!(req.security_status_req_id, "STATREQ456");
        assert_eq!(req.symbol, Some("TSLA".to_string()));
        assert_eq!(req.security_id, Some("88160R101".to_string()));
        assert_eq!(req.subscription_request_type, Some('1'));
    }

    #[test]
    fn test_security_status_creation() {
        let status = SecurityStatus::new("AAPL".to_string());

        assert_eq!(status.symbol, "AAPL");
        assert_eq!(status.security_trading_status, None);
    }

    #[test]
    fn test_security_status_with_trading_halt() {
        let status = SecurityStatus::new("AAPL".to_string())
            .with_request_id("STATREQ123".to_string())
            .with_trading_status(SecurityTradingStatus::TradingHalt)
            .with_halt_reason(HaltReason::NewsPending)
            .with_transact_time("20250110-14:30:00".to_string());

        assert_eq!(status.symbol, "AAPL");
        assert_eq!(status.security_status_req_id, Some("STATREQ123".to_string()));
        assert_eq!(status.security_trading_status, Some(SecurityTradingStatus::TradingHalt));
        assert_eq!(status.halt_reason, Some(HaltReason::NewsPending));
        assert_eq!(status.transact_time, Some("20250110-14:30:00".to_string()));
    }

    #[test]
    fn test_security_status_resume() {
        let status = SecurityStatus::new("GOOGL".to_string())
            .with_trading_status(SecurityTradingStatus::Resume)
            .with_transact_time("20250110-15:00:00".to_string());

        assert_eq!(status.security_trading_status, Some(SecurityTradingStatus::Resume));
        assert_eq!(status.halt_reason, None);
    }

    #[test]
    fn test_security_type_request_creation() {
        let req = SecurityTypeRequest::new("TYPEREQ123".to_string());

        assert_eq!(req.security_req_id, "TYPEREQ123");
        assert_eq!(req.security_type, None);
    }

    #[test]
    fn test_security_type_request_with_filters() {
        let req = SecurityTypeRequest::new("TYPEREQ456".to_string())
            .with_security_type("OPT".to_string())
            .with_product(1);

        assert_eq!(req.security_req_id, "TYPEREQ456");
        assert_eq!(req.security_type, Some("OPT".to_string()));
        assert_eq!(req.product, Some(1));
    }

    #[test]
    fn test_security_types_creation() {
        let types = SecurityTypes::new(
            "TYPEREQ123".to_string(),
            "TYPERESP789".to_string(),
            SecurityRequestResult::ValidRequest,
        );

        assert_eq!(types.security_req_id, "TYPEREQ123");
        assert_eq!(types.security_response_id, "TYPERESP789");
        assert_eq!(types.security_request_result, SecurityRequestResult::ValidRequest);
    }

    #[test]
    fn test_security_types_with_details() {
        let types = SecurityTypes::new(
            "TYPEREQ123".to_string(),
            "TYPERESP789".to_string(),
            SecurityRequestResult::ValidRequest,
        )
        .with_total_num_types(25)
        .with_last_fragment(true)
        .with_trading_session("REG".to_string());

        assert_eq!(types.total_num_security_types, Some(25));
        assert_eq!(types.last_fragment, Some(true));
        assert_eq!(types.trading_session_id, Some("REG".to_string()));
    }

    #[test]
    fn test_derivative_security_list_request_creation() {
        let req = DerivativeSecurityListRequest::new(
            "DERIVREQ123".to_string(),
            SecurityListRequestType::AllSecurities,
        );

        assert_eq!(req.security_req_id, "DERIVREQ123");
        assert_eq!(req.security_list_request_type, SecurityListRequestType::AllSecurities);
        assert_eq!(req.underlying_symbol, None);
    }

    #[test]
    fn test_derivative_security_list_request_with_underlying() {
        let req = DerivativeSecurityListRequest::new(
            "DERIVREQ456".to_string(),
            SecurityListRequestType::SecurityTypeAndOrCFICode,
        )
        .with_underlying_symbol("SPY".to_string())
        .with_security_type("OPT".to_string())
        .with_product(1);

        assert_eq!(req.security_req_id, "DERIVREQ456");
        assert_eq!(req.underlying_symbol, Some("SPY".to_string()));
        assert_eq!(req.security_type, Some("OPT".to_string()));
        assert_eq!(req.product, Some(1));
    }

    #[test]
    fn test_derivative_security_list_request_with_option_details() {
        let req = DerivativeSecurityListRequest::new(
            "DERIVREQ789".to_string(),
            SecurityListRequestType::SecurityTypeAndOrCFICode,
        )
        .with_underlying_symbol("AAPL".to_string())
        .with_underlying_security_id("037833100".to_string(), "1".to_string())
        .with_security_type("OPT".to_string())
        .with_option_details(150.0, 1) // Call option with strike 150
        .with_maturity("202506".to_string())
        .with_trading_session("REG".to_string(), None);

        assert_eq!(req.underlying_symbol, Some("AAPL".to_string()));
        assert_eq!(req.underlying_security_id, Some("037833100".to_string()));
        assert_eq!(req.underlying_security_id_source, Some("1".to_string()));
        assert_eq!(req.strike_price, Some(150.0));
        assert_eq!(req.put_or_call, Some(1));
        assert_eq!(req.maturity_month_year, Some("202506".to_string()));
        assert_eq!(req.trading_session_id, Some("REG".to_string()));
    }

    #[test]
    fn test_derivative_security_list_creation() {
        let list = DerivativeSecurityList::new(
            "DERIVREQ123".to_string(),
            "DERIVRESP789".to_string(),
            SecurityRequestResult::ValidRequest,
        );

        assert_eq!(list.security_req_id, "DERIVREQ123");
        assert_eq!(list.security_response_id, "DERIVRESP789");
        assert_eq!(list.security_request_result, SecurityRequestResult::ValidRequest);
        assert_eq!(list.underlying_symbol, None);
    }

    #[test]
    fn test_derivative_security_list_with_details() {
        let list = DerivativeSecurityList::new(
            "DERIVREQ123".to_string(),
            "DERIVRESP789".to_string(),
            SecurityRequestResult::ValidRequest,
        )
        .with_underlying_symbol("SPY".to_string())
        .with_underlying_security_id("78462F103".to_string(), "1".to_string())
        .with_total_num_securities(250)
        .with_last_fragment(true)
        .with_security_report_id("DERIVRPT001".to_string())
        .with_clearing_business_date("20250110".to_string())
        .with_security_type("OPT".to_string())
        .with_trading_session("REG".to_string(), Some("REG1".to_string()));

        assert_eq!(list.underlying_symbol, Some("SPY".to_string()));
        assert_eq!(list.underlying_security_id, Some("78462F103".to_string()));
        assert_eq!(list.underlying_security_id_source, Some("1".to_string()));
        assert_eq!(list.total_num_securities, Some(250));
        assert_eq!(list.last_fragment, Some(true));
        assert_eq!(list.security_report_id, Some("DERIVRPT001".to_string()));
        assert_eq!(list.clearing_business_date, Some("20250110".to_string()));
        assert_eq!(list.security_type, Some("OPT".to_string()));
        assert_eq!(list.trading_session_id, Some("REG".to_string()));
        assert_eq!(list.trading_session_sub_id, Some("REG1".to_string()));
    }

    #[test]
    fn test_derivative_security_list_error_response() {
        let list = DerivativeSecurityList::new(
            "DERIVREQ999".to_string(),
            "DERIVRESP999".to_string(),
            SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData,
        );

        assert_eq!(list.security_request_result, SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData);
        assert_eq!(list.total_num_securities, None);
    }

    #[test]
    fn test_security_mass_status_request_creation() {
        let req = SecurityMassStatusRequest::new("MASSREQ123".to_string(), 1);

        assert_eq!(req.security_status_req_id, "MASSREQ123");
        assert_eq!(req.mass_status_req_type, 1);
        assert_eq!(req.security_type, None);
    }

    #[test]
    fn test_security_mass_status_request_with_filters() {
        let req = SecurityMassStatusRequest::new("MASSREQ456".to_string(), 1)
            .with_security_type("CS".to_string())
            .with_product(4)
            .with_trading_session("REG".to_string(), Some("REG1".to_string()))
            .with_market("XNYS".to_string(), Some("MAIN".to_string()))
            .with_currency("USD".to_string());

        assert_eq!(req.security_status_req_id, "MASSREQ456");
        assert_eq!(req.mass_status_req_type, 1);
        assert_eq!(req.security_type, Some("CS".to_string()));
        assert_eq!(req.product, Some(4));
        assert_eq!(req.trading_session_id, Some("REG".to_string()));
        assert_eq!(req.trading_session_sub_id, Some("REG1".to_string()));
        assert_eq!(req.market_id, Some("XNYS".to_string()));
        assert_eq!(req.market_segment_id, Some("MAIN".to_string()));
        assert_eq!(req.currency, Some("USD".to_string()));
    }

    #[test]
    fn test_security_mass_status_request_for_derivatives() {
        let req = SecurityMassStatusRequest::new("MASSREQ789".to_string(), 2)
            .with_underlying_symbol("SPY".to_string())
            .with_security_type("OPT".to_string())
            .with_product(1);

        assert_eq!(req.mass_status_req_type, 2);
        assert_eq!(req.underlying_symbol, Some("SPY".to_string()));
        assert_eq!(req.security_type, Some("OPT".to_string()));
        assert_eq!(req.product, Some(1));
    }

    #[test]
    fn test_security_mass_status_creation() {
        let status = SecurityMassStatus::new(
            "MASSREQ123".to_string(),
            "MASSSTAT789".to_string(),
            SecurityRequestResult::ValidRequest,
        );

        assert_eq!(status.security_status_req_id, "MASSREQ123");
        assert_eq!(status.security_mass_status_req_id, "MASSSTAT789");
        assert_eq!(status.security_request_result, SecurityRequestResult::ValidRequest);
        assert_eq!(status.mass_status_req_type, None);
    }

    #[test]
    fn test_security_mass_status_with_details() {
        let status = SecurityMassStatus::new(
            "MASSREQ123".to_string(),
            "MASSSTAT789".to_string(),
            SecurityRequestResult::ValidRequest,
        )
        .with_mass_status_req_type(1)
        .with_fragment_info(500, true)
        .with_market("XNYS".to_string(), Some("MAIN".to_string()))
        .with_trading_session("REG".to_string(), Some("REG1".to_string()))
        .with_security_type("CS".to_string())
        .with_currency("USD".to_string())
        .with_transact_time("20250110-16:00:00".to_string());

        assert_eq!(status.mass_status_req_type, Some(1));
        assert_eq!(status.total_num_securities, Some(500));
        assert_eq!(status.last_fragment, Some(true));
        assert_eq!(status.market_id, Some("XNYS".to_string()));
        assert_eq!(status.market_segment_id, Some("MAIN".to_string()));
        assert_eq!(status.trading_session_id, Some("REG".to_string()));
        assert_eq!(status.trading_session_sub_id, Some("REG1".to_string()));
        assert_eq!(status.security_type, Some("CS".to_string()));
        assert_eq!(status.currency, Some("USD".to_string()));
        assert_eq!(status.transact_time, Some("20250110-16:00:00".to_string()));
    }

    #[test]
    fn test_security_mass_status_multi_fragment() {
        let status = SecurityMassStatus::new(
            "MASSREQ456".to_string(),
            "MASSSTAT001".to_string(),
            SecurityRequestResult::ValidRequest,
        )
        .with_fragment_info(1000, false);

        assert_eq!(status.total_num_securities, Some(1000));
        assert_eq!(status.last_fragment, Some(false));
    }

    #[test]
    fn test_security_mass_status_error_response() {
        let status = SecurityMassStatus::new(
            "MASSREQ999".to_string(),
            "MASSSTAT999".to_string(),
            SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData,
        );

        assert_eq!(status.security_request_result, SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData);
        assert_eq!(status.total_num_securities, None);
    }
}
