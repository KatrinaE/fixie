use serde::{Deserialize, Serialize};


/// MDReqRejReason (Tag 281) - Reason for rejecting a Market Data Request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MDReqRejReason {
    UnknownSymbol,                  // 0 - Unknown symbol
    DuplicateMDReqID,               // 1 - Duplicate MDReqID
    InsufficientBandwidth,          // 2 - Insufficient bandwidth
    InsufficientPermissions,        // 3 - Insufficient permissions
    UnsupportedSubscriptionRequestType, // 4 - Unsupported SubscriptionRequestType
    UnsupportedMarketDepth,         // 5 - Unsupported MarketDepth
    UnsupportedMDUpdateType,        // 6 - Unsupported MDUpdateType
    UnsupportedAggregatedBook,      // 7 - Unsupported AggregatedBook
    UnsupportedMDEntryType,         // 8 - Unsupported MDEntryType
    UnsupportedTradingSessionID,    // 9 - Unsupported TradingSessionID
    UnsupportedScope,               // A - Unsupported Scope
    UnsupportedOpenCloseSettleFlag, // B - Unsupported OpenCloseSettleFlag
    UnsupportedMDImplicitDelete,    // C - Unsupported MDImplicitDelete
}

impl MDReqRejReason {
    pub fn to_fix(&self) -> &'static str {
        match self {
            MDReqRejReason::UnknownSymbol => "0",
            MDReqRejReason::DuplicateMDReqID => "1",
            MDReqRejReason::InsufficientBandwidth => "2",
            MDReqRejReason::InsufficientPermissions => "3",
            MDReqRejReason::UnsupportedSubscriptionRequestType => "4",
            MDReqRejReason::UnsupportedMarketDepth => "5",
            MDReqRejReason::UnsupportedMDUpdateType => "6",
            MDReqRejReason::UnsupportedAggregatedBook => "7",
            MDReqRejReason::UnsupportedMDEntryType => "8",
            MDReqRejReason::UnsupportedTradingSessionID => "9",
            MDReqRejReason::UnsupportedScope => "A",
            MDReqRejReason::UnsupportedOpenCloseSettleFlag => "B",
            MDReqRejReason::UnsupportedMDImplicitDelete => "C",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(MDReqRejReason::UnknownSymbol),
            "1" => Some(MDReqRejReason::DuplicateMDReqID),
            "2" => Some(MDReqRejReason::InsufficientBandwidth),
            "3" => Some(MDReqRejReason::InsufficientPermissions),
            "4" => Some(MDReqRejReason::UnsupportedSubscriptionRequestType),
            "5" => Some(MDReqRejReason::UnsupportedMarketDepth),
            "6" => Some(MDReqRejReason::UnsupportedMDUpdateType),
            "7" => Some(MDReqRejReason::UnsupportedAggregatedBook),
            "8" => Some(MDReqRejReason::UnsupportedMDEntryType),
            "9" => Some(MDReqRejReason::UnsupportedTradingSessionID),
            "A" => Some(MDReqRejReason::UnsupportedScope),
            "B" => Some(MDReqRejReason::UnsupportedOpenCloseSettleFlag),
            "C" => Some(MDReqRejReason::UnsupportedMDImplicitDelete),
            _ => None,
        }
    }
}


/// MDUpdateType (Tag 265) - Type of market data update
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MDUpdateType {
    FullRefresh,       // 0 - Full refresh
    IncrementalRefresh, // 1 - Incremental refresh
}

impl MDUpdateType {
    pub fn to_fix(&self) -> char {
        match self {
            MDUpdateType::FullRefresh => '0',
            MDUpdateType::IncrementalRefresh => '1',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(MDUpdateType::FullRefresh),
            '1' => Some(MDUpdateType::IncrementalRefresh),
            _ => None,
        }
    }
}


/// SubscriptionRequestType (Tag 263) - Type of subscription request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionRequestType {
    Snapshot,                        // 0 - Snapshot
    SnapshotPlusUpdates,             // 1 - Snapshot + Updates (Subscribe)
    DisablePreviousSnapshot,         // 2 - Disable previous Snapshot + Updates (Unsubscribe)
}

impl SubscriptionRequestType {
    pub fn to_fix(&self) -> char {
        match self {
            SubscriptionRequestType::Snapshot => '0',
            SubscriptionRequestType::SnapshotPlusUpdates => '1',
            SubscriptionRequestType::DisablePreviousSnapshot => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(SubscriptionRequestType::Snapshot),
            '1' => Some(SubscriptionRequestType::SnapshotPlusUpdates),
            '2' => Some(SubscriptionRequestType::DisablePreviousSnapshot),
            _ => None,
        }
    }
}


/// MDEntryType (Tag 269) - Type of Market Data entry
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MDEntryType {
    Bid,                  // 0
    Offer,                // 1
    Trade,                // 2
    IndexValue,           // 3
    OpeningPrice,         // 4
    ClosingPrice,         // 5
    SettlementPrice,      // 6
    TradingSessionHighPrice, // 7
    TradingSessionLowPrice,  // 8
    TradingSessionVWAPPrice, // 9
    Imbalance,            // A
    TradeVolume,          // B
    OpenInterest,         // C
}

impl MDEntryType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            MDEntryType::Bid => "0",
            MDEntryType::Offer => "1",
            MDEntryType::Trade => "2",
            MDEntryType::IndexValue => "3",
            MDEntryType::OpeningPrice => "4",
            MDEntryType::ClosingPrice => "5",
            MDEntryType::SettlementPrice => "6",
            MDEntryType::TradingSessionHighPrice => "7",
            MDEntryType::TradingSessionLowPrice => "8",
            MDEntryType::TradingSessionVWAPPrice => "9",
            MDEntryType::Imbalance => "A",
            MDEntryType::TradeVolume => "B",
            MDEntryType::OpenInterest => "C",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(MDEntryType::Bid),
            "1" => Some(MDEntryType::Offer),
            "2" => Some(MDEntryType::Trade),
            "3" => Some(MDEntryType::IndexValue),
            "4" => Some(MDEntryType::OpeningPrice),
            "5" => Some(MDEntryType::ClosingPrice),
            "6" => Some(MDEntryType::SettlementPrice),
            "7" => Some(MDEntryType::TradingSessionHighPrice),
            "8" => Some(MDEntryType::TradingSessionLowPrice),
            "9" => Some(MDEntryType::TradingSessionVWAPPrice),
            "A" => Some(MDEntryType::Imbalance),
            "B" => Some(MDEntryType::TradeVolume),
            "C" => Some(MDEntryType::OpenInterest),
            _ => None,
        }
    }
}
