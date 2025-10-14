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

// ============================================================================
// Program Trading / List Trading Enums
// ============================================================================

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_type_conversions() {
        assert_eq!(BidType::NonDisclosed.to_fix(), '1');
        assert_eq!(BidType::Disclosed.to_fix(), '2');
        assert_eq!(BidType::NoBiddingProcess.to_fix(), '3');

        assert_eq!(BidType::from_fix('1'), Some(BidType::NonDisclosed));
        assert_eq!(BidType::from_fix('2'), Some(BidType::Disclosed));
        assert_eq!(BidType::from_fix('3'), Some(BidType::NoBiddingProcess));
        assert_eq!(BidType::from_fix('9'), None);
    }

    #[test]
    fn test_prog_rpt_reqs_conversions() {
        assert_eq!(ProgRptReqs::BuySideRequests.to_fix(), '1');
        assert_eq!(ProgRptReqs::SellSideRequests.to_fix(), '2');
        assert_eq!(ProgRptReqs::RealTimeExecutions.to_fix(), '3');

        assert_eq!(ProgRptReqs::from_fix('1'), Some(ProgRptReqs::BuySideRequests));
        assert_eq!(ProgRptReqs::from_fix('2'), Some(ProgRptReqs::SellSideRequests));
        assert_eq!(ProgRptReqs::from_fix('3'), Some(ProgRptReqs::RealTimeExecutions));
        assert_eq!(ProgRptReqs::from_fix('9'), None);
    }

    #[test]
    fn test_list_exec_inst_type_conversions() {
        assert_eq!(ListExecInstType::Immediate.to_fix(), '1');
        assert_eq!(ListExecInstType::WaitForInstruction.to_fix(), '2');
        assert_eq!(ListExecInstType::ExchangeSwitchCIVOrder.to_fix(), '3');
        assert_eq!(ListExecInstType::SellDriven.to_fix(), '4');
        assert_eq!(ListExecInstType::BuyDrivenCash.to_fix(), '5');

        assert_eq!(ListExecInstType::from_fix('1'), Some(ListExecInstType::Immediate));
        assert_eq!(ListExecInstType::from_fix('5'), Some(ListExecInstType::BuyDrivenCash));
        assert_eq!(ListExecInstType::from_fix('9'), None);
    }

    #[test]
    fn test_list_status_type_conversions() {
        assert_eq!(ListStatusType::Ack.to_fix(), '1');
        assert_eq!(ListStatusType::Alert.to_fix(), '6');

        assert_eq!(ListStatusType::from_fix('1'), Some(ListStatusType::Ack));
        assert_eq!(ListStatusType::from_fix('4'), Some(ListStatusType::ExecStarted));
        assert_eq!(ListStatusType::from_fix('9'), None);
    }

    #[test]
    fn test_list_order_status_conversions() {
        assert_eq!(ListOrderStatus::InBiddingProcess.to_fix(), '1');
        assert_eq!(ListOrderStatus::Reject.to_fix(), '7');

        assert_eq!(ListOrderStatus::from_fix('1'), Some(ListOrderStatus::InBiddingProcess));
        assert_eq!(ListOrderStatus::from_fix('7'), Some(ListOrderStatus::Reject));
        assert_eq!(ListOrderStatus::from_fix('9'), None);
    }

    #[test]
    fn test_bid_descriptor_type_conversions() {
        assert_eq!(BidDescriptorType::Sector.to_fix(), '1');
        assert_eq!(BidDescriptorType::Country.to_fix(), '2');
        assert_eq!(BidDescriptorType::Index.to_fix(), '3');

        assert_eq!(BidDescriptorType::from_fix('1'), Some(BidDescriptorType::Sector));
        assert_eq!(BidDescriptorType::from_fix('3'), Some(BidDescriptorType::Index));
        assert_eq!(BidDescriptorType::from_fix('9'), None);
    }

    #[test]
    fn test_side_value_ind_conversions() {
        assert_eq!(SideValueInd::SideValue1.to_fix(), '1');
        assert_eq!(SideValueInd::SideValue2.to_fix(), '2');

        assert_eq!(SideValueInd::from_fix('1'), Some(SideValueInd::SideValue1));
        assert_eq!(SideValueInd::from_fix('2'), Some(SideValueInd::SideValue2));
        assert_eq!(SideValueInd::from_fix('9'), None);
    }

    #[test]
    fn test_net_gross_ind_conversions() {
        assert_eq!(NetGrossInd::Net.to_fix(), '1');
        assert_eq!(NetGrossInd::Gross.to_fix(), '2');

        assert_eq!(NetGrossInd::from_fix('1'), Some(NetGrossInd::Net));
        assert_eq!(NetGrossInd::from_fix('2'), Some(NetGrossInd::Gross));
        assert_eq!(NetGrossInd::from_fix('9'), None);
    }

    #[test]
    fn test_price_type_conversions() {
        assert_eq!(PriceType::Percentage.to_fix(), "1");
        assert_eq!(PriceType::PerUnit.to_fix(), "2");
        assert_eq!(PriceType::FixedCabinetPrice.to_fix(), "10");
        assert_eq!(PriceType::VariableCabinetPrice.to_fix(), "11");

        assert_eq!(PriceType::from_fix("1"), Some(PriceType::Percentage));
        assert_eq!(PriceType::from_fix("9"), Some(PriceType::Yield));
        assert_eq!(PriceType::from_fix("10"), Some(PriceType::FixedCabinetPrice));
        assert_eq!(PriceType::from_fix("11"), Some(PriceType::VariableCabinetPrice));
        assert_eq!(PriceType::from_fix("99"), None);
    }
}

// ============================================================================
// Mass Order Message Enums
// ============================================================================

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

/// MassActionType (Tag 1373) - Type of mass action requested
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MassActionType {
    SuspendOrders,                    // 1
    ReleaseOrdersFromSuspension,      // 2
    CancelOrders,                     // 3
}

impl MassActionType {
    pub fn to_fix(&self) -> char {
        match self {
            MassActionType::SuspendOrders => '1',
            MassActionType::ReleaseOrdersFromSuspension => '2',
            MassActionType::CancelOrders => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(MassActionType::SuspendOrders),
            '2' => Some(MassActionType::ReleaseOrdersFromSuspension),
            '3' => Some(MassActionType::CancelOrders),
            _ => None,
        }
    }
}

/// MassActionResponse (Tag 1375) - Response to mass action request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MassActionResponse {
    Rejected,  // 0
    Accepted,  // 1
}

impl MassActionResponse {
    pub fn to_fix(&self) -> char {
        match self {
            MassActionResponse::Rejected => '0',
            MassActionResponse::Accepted => '1',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(MassActionResponse::Rejected),
            '1' => Some(MassActionResponse::Accepted),
            _ => None,
        }
    }
}

/// MassStatusReqType (Tag 585) - Type of mass status request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MassStatusReqType {
    StatusForOrdersForParty,              // 1
    StatusForOrdersForSecurity,           // 2
    StatusForOrdersForUnderlyingSecurity, // 3
    StatusForOrdersForProduct,            // 4
    StatusForOrdersForCFICode,            // 5
    StatusForOrdersForSecurityType,       // 6
    StatusForOrdersForTradingSession,     // 7
    StatusForAllOrders,                   // 8
    StatusForOrdersForPartyGroup,         // 9
}

impl MassStatusReqType {
    pub fn to_fix(&self) -> char {
        match self {
            MassStatusReqType::StatusForOrdersForParty => '1',
            MassStatusReqType::StatusForOrdersForSecurity => '2',
            MassStatusReqType::StatusForOrdersForUnderlyingSecurity => '3',
            MassStatusReqType::StatusForOrdersForProduct => '4',
            MassStatusReqType::StatusForOrdersForCFICode => '5',
            MassStatusReqType::StatusForOrdersForSecurityType => '6',
            MassStatusReqType::StatusForOrdersForTradingSession => '7',
            MassStatusReqType::StatusForAllOrders => '8',
            MassStatusReqType::StatusForOrdersForPartyGroup => '9',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(MassStatusReqType::StatusForOrdersForParty),
            '2' => Some(MassStatusReqType::StatusForOrdersForSecurity),
            '3' => Some(MassStatusReqType::StatusForOrdersForUnderlyingSecurity),
            '4' => Some(MassStatusReqType::StatusForOrdersForProduct),
            '5' => Some(MassStatusReqType::StatusForOrdersForCFICode),
            '6' => Some(MassStatusReqType::StatusForOrdersForSecurityType),
            '7' => Some(MassStatusReqType::StatusForOrdersForTradingSession),
            '8' => Some(MassStatusReqType::StatusForAllOrders),
            '9' => Some(MassStatusReqType::StatusForOrdersForPartyGroup),
            _ => None,
        }
    }
}

