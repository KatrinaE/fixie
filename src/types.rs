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
