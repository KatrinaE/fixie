use serde::{Deserialize, Serialize};


/// BidType (Tag 394) - Type of bid request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BidType {
    NonDisclosed,      // 1 - No bid details provided upfront
    Disclosed,         // 2 - Specific orders disclosed
    NoBiddingProcess,  // 3 - No competitive bidding
}

impl BidType {
    pub fn to_fix(&self) -> char {
        match self {
            BidType::NonDisclosed => '1',
            BidType::Disclosed => '2',
            BidType::NoBiddingProcess => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(BidType::NonDisclosed),
            '2' => Some(BidType::Disclosed),
            '3' => Some(BidType::NoBiddingProcess),
            _ => None,
        }
    }
}


/// ProgRptReqs (Tag 414) - Program reporting requirements
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProgRptReqs {
    BuySideRequests,     // 1
    SellSideRequests,    // 2
    RealTimeExecutions,  // 3
}

impl ProgRptReqs {
    pub fn to_fix(&self) -> char {
        match self {
            ProgRptReqs::BuySideRequests => '1',
            ProgRptReqs::SellSideRequests => '2',
            ProgRptReqs::RealTimeExecutions => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(ProgRptReqs::BuySideRequests),
            '2' => Some(ProgRptReqs::SellSideRequests),
            '3' => Some(ProgRptReqs::RealTimeExecutions),
            _ => None,
        }
    }
}


/// ListExecInstType (Tag 433) - List execution instruction type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListExecInstType {
    Immediate,              // 1
    WaitForInstruction,     // 2
    ExchangeSwitchCIVOrder, // 3
    SellDriven,             // 4
    BuyDrivenCash,          // 5
}

impl ListExecInstType {
    pub fn to_fix(&self) -> char {
        match self {
            ListExecInstType::Immediate => '1',
            ListExecInstType::WaitForInstruction => '2',
            ListExecInstType::ExchangeSwitchCIVOrder => '3',
            ListExecInstType::SellDriven => '4',
            ListExecInstType::BuyDrivenCash => '5',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(ListExecInstType::Immediate),
            '2' => Some(ListExecInstType::WaitForInstruction),
            '3' => Some(ListExecInstType::ExchangeSwitchCIVOrder),
            '4' => Some(ListExecInstType::SellDriven),
            '5' => Some(ListExecInstType::BuyDrivenCash),
            _ => None,
        }
    }
}


/// ListStatusType (Tag 429) - Type of list status message
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListStatusType {
    Ack,         // 1
    Response,    // 2
    Timed,       // 3
    ExecStarted, // 4
    AllDone,     // 5
    Alert,       // 6
}

impl ListStatusType {
    pub fn to_fix(&self) -> char {
        match self {
            ListStatusType::Ack => '1',
            ListStatusType::Response => '2',
            ListStatusType::Timed => '3',
            ListStatusType::ExecStarted => '4',
            ListStatusType::AllDone => '5',
            ListStatusType::Alert => '6',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(ListStatusType::Ack),
            '2' => Some(ListStatusType::Response),
            '3' => Some(ListStatusType::Timed),
            '4' => Some(ListStatusType::ExecStarted),
            '5' => Some(ListStatusType::AllDone),
            '6' => Some(ListStatusType::Alert),
            _ => None,
        }
    }
}


/// ListOrderStatus (Tag 431) - Status of list order
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListOrderStatus {
    InBiddingProcess,   // 1
    ReceivedForExecution, // 2
    Executing,          // 3
    Cancelling,         // 4
    Alert,              // 5
    AllDone,            // 6
    Reject,             // 7
}

impl ListOrderStatus {
    pub fn to_fix(&self) -> char {
        match self {
            ListOrderStatus::InBiddingProcess => '1',
            ListOrderStatus::ReceivedForExecution => '2',
            ListOrderStatus::Executing => '3',
            ListOrderStatus::Cancelling => '4',
            ListOrderStatus::Alert => '5',
            ListOrderStatus::AllDone => '6',
            ListOrderStatus::Reject => '7',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(ListOrderStatus::InBiddingProcess),
            '2' => Some(ListOrderStatus::ReceivedForExecution),
            '3' => Some(ListOrderStatus::Executing),
            '4' => Some(ListOrderStatus::Cancelling),
            '5' => Some(ListOrderStatus::Alert),
            '6' => Some(ListOrderStatus::AllDone),
            '7' => Some(ListOrderStatus::Reject),
            _ => None,
        }
    }
}