/// OrderResponseLevel (Tag 2427) - Level of detail in order acknowledgement
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderResponseLevel {
    NoAck,         // 0 - No acknowledgement
    OnlyAckErrors, // 1 - Only acknowledge orders with errors
    AckEachOrder,  // 2 - Acknowledge each order
}

impl OrderResponseLevel {
    pub fn to_fix(&self) -> char {
        match self {
            OrderResponseLevel::NoAck => '0',
            OrderResponseLevel::OnlyAckErrors => '1',
            OrderResponseLevel::AckEachOrder => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(OrderResponseLevel::NoAck),
            '1' => Some(OrderResponseLevel::OnlyAckErrors),
            '2' => Some(OrderResponseLevel::AckEachOrder),
            _ => None,
        }
    }
}

// ============================================================================
// Multileg Order Message Enums
// ============================================================================

/// MultilegReportingType (Tag 442) - Type of multileg reporting
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MultilegReportingType {
    SingleSecurity,              // 1 - Report as single security
    IndividualLegOfMultilegSec,  // 2 - Report individual leg of multileg security
    MultilegSecurity,            // 3 - Report multileg security
}

impl MultilegReportingType {
    pub fn to_fix(&self) -> char {
        match self {
            MultilegReportingType::SingleSecurity => '1',
            MultilegReportingType::IndividualLegOfMultilegSec => '2',
            MultilegReportingType::MultilegSecurity => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(MultilegReportingType::SingleSecurity),
            '2' => Some(MultilegReportingType::IndividualLegOfMultilegSec),
            '3' => Some(MultilegReportingType::MultilegSecurity),
            _ => None,
        }
    }
}

/// MultilegModel (Tag 1377) - Multileg security definition type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MultilegModel {
    PredefinedMultilegSecurity,  // 0 - Exchange-defined multileg security
    UserDefinedMultileg,         // 1 - User-defined multileg security
}

impl MultilegModel {
    pub fn to_fix(&self) -> char {
        match self {
            MultilegModel::PredefinedMultilegSecurity => '0',
            MultilegModel::UserDefinedMultileg => '1',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(MultilegModel::PredefinedMultilegSecurity),
            '1' => Some(MultilegModel::UserDefinedMultileg),
            _ => None,
        }
    }
}

/// MultilegPriceMethod (Tag 1378) - Method for pricing multileg security
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MultilegPriceMethod {
    NetPrice,                       // 0 - Net price across all legs
    ReversedNetPrice,               // 1 - Reversed net price
    YieldDifference,                // 2 - Yield difference (for bonds)
    Individual,                     // 3 - Individual leg prices
    ContractWeightedAveragePrice,   // 4 - Contract-weighted average price
    MultipliedPrice,                // 5 - Multiplied price
}

impl MultilegPriceMethod {
    pub fn to_fix(&self) -> char {
        match self {
            MultilegPriceMethod::NetPrice => '0',
            MultilegPriceMethod::ReversedNetPrice => '1',
            MultilegPriceMethod::YieldDifference => '2',
            MultilegPriceMethod::Individual => '3',
            MultilegPriceMethod::ContractWeightedAveragePrice => '4',
            MultilegPriceMethod::MultipliedPrice => '5',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(MultilegPriceMethod::NetPrice),
            '1' => Some(MultilegPriceMethod::ReversedNetPrice),
            '2' => Some(MultilegPriceMethod::YieldDifference),
            '3' => Some(MultilegPriceMethod::Individual),
            '4' => Some(MultilegPriceMethod::ContractWeightedAveragePrice),
            '5' => Some(MultilegPriceMethod::MultipliedPrice),
            _ => None,
        }
    }
}

/// BusinessRejectReason (Tag 380) - Reason for business-level rejection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BusinessRejectReason {
    Other,                                  // 0 - Other/unknown reason
    UnknownID,                             // 1 - Unknown ID
    UnknownSecurity,                       // 2 - Unknown security
    UnsupportedMessageType,                // 3 - Unsupported message type
    ApplicationNotAvailable,               // 4 - Application not available
    ConditionallyRequiredFieldMissing,     // 5 - Conditionally required field missing
    NotAuthorized,                         // 6 - Not authorized
    DeliverToFirmNotAvailableAtThisTime,   // 7 - DeliverToFirm not available at this time
}

/// NetworkRequestType (Tag 935) - Type of network request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkRequestType {
    Snapshot,                    // 1 - Snapshot of current status
    Subscribe,                   // 2 - Subscribe to updates
    StopSubscribing,             // 4 - Stop subscribing
    LevelOfDetail,               // 8 - Level of detail
}

/// NetworkStatusResponseType (Tag 937) - Type of network response
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkStatusResponseType {
    Full,                        // 1 - Full refresh
    IncrementalUpdate,           // 2 - Incremental update
}

/// NetworkSystemStatus (Tag 928) - Status of a counterparty system
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkSystemStatus {
    Online,                      // 0 - System is online
    TemporarilyUnavailable,      // 1 - Temporarily unavailable
    Offline,                     // 2 - System is offline
}

/// ApplReqType (Tag 1347) - Type of application request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApplReqType {
    Retransmission,              // 0 - Retransmit messages
    Subscription,                // 1 - Subscribe to messages
    RequestLastSeqNum,           // 2 - Request last sequence number
    RequestApplications,         // 3 - Request list of applications
    Unsubscribe,                 // 4 - Unsubscribe
    CancelRetransmission,        // 5 - Cancel retransmission request
    CancelRetransmissionAndUnsubscribe, // 6 - Cancel and unsubscribe
}

/// ApplResponseType (Tag 1348) - Type of application response
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApplResponseType {
    RequestSuccessfullyProcessed, // 0 - Request processed successfully
    ApplicationDoesNotExist,      // 1 - Application does not exist
    MessagesNotAvailable,         // 2 - Messages not available
}

/// ApplReportType (Tag 1426) - Type of application report
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApplReportType {
    ApplSeqNumReset,             // 0 - Application sequence number reset
    LastMessageSent,             // 1 - Last message sent
    ApplicationAlive,            // 2 - Application alive message
    ApplicationMessageRestart,   // 3 - Application message restart
}

/// UserRequestType (Tag 924) - Type of user request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserRequestType {
    LogOnUser,                   // 1 - Log on user
    LogOffUser,                  // 2 - Log off user
    ChangePasswordForUser,       // 3 - Change password
    RequestIndividualUserStatus, // 4 - Request user status
}

/// UserStatus (Tag 926) - Status of user
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserStatus {
    LoggedIn,                    // 1 - User is logged in
    NotLoggedIn,                 // 2 - User is not logged in
    UserNotRecognized,           // 3 - User not recognized
    PasswordIncorrect,           // 4 - Password incorrect
    PasswordChanged,             // 5 - Password changed
    Other,                       // 6 - Other status
}

impl BusinessRejectReason {
    pub fn to_fix(&self) -> char {
        match self {
            BusinessRejectReason::Other => '0',
            BusinessRejectReason::UnknownID => '1',
            BusinessRejectReason::UnknownSecurity => '2',
            BusinessRejectReason::UnsupportedMessageType => '3',
            BusinessRejectReason::ApplicationNotAvailable => '4',
            BusinessRejectReason::ConditionallyRequiredFieldMissing => '5',
            BusinessRejectReason::NotAuthorized => '6',
            BusinessRejectReason::DeliverToFirmNotAvailableAtThisTime => '7',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(BusinessRejectReason::Other),
            '1' => Some(BusinessRejectReason::UnknownID),
            '2' => Some(BusinessRejectReason::UnknownSecurity),
            '3' => Some(BusinessRejectReason::UnsupportedMessageType),
            '4' => Some(BusinessRejectReason::ApplicationNotAvailable),
            '5' => Some(BusinessRejectReason::ConditionallyRequiredFieldMissing),
            '6' => Some(BusinessRejectReason::NotAuthorized),
            '7' => Some(BusinessRejectReason::DeliverToFirmNotAvailableAtThisTime),
            _ => None,
        }
    }
}

