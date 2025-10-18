use serde::{Deserialize, Serialize};


/// QuoteType (Tag 537) - Type of quote
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuoteType {
    Indicative,          // 0 - Indicative quote
    Tradeable,           // 1 - Tradeable quote
    RestrictedTradeable, // 2 - Restricted tradeable
    Counter,             // 3 - Counter quote
}

impl QuoteType {
    pub fn to_fix(&self) -> char {
        match self {
            QuoteType::Indicative => '0',
            QuoteType::Tradeable => '1',
            QuoteType::RestrictedTradeable => '2',
            QuoteType::Counter => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(QuoteType::Indicative),
            '1' => Some(QuoteType::Tradeable),
            '2' => Some(QuoteType::RestrictedTradeable),
            '3' => Some(QuoteType::Counter),
            _ => None,
        }
    }
}


/// QuoteRequestType (Tag 303) - Type of quote request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuoteRequestType {
    Manual,    // 1 - Manual quote request
    Automatic, // 2 - Automatic quote request
}

impl QuoteRequestType {
    pub fn to_fix(&self) -> char {
        match self {
            QuoteRequestType::Manual => '1',
            QuoteRequestType::Automatic => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(QuoteRequestType::Manual),
            '2' => Some(QuoteRequestType::Automatic),
            _ => None,
        }
    }
}


/// QuoteCancelType (Tag 298) - Type of quote cancellation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuoteCancelType {
    CancelForOneOrMoreSecurities,     // 1
    CancelForSecurityTypes,            // 2
    CancelForUnderlyingSecurity,       // 3
    CancelAllQuotes,                   // 4
    CancelQuoteSpecifiedInQuoteID,     // 5
    CancelByQuoteType,                 // 6
    CancelForSecurityIssuer,           // 7
    CancelForIssuerOfUnderlyingSecurity, // 8
}

impl QuoteCancelType {
    pub fn to_fix(&self) -> char {
        match self {
            QuoteCancelType::CancelForOneOrMoreSecurities => '1',
            QuoteCancelType::CancelForSecurityTypes => '2',
            QuoteCancelType::CancelForUnderlyingSecurity => '3',
            QuoteCancelType::CancelAllQuotes => '4',
            QuoteCancelType::CancelQuoteSpecifiedInQuoteID => '5',
            QuoteCancelType::CancelByQuoteType => '6',
            QuoteCancelType::CancelForSecurityIssuer => '7',
            QuoteCancelType::CancelForIssuerOfUnderlyingSecurity => '8',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(QuoteCancelType::CancelForOneOrMoreSecurities),
            '2' => Some(QuoteCancelType::CancelForSecurityTypes),
            '3' => Some(QuoteCancelType::CancelForUnderlyingSecurity),
            '4' => Some(QuoteCancelType::CancelAllQuotes),
            '5' => Some(QuoteCancelType::CancelQuoteSpecifiedInQuoteID),
            '6' => Some(QuoteCancelType::CancelByQuoteType),
            '7' => Some(QuoteCancelType::CancelForSecurityIssuer),
            '8' => Some(QuoteCancelType::CancelForIssuerOfUnderlyingSecurity),
            _ => None,
        }
    }
}


/// QuoteResponseLevel (Tag 301) - Level of response requested from quote request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuoteResponseLevel {
    NoAcknowledgement,    // 0
    AcknowledgeOnlyNegativeOrErroneous, // 1
    AcknowledgeEachQuote, // 2
    SummaryAcknowledgement, // 3
}

impl QuoteResponseLevel {
    pub fn to_fix(&self) -> char {
        match self {
            QuoteResponseLevel::NoAcknowledgement => '0',
            QuoteResponseLevel::AcknowledgeOnlyNegativeOrErroneous => '1',
            QuoteResponseLevel::AcknowledgeEachQuote => '2',
            QuoteResponseLevel::SummaryAcknowledgement => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(QuoteResponseLevel::NoAcknowledgement),
            '1' => Some(QuoteResponseLevel::AcknowledgeOnlyNegativeOrErroneous),
            '2' => Some(QuoteResponseLevel::AcknowledgeEachQuote),
            '3' => Some(QuoteResponseLevel::SummaryAcknowledgement),
            _ => None,
        }
    }
}


