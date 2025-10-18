use serde::{Deserialize, Serialize};


/// SecurityRequestType (Tag 321) - Type of security data request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityRequestType {
    RequestSecurityIdentityAndSpecifications,      // 0
    RequestSecurityIdentityForSpecifications,      // 1
    RequestListSecurityTypes,                      // 2
    RequestListSecurities,                          // 3
    Symbol,                                         // 4
    SecurityTypeOrCFICode,                         // 5
    Product,                                        // 6
    TradingSessionID,                              // 7
    AllSecurities,                                 // 8
    MarketIDOrMarketIDPlusMarketSegmentID,         // 9
}

impl SecurityRequestType {
    pub fn to_fix(&self) -> char {
        match self {
            SecurityRequestType::RequestSecurityIdentityAndSpecifications => '0',
            SecurityRequestType::RequestSecurityIdentityForSpecifications => '1',
            SecurityRequestType::RequestListSecurityTypes => '2',
            SecurityRequestType::RequestListSecurities => '3',
            SecurityRequestType::Symbol => '4',
            SecurityRequestType::SecurityTypeOrCFICode => '5',
            SecurityRequestType::Product => '6',
            SecurityRequestType::TradingSessionID => '7',
            SecurityRequestType::AllSecurities => '8',
            SecurityRequestType::MarketIDOrMarketIDPlusMarketSegmentID => '9',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(SecurityRequestType::RequestSecurityIdentityAndSpecifications),
            '1' => Some(SecurityRequestType::RequestSecurityIdentityForSpecifications),
            '2' => Some(SecurityRequestType::RequestListSecurityTypes),
            '3' => Some(SecurityRequestType::RequestListSecurities),
            '4' => Some(SecurityRequestType::Symbol),
            '5' => Some(SecurityRequestType::SecurityTypeOrCFICode),
            '6' => Some(SecurityRequestType::Product),
            '7' => Some(SecurityRequestType::TradingSessionID),
            '8' => Some(SecurityRequestType::AllSecurities),
            '9' => Some(SecurityRequestType::MarketIDOrMarketIDPlusMarketSegmentID),
            _ => None,
        }
    }
}


/// SecurityRequestResult (Tag 560) - Result of security definition request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityRequestResult {
    ValidRequest,                        // 0
    InvalidOrUnsupportedRequest,         // 1
    NoInstrumentsFound,                  // 2
    NotAuthorizedToRetrieveInstrumentData, // 3
    InstrumentDataTemporarilyUnavailable, // 4
    RequestForInstrumentDataNotSupported, // 5
}

impl SecurityRequestResult {
    pub fn to_fix(&self) -> char {
        match self {
            SecurityRequestResult::ValidRequest => '0',
            SecurityRequestResult::InvalidOrUnsupportedRequest => '1',
            SecurityRequestResult::NoInstrumentsFound => '2',
            SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData => '3',
            SecurityRequestResult::InstrumentDataTemporarilyUnavailable => '4',
            SecurityRequestResult::RequestForInstrumentDataNotSupported => '5',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(SecurityRequestResult::ValidRequest),
            '1' => Some(SecurityRequestResult::InvalidOrUnsupportedRequest),
            '2' => Some(SecurityRequestResult::NoInstrumentsFound),
            '3' => Some(SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData),
            '4' => Some(SecurityRequestResult::InstrumentDataTemporarilyUnavailable),
            '5' => Some(SecurityRequestResult::RequestForInstrumentDataNotSupported),
            _ => None,
        }
    }
}


/// SecurityListRequestType (Tag 559) - Type of security list request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityListRequestType {
    Symbol,                              // 0
    SecurityTypeAndOrCFICode,           // 1
    Product,                             // 2
    TradingSessionID,                    // 3
    AllSecurities,                       // 4
    MarketIDOrMarketIDPlusMarketSegmentID, // 5
}

impl SecurityListRequestType {
    pub fn to_fix(&self) -> char {
        match self {
            SecurityListRequestType::Symbol => '0',
            SecurityListRequestType::SecurityTypeAndOrCFICode => '1',
            SecurityListRequestType::Product => '2',
            SecurityListRequestType::TradingSessionID => '3',
            SecurityListRequestType::AllSecurities => '4',
            SecurityListRequestType::MarketIDOrMarketIDPlusMarketSegmentID => '5',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(SecurityListRequestType::Symbol),
            '1' => Some(SecurityListRequestType::SecurityTypeAndOrCFICode),
            '2' => Some(SecurityListRequestType::Product),
            '3' => Some(SecurityListRequestType::TradingSessionID),
            '4' => Some(SecurityListRequestType::AllSecurities),
            '5' => Some(SecurityListRequestType::MarketIDOrMarketIDPlusMarketSegmentID),
            _ => None,
        }
    }
}


/// SecurityUpdateAction (Tag 980) - Action taken for security data
///
/// Uses same values as TradSesUpdateAction (Tag 1327) and MarketUpdateAction (Tag 1395)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityUpdateAction {
    Add,     // A - Add security
    Delete,  // D - Delete security
    Modify,  // M - Modify security
}

impl SecurityUpdateAction {
    pub fn to_fix(&self) -> char {
        match self {
            SecurityUpdateAction::Add => 'A',
            SecurityUpdateAction::Delete => 'D',
            SecurityUpdateAction::Modify => 'M',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'A' => Some(SecurityUpdateAction::Add),
            'D' => Some(SecurityUpdateAction::Delete),
            'M' => Some(SecurityUpdateAction::Modify),
            _ => None,
        }
    }
}