/// BidDescriptorType (Tag 399) - Type of bid descriptor
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BidDescriptorType {
    Sector,   // 1
    Country,  // 2
    Index,    // 3
}

impl BidDescriptorType {
    pub fn to_fix(&self) -> char {
        match self {
            BidDescriptorType::Sector => '1',
            BidDescriptorType::Country => '2',
            BidDescriptorType::Index => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(BidDescriptorType::Sector),
            '2' => Some(BidDescriptorType::Country),
            '3' => Some(BidDescriptorType::Index),
            _ => None,
        }
    }
}


/// SideValueInd (Tag 401) - Side value indicator
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SideValueInd {
    SideValue1,  // 1
    SideValue2,  // 2
}

impl SideValueInd {
    pub fn to_fix(&self) -> char {
        match self {
            SideValueInd::SideValue1 => '1',
            SideValueInd::SideValue2 => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(SideValueInd::SideValue1),
            '2' => Some(SideValueInd::SideValue2),
            _ => None,
        }
    }
}


/// NetGrossInd (Tag 430) - Net or gross indicator
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetGrossInd {
    Net,   // 1
    Gross, // 2
}

impl NetGrossInd {
    pub fn to_fix(&self) -> char {
        match self {
            NetGrossInd::Net => '1',
            NetGrossInd::Gross => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(NetGrossInd::Net),
            '2' => Some(NetGrossInd::Gross),
            _ => None,
        }
    }
}


/// PriceType (Tag 423) - Type of price
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PriceType {
    Percentage,           // 1
    PerUnit,              // 2
    FixedAmount,          // 3
    Discount,             // 4
    Premium,              // 5
    Spread,               // 6
    TEDPrice,             // 7
    TEDYield,             // 8
    Yield,                // 9
    FixedCabinetPrice,    // 10
    VariableCabinetPrice, // 11
}

impl PriceType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            PriceType::Percentage => "1",
            PriceType::PerUnit => "2",
            PriceType::FixedAmount => "3",
            PriceType::Discount => "4",
            PriceType::Premium => "5",
            PriceType::Spread => "6",
            PriceType::TEDPrice => "7",
            PriceType::TEDYield => "8",
            PriceType::Yield => "9",
            PriceType::FixedCabinetPrice => "10",
            PriceType::VariableCabinetPrice => "11",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "1" => Some(PriceType::Percentage),
            "2" => Some(PriceType::PerUnit),
            "3" => Some(PriceType::FixedAmount),
            "4" => Some(PriceType::Discount),
            "5" => Some(PriceType::Premium),
            "6" => Some(PriceType::Spread),
            "7" => Some(PriceType::TEDPrice),
            "8" => Some(PriceType::TEDYield),
            "9" => Some(PriceType::Yield),
            "10" => Some(PriceType::FixedCabinetPrice),
            "11" => Some(PriceType::VariableCabinetPrice),
            _ => None,
        }
    }
}


/// OrderEntryAction (Tag 2429) - Action for OrderEntryGrp
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderEntryAction {
    Add,      // 1 - Add new order
    Modify,   // 2 - Modify existing order
    Delete,   // 3 - Delete order
    Suspend,  // 4 - Suspend order
    Release,  // 5 - Release suspended order
}

impl OrderEntryAction {
    pub fn to_fix(&self) -> char {
        match self {
            OrderEntryAction::Add => '1',
            OrderEntryAction::Modify => '2',
            OrderEntryAction::Delete => '3',
            OrderEntryAction::Suspend => '4',
            OrderEntryAction::Release => '5',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(OrderEntryAction::Add),
            '2' => Some(OrderEntryAction::Modify),
            '3' => Some(OrderEntryAction::Delete),
            '4' => Some(OrderEntryAction::Suspend),
            '5' => Some(OrderEntryAction::Release),
            _ => None,
        }
    }
}