impl NetworkRequestType {
    pub fn to_fix(&self) -> char {
        match self {
            NetworkRequestType::Snapshot => '1',
            NetworkRequestType::Subscribe => '2',
            NetworkRequestType::StopSubscribing => '4',
            NetworkRequestType::LevelOfDetail => '8',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(NetworkRequestType::Snapshot),
            '2' => Some(NetworkRequestType::Subscribe),
            '4' => Some(NetworkRequestType::StopSubscribing),
            '8' => Some(NetworkRequestType::LevelOfDetail),
            _ => None,
        }
    }
}

impl NetworkStatusResponseType {
    pub fn to_fix(&self) -> char {
        match self {
            NetworkStatusResponseType::Full => '1',
            NetworkStatusResponseType::IncrementalUpdate => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(NetworkStatusResponseType::Full),
            '2' => Some(NetworkStatusResponseType::IncrementalUpdate),
            _ => None,
        }
    }
}

impl NetworkSystemStatus {
    pub fn to_fix(&self) -> char {
        match self {
            NetworkSystemStatus::Online => '0',
            NetworkSystemStatus::TemporarilyUnavailable => '1',
            NetworkSystemStatus::Offline => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(NetworkSystemStatus::Online),
            '1' => Some(NetworkSystemStatus::TemporarilyUnavailable),
            '2' => Some(NetworkSystemStatus::Offline),
            _ => None,
        }
    }
}

impl ApplReqType {
    pub fn to_fix(&self) -> char {
        match self {
            ApplReqType::Retransmission => '0',
            ApplReqType::Subscription => '1',
            ApplReqType::RequestLastSeqNum => '2',
            ApplReqType::RequestApplications => '3',
            ApplReqType::Unsubscribe => '4',
            ApplReqType::CancelRetransmission => '5',
            ApplReqType::CancelRetransmissionAndUnsubscribe => '6',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(ApplReqType::Retransmission),
            '1' => Some(ApplReqType::Subscription),
            '2' => Some(ApplReqType::RequestLastSeqNum),
            '3' => Some(ApplReqType::RequestApplications),
            '4' => Some(ApplReqType::Unsubscribe),
            '5' => Some(ApplReqType::CancelRetransmission),
            '6' => Some(ApplReqType::CancelRetransmissionAndUnsubscribe),
            _ => None,
        }
    }
}

impl ApplResponseType {
    pub fn to_fix(&self) -> char {
        match self {
            ApplResponseType::RequestSuccessfullyProcessed => '0',
            ApplResponseType::ApplicationDoesNotExist => '1',
            ApplResponseType::MessagesNotAvailable => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(ApplResponseType::RequestSuccessfullyProcessed),
            '1' => Some(ApplResponseType::ApplicationDoesNotExist),
            '2' => Some(ApplResponseType::MessagesNotAvailable),
            _ => None,
        }
    }
}

impl ApplReportType {
    pub fn to_fix(&self) -> char {
        match self {
            ApplReportType::ApplSeqNumReset => '0',
            ApplReportType::LastMessageSent => '1',
            ApplReportType::ApplicationAlive => '2',
            ApplReportType::ApplicationMessageRestart => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(ApplReportType::ApplSeqNumReset),
            '1' => Some(ApplReportType::LastMessageSent),
            '2' => Some(ApplReportType::ApplicationAlive),
            '3' => Some(ApplReportType::ApplicationMessageRestart),
            _ => None,
        }
    }
}

impl UserRequestType {
    pub fn to_fix(&self) -> char {
        match self {
            UserRequestType::LogOnUser => '1',
            UserRequestType::LogOffUser => '2',
            UserRequestType::ChangePasswordForUser => '3',
            UserRequestType::RequestIndividualUserStatus => '4',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(UserRequestType::LogOnUser),
            '2' => Some(UserRequestType::LogOffUser),
            '3' => Some(UserRequestType::ChangePasswordForUser),
            '4' => Some(UserRequestType::RequestIndividualUserStatus),
            _ => None,
        }
    }
}

impl UserStatus {
    pub fn to_fix(&self) -> char {
        match self {
            UserStatus::LoggedIn => '1',
            UserStatus::NotLoggedIn => '2',
            UserStatus::UserNotRecognized => '3',
            UserStatus::PasswordIncorrect => '4',
            UserStatus::PasswordChanged => '5',
            UserStatus::Other => '6',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(UserStatus::LoggedIn),
            '2' => Some(UserStatus::NotLoggedIn),
            '3' => Some(UserStatus::UserNotRecognized),
            '4' => Some(UserStatus::PasswordIncorrect),
            '5' => Some(UserStatus::PasswordChanged),
            '6' => Some(UserStatus::Other),
            _ => None,
        }
    }
}

// ============================================================================
// Mass Order Enum Tests
// ============================================================================

#[cfg(test)]
mod mass_order_tests {
    use super::*;

    #[test]
    fn test_order_entry_action_conversions() {
        assert_eq!(OrderEntryAction::Add.to_fix(), '1');
        assert_eq!(OrderEntryAction::Modify.to_fix(), '2');
        assert_eq!(OrderEntryAction::Delete.to_fix(), '3');
        assert_eq!(OrderEntryAction::Suspend.to_fix(), '4');
        assert_eq!(OrderEntryAction::Release.to_fix(), '5');

        assert_eq!(OrderEntryAction::from_fix('1'), Some(OrderEntryAction::Add));
        assert_eq!(OrderEntryAction::from_fix('2'), Some(OrderEntryAction::Modify));
        assert_eq!(OrderEntryAction::from_fix('3'), Some(OrderEntryAction::Delete));
        assert_eq!(OrderEntryAction::from_fix('4'), Some(OrderEntryAction::Suspend));
        assert_eq!(OrderEntryAction::from_fix('5'), Some(OrderEntryAction::Release));
        assert_eq!(OrderEntryAction::from_fix('9'), None);
    }

    #[test]
    fn test_mass_action_type_conversions() {
        assert_eq!(MassActionType::SuspendOrders.to_fix(), '1');
        assert_eq!(MassActionType::ReleaseOrdersFromSuspension.to_fix(), '2');
        assert_eq!(MassActionType::CancelOrders.to_fix(), '3');

        assert_eq!(MassActionType::from_fix('1'), Some(MassActionType::SuspendOrders));
        assert_eq!(MassActionType::from_fix('2'), Some(MassActionType::ReleaseOrdersFromSuspension));
        assert_eq!(MassActionType::from_fix('3'), Some(MassActionType::CancelOrders));
        assert_eq!(MassActionType::from_fix('9'), None);
    }

    #[test]
    fn test_mass_action_response_conversions() {
        assert_eq!(MassActionResponse::Rejected.to_fix(), '0');
        assert_eq!(MassActionResponse::Accepted.to_fix(), '1');

        assert_eq!(MassActionResponse::from_fix('0'), Some(MassActionResponse::Rejected));
        assert_eq!(MassActionResponse::from_fix('1'), Some(MassActionResponse::Accepted));
        assert_eq!(MassActionResponse::from_fix('9'), None);
    }

    #[test]
    fn test_mass_status_req_type_conversions() {
        assert_eq!(MassStatusReqType::StatusForOrdersForParty.to_fix(), '1');
        assert_eq!(MassStatusReqType::StatusForOrdersForSecurity.to_fix(), '2');
        assert_eq!(MassStatusReqType::StatusForAllOrders.to_fix(), '8');
        assert_eq!(MassStatusReqType::StatusForOrdersForPartyGroup.to_fix(), '9');

        assert_eq!(MassStatusReqType::from_fix('1'), Some(MassStatusReqType::StatusForOrdersForParty));
        assert_eq!(MassStatusReqType::from_fix('8'), Some(MassStatusReqType::StatusForAllOrders));
        assert_eq!(MassStatusReqType::from_fix('0'), None);
    }