/// QuoteRequestRejectReason (Tag 658) - Reason for rejecting a quote request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuoteRequestRejectReason {
    UnknownSymbol,           // 1
    ExchangeClosed,          // 2
    ExceedsLimit,            // 3
    TooLateToEnter,          // 4
    InvalidPrice,            // 5
    NotAuthorized,           // 6
    NoMatchForInquiry,       // 7
    NoMarketForInstrument,   // 8
    NoInventory,             // 9
    Pass,                    // 10
    InsufficientCredit,      // 11
    Other,                   // 99
}

impl QuoteRequestRejectReason {
    pub fn to_fix(&self) -> &'static str {
        match self {
            QuoteRequestRejectReason::UnknownSymbol => "1",
            QuoteRequestRejectReason::ExchangeClosed => "2",
            QuoteRequestRejectReason::ExceedsLimit => "3",
            QuoteRequestRejectReason::TooLateToEnter => "4",
            QuoteRequestRejectReason::InvalidPrice => "5",
            QuoteRequestRejectReason::NotAuthorized => "6",
            QuoteRequestRejectReason::NoMatchForInquiry => "7",
            QuoteRequestRejectReason::NoMarketForInstrument => "8",
            QuoteRequestRejectReason::NoInventory => "9",
            QuoteRequestRejectReason::Pass => "10",
            QuoteRequestRejectReason::InsufficientCredit => "11",
            QuoteRequestRejectReason::Other => "99",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "1" => Some(QuoteRequestRejectReason::UnknownSymbol),
            "2" => Some(QuoteRequestRejectReason::ExchangeClosed),
            "3" => Some(QuoteRequestRejectReason::ExceedsLimit),
            "4" => Some(QuoteRequestRejectReason::TooLateToEnter),
            "5" => Some(QuoteRequestRejectReason::InvalidPrice),
            "6" => Some(QuoteRequestRejectReason::NotAuthorized),
            "7" => Some(QuoteRequestRejectReason::NoMatchForInquiry),
            "8" => Some(QuoteRequestRejectReason::NoMarketForInstrument),
            "9" => Some(QuoteRequestRejectReason::NoInventory),
            "10" => Some(QuoteRequestRejectReason::Pass),
            "11" => Some(QuoteRequestRejectReason::InsufficientCredit),
            "99" => Some(QuoteRequestRejectReason::Other),
            _ => None,
        }
    }
}


/// QuoteStatus (Tag 297) - Status of quote
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuoteStatus {
    Accepted,                          // 0
    CancelForSymbolDeprecated,         // 1
    CanceledForSecurityTypeDeprecated, // 2
    CanceledForUnderlyingSymbolDeprecated, // 3
    CanceledAllDeprecated,             // 4
    Rejected,                          // 5
    RemovedFromMarket,                 // 6
    Expired,                           // 7
    Query,                             // 8
    QuoteNotFound,                     // 9
    Pending,                           // 10
    Pass,                              // 11
    LockedMarketWarning,               // 12
    CrossMarketWarning,                // 13
    CanceledDueToLockMarket,           // 14
    CanceledDueToCrossMarket,          // 15
    Active,                            // 16
    Canceled,                          // 17
    UnsolicitedQuoteReplenishment,     // 18
    PendingEndTrade,                   // 19
    TooLateToEnd,                      // 20
}

