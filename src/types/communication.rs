use serde::{Deserialize, Serialize};


/// EmailType (Tag 94) - Type of email message
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmailType {
    New,        // 0 - New email
    Reply,      // 1 - Reply to previous email
    AdminReply, // 2 - Administrative reply
}

impl EmailType {
    pub fn to_fix(&self) -> char {
        match self {
            EmailType::New => '0',
            EmailType::Reply => '1',
            EmailType::AdminReply => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(EmailType::New),
            '1' => Some(EmailType::Reply),
            '2' => Some(EmailType::AdminReply),
            _ => None,
        }
    }
}


/// NewsRefType (Tag 1477) - Type of news reference
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NewsRefType {
    Replacement,  // 0 - Replacement (cancels original and replaces)
    Cancellation, // 1 - Cancellation (cancels original)
    Supplement,   // 2 - Supplement (adds to original)
}

impl NewsRefType {
    pub fn to_fix(&self) -> char {
        match self {
            NewsRefType::Replacement => '0',
            NewsRefType::Cancellation => '1',
            NewsRefType::Supplement => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(NewsRefType::Replacement),
            '1' => Some(NewsRefType::Cancellation),
            '2' => Some(NewsRefType::Supplement),
            _ => None,
        }
    }
}


/// NewsCategory (Tag 1473) - Category of news
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NewsCategory {
    CompanyNews,     // 0 - Company news
    MarketplaceNews, // 1 - Marketplace news
    FinancialNews,   // 2 - Financial news related to markets
    TechnicalNews,   // 3 - Technical news
    OtherNews,       // 99 - Other news
}

impl NewsCategory {
    pub fn to_fix(&self) -> &'static str {
        match self {
            NewsCategory::CompanyNews => "0",
            NewsCategory::MarketplaceNews => "1",
            NewsCategory::FinancialNews => "2",
            NewsCategory::TechnicalNews => "3",
            NewsCategory::OtherNews => "99",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(NewsCategory::CompanyNews),
            "1" => Some(NewsCategory::MarketplaceNews),
            "2" => Some(NewsCategory::FinancialNews),
            "3" => Some(NewsCategory::TechnicalNews),
            "99" => Some(NewsCategory::OtherNews),
            _ => None,
        }
    }
}
