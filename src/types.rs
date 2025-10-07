use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub fn as_str(&self) -> &str {
        match self {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        }
    }

    pub fn to_fix(&self) -> char {
        match self {
            Side::Buy => '1',  // FIX: 1 = Buy
            Side::Sell => '2', // FIX: 2 = Sell
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(Side::Buy),
            '2' => Some(Side::Sell),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrdType {
    Market,
    Limit,
}

impl OrdType {
    pub fn as_str(&self) -> &str {
        match self {
            OrdType::Market => "MARKET",
            OrdType::Limit => "LIMIT",
        }
    }

    pub fn to_fix(&self) -> char {
        match self {
            OrdType::Market => '1', // FIX: 1 = Market
            OrdType::Limit => '2',  // FIX: 2 = Limit
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(OrdType::Market),
            '2' => Some(OrdType::Limit),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrdStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

impl OrdStatus {
    pub fn to_fix(&self) -> char {
        match self {
            OrdStatus::New => '0',
            OrdStatus::PartiallyFilled => '1',
            OrdStatus::Filled => '2',
            OrdStatus::Cancelled => '4',
            OrdStatus::Rejected => '8',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(OrdStatus::New),
            '1' => Some(OrdStatus::PartiallyFilled),
            '2' => Some(OrdStatus::Filled),
            '4' => Some(OrdStatus::Cancelled),
            '8' => Some(OrdStatus::Rejected),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecType {
    New,
    PartialFill,
    Fill,
    Cancelled,
    Rejected,
}

impl ExecType {
    pub fn to_fix(&self) -> char {
        match self {
            ExecType::New => '0',
            ExecType::PartialFill => '1',
            ExecType::Fill => '2',
            ExecType::Cancelled => '4',
            ExecType::Rejected => '8',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(ExecType::New),
            '1' => Some(ExecType::PartialFill),
            '2' => Some(ExecType::Fill),
            '4' => Some(ExecType::Cancelled),
            '8' => Some(ExecType::Rejected),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DKReason {
    UnknownSymbol,           // A
    WrongSide,               // B
    QuantityExceedsOrder,    // C
    NoMatchingOrder,         // D
    PriceExceedsLimit,       // E
    CalculationDifference,   // F
    Other,                   // Z
}

impl DKReason {
    pub fn to_fix(&self) -> char {
        match self {
            DKReason::UnknownSymbol => 'A',
            DKReason::WrongSide => 'B',
            DKReason::QuantityExceedsOrder => 'C',
            DKReason::NoMatchingOrder => 'D',
            DKReason::PriceExceedsLimit => 'E',
            DKReason::CalculationDifference => 'F',
            DKReason::Other => 'Z',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'A' => Some(DKReason::UnknownSymbol),
            'B' => Some(DKReason::WrongSide),
            'C' => Some(DKReason::QuantityExceedsOrder),
            'D' => Some(DKReason::NoMatchingOrder),
            'E' => Some(DKReason::PriceExceedsLimit),
            'F' => Some(DKReason::CalculationDifference),
            'Z' => Some(DKReason::Other),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecAckStatus {
    Received,   // 0
    Accepted,   // 1
    DontKnow,   // 2 (rejected/DK'd)
}

impl ExecAckStatus {
    pub fn to_fix(&self) -> char {
        match self {
            ExecAckStatus::Received => '0',
            ExecAckStatus::Accepted => '1',
            ExecAckStatus::DontKnow => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(ExecAckStatus::Received),
            '1' => Some(ExecAckStatus::Accepted),
            '2' => Some(ExecAckStatus::DontKnow),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MassCancelRequestType {
    CancelOrdersForSecurity,          // 1
    CancelOrdersForUnderlyingSecurity, // 2
    CancelOrdersForProduct,           // 3
    CancelOrdersForCFICode,           // 4
    CancelOrdersForSecurityType,      // 5
    CancelOrdersForTradingSession,    // 6
    CancelAllOrders,                  // 7
}

impl MassCancelRequestType {
    pub fn to_fix(&self) -> char {
        match self {
            MassCancelRequestType::CancelOrdersForSecurity => '1',
            MassCancelRequestType::CancelOrdersForUnderlyingSecurity => '2',
            MassCancelRequestType::CancelOrdersForProduct => '3',
            MassCancelRequestType::CancelOrdersForCFICode => '4',
            MassCancelRequestType::CancelOrdersForSecurityType => '5',
            MassCancelRequestType::CancelOrdersForTradingSession => '6',
            MassCancelRequestType::CancelAllOrders => '7',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(MassCancelRequestType::CancelOrdersForSecurity),
            '2' => Some(MassCancelRequestType::CancelOrdersForUnderlyingSecurity),
            '3' => Some(MassCancelRequestType::CancelOrdersForProduct),
            '4' => Some(MassCancelRequestType::CancelOrdersForCFICode),
            '5' => Some(MassCancelRequestType::CancelOrdersForSecurityType),
            '6' => Some(MassCancelRequestType::CancelOrdersForTradingSession),
            '7' => Some(MassCancelRequestType::CancelAllOrders),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MassCancelResponse {
    CancelRequestRejected,                 // 0
    CancelOrdersForSecurity,               // 1
    CancelOrdersForUnderlyingSecurity,     // 2
    CancelOrdersForProduct,                // 3
    CancelOrdersForCFICode,                // 4
    CancelOrdersForSecurityType,           // 5
    CancelOrdersForTradingSession,         // 6
    CancelAllOrders,                       // 7
}

impl MassCancelResponse {
    pub fn to_fix(&self) -> char {
        match self {
            MassCancelResponse::CancelRequestRejected => '0',
            MassCancelResponse::CancelOrdersForSecurity => '1',
            MassCancelResponse::CancelOrdersForUnderlyingSecurity => '2',
            MassCancelResponse::CancelOrdersForProduct => '3',
            MassCancelResponse::CancelOrdersForCFICode => '4',
            MassCancelResponse::CancelOrdersForSecurityType => '5',
            MassCancelResponse::CancelOrdersForTradingSession => '6',
            MassCancelResponse::CancelAllOrders => '7',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(MassCancelResponse::CancelRequestRejected),
            '1' => Some(MassCancelResponse::CancelOrdersForSecurity),
            '2' => Some(MassCancelResponse::CancelOrdersForUnderlyingSecurity),
            '3' => Some(MassCancelResponse::CancelOrdersForProduct),
            '4' => Some(MassCancelResponse::CancelOrdersForCFICode),
            '5' => Some(MassCancelResponse::CancelOrdersForSecurityType),
            '6' => Some(MassCancelResponse::CancelOrdersForTradingSession),
            '7' => Some(MassCancelResponse::CancelAllOrders),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CrossType {
    CrossAON,       // 1 - Cross AON (All or None)
    CrossIOC,       // 2 - Cross IOC (Immediate or Cancel)
    CrossOneSide,   // 3 - Cross One Side
    CrossSamePrice, // 4 - Cross Same Price
}

impl CrossType {
    pub fn to_fix(&self) -> char {
        match self {
            CrossType::CrossAON => '1',
            CrossType::CrossIOC => '2',
            CrossType::CrossOneSide => '3',
            CrossType::CrossSamePrice => '4',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(CrossType::CrossAON),
            '2' => Some(CrossType::CrossIOC),
            '3' => Some(CrossType::CrossOneSide),
            '4' => Some(CrossType::CrossSamePrice),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CrossPrioritization {
    None,             // 0 - No prioritization
    BuySidePriority,  // 1 - Buy side is prioritized
    SellSidePriority, // 2 - Sell side is prioritized
}

impl CrossPrioritization {
    pub fn to_fix(&self) -> char {
        match self {
            CrossPrioritization::None => '0',
            CrossPrioritization::BuySidePriority => '1',
            CrossPrioritization::SellSidePriority => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(CrossPrioritization::None),
            '1' => Some(CrossPrioritization::BuySidePriority),
            '2' => Some(CrossPrioritization::SellSidePriority),
            _ => None,
        }
    }
}
