use serde::{Deserialize, Serialize};


/// TradSesStatus (Tag 340) - State of the trading session
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradSesStatus {
    Unknown,       // 0 - Unknown status
    Halted,        // 1 - Halted
    Open,          // 2 - Open
    Closed,        // 3 - Closed
    PreOpen,       // 4 - Pre-Open
    PreClose,      // 5 - Pre-Close
    RequestRejected, // 6 - Request Rejected
}

impl TradSesStatus {
    pub fn to_fix(&self) -> char {
        match self {
            TradSesStatus::Unknown => '0',
            TradSesStatus::Halted => '1',
            TradSesStatus::Open => '2',
            TradSesStatus::Closed => '3',
            TradSesStatus::PreOpen => '4',
            TradSesStatus::PreClose => '5',
            TradSesStatus::RequestRejected => '6',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(TradSesStatus::Unknown),
            '1' => Some(TradSesStatus::Halted),
            '2' => Some(TradSesStatus::Open),
            '3' => Some(TradSesStatus::Closed),
            '4' => Some(TradSesStatus::PreOpen),
            '5' => Some(TradSesStatus::PreClose),
            '6' => Some(TradSesStatus::RequestRejected),
            _ => None,
        }
    }
}


/// TradSesMethod (Tag 338) - Method of trading
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradSesMethod {
    Electronic,   // 1 - Electronic trading
    OpenOutcry,   // 2 - Open outcry
    TwoParty,     // 3 - Two party (direct negotiation)
}

impl TradSesMethod {
    pub fn to_fix(&self) -> char {
        match self {
            TradSesMethod::Electronic => '1',
            TradSesMethod::OpenOutcry => '2',
            TradSesMethod::TwoParty => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(TradSesMethod::Electronic),
            '2' => Some(TradSesMethod::OpenOutcry),
            '3' => Some(TradSesMethod::TwoParty),
            _ => None,
        }
    }
}


/// TradSesMode (Tag 339) - Trading session mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradSesMode {
    Testing,     // 1 - Testing mode
    Simulated,   // 2 - Simulated trading
    Production,  // 3 - Production (live) trading
}

impl TradSesMode {
    pub fn to_fix(&self) -> char {
        match self {
            TradSesMode::Testing => '1',
            TradSesMode::Simulated => '2',
            TradSesMode::Production => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(TradSesMode::Testing),
            '2' => Some(TradSesMode::Simulated),
            '3' => Some(TradSesMode::Production),
            _ => None,
        }
    }
}


/// TradSesStatusRejReason (Tag 567) - Reason for rejecting a trading session status request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradSesStatusRejReason {
    UnknownOrInvalidTradingSessionID, // 1 - Unknown or invalid TradingSessionID
    Other,                             // 99 - Other reason
}

impl TradSesStatusRejReason {
    pub fn to_fix(&self) -> &'static str {
        match self {
            TradSesStatusRejReason::UnknownOrInvalidTradingSessionID => "1",
            TradSesStatusRejReason::Other => "99",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "1" => Some(TradSesStatusRejReason::UnknownOrInvalidTradingSessionID),
            "99" => Some(TradSesStatusRejReason::Other),
            _ => None,
        }
    }
}


/// TradSesUpdateAction (Tag 1327) - Action taken for trading sessions
///
/// Uses same values as SecurityUpdateAction (Tag 980)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradSesUpdateAction {
    Add,     // A - Add trading session
    Delete,  // D - Delete trading session
    Modify,  // M - Modify trading session
}

impl TradSesUpdateAction {
    pub fn to_fix(&self) -> char {
        match self {
            TradSesUpdateAction::Add => 'A',
            TradSesUpdateAction::Delete => 'D',
            TradSesUpdateAction::Modify => 'M',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'A' => Some(TradSesUpdateAction::Add),
            'D' => Some(TradSesUpdateAction::Delete),
            'M' => Some(TradSesUpdateAction::Modify),
            _ => None,
        }
    }
}


/// MarketUpdateAction (Tag 1395) - Action taken for market definitions
///
/// Uses same values as SecurityUpdateAction (Tag 980) and TradSesUpdateAction (Tag 1327)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MarketUpdateAction {
    Add,     // A - Add market definition
    Delete,  // D - Delete market definition
    Modify,  // M - Modify market definition
}

impl MarketUpdateAction {
    pub fn to_fix(&self) -> char {
        match self {
            MarketUpdateAction::Add => 'A',
            MarketUpdateAction::Delete => 'D',
            MarketUpdateAction::Modify => 'M',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'A' => Some(MarketUpdateAction::Add),
            'D' => Some(MarketUpdateAction::Delete),
            'M' => Some(MarketUpdateAction::Modify),
            _ => None,
        }
    }
}