    #[test]
    fn test_order_response_level_conversions() {
        assert_eq!(OrderResponseLevel::NoAck.to_fix(), '0');
        assert_eq!(OrderResponseLevel::OnlyAckErrors.to_fix(), '1');
        assert_eq!(OrderResponseLevel::AckEachOrder.to_fix(), '2');

        assert_eq!(OrderResponseLevel::from_fix('0'), Some(OrderResponseLevel::NoAck));
        assert_eq!(OrderResponseLevel::from_fix('1'), Some(OrderResponseLevel::OnlyAckErrors));
        assert_eq!(OrderResponseLevel::from_fix('2'), Some(OrderResponseLevel::AckEachOrder));
        assert_eq!(OrderResponseLevel::from_fix('9'), None);
    }

    // ========================================================================
    // Multileg Order Tests
    // ========================================================================

    #[test]
    fn test_multileg_reporting_type_conversions() {
        assert_eq!(MultilegReportingType::SingleSecurity.to_fix(), '1');
        assert_eq!(MultilegReportingType::IndividualLegOfMultilegSec.to_fix(), '2');
        assert_eq!(MultilegReportingType::MultilegSecurity.to_fix(), '3');

        assert_eq!(MultilegReportingType::from_fix('1'), Some(MultilegReportingType::SingleSecurity));
        assert_eq!(MultilegReportingType::from_fix('2'), Some(MultilegReportingType::IndividualLegOfMultilegSec));
        assert_eq!(MultilegReportingType::from_fix('3'), Some(MultilegReportingType::MultilegSecurity));
        assert_eq!(MultilegReportingType::from_fix('9'), None);
    }

    #[test]
    fn test_multileg_model_conversions() {
        assert_eq!(MultilegModel::PredefinedMultilegSecurity.to_fix(), '0');
        assert_eq!(MultilegModel::UserDefinedMultileg.to_fix(), '1');

        assert_eq!(MultilegModel::from_fix('0'), Some(MultilegModel::PredefinedMultilegSecurity));
        assert_eq!(MultilegModel::from_fix('1'), Some(MultilegModel::UserDefinedMultileg));
        assert_eq!(MultilegModel::from_fix('9'), None);
    }

    #[test]
    fn test_multileg_price_method_conversions() {
        assert_eq!(MultilegPriceMethod::NetPrice.to_fix(), '0');
        assert_eq!(MultilegPriceMethod::ReversedNetPrice.to_fix(), '1');
        assert_eq!(MultilegPriceMethod::YieldDifference.to_fix(), '2');
        assert_eq!(MultilegPriceMethod::Individual.to_fix(), '3');
        assert_eq!(MultilegPriceMethod::ContractWeightedAveragePrice.to_fix(), '4');
        assert_eq!(MultilegPriceMethod::MultipliedPrice.to_fix(), '5');

        assert_eq!(MultilegPriceMethod::from_fix('0'), Some(MultilegPriceMethod::NetPrice));
        assert_eq!(MultilegPriceMethod::from_fix('3'), Some(MultilegPriceMethod::Individual));
        assert_eq!(MultilegPriceMethod::from_fix('5'), Some(MultilegPriceMethod::MultipliedPrice));
        assert_eq!(MultilegPriceMethod::from_fix('9'), None);
    }

    #[test]
    fn test_business_reject_reason_conversions() {
        assert_eq!(BusinessRejectReason::Other.to_fix(), '0');
        assert_eq!(BusinessRejectReason::UnknownID.to_fix(), '1');
        assert_eq!(BusinessRejectReason::UnknownSecurity.to_fix(), '2');
        assert_eq!(BusinessRejectReason::UnsupportedMessageType.to_fix(), '3');
        assert_eq!(BusinessRejectReason::ApplicationNotAvailable.to_fix(), '4');
        assert_eq!(BusinessRejectReason::ConditionallyRequiredFieldMissing.to_fix(), '5');
        assert_eq!(BusinessRejectReason::NotAuthorized.to_fix(), '6');
        assert_eq!(BusinessRejectReason::DeliverToFirmNotAvailableAtThisTime.to_fix(), '7');

        assert_eq!(BusinessRejectReason::from_fix('0'), Some(BusinessRejectReason::Other));
        assert_eq!(BusinessRejectReason::from_fix('3'), Some(BusinessRejectReason::UnsupportedMessageType));
        assert_eq!(BusinessRejectReason::from_fix('7'), Some(BusinessRejectReason::DeliverToFirmNotAvailableAtThisTime));
        assert_eq!(BusinessRejectReason::from_fix('9'), None);
    }

    #[test]
    fn test_network_request_type_conversions() {
        assert_eq!(NetworkRequestType::Snapshot.to_fix(), '1');
        assert_eq!(NetworkRequestType::Subscribe.to_fix(), '2');
        assert_eq!(NetworkRequestType::StopSubscribing.to_fix(), '4');
        assert_eq!(NetworkRequestType::LevelOfDetail.to_fix(), '8');

        assert_eq!(NetworkRequestType::from_fix('1'), Some(NetworkRequestType::Snapshot));
        assert_eq!(NetworkRequestType::from_fix('2'), Some(NetworkRequestType::Subscribe));
        assert_eq!(NetworkRequestType::from_fix('4'), Some(NetworkRequestType::StopSubscribing));
        assert_eq!(NetworkRequestType::from_fix('8'), Some(NetworkRequestType::LevelOfDetail));
        assert_eq!(NetworkRequestType::from_fix('9'), None);
    }

    #[test]
    fn test_network_status_response_type_conversions() {
        assert_eq!(NetworkStatusResponseType::Full.to_fix(), '1');
        assert_eq!(NetworkStatusResponseType::IncrementalUpdate.to_fix(), '2');

        assert_eq!(NetworkStatusResponseType::from_fix('1'), Some(NetworkStatusResponseType::Full));
        assert_eq!(NetworkStatusResponseType::from_fix('2'), Some(NetworkStatusResponseType::IncrementalUpdate));
        assert_eq!(NetworkStatusResponseType::from_fix('9'), None);
    }

    #[test]
    fn test_network_system_status_conversions() {
        assert_eq!(NetworkSystemStatus::Online.to_fix(), '0');
        assert_eq!(NetworkSystemStatus::TemporarilyUnavailable.to_fix(), '1');
        assert_eq!(NetworkSystemStatus::Offline.to_fix(), '2');

        assert_eq!(NetworkSystemStatus::from_fix('0'), Some(NetworkSystemStatus::Online));
        assert_eq!(NetworkSystemStatus::from_fix('1'), Some(NetworkSystemStatus::TemporarilyUnavailable));
        assert_eq!(NetworkSystemStatus::from_fix('2'), Some(NetworkSystemStatus::Offline));
        assert_eq!(NetworkSystemStatus::from_fix('9'), None);
    }

    #[test]
    fn test_appl_req_type_conversions() {
        assert_eq!(ApplReqType::Retransmission.to_fix(), '0');
        assert_eq!(ApplReqType::Subscription.to_fix(), '1');
        assert_eq!(ApplReqType::CancelRetransmissionAndUnsubscribe.to_fix(), '6');

        assert_eq!(ApplReqType::from_fix('0'), Some(ApplReqType::Retransmission));
        assert_eq!(ApplReqType::from_fix('3'), Some(ApplReqType::RequestApplications));
        assert_eq!(ApplReqType::from_fix('9'), None);
    }

    #[test]
    fn test_appl_response_type_conversions() {
        assert_eq!(ApplResponseType::RequestSuccessfullyProcessed.to_fix(), '0');
        assert_eq!(ApplResponseType::ApplicationDoesNotExist.to_fix(), '1');
        assert_eq!(ApplResponseType::MessagesNotAvailable.to_fix(), '2');

        assert_eq!(ApplResponseType::from_fix('0'), Some(ApplResponseType::RequestSuccessfullyProcessed));
        assert_eq!(ApplResponseType::from_fix('2'), Some(ApplResponseType::MessagesNotAvailable));
        assert_eq!(ApplResponseType::from_fix('9'), None);
    }

