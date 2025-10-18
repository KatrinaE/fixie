use serde::{Deserialize, Serialize};


/// IOITransType (Tag 28) - Type of IOI transaction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IOITransType {
    New,       // N - New IOI
    Cancel,    // C - Cancel IOI
    Replace,   // R - Replace IOI
}

impl IOITransType {
    pub fn to_fix(&self) -> char {
        match self {
            IOITransType::New => 'N',
            IOITransType::Cancel => 'C',
            IOITransType::Replace => 'R',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'N' => Some(IOITransType::New),
            'C' => Some(IOITransType::Cancel),
            'R' => Some(IOITransType::Replace),
            _ => None,
        }
    }
}


/// IOIQltyInd (Tag 25) - Quality indicator for IOI
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IOIQltyInd {
    Low,      // L - Low quality
    Medium,   // M - Medium quality
    High,     // H - High quality
}

impl IOIQltyInd {
    pub fn to_fix(&self) -> char {
        match self {
            IOIQltyInd::Low => 'L',
            IOIQltyInd::Medium => 'M',
            IOIQltyInd::High => 'H',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'L' => Some(IOIQltyInd::Low),
            'M' => Some(IOIQltyInd::Medium),
            'H' => Some(IOIQltyInd::High),
            _ => None,
        }
    }
}


/// AdvSide (Tag 4) - Side of advertisement
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdvSide {
    Buy,   // B - Buy
    Sell,  // S - Sell
    Cross, // X - Cross
    Trade, // T - Trade
}

impl AdvSide {
    pub fn to_fix(&self) -> char {
        match self {
            AdvSide::Buy => 'B',
            AdvSide::Sell => 'S',
            AdvSide::Cross => 'X',
            AdvSide::Trade => 'T',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'B' => Some(AdvSide::Buy),
            'S' => Some(AdvSide::Sell),
            'X' => Some(AdvSide::Cross),
            'T' => Some(AdvSide::Trade),
            _ => None,
        }
    }
}


/// AdvTransType (Tag 5) - Type of advertisement transaction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdvTransType {
    New,     // N - New
    Cancel,  // C - Cancel
    Replace, // R - Replace
}

impl AdvTransType {
    pub fn to_fix(&self) -> char {
        match self {
            AdvTransType::New => 'N',
            AdvTransType::Cancel => 'C',
            AdvTransType::Replace => 'R',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'N' => Some(AdvTransType::New),
            'C' => Some(AdvTransType::Cancel),
            'R' => Some(AdvTransType::Replace),
            _ => None,
        }
    }
}


/// QtyType (Tag 854) - Type of quantity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QtyType {
    Units,      // 0 - Units
    Contracts,  // 1 - Contracts
}

impl QtyType {
    pub fn to_fix(&self) -> char {
        match self {
            QtyType::Units => '0',
            QtyType::Contracts => '1',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(QtyType::Units),
            '1' => Some(QtyType::Contracts),
            _ => None,
        }
    }
}