/// SecurityTradingStatus (Tag 326) - Trading status of a security
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityTradingStatus {
    OpeningDelay,                          // 1
    TradingHalt,                           // 2
    Resume,                                // 3
    NoOpenNoResume,                        // 4
    PriceIndication,                       // 5
    TradingRangeIndication,                // 6
    MarketImbalanceBuy,                    // 7
    MarketImbalanceSell,                   // 8
    MarketOnCloseImbalanceBuy,             // 9
    MarketOnCloseImbalanceSell,            // 10
    NoMarketImbalance,                     // 12
    NoMarketOnCloseImbalance,              // 13
    ITSPreOpening,                         // 14
    NewPriceIndication,                    // 15
    TradeDisseminationTime,                // 16
    ReadyToTrade,                          // 17
    NotAvailableForTrading,                // 18
    NotTradedOnThisMarket,                 // 19
    UnknownOrInvalid,                      // 20
    PreOpen,                               // 21
    OpeningRotation,                       // 22
    FastMarket,                            // 23
    PreCross,                              // 24
    Cross,                                 // 25
    PostClose,                             // 26
}

impl SecurityTradingStatus {
    pub fn to_fix(&self) -> &'static str {
        match self {
            SecurityTradingStatus::OpeningDelay => "1",
            SecurityTradingStatus::TradingHalt => "2",
            SecurityTradingStatus::Resume => "3",
            SecurityTradingStatus::NoOpenNoResume => "4",
            SecurityTradingStatus::PriceIndication => "5",
            SecurityTradingStatus::TradingRangeIndication => "6",
            SecurityTradingStatus::MarketImbalanceBuy => "7",
            SecurityTradingStatus::MarketImbalanceSell => "8",
            SecurityTradingStatus::MarketOnCloseImbalanceBuy => "9",
            SecurityTradingStatus::MarketOnCloseImbalanceSell => "10",
            SecurityTradingStatus::NoMarketImbalance => "12",
            SecurityTradingStatus::NoMarketOnCloseImbalance => "13",
            SecurityTradingStatus::ITSPreOpening => "14",
            SecurityTradingStatus::NewPriceIndication => "15",
            SecurityTradingStatus::TradeDisseminationTime => "16",
            SecurityTradingStatus::ReadyToTrade => "17",
            SecurityTradingStatus::NotAvailableForTrading => "18",
            SecurityTradingStatus::NotTradedOnThisMarket => "19",
            SecurityTradingStatus::UnknownOrInvalid => "20",
            SecurityTradingStatus::PreOpen => "21",
            SecurityTradingStatus::OpeningRotation => "22",
            SecurityTradingStatus::FastMarket => "23",
            SecurityTradingStatus::PreCross => "24",
            SecurityTradingStatus::Cross => "25",
            SecurityTradingStatus::PostClose => "26",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "1" => Some(SecurityTradingStatus::OpeningDelay),
            "2" => Some(SecurityTradingStatus::TradingHalt),
            "3" => Some(SecurityTradingStatus::Resume),
            "4" => Some(SecurityTradingStatus::NoOpenNoResume),
            "5" => Some(SecurityTradingStatus::PriceIndication),
            "6" => Some(SecurityTradingStatus::TradingRangeIndication),
            "7" => Some(SecurityTradingStatus::MarketImbalanceBuy),
            "8" => Some(SecurityTradingStatus::MarketImbalanceSell),
            "9" => Some(SecurityTradingStatus::MarketOnCloseImbalanceBuy),
            "10" => Some(SecurityTradingStatus::MarketOnCloseImbalanceSell),
            "12" => Some(SecurityTradingStatus::NoMarketImbalance),
            "13" => Some(SecurityTradingStatus::NoMarketOnCloseImbalance),
            "14" => Some(SecurityTradingStatus::ITSPreOpening),
            "15" => Some(SecurityTradingStatus::NewPriceIndication),
            "16" => Some(SecurityTradingStatus::TradeDisseminationTime),
            "17" => Some(SecurityTradingStatus::ReadyToTrade),
            "18" => Some(SecurityTradingStatus::NotAvailableForTrading),
            "19" => Some(SecurityTradingStatus::NotTradedOnThisMarket),
            "20" => Some(SecurityTradingStatus::UnknownOrInvalid),
            "21" => Some(SecurityTradingStatus::PreOpen),
            "22" => Some(SecurityTradingStatus::OpeningRotation),
            "23" => Some(SecurityTradingStatus::FastMarket),
            "24" => Some(SecurityTradingStatus::PreCross),
            "25" => Some(SecurityTradingStatus::Cross),
            "26" => Some(SecurityTradingStatus::PostClose),
            _ => None,
        }
    }
}


/// HaltReason (Tag 327) - Reason for trading halt
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum HaltReason {
    NewsDissemination,                     // 0
    OrderInflux,                           // 1
    OrderImbalance,                        // 2
    AdditionalInformation,                 // 3
    NewsPending,                           // 4
    EquipmentChangeover,                   // 5
}

impl HaltReason {
    pub fn to_fix(&self) -> char {
        match self {
            HaltReason::NewsDissemination => '0',
            HaltReason::OrderInflux => '1',
            HaltReason::OrderImbalance => '2',
            HaltReason::AdditionalInformation => '3',
            HaltReason::NewsPending => '4',
            HaltReason::EquipmentChangeover => '5',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(HaltReason::NewsDissemination),
            '1' => Some(HaltReason::OrderInflux),
            '2' => Some(HaltReason::OrderImbalance),
            '3' => Some(HaltReason::AdditionalInformation),
            '4' => Some(HaltReason::NewsPending),
            '5' => Some(HaltReason::EquipmentChangeover),
            _ => None,
        }
    }
}