    #[test]
    fn test_appl_report_type_conversions() {
        assert_eq!(ApplReportType::ApplSeqNumReset.to_fix(), '0');
        assert_eq!(ApplReportType::LastMessageSent.to_fix(), '1');
        assert_eq!(ApplReportType::ApplicationAlive.to_fix(), '2');
        assert_eq!(ApplReportType::ApplicationMessageRestart.to_fix(), '3');

        assert_eq!(ApplReportType::from_fix('0'), Some(ApplReportType::ApplSeqNumReset));
        assert_eq!(ApplReportType::from_fix('3'), Some(ApplReportType::ApplicationMessageRestart));
        assert_eq!(ApplReportType::from_fix('9'), None);
    }

    #[test]
    fn test_user_request_type_conversions() {
        assert_eq!(UserRequestType::LogOnUser.to_fix(), '1');
        assert_eq!(UserRequestType::ChangePasswordForUser.to_fix(), '3');

        assert_eq!(UserRequestType::from_fix('1'), Some(UserRequestType::LogOnUser));
        assert_eq!(UserRequestType::from_fix('4'), Some(UserRequestType::RequestIndividualUserStatus));
        assert_eq!(UserRequestType::from_fix('9'), None);
    }

    #[test]
    fn test_user_status_conversions() {
        assert_eq!(UserStatus::LoggedIn.to_fix(), '1');
        assert_eq!(UserStatus::PasswordChanged.to_fix(), '5');

        assert_eq!(UserStatus::from_fix('1'), Some(UserStatus::LoggedIn));
        assert_eq!(UserStatus::from_fix('6'), Some(UserStatus::Other));
        assert_eq!(UserStatus::from_fix('9'), None);
    }
}

// ============================================================================
// Pre-Trade Messages - Section Markers
// ============================================================================
// This section is pre-allocated for parallel development of Pre-Trade messages.
// Each category will add enums in their designated section to avoid merge conflicts.

// ============================================================================
// [SECTION 100] Indication Messages (Advertisement, IOI, CrossRequest, CrossRequestAck)
// Messages: Advertisement(7), IOI(6), CrossRequest(DS), CrossRequestAck(DT)
// Tags: 2-5, 23, 25-28, 104, 149, 199, 215-217, 376, 854, 1300-1301, 2404, 2672
// Implementation: feature/pretrade-indication
// ============================================================================

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

#[cfg(test)]
mod indication_enum_tests {
    use super::*;

    #[test]
    fn test_ioi_trans_type_conversions() {
        assert_eq!(IOITransType::New.to_fix(), 'N');
        assert_eq!(IOITransType::Cancel.to_fix(), 'C');
        assert_eq!(IOITransType::Replace.to_fix(), 'R');

        assert_eq!(IOITransType::from_fix('N'), Some(IOITransType::New));
        assert_eq!(IOITransType::from_fix('C'), Some(IOITransType::Cancel));
        assert_eq!(IOITransType::from_fix('R'), Some(IOITransType::Replace));
        assert_eq!(IOITransType::from_fix('X'), None);
    }

    #[test]
    fn test_ioi_qlty_ind_conversions() {
        assert_eq!(IOIQltyInd::Low.to_fix(), 'L');
        assert_eq!(IOIQltyInd::Medium.to_fix(), 'M');
        assert_eq!(IOIQltyInd::High.to_fix(), 'H');

        assert_eq!(IOIQltyInd::from_fix('L'), Some(IOIQltyInd::Low));
        assert_eq!(IOIQltyInd::from_fix('M'), Some(IOIQltyInd::Medium));
        assert_eq!(IOIQltyInd::from_fix('H'), Some(IOIQltyInd::High));
        assert_eq!(IOIQltyInd::from_fix('X'), None);
    }

    #[test]
    fn test_adv_side_conversions() {
        assert_eq!(AdvSide::Buy.to_fix(), 'B');
        assert_eq!(AdvSide::Sell.to_fix(), 'S');
        assert_eq!(AdvSide::Cross.to_fix(), 'X');
        assert_eq!(AdvSide::Trade.to_fix(), 'T');

        assert_eq!(AdvSide::from_fix('B'), Some(AdvSide::Buy));
        assert_eq!(AdvSide::from_fix('S'), Some(AdvSide::Sell));
        assert_eq!(AdvSide::from_fix('X'), Some(AdvSide::Cross));
        assert_eq!(AdvSide::from_fix('T'), Some(AdvSide::Trade));
        assert_eq!(AdvSide::from_fix('Z'), None);
    }

    #[test]
    fn test_adv_trans_type_conversions() {
        assert_eq!(AdvTransType::New.to_fix(), 'N');
        assert_eq!(AdvTransType::Cancel.to_fix(), 'C');
        assert_eq!(AdvTransType::Replace.to_fix(), 'R');

        assert_eq!(AdvTransType::from_fix('N'), Some(AdvTransType::New));
        assert_eq!(AdvTransType::from_fix('C'), Some(AdvTransType::Cancel));
        assert_eq!(AdvTransType::from_fix('R'), Some(AdvTransType::Replace));
        assert_eq!(AdvTransType::from_fix('X'), None);
    }

    #[test]
    fn test_qty_type_conversions() {
        assert_eq!(QtyType::Units.to_fix(), '0');
        assert_eq!(QtyType::Contracts.to_fix(), '1');

        assert_eq!(QtyType::from_fix('0'), Some(QtyType::Units));
        assert_eq!(QtyType::from_fix('1'), Some(QtyType::Contracts));
        assert_eq!(QtyType::from_fix('9'), None);
    }
}

// ============================================================================
// [SECTION 200] Event Communication Messages (Email, News)
// Messages: Email(C), News(B)
// Tags: 42-43, 94, 146-148, 212-214, 1473, 1477
// Implementation: feature/pretrade-event-communication
// ============================================================================

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

#[cfg(test)]
mod event_communication_enum_tests {
    use super::*;

    #[test]
    fn test_email_type_conversions() {
        assert_eq!(EmailType::New.to_fix(), '0');
        assert_eq!(EmailType::Reply.to_fix(), '1');
        assert_eq!(EmailType::AdminReply.to_fix(), '2');

        assert_eq!(EmailType::from_fix('0'), Some(EmailType::New));
        assert_eq!(EmailType::from_fix('1'), Some(EmailType::Reply));
        assert_eq!(EmailType::from_fix('2'), Some(EmailType::AdminReply));
        assert_eq!(EmailType::from_fix('9'), None);
    }

    #[test]
    fn test_news_ref_type_conversions() {
        assert_eq!(NewsRefType::Replacement.to_fix(), '0');
        assert_eq!(NewsRefType::Cancellation.to_fix(), '1');
        assert_eq!(NewsRefType::Supplement.to_fix(), '2');

        assert_eq!(NewsRefType::from_fix('0'), Some(NewsRefType::Replacement));
        assert_eq!(NewsRefType::from_fix('1'), Some(NewsRefType::Cancellation));
        assert_eq!(NewsRefType::from_fix('2'), Some(NewsRefType::Supplement));
        assert_eq!(NewsRefType::from_fix('9'), None);
    }

    #[test]
    fn test_news_category_conversions() {
        assert_eq!(NewsCategory::CompanyNews.to_fix(), "0");
        assert_eq!(NewsCategory::MarketplaceNews.to_fix(), "1");
        assert_eq!(NewsCategory::FinancialNews.to_fix(), "2");
        assert_eq!(NewsCategory::TechnicalNews.to_fix(), "3");
        assert_eq!(NewsCategory::OtherNews.to_fix(), "99");

        assert_eq!(NewsCategory::from_fix("0"), Some(NewsCategory::CompanyNews));
        assert_eq!(NewsCategory::from_fix("1"), Some(NewsCategory::MarketplaceNews));
        assert_eq!(NewsCategory::from_fix("2"), Some(NewsCategory::FinancialNews));
        assert_eq!(NewsCategory::from_fix("3"), Some(NewsCategory::TechnicalNews));
        assert_eq!(NewsCategory::from_fix("99"), Some(NewsCategory::OtherNews));
        assert_eq!(NewsCategory::from_fix("100"), None);
    }
}