impl QuoteStatus {
    pub fn to_fix(&self) -> &'static str {
        match self {
            QuoteStatus::Accepted => "0",
            QuoteStatus::CancelForSymbolDeprecated => "1",
            QuoteStatus::CanceledForSecurityTypeDeprecated => "2",
            QuoteStatus::CanceledForUnderlyingSymbolDeprecated => "3",
            QuoteStatus::CanceledAllDeprecated => "4",
            QuoteStatus::Rejected => "5",
            QuoteStatus::RemovedFromMarket => "6",
            QuoteStatus::Expired => "7",
            QuoteStatus::Query => "8",
            QuoteStatus::QuoteNotFound => "9",
            QuoteStatus::Pending => "10",
            QuoteStatus::Pass => "11",
            QuoteStatus::LockedMarketWarning => "12",
            QuoteStatus::CrossMarketWarning => "13",
            QuoteStatus::CanceledDueToLockMarket => "14",
            QuoteStatus::CanceledDueToCrossMarket => "15",
            QuoteStatus::Active => "16",
            QuoteStatus::Canceled => "17",
            QuoteStatus::UnsolicitedQuoteReplenishment => "18",
            QuoteStatus::PendingEndTrade => "19",
            QuoteStatus::TooLateToEnd => "20",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(QuoteStatus::Accepted),
            "1" => Some(QuoteStatus::CancelForSymbolDeprecated),
            "2" => Some(QuoteStatus::CanceledForSecurityTypeDeprecated),
            "3" => Some(QuoteStatus::CanceledForUnderlyingSymbolDeprecated),
            "4" => Some(QuoteStatus::CanceledAllDeprecated),
            "5" => Some(QuoteStatus::Rejected),
            "6" => Some(QuoteStatus::RemovedFromMarket),
            "7" => Some(QuoteStatus::Expired),
            "8" => Some(QuoteStatus::Query),
            "9" => Some(QuoteStatus::QuoteNotFound),
            "10" => Some(QuoteStatus::Pending),
            "11" => Some(QuoteStatus::Pass),
            "12" => Some(QuoteStatus::LockedMarketWarning),
            "13" => Some(QuoteStatus::CrossMarketWarning),
            "14" => Some(QuoteStatus::CanceledDueToLockMarket),
            "15" => Some(QuoteStatus::CanceledDueToCrossMarket),
            "16" => Some(QuoteStatus::Active),
            "17" => Some(QuoteStatus::Canceled),
            "18" => Some(QuoteStatus::UnsolicitedQuoteReplenishment),
            "19" => Some(QuoteStatus::PendingEndTrade),
            "20" => Some(QuoteStatus::TooLateToEnd),
            _ => None,
        }
    }
}


/// QuoteCondition (Tag 276) - Condition of quote
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuoteCondition {
    ReservedSAM,              // 0
    NoActiveSAM,              // 1
    Restricted,               // 2
    RestOfBookVWAP,           // 3
    BetterPricesInConditionalOrders, // 4
    MedianPrice,              // 5
    FullCurve,                // 6
    FlatCurve,                // 7
    OpenActive,               // A
    ClosedInactive,           // B
    ExchangeBest,             // C
    ConsolidatedBest,         // D
    Locked,                   // E
    Crossed,                  // F
    Depth,                    // G
    FastTrading,              // H
    NonFirm,                  // I
    OutrightPrice,            // J
    ImpliedPrice,             // K
    ManualSlowQuote,          // L
    DepthOnOffer,             // M
    DepthOnBid,               // N
    Closing,                  // O
    NewsDissemination,        // P
    TradingRange,             // Q
    OrderInflux,              // R
    DueToRelated,             // S
    NewsPending,              // T
    AdditionalInfo,           // U
    AdditionalInfoDueToRelated, // V
    Resume,                   // W
    ViewOfCommon,             // X
    VolumeAlert,              // Y
    OrderImbalance,           // Z
    EquipmentChangeover,      // a
    NoOpenNoResume,           // b
    RegularETH,               // c
    AutomaticExecution,       // d
    AutomaticExecutionETH,    // e
    FastMarketETH,            // f
    InactiveETH,              // g
    Rotation,                 // h
    RotationETH,              // i
    Halt,                     // j
    HaltETH,                  // k
    DueToNewsDissemination,   // l
    DueToNewsPending,         // m
    TradingResume,            // n
    OutOfSequence,            // o
    BidSpecialist,            // p
    OfferSpecialist,          // q
    BidOfferSpecialist,       // r
    EndOfDaySAM,              // s
    ForbiddenSAM,             // t
    FrozenSAM,                // u
    PreOpeningSAM,            // v
    OpeningSAM,               // w
    OpenSAM,                  // x
    SurveillanceSAM,          // y
    SuspendedSAM,             // z
}

