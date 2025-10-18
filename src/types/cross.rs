use serde::{Deserialize, Serialize};


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