// ============================================================================
// [SECTION 300] Quotation/Negotiation Messages
// Messages: Quote(S), QuoteRequest(R), QuoteCancel(Z), QuoteStatusRequest(a),
//           QuoteResponse(AJ), QuoteRequestReject(AG), RFQRequest(AH),
//           QuoteAcknowledgment(CW), QuoteStatusReport(AI), MassQuote(i),
//           MassQuoteAcknowledgment(b)
// Tags: 117-121, 131-141, 297-303, 537-538, 648-649, etc.
// Implementation: feature/pretrade-quotation
// ============================================================================

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

#[cfg(test)]
mod quotation_tests {
    use super::*;

    #[test]
    fn test_quote_type_conversions() {
        assert_eq!(QuoteType::Indicative.to_fix(), '0');
        assert_eq!(QuoteType::Tradeable.to_fix(), '1');
        assert_eq!(QuoteType::RestrictedTradeable.to_fix(), '2');
        assert_eq!(QuoteType::Counter.to_fix(), '3');

        assert_eq!(QuoteType::from_fix('0'), Some(QuoteType::Indicative));
        assert_eq!(QuoteType::from_fix('1'), Some(QuoteType::Tradeable));
        assert_eq!(QuoteType::from_fix('2'), Some(QuoteType::RestrictedTradeable));
        assert_eq!(QuoteType::from_fix('3'), Some(QuoteType::Counter));
        assert_eq!(QuoteType::from_fix('9'), None);
    }

    #[test]
    fn test_quote_request_type_conversions() {
        assert_eq!(QuoteRequestType::Manual.to_fix(), '1');
        assert_eq!(QuoteRequestType::Automatic.to_fix(), '2');

        assert_eq!(QuoteRequestType::from_fix('1'), Some(QuoteRequestType::Manual));
        assert_eq!(QuoteRequestType::from_fix('2'), Some(QuoteRequestType::Automatic));
        assert_eq!(QuoteRequestType::from_fix('9'), None);
    }

    #[test]
    fn test_quote_cancel_type_conversions() {
        assert_eq!(QuoteCancelType::CancelAllQuotes.to_fix(), '4');
        assert_eq!(QuoteCancelType::CancelByQuoteType.to_fix(), '6');

        assert_eq!(QuoteCancelType::from_fix('1'), Some(QuoteCancelType::CancelForOneOrMoreSecurities));
        assert_eq!(QuoteCancelType::from_fix('4'), Some(QuoteCancelType::CancelAllQuotes));
        assert_eq!(QuoteCancelType::from_fix('8'), Some(QuoteCancelType::CancelForIssuerOfUnderlyingSecurity));
        assert_eq!(QuoteCancelType::from_fix('9'), None);
    }

    #[test]
    fn test_quote_response_level_conversions() {
        assert_eq!(QuoteResponseLevel::NoAcknowledgement.to_fix(), '0');
        assert_eq!(QuoteResponseLevel::AcknowledgeEachQuote.to_fix(), '2');

        assert_eq!(QuoteResponseLevel::from_fix('0'), Some(QuoteResponseLevel::NoAcknowledgement));
        assert_eq!(QuoteResponseLevel::from_fix('3'), Some(QuoteResponseLevel::SummaryAcknowledgement));
        assert_eq!(QuoteResponseLevel::from_fix('9'), None);
    }

    #[test]
    fn test_quote_request_reject_reason_conversions() {
        assert_eq!(QuoteRequestRejectReason::UnknownSymbol.to_fix(), "1");
        assert_eq!(QuoteRequestRejectReason::Other.to_fix(), "99");

        assert_eq!(QuoteRequestRejectReason::from_fix("1"), Some(QuoteRequestRejectReason::UnknownSymbol));
        assert_eq!(QuoteRequestRejectReason::from_fix("10"), Some(QuoteRequestRejectReason::Pass));
        assert_eq!(QuoteRequestRejectReason::from_fix("99"), Some(QuoteRequestRejectReason::Other));
        assert_eq!(QuoteRequestRejectReason::from_fix("100"), None);
    }

    #[test]
    fn test_quote_status_conversions() {
        assert_eq!(QuoteStatus::Accepted.to_fix(), "0");
        assert_eq!(QuoteStatus::CancelForSymbolDeprecated.to_fix(), "1");
        assert_eq!(QuoteStatus::Active.to_fix(), "16");
        assert_eq!(QuoteStatus::TooLateToEnd.to_fix(), "20");

        assert_eq!(QuoteStatus::from_fix("0"), Some(QuoteStatus::Accepted));
        assert_eq!(QuoteStatus::from_fix("1"), Some(QuoteStatus::CancelForSymbolDeprecated));
        assert_eq!(QuoteStatus::from_fix("10"), Some(QuoteStatus::Pending));
        assert_eq!(QuoteStatus::from_fix("17"), Some(QuoteStatus::Canceled));
        assert_eq!(QuoteStatus::from_fix("20"), Some(QuoteStatus::TooLateToEnd));
        assert_eq!(QuoteStatus::from_fix("99"), None);
    }

    #[test]
    fn test_quote_condition_conversions() {
        assert_eq!(QuoteCondition::ReservedSAM.to_fix(), '0');
        assert_eq!(QuoteCondition::FlatCurve.to_fix(), '7');
        assert_eq!(QuoteCondition::OpenActive.to_fix(), 'A');
        assert_eq!(QuoteCondition::Locked.to_fix(), 'E');
        assert_eq!(QuoteCondition::OrderImbalance.to_fix(), 'Z');
        assert_eq!(QuoteCondition::EquipmentChangeover.to_fix(), 'a');
        assert_eq!(QuoteCondition::SuspendedSAM.to_fix(), 'z');

        assert_eq!(QuoteCondition::from_fix('0'), Some(QuoteCondition::ReservedSAM));
        assert_eq!(QuoteCondition::from_fix('A'), Some(QuoteCondition::OpenActive));
        assert_eq!(QuoteCondition::from_fix('F'), Some(QuoteCondition::Crossed));
        assert_eq!(QuoteCondition::from_fix('L'), Some(QuoteCondition::ManualSlowQuote));
        assert_eq!(QuoteCondition::from_fix('Z'), Some(QuoteCondition::OrderImbalance));
        assert_eq!(QuoteCondition::from_fix('a'), Some(QuoteCondition::EquipmentChangeover));
        assert_eq!(QuoteCondition::from_fix('z'), Some(QuoteCondition::SuspendedSAM));
        assert_eq!(QuoteCondition::from_fix('!'), None);
    }
}

// ============================================================================
// [SECTION 400] Market Data Messages
// Messages: MarketDataRequest(V), MarketDataSnapshotFullRefresh(W),
//           MarketDataRequestReject(Y)
// Tags: 262-268, 269-271, 286-291, etc.
// Implementation: feature/pretrade-market-data
// ============================================================================

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

#[cfg(test)]
mod market_data_enum_tests {
    use super::*;

    #[test]
    fn test_md_req_rej_reason_conversions() {
        assert_eq!(MDReqRejReason::UnknownSymbol.to_fix(), "0");
        assert_eq!(MDReqRejReason::DuplicateMDReqID.to_fix(), "1");
        assert_eq!(MDReqRejReason::UnsupportedMDImplicitDelete.to_fix(), "C");

        assert_eq!(MDReqRejReason::from_fix("0"), Some(MDReqRejReason::UnknownSymbol));
        assert_eq!(MDReqRejReason::from_fix("A"), Some(MDReqRejReason::UnsupportedScope));
        assert_eq!(MDReqRejReason::from_fix("C"), Some(MDReqRejReason::UnsupportedMDImplicitDelete));
        assert_eq!(MDReqRejReason::from_fix("Z"), None);
    }