impl QuoteCondition {
    pub fn to_fix(&self) -> char {
        match self {
            QuoteCondition::ReservedSAM => '0',
            QuoteCondition::NoActiveSAM => '1',
            QuoteCondition::Restricted => '2',
            QuoteCondition::RestOfBookVWAP => '3',
            QuoteCondition::BetterPricesInConditionalOrders => '4',
            QuoteCondition::MedianPrice => '5',
            QuoteCondition::FullCurve => '6',
            QuoteCondition::FlatCurve => '7',
            QuoteCondition::OpenActive => 'A',
            QuoteCondition::ClosedInactive => 'B',
            QuoteCondition::ExchangeBest => 'C',
            QuoteCondition::ConsolidatedBest => 'D',
            QuoteCondition::Locked => 'E',
            QuoteCondition::Crossed => 'F',
            QuoteCondition::Depth => 'G',
            QuoteCondition::FastTrading => 'H',
            QuoteCondition::NonFirm => 'I',
            QuoteCondition::OutrightPrice => 'J',
            QuoteCondition::ImpliedPrice => 'K',
            QuoteCondition::ManualSlowQuote => 'L',
            QuoteCondition::DepthOnOffer => 'M',
            QuoteCondition::DepthOnBid => 'N',
            QuoteCondition::Closing => 'O',
            QuoteCondition::NewsDissemination => 'P',
            QuoteCondition::TradingRange => 'Q',
            QuoteCondition::OrderInflux => 'R',
            QuoteCondition::DueToRelated => 'S',
            QuoteCondition::NewsPending => 'T',
            QuoteCondition::AdditionalInfo => 'U',
            QuoteCondition::AdditionalInfoDueToRelated => 'V',
            QuoteCondition::Resume => 'W',
            QuoteCondition::ViewOfCommon => 'X',
            QuoteCondition::VolumeAlert => 'Y',
            QuoteCondition::OrderImbalance => 'Z',
            QuoteCondition::EquipmentChangeover => 'a',
            QuoteCondition::NoOpenNoResume => 'b',
            QuoteCondition::RegularETH => 'c',
            QuoteCondition::AutomaticExecution => 'd',
            QuoteCondition::AutomaticExecutionETH => 'e',
            QuoteCondition::FastMarketETH => 'f',
            QuoteCondition::InactiveETH => 'g',
            QuoteCondition::Rotation => 'h',
            QuoteCondition::RotationETH => 'i',
            QuoteCondition::Halt => 'j',
            QuoteCondition::HaltETH => 'k',
            QuoteCondition::DueToNewsDissemination => 'l',
            QuoteCondition::DueToNewsPending => 'm',
            QuoteCondition::TradingResume => 'n',
            QuoteCondition::OutOfSequence => 'o',
            QuoteCondition::BidSpecialist => 'p',
            QuoteCondition::OfferSpecialist => 'q',
            QuoteCondition::BidOfferSpecialist => 'r',
            QuoteCondition::EndOfDaySAM => 's',
            QuoteCondition::ForbiddenSAM => 't',
            QuoteCondition::FrozenSAM => 'u',
            QuoteCondition::PreOpeningSAM => 'v',
            QuoteCondition::OpeningSAM => 'w',
            QuoteCondition::OpenSAM => 'x',
            QuoteCondition::SurveillanceSAM => 'y',
            QuoteCondition::SuspendedSAM => 'z',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(QuoteCondition::ReservedSAM),
            '1' => Some(QuoteCondition::NoActiveSAM),
            '2' => Some(QuoteCondition::Restricted),
            '3' => Some(QuoteCondition::RestOfBookVWAP),
            '4' => Some(QuoteCondition::BetterPricesInConditionalOrders),
            '5' => Some(QuoteCondition::MedianPrice),
            '6' => Some(QuoteCondition::FullCurve),
            '7' => Some(QuoteCondition::FlatCurve),
            'A' => Some(QuoteCondition::OpenActive),
            'B' => Some(QuoteCondition::ClosedInactive),
            'C' => Some(QuoteCondition::ExchangeBest),
            'D' => Some(QuoteCondition::ConsolidatedBest),
            'E' => Some(QuoteCondition::Locked),
            'F' => Some(QuoteCondition::Crossed),
            'G' => Some(QuoteCondition::Depth),
            'H' => Some(QuoteCondition::FastTrading),
            'I' => Some(QuoteCondition::NonFirm),
            'J' => Some(QuoteCondition::OutrightPrice),
            'K' => Some(QuoteCondition::ImpliedPrice),
            'L' => Some(QuoteCondition::ManualSlowQuote),
            'M' => Some(QuoteCondition::DepthOnOffer),
            'N' => Some(QuoteCondition::DepthOnBid),
            'O' => Some(QuoteCondition::Closing),
            'P' => Some(QuoteCondition::NewsDissemination),
            'Q' => Some(QuoteCondition::TradingRange),
            'R' => Some(QuoteCondition::OrderInflux),
            'S' => Some(QuoteCondition::DueToRelated),
            'T' => Some(QuoteCondition::NewsPending),
            'U' => Some(QuoteCondition::AdditionalInfo),
            'V' => Some(QuoteCondition::AdditionalInfoDueToRelated),
            'W' => Some(QuoteCondition::Resume),
            'X' => Some(QuoteCondition::ViewOfCommon),
            'Y' => Some(QuoteCondition::VolumeAlert),
            'Z' => Some(QuoteCondition::OrderImbalance),
            'a' => Some(QuoteCondition::EquipmentChangeover),
            'b' => Some(QuoteCondition::NoOpenNoResume),
            'c' => Some(QuoteCondition::RegularETH),
            'd' => Some(QuoteCondition::AutomaticExecution),
            'e' => Some(QuoteCondition::AutomaticExecutionETH),
            'f' => Some(QuoteCondition::FastMarketETH),
            'g' => Some(QuoteCondition::InactiveETH),
            'h' => Some(QuoteCondition::Rotation),
            'i' => Some(QuoteCondition::RotationETH),
            'j' => Some(QuoteCondition::Halt),
            'k' => Some(QuoteCondition::HaltETH),
            'l' => Some(QuoteCondition::DueToNewsDissemination),
            'm' => Some(QuoteCondition::DueToNewsPending),
            'n' => Some(QuoteCondition::TradingResume),
            'o' => Some(QuoteCondition::OutOfSequence),
            'p' => Some(QuoteCondition::BidSpecialist),
            'q' => Some(QuoteCondition::OfferSpecialist),
            'r' => Some(QuoteCondition::BidOfferSpecialist),
            's' => Some(QuoteCondition::EndOfDaySAM),
            't' => Some(QuoteCondition::ForbiddenSAM),
            'u' => Some(QuoteCondition::FrozenSAM),
            'v' => Some(QuoteCondition::PreOpeningSAM),
            'w' => Some(QuoteCondition::OpeningSAM),
            'x' => Some(QuoteCondition::OpenSAM),
            'y' => Some(QuoteCondition::SurveillanceSAM),
            'z' => Some(QuoteCondition::SuspendedSAM),
            _ => None,
        }
    }
}