    #[test]
    fn test_md_update_type_conversions() {
        assert_eq!(MDUpdateType::FullRefresh.to_fix(), '0');
        assert_eq!(MDUpdateType::IncrementalRefresh.to_fix(), '1');

        assert_eq!(MDUpdateType::from_fix('0'), Some(MDUpdateType::FullRefresh));
        assert_eq!(MDUpdateType::from_fix('1'), Some(MDUpdateType::IncrementalRefresh));
        assert_eq!(MDUpdateType::from_fix('9'), None);
    }

    #[test]
    fn test_subscription_request_type_conversions() {
        assert_eq!(SubscriptionRequestType::Snapshot.to_fix(), '0');
        assert_eq!(SubscriptionRequestType::SnapshotPlusUpdates.to_fix(), '1');
        assert_eq!(SubscriptionRequestType::DisablePreviousSnapshot.to_fix(), '2');

        assert_eq!(SubscriptionRequestType::from_fix('0'), Some(SubscriptionRequestType::Snapshot));
        assert_eq!(SubscriptionRequestType::from_fix('1'), Some(SubscriptionRequestType::SnapshotPlusUpdates));
        assert_eq!(SubscriptionRequestType::from_fix('2'), Some(SubscriptionRequestType::DisablePreviousSnapshot));
        assert_eq!(SubscriptionRequestType::from_fix('9'), None);
    }

    #[test]
    fn test_md_entry_type_conversions() {
        assert_eq!(MDEntryType::Bid.to_fix(), "0");
        assert_eq!(MDEntryType::Offer.to_fix(), "1");
        assert_eq!(MDEntryType::Trade.to_fix(), "2");
        assert_eq!(MDEntryType::OpenInterest.to_fix(), "C");

        assert_eq!(MDEntryType::from_fix("0"), Some(MDEntryType::Bid));
        assert_eq!(MDEntryType::from_fix("1"), Some(MDEntryType::Offer));
        assert_eq!(MDEntryType::from_fix("A"), Some(MDEntryType::Imbalance));
        assert_eq!(MDEntryType::from_fix("C"), Some(MDEntryType::OpenInterest));
        assert_eq!(MDEntryType::from_fix("Z"), None);
    }
}

// ============================================================================
// [SECTION 500] Market Structure Reference Data Messages
// Messages: MarketDefinition(BU), MarketDefinitionRequest(BT),
//           MarketDefinitionUpdateReport(BV), TradingSessionList(BJ),
//           TradingSessionListRequest(BI), TradingSessionListUpdateReport(BS),
//           TradingSessionStatus(h), TradingSessionStatusRequest(g)
// Tags: 325, 326, 338-340, 1300-1301, 1022-1024, etc.
// Implementation: feature/pretrade-market-structure
// ============================================================================

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

#[cfg(test)]
mod market_structure_enum_tests {
    use super::*;

    #[test]
    fn test_trad_ses_status_conversions() {
        assert_eq!(TradSesStatus::Unknown.to_fix(), '0');
        assert_eq!(TradSesStatus::Halted.to_fix(), '1');
        assert_eq!(TradSesStatus::Open.to_fix(), '2');
        assert_eq!(TradSesStatus::Closed.to_fix(), '3');
        assert_eq!(TradSesStatus::PreOpen.to_fix(), '4');
        assert_eq!(TradSesStatus::PreClose.to_fix(), '5');
        assert_eq!(TradSesStatus::RequestRejected.to_fix(), '6');

        assert_eq!(TradSesStatus::from_fix('0'), Some(TradSesStatus::Unknown));
        assert_eq!(TradSesStatus::from_fix('1'), Some(TradSesStatus::Halted));
        assert_eq!(TradSesStatus::from_fix('2'), Some(TradSesStatus::Open));
        assert_eq!(TradSesStatus::from_fix('3'), Some(TradSesStatus::Closed));
        assert_eq!(TradSesStatus::from_fix('4'), Some(TradSesStatus::PreOpen));
        assert_eq!(TradSesStatus::from_fix('5'), Some(TradSesStatus::PreClose));
        assert_eq!(TradSesStatus::from_fix('6'), Some(TradSesStatus::RequestRejected));
        assert_eq!(TradSesStatus::from_fix('9'), None);
    }

    #[test]
    fn test_trad_ses_method_conversions() {
        assert_eq!(TradSesMethod::Electronic.to_fix(), '1');
        assert_eq!(TradSesMethod::OpenOutcry.to_fix(), '2');
        assert_eq!(TradSesMethod::TwoParty.to_fix(), '3');

        assert_eq!(TradSesMethod::from_fix('1'), Some(TradSesMethod::Electronic));
        assert_eq!(TradSesMethod::from_fix('2'), Some(TradSesMethod::OpenOutcry));
        assert_eq!(TradSesMethod::from_fix('3'), Some(TradSesMethod::TwoParty));
        assert_eq!(TradSesMethod::from_fix('9'), None);
    }

    #[test]
    fn test_trad_ses_mode_conversions() {
        assert_eq!(TradSesMode::Testing.to_fix(), '1');
        assert_eq!(TradSesMode::Simulated.to_fix(), '2');
        assert_eq!(TradSesMode::Production.to_fix(), '3');

        assert_eq!(TradSesMode::from_fix('1'), Some(TradSesMode::Testing));
        assert_eq!(TradSesMode::from_fix('2'), Some(TradSesMode::Simulated));
        assert_eq!(TradSesMode::from_fix('3'), Some(TradSesMode::Production));
        assert_eq!(TradSesMode::from_fix('9'), None);
    }

    #[test]
    fn test_trad_ses_status_rej_reason_conversions() {
        assert_eq!(TradSesStatusRejReason::UnknownOrInvalidTradingSessionID.to_fix(), "1");
        assert_eq!(TradSesStatusRejReason::Other.to_fix(), "99");

        assert_eq!(TradSesStatusRejReason::from_fix("1"), Some(TradSesStatusRejReason::UnknownOrInvalidTradingSessionID));
        assert_eq!(TradSesStatusRejReason::from_fix("99"), Some(TradSesStatusRejReason::Other));
        assert_eq!(TradSesStatusRejReason::from_fix("100"), None);
    }

    #[test]
    fn test_trad_ses_update_action_conversions() {
        assert_eq!(TradSesUpdateAction::Add.to_fix(), 'A');
        assert_eq!(TradSesUpdateAction::Delete.to_fix(), 'D');
        assert_eq!(TradSesUpdateAction::Modify.to_fix(), 'M');

        assert_eq!(TradSesUpdateAction::from_fix('A'), Some(TradSesUpdateAction::Add));
        assert_eq!(TradSesUpdateAction::from_fix('D'), Some(TradSesUpdateAction::Delete));
        assert_eq!(TradSesUpdateAction::from_fix('M'), Some(TradSesUpdateAction::Modify));
        assert_eq!(TradSesUpdateAction::from_fix('Z'), None);
    }

    #[test]
    fn test_market_update_action_conversions() {
        assert_eq!(MarketUpdateAction::Add.to_fix(), 'A');
        assert_eq!(MarketUpdateAction::Delete.to_fix(), 'D');
        assert_eq!(MarketUpdateAction::Modify.to_fix(), 'M');

        assert_eq!(MarketUpdateAction::from_fix('A'), Some(MarketUpdateAction::Add));
        assert_eq!(MarketUpdateAction::from_fix('D'), Some(MarketUpdateAction::Delete));
        assert_eq!(MarketUpdateAction::from_fix('M'), Some(MarketUpdateAction::Modify));
        assert_eq!(MarketUpdateAction::from_fix('Z'), None);
    }
}

// ============================================================================
// [SECTION 600] Securities Reference Data Messages
// Messages: SecurityDefinition(d), SecurityDefinitionRequest(c),
//           SecurityDefinitionUpdateReport(BP), SecurityList(y),
//           SecurityListRequest(x), SecurityListUpdateReport(BK),
//           SecurityStatus(f), SecurityStatusRequest(e), SecurityTypes(w),
//           SecurityTypeRequest(v), DerivativeSecurityList(AA),
//           DerivativeSecurityListRequest(z), DerivativeSecurityListUpdateReport(BR),
//           SecurityMassStatus(CO), SecurityMassStatusRequest(CN)
// Tags: 167, 460, 461, 541, 555, 762-764, 827-829, etc.
// Implementation: feature/pretrade-securities-reference
// ============================================================================

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

// ============================================================================
// Post-Trade: Account Reporting Enums (Section 700)
// Reserved for AccountReportingType, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Allocation Enums (Section 720)
// Reserved for AllocType, AllocTransType, AllocStatus, AllocRejCode, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Confirmation Enums (Section 730)
// Reserved for ConfirmType, ConfirmStatus, ConfirmTransType, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Position Maintenance Enums (Section 710)
// Reserved for PosReqType, PosTransType, PosMaintAction, PosMaintResult, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Settlement Instruction Enums (Section 740)
// Reserved for SettlInstSource, SettlInstTransType, SettlInstReqRejCode, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Trade Capture Reporting Enums (Section 750)
// Reserved for TradeRequestType, TradeReportType, TrdType, etc.
// ============================================================================


// ============================================================================
// Securities Reference Enum Tests
// ============================================================================

#[cfg(test)]
mod securities_reference_tests {
    use super::*;

    #[test]
    fn test_security_request_type_conversions() {
        assert_eq!(SecurityRequestType::RequestSecurityIdentityAndSpecifications.to_fix(), '0');
        assert_eq!(SecurityRequestType::RequestSecurityIdentityForSpecifications.to_fix(), '1');
        assert_eq!(SecurityRequestType::RequestListSecurityTypes.to_fix(), '2');
        assert_eq!(SecurityRequestType::RequestListSecurities.to_fix(), '3');
        assert_eq!(SecurityRequestType::Symbol.to_fix(), '4');
        assert_eq!(SecurityRequestType::SecurityTypeOrCFICode.to_fix(), '5');
        assert_eq!(SecurityRequestType::Product.to_fix(), '6');
        assert_eq!(SecurityRequestType::TradingSessionID.to_fix(), '7');
        assert_eq!(SecurityRequestType::AllSecurities.to_fix(), '8');
        assert_eq!(SecurityRequestType::MarketIDOrMarketIDPlusMarketSegmentID.to_fix(), '9');

        assert_eq!(SecurityRequestType::from_fix('0'), Some(SecurityRequestType::RequestSecurityIdentityAndSpecifications));
        assert_eq!(SecurityRequestType::from_fix('5'), Some(SecurityRequestType::SecurityTypeOrCFICode));
        assert_eq!(SecurityRequestType::from_fix('9'), Some(SecurityRequestType::MarketIDOrMarketIDPlusMarketSegmentID));
        assert_eq!(SecurityRequestType::from_fix('X'), None);
    }

    #[test]
    fn test_security_request_result_conversions() {
        assert_eq!(SecurityRequestResult::ValidRequest.to_fix(), '0');
        assert_eq!(SecurityRequestResult::InvalidOrUnsupportedRequest.to_fix(), '1');
        assert_eq!(SecurityRequestResult::NoInstrumentsFound.to_fix(), '2');
        assert_eq!(SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData.to_fix(), '3');
        assert_eq!(SecurityRequestResult::InstrumentDataTemporarilyUnavailable.to_fix(), '4');
        assert_eq!(SecurityRequestResult::RequestForInstrumentDataNotSupported.to_fix(), '5');

        assert_eq!(SecurityRequestResult::from_fix('0'), Some(SecurityRequestResult::ValidRequest));
        assert_eq!(SecurityRequestResult::from_fix('3'), Some(SecurityRequestResult::NotAuthorizedToRetrieveInstrumentData));
        assert_eq!(SecurityRequestResult::from_fix('5'), Some(SecurityRequestResult::RequestForInstrumentDataNotSupported));
        assert_eq!(SecurityRequestResult::from_fix('9'), None);
    }

    #[test]
    fn test_security_list_request_type_conversions() {
        assert_eq!(SecurityListRequestType::Symbol.to_fix(), '0');
        assert_eq!(SecurityListRequestType::SecurityTypeAndOrCFICode.to_fix(), '1');
        assert_eq!(SecurityListRequestType::Product.to_fix(), '2');
        assert_eq!(SecurityListRequestType::TradingSessionID.to_fix(), '3');
        assert_eq!(SecurityListRequestType::AllSecurities.to_fix(), '4');
        assert_eq!(SecurityListRequestType::MarketIDOrMarketIDPlusMarketSegmentID.to_fix(), '5');

        assert_eq!(SecurityListRequestType::from_fix('0'), Some(SecurityListRequestType::Symbol));
        assert_eq!(SecurityListRequestType::from_fix('1'), Some(SecurityListRequestType::SecurityTypeAndOrCFICode));
        assert_eq!(SecurityListRequestType::from_fix('5'), Some(SecurityListRequestType::MarketIDOrMarketIDPlusMarketSegmentID));
        assert_eq!(SecurityListRequestType::from_fix('9'), None);
    }

    #[test]
    fn test_security_update_action_conversions() {
        assert_eq!(SecurityUpdateAction::Add.to_fix(), 'A');
        assert_eq!(SecurityUpdateAction::Delete.to_fix(), 'D');
        assert_eq!(SecurityUpdateAction::Modify.to_fix(), 'M');

        assert_eq!(SecurityUpdateAction::from_fix('A'), Some(SecurityUpdateAction::Add));
        assert_eq!(SecurityUpdateAction::from_fix('D'), Some(SecurityUpdateAction::Delete));
        assert_eq!(SecurityUpdateAction::from_fix('M'), Some(SecurityUpdateAction::Modify));
        assert_eq!(SecurityUpdateAction::from_fix('X'), None);
    }

    #[test]
    fn test_security_trading_status_conversions() {
        assert_eq!(SecurityTradingStatus::OpeningDelay.to_fix(), "1");
        assert_eq!(SecurityTradingStatus::TradingHalt.to_fix(), "2");
        assert_eq!(SecurityTradingStatus::Resume.to_fix(), "3");
        assert_eq!(SecurityTradingStatus::NoOpenNoResume.to_fix(), "4");
        assert_eq!(SecurityTradingStatus::PriceIndication.to_fix(), "5");
        assert_eq!(SecurityTradingStatus::MarketOnCloseImbalanceSell.to_fix(), "10");
        assert_eq!(SecurityTradingStatus::NoMarketImbalance.to_fix(), "12");
        assert_eq!(SecurityTradingStatus::PreOpen.to_fix(), "21");
        assert_eq!(SecurityTradingStatus::PostClose.to_fix(), "26");

        assert_eq!(SecurityTradingStatus::from_fix("1"), Some(SecurityTradingStatus::OpeningDelay));
        assert_eq!(SecurityTradingStatus::from_fix("2"), Some(SecurityTradingStatus::TradingHalt));
        assert_eq!(SecurityTradingStatus::from_fix("10"), Some(SecurityTradingStatus::MarketOnCloseImbalanceSell));
        assert_eq!(SecurityTradingStatus::from_fix("12"), Some(SecurityTradingStatus::NoMarketImbalance));
        assert_eq!(SecurityTradingStatus::from_fix("26"), Some(SecurityTradingStatus::PostClose));
        assert_eq!(SecurityTradingStatus::from_fix("99"), None);
    }

    #[test]
    fn test_halt_reason_conversions() {
        assert_eq!(HaltReason::NewsDissemination.to_fix(), '0');
        assert_eq!(HaltReason::OrderInflux.to_fix(), '1');
        assert_eq!(HaltReason::OrderImbalance.to_fix(), '2');
        assert_eq!(HaltReason::AdditionalInformation.to_fix(), '3');
        assert_eq!(HaltReason::NewsPending.to_fix(), '4');
        assert_eq!(HaltReason::EquipmentChangeover.to_fix(), '5');

        assert_eq!(HaltReason::from_fix('0'), Some(HaltReason::NewsDissemination));
        assert_eq!(HaltReason::from_fix('2'), Some(HaltReason::OrderImbalance));
        assert_eq!(HaltReason::from_fix('5'), Some(HaltReason::EquipmentChangeover));
        assert_eq!(HaltReason::from_fix('9'), None);
    }
}
