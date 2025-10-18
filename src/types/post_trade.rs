use serde::{Deserialize, Serialize};


/// AllocTransType (Tag 71) - Identifies allocation transaction type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocTransType {
    New,                              // 0
    Replace,                          // 1
    Cancel,                           // 2
    Preliminary,                      // 3 - Deprecated
    Calculated,                       // 4 - Deprecated
    CalculatedWithoutPreliminary,     // 5 - Deprecated
    Reversal,                         // 6
}

impl AllocTransType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::New => "0",
            Self::Replace => "1",
            Self::Cancel => "2",
            Self::Preliminary => "3",
            Self::Calculated => "4",
            Self::CalculatedWithoutPreliminary => "5",
            Self::Reversal => "6",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "0" => Ok(Self::New),
            "1" => Ok(Self::Replace),
            "2" => Ok(Self::Cancel),
            "3" => Ok(Self::Preliminary),
            "4" => Ok(Self::Calculated),
            "5" => Ok(Self::CalculatedWithoutPreliminary),
            "6" => Ok(Self::Reversal),
            _ => Err(format!("Invalid AllocTransType: {}", value)),
        }
    }
}


/// AllocType (Tag 626) - Purpose of allocation message
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocType {
    Calculated,                                  // 1
    Preliminary,                                 // 2
    SellsideCalculatedUsingPreliminary,         // 3 - Deprecated
    SellsideCalculatedWithoutPreliminary,       // 4 - Deprecated
    ReadyToBookSingleOrder,                     // 5
    BuysideReadyToBookCombinedSetOfOrders,      // 6 - Deprecated
    WarehouseInstruction,                        // 7
    RequestToIntermediary,                       // 8
    Accept,                                      // 9
    Reject,                                      // 10
    AcceptPending,                               // 11
    IncompleteGroup,                             // 12
    CompleteGroup,                               // 13
    ReversalPending,                             // 14
}

impl AllocType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::Calculated => "1",
            Self::Preliminary => "2",
            Self::SellsideCalculatedUsingPreliminary => "3",
            Self::SellsideCalculatedWithoutPreliminary => "4",
            Self::ReadyToBookSingleOrder => "5",
            Self::BuysideReadyToBookCombinedSetOfOrders => "6",
            Self::WarehouseInstruction => "7",
            Self::RequestToIntermediary => "8",
            Self::Accept => "9",
            Self::Reject => "10",
            Self::AcceptPending => "11",
            Self::IncompleteGroup => "12",
            Self::CompleteGroup => "13",
            Self::ReversalPending => "14",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "1" => Ok(Self::Calculated),
            "2" => Ok(Self::Preliminary),
            "3" => Ok(Self::SellsideCalculatedUsingPreliminary),
            "4" => Ok(Self::SellsideCalculatedWithoutPreliminary),
            "5" => Ok(Self::ReadyToBookSingleOrder),
            "6" => Ok(Self::BuysideReadyToBookCombinedSetOfOrders),
            "7" => Ok(Self::WarehouseInstruction),
            "8" => Ok(Self::RequestToIntermediary),
            "9" => Ok(Self::Accept),
            "10" => Ok(Self::Reject),
            "11" => Ok(Self::AcceptPending),
            "12" => Ok(Self::IncompleteGroup),
            "13" => Ok(Self::CompleteGroup),
            "14" => Ok(Self::ReversalPending),
            _ => Err(format!("Invalid AllocType: {}", value)),
        }
    }
}


/// AllocStatus (Tag 87) - Status of allocation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocStatus {
    Accepted,                // 0
    BlockLevelReject,        // 1
    AccountLevelReject,      // 2
    Received,                // 3
    Incomplete,              // 4
    RejectedByIntermediary,  // 5
    AllocationPending,       // 6
    Reversed,                // 7
}

impl AllocStatus {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::Accepted => "0",
            Self::BlockLevelReject => "1",
            Self::AccountLevelReject => "2",
            Self::Received => "3",
            Self::Incomplete => "4",
            Self::RejectedByIntermediary => "5",
            Self::AllocationPending => "6",
            Self::Reversed => "7",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "0" => Ok(Self::Accepted),
            "1" => Ok(Self::BlockLevelReject),
            "2" => Ok(Self::AccountLevelReject),
            "3" => Ok(Self::Received),
            "4" => Ok(Self::Incomplete),
            "5" => Ok(Self::RejectedByIntermediary),
            "6" => Ok(Self::AllocationPending),
            "7" => Ok(Self::Reversed),
            _ => Err(format!("Invalid AllocStatus: {}", value)),
        }
    }
}


/// AllocRejCode (Tag 88) - Reason for rejection of allocation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocRejCode {
    UnknownAccount,                  // 0
    IncorrectQuantity,               // 1
    IncorrectAveragegPrice,          // 2
    UnknownExecutingBrokerMnemonic,  // 3
    CommissionDifference,            // 4
    UnknownOrderID,                  // 5
    UnknownListID,                   // 6
    Other,                           // 7
    IncorrectAllocatedQuantity,      // 8
    CalculationDifference,           // 9
    UnknownOrStaleExecID,            // 10
    MismatchedData,                  // 11
    UnknownClOrdID,                  // 12
    WarehouseRequestRejected,        // 13
    OtherExtended,                   // 99
}

impl AllocRejCode {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::UnknownAccount => "0",
            Self::IncorrectQuantity => "1",
            Self::IncorrectAveragegPrice => "2",
            Self::UnknownExecutingBrokerMnemonic => "3",
            Self::CommissionDifference => "4",
            Self::UnknownOrderID => "5",
            Self::UnknownListID => "6",
            Self::Other => "7",
            Self::IncorrectAllocatedQuantity => "8",
            Self::CalculationDifference => "9",
            Self::UnknownOrStaleExecID => "10",
            Self::MismatchedData => "11",
            Self::UnknownClOrdID => "12",
            Self::WarehouseRequestRejected => "13",
            Self::OtherExtended => "99",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "0" => Ok(Self::UnknownAccount),
            "1" => Ok(Self::IncorrectQuantity),
            "2" => Ok(Self::IncorrectAveragegPrice),
            "3" => Ok(Self::UnknownExecutingBrokerMnemonic),
            "4" => Ok(Self::CommissionDifference),
            "5" => Ok(Self::UnknownOrderID),
            "6" => Ok(Self::UnknownListID),
            "7" => Ok(Self::Other),
            "8" => Ok(Self::IncorrectAllocatedQuantity),
            "9" => Ok(Self::CalculationDifference),
            "10" => Ok(Self::UnknownOrStaleExecID),
            "11" => Ok(Self::MismatchedData),
            "12" => Ok(Self::UnknownClOrdID),
            "13" => Ok(Self::WarehouseRequestRejected),
            "99" => Ok(Self::OtherExtended),
            _ => Err(format!("Invalid AllocRejCode: {}", value)),
        }
    }
}


/// AllocCancReplaceReason (Tag 796) - Reason for cancellation or replacement
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocCancReplaceReason {
    OriginalDetailsIncorrect,    // 1
    ChangeInUnderlyingOrderDetails, // 2
    Other,                       // 99
}

impl AllocCancReplaceReason {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::OriginalDetailsIncorrect => "1",
            Self::ChangeInUnderlyingOrderDetails => "2",
            Self::Other => "99",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "1" => Ok(Self::OriginalDetailsIncorrect),
            "2" => Ok(Self::ChangeInUnderlyingOrderDetails),
            "99" => Ok(Self::Other),
            _ => Err(format!("Invalid AllocCancReplaceReason: {}", value)),
        }
    }
}


/// AllocIntermedReqType (Tag 808) - Type of intermediary request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocIntermedReqType {
    PendingAccept,           // 1
    PendingRelease,          // 2
    PendingReversal,         // 3
    Accept,                  // 4
    BlockLevelReject,        // 5
    AccountLevelReject,      // 6
}

impl AllocIntermedReqType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::PendingAccept => "1",
            Self::PendingRelease => "2",
            Self::PendingReversal => "3",
            Self::Accept => "4",
            Self::BlockLevelReject => "5",
            Self::AccountLevelReject => "6",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "1" => Ok(Self::PendingAccept),
            "2" => Ok(Self::PendingRelease),
            "3" => Ok(Self::PendingReversal),
            "4" => Ok(Self::Accept),
            "5" => Ok(Self::BlockLevelReject),
            "6" => Ok(Self::AccountLevelReject),
            _ => Err(format!("Invalid AllocIntermedReqType: {}", value)),
        }
    }
}


/// AllocReportType (Tag 794) - Type of allocation report
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocReportType {
    PreliminaryRequestToIntermediary,          // 2
    SellsideCalculatedUsingPreliminary,        // 3
    SellsideCalculatedWithoutPreliminary,      // 4
    WarehouseRecap,                            // 5
    RequestToIntermediary,                     // 8
    Accept,                                    // 9
    Reject,                                    // 10
    AcceptPending,                             // 11
    Complete,                                  // 12
    ReversePending,                            // 14
}

impl AllocReportType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::PreliminaryRequestToIntermediary => "2",
            Self::SellsideCalculatedUsingPreliminary => "3",
            Self::SellsideCalculatedWithoutPreliminary => "4",
            Self::WarehouseRecap => "5",
            Self::RequestToIntermediary => "8",
            Self::Accept => "9",
            Self::Reject => "10",
            Self::AcceptPending => "11",
            Self::Complete => "12",
            Self::ReversePending => "14",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "2" => Ok(Self::PreliminaryRequestToIntermediary),
            "3" => Ok(Self::SellsideCalculatedUsingPreliminary),
            "4" => Ok(Self::SellsideCalculatedWithoutPreliminary),
            "5" => Ok(Self::WarehouseRecap),
            "8" => Ok(Self::RequestToIntermediary),
            "9" => Ok(Self::Accept),
            "10" => Ok(Self::Reject),
            "11" => Ok(Self::AcceptPending),
            "12" => Ok(Self::Complete),
            "14" => Ok(Self::ReversePending),
            _ => Err(format!("Invalid AllocReportType: {}", value)),
        }
    }
}


/// AvgPxIndicator (Tag 819) - Average pricing indicator
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AvgPxIndicator {
    NoAveragePricing,                  // 0
    TradeIsPartOfAveragePriceGroup,    // 1
    LastTradeIsAveragePriceGroup,      // 2
}

impl AvgPxIndicator {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::NoAveragePricing => "0",
            Self::TradeIsPartOfAveragePriceGroup => "1",
            Self::LastTradeIsAveragePriceGroup => "2",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "0" => Ok(Self::NoAveragePricing),
            "1" => Ok(Self::TradeIsPartOfAveragePriceGroup),
            "2" => Ok(Self::LastTradeIsAveragePriceGroup),
            _ => Err(format!("Invalid AvgPxIndicator: {}", value)),
        }
    }
}


/// AllocRequestStatus (Tag 2768) - Status of allocation instruction alert request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AllocRequestStatus {
    Accepted,    // 0
    Rejected,    // 1
}

impl AllocRequestStatus {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::Accepted => "0",
            Self::Rejected => "1",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "0" => Ok(Self::Accepted),
            "1" => Ok(Self::Rejected),
            _ => Err(format!("Invalid AllocRequestStatus: {}", value)),
        }
    }
}


/// MatchStatus (Tag 573) - Status of match
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MatchStatus {
    ComparedMatchedOrAffirmed,           // 0
    UncomparedUnmatchedOrUnaffirmed,     // 1
    AdvisoryOrAlert,                     // 2
}

impl MatchStatus {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::ComparedMatchedOrAffirmed => "0",
            Self::UncomparedUnmatchedOrUnaffirmed => "1",
            Self::AdvisoryOrAlert => "2",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "0" => Ok(Self::ComparedMatchedOrAffirmed),
            "1" => Ok(Self::UncomparedUnmatchedOrUnaffirmed),
            "2" => Ok(Self::AdvisoryOrAlert),
            _ => Err(format!("Invalid MatchStatus: {}", value)),
        }
    }
}


/// IndividualAllocRejCode (Tag 776) - Reason for rejection at individual allocation level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IndividualAllocRejCode {
    UnknownAccount,                  // 0
    IncorrectQuantity,               // 1
    IncorrectAveragePrice,           // 2
    UnknownExecutingBrokerMnemonic,  // 3
    CommissionDifference,            // 4
    UnknownOrderID,                  // 5
    UnknownListID,                   // 6
    Other,                           // 7
    IncorrectAllocatedQuantity,      // 8
    CalculationDifference,           // 9
    UnknownOrStaleExecID,            // 10
    MismatchedData,                  // 11
    UnknownClOrdID,                  // 12
    WarehouseRequestRejected,        // 13
}

impl IndividualAllocRejCode {
    pub fn to_fix(&self) -> &'static str {
        match self {
            Self::UnknownAccount => "0",
            Self::IncorrectQuantity => "1",
            Self::IncorrectAveragePrice => "2",
            Self::UnknownExecutingBrokerMnemonic => "3",
            Self::CommissionDifference => "4",
            Self::UnknownOrderID => "5",
            Self::UnknownListID => "6",
            Self::Other => "7",
            Self::IncorrectAllocatedQuantity => "8",
            Self::CalculationDifference => "9",
            Self::UnknownOrStaleExecID => "10",
            Self::MismatchedData => "11",
            Self::UnknownClOrdID => "12",
            Self::WarehouseRequestRejected => "13",
        }
    }

    pub fn from_fix(value: &str) -> Result<Self, String> {
        match value {
            "0" => Ok(Self::UnknownAccount),
            "1" => Ok(Self::IncorrectQuantity),
            "2" => Ok(Self::IncorrectAveragePrice),
            "3" => Ok(Self::UnknownExecutingBrokerMnemonic),
            "4" => Ok(Self::CommissionDifference),
            "5" => Ok(Self::UnknownOrderID),
            "6" => Ok(Self::UnknownListID),
            "7" => Ok(Self::Other),
            "8" => Ok(Self::IncorrectAllocatedQuantity),
            "9" => Ok(Self::CalculationDifference),
            "10" => Ok(Self::UnknownOrStaleExecID),
            "11" => Ok(Self::MismatchedData),
            "12" => Ok(Self::UnknownClOrdID),
            "13" => Ok(Self::WarehouseRequestRejected),
            _ => Err(format!("Invalid IndividualAllocRejCode: {}", value)),
        }
    }
}


/// ConfirmType (Tag 773) - Type of Confirmation message
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfirmType {
    /// Status
    Status,
    /// Confirmation
    Confirmation,
    /// Confirmation Request Rejected
    ConfirmationRequestRejected,
}

impl ConfirmType {
    pub fn to_fix(&self) -> u32 {
        match self {
            ConfirmType::Status => 1,
            ConfirmType::Confirmation => 2,
            ConfirmType::ConfirmationRequestRejected => 3,
        }
    }

    pub fn from_fix(value: u32) -> Option<Self> {
        match value {
            1 => Some(ConfirmType::Status),
            2 => Some(ConfirmType::Confirmation),
            3 => Some(ConfirmType::ConfirmationRequestRejected),
            _ => None,
        }
    }
}


/// ConfirmStatus (Tag 665) - Status of Confirmation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfirmStatus {
    /// Received
    Received,
    /// Mismatched Account
    MismatchedAccount,
    /// Missing Settlement Instructions
    MissingSettlementInstructions,
    /// Confirmed
    Confirmed,
    /// Request Rejected
    RequestRejected,
}

impl ConfirmStatus {
    pub fn to_fix(&self) -> u32 {
        match self {
            ConfirmStatus::Received => 1,
            ConfirmStatus::MismatchedAccount => 2,
            ConfirmStatus::MissingSettlementInstructions => 3,
            ConfirmStatus::Confirmed => 4,
            ConfirmStatus::RequestRejected => 5,
        }
    }

    pub fn from_fix(value: u32) -> Option<Self> {
        match value {
            1 => Some(ConfirmStatus::Received),
            2 => Some(ConfirmStatus::MismatchedAccount),
            3 => Some(ConfirmStatus::MissingSettlementInstructions),
            4 => Some(ConfirmStatus::Confirmed),
            5 => Some(ConfirmStatus::RequestRejected),
            _ => None,
        }
    }
}


/// ConfirmTransType (Tag 666) - Confirmation transaction type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfirmTransType {
    /// New
    New,
    /// Replace
    Replace,
    /// Cancel
    Cancel,
}

impl ConfirmTransType {
    pub fn to_fix(&self) -> u32 {
        match self {
            ConfirmTransType::New => 0,
            ConfirmTransType::Replace => 1,
            ConfirmTransType::Cancel => 2,
        }
    }

    pub fn from_fix(value: u32) -> Option<Self> {
        match value {
            0 => Some(ConfirmTransType::New),
            1 => Some(ConfirmTransType::Replace),
            2 => Some(ConfirmTransType::Cancel),
            _ => None,
        }
    }
}


/// AffirmStatus (Tag 940) - Status of affirmation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AffirmStatus {
    /// Received
    Received,
    /// Confirm Rejected (not affirmed)
    ConfirmRejected,
    /// Affirmed
    Affirmed,
}

impl AffirmStatus {
    pub fn to_fix(&self) -> u32 {
        match self {
            AffirmStatus::Received => 1,
            AffirmStatus::ConfirmRejected => 2,
            AffirmStatus::Affirmed => 3,
        }
    }

    pub fn from_fix(value: u32) -> Option<Self> {
        match value {
            1 => Some(AffirmStatus::Received),
            2 => Some(AffirmStatus::ConfirmRejected),
            3 => Some(AffirmStatus::Affirmed),
            _ => None,
        }
    }
}


/// ConfirmRejReason (Tag 774) - Reason for confirmation rejection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfirmRejReason {
    /// Mismatched account
    MismatchedAccount,
    /// Missing settlement instructions
    MissingSettlementInstructions,
    /// Other (Reserved100Plus values allowed)
    Other,
}

impl ConfirmRejReason {
    pub fn to_fix(&self) -> u32 {
        match self {
            ConfirmRejReason::MismatchedAccount => 1,
            ConfirmRejReason::MissingSettlementInstructions => 2,
            ConfirmRejReason::Other => 99,
        }
    }

    pub fn from_fix(value: u32) -> Option<Self> {
        match value {
            1 => Some(ConfirmRejReason::MismatchedAccount),
            2 => Some(ConfirmRejReason::MissingSettlementInstructions),
            99 => Some(ConfirmRejReason::Other),
            _ if value >= 100 => Some(ConfirmRejReason::Other), // Reserved100Plus
            _ => None,
        }
    }
}


/// PosReqType (Tag 724) - Type of position request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PosReqType {
    Positions,          // 0
    Trades,             // 1
    Exercises,          // 2
    Assignments,        // 3
    SettlementActivity, // 4
    BackoutMessage,     // 5
    DeltaPositions,     // 6
}

impl PosReqType {
    pub fn to_fix(&self) -> char {
        match self {
            PosReqType::Positions => '0',
            PosReqType::Trades => '1',
            PosReqType::Exercises => '2',
            PosReqType::Assignments => '3',
            PosReqType::SettlementActivity => '4',
            PosReqType::BackoutMessage => '5',
            PosReqType::DeltaPositions => '6',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(PosReqType::Positions),
            '1' => Some(PosReqType::Trades),
            '2' => Some(PosReqType::Exercises),
            '3' => Some(PosReqType::Assignments),
            '4' => Some(PosReqType::SettlementActivity),
            '5' => Some(PosReqType::BackoutMessage),
            '6' => Some(PosReqType::DeltaPositions),
            _ => None,
        }
    }
}


/// PosTransType (Tag 709) - Type of position transaction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PosTransType {
    Exercise,                  // 1
    DoNotExercise,             // 2
    PositionAdjustment,        // 3
    PositionChangeSubmission,  // 4
    Pledge,                    // 5
    LargeTraderSubmission,     // 6
    LargePositionsReporting,   // 7
    LongHoldings,              // 8
    InternalTransfer,          // 9
    TransferOfFirm,            // 10
    ExternalTransfer,          // 11
    CorporateAction,           // 12
    Notification,              // 13
    PositionCreation,          // 14
    CloseOut,                  // 15
    Reopen,                    // 16
}

impl PosTransType {
    pub fn to_fix(&self) -> &str {
        match self {
            PosTransType::Exercise => "1",
            PosTransType::DoNotExercise => "2",
            PosTransType::PositionAdjustment => "3",
            PosTransType::PositionChangeSubmission => "4",
            PosTransType::Pledge => "5",
            PosTransType::LargeTraderSubmission => "6",
            PosTransType::LargePositionsReporting => "7",
            PosTransType::LongHoldings => "8",
            PosTransType::InternalTransfer => "9",
            PosTransType::TransferOfFirm => "10",
            PosTransType::ExternalTransfer => "11",
            PosTransType::CorporateAction => "12",
            PosTransType::Notification => "13",
            PosTransType::PositionCreation => "14",
            PosTransType::CloseOut => "15",
            PosTransType::Reopen => "16",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "1" => Some(PosTransType::Exercise),
            "2" => Some(PosTransType::DoNotExercise),
            "3" => Some(PosTransType::PositionAdjustment),
            "4" => Some(PosTransType::PositionChangeSubmission),
            "5" => Some(PosTransType::Pledge),
            "6" => Some(PosTransType::LargeTraderSubmission),
            "7" => Some(PosTransType::LargePositionsReporting),
            "8" => Some(PosTransType::LongHoldings),
            "9" => Some(PosTransType::InternalTransfer),
            "10" => Some(PosTransType::TransferOfFirm),
            "11" => Some(PosTransType::ExternalTransfer),
            "12" => Some(PosTransType::CorporateAction),
            "13" => Some(PosTransType::Notification),
            "14" => Some(PosTransType::PositionCreation),
            "15" => Some(PosTransType::CloseOut),
            "16" => Some(PosTransType::Reopen),
            _ => None,
        }
    }
}


/// PosMaintAction (Tag 712) - Maintenance action being performed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PosMaintAction {
    New,     // 1
    Replace, // 2
    Cancel,  // 3
    Reverse, // 4
}

impl PosMaintAction {
    pub fn to_fix(&self) -> char {
        match self {
            PosMaintAction::New => '1',
            PosMaintAction::Replace => '2',
            PosMaintAction::Cancel => '3',
            PosMaintAction::Reverse => '4',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(PosMaintAction::New),
            '2' => Some(PosMaintAction::Replace),
            '3' => Some(PosMaintAction::Cancel),
            '4' => Some(PosMaintAction::Reverse),
            _ => None,
        }
    }
}


/// PosMaintResult (Tag 723) - Result of position maintenance
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PosMaintResult {
    SuccessfulCompletion, // 0
    Rejected,             // 1
    Other,                // 99
}

impl PosMaintResult {
    pub fn to_fix(&self) -> &str {
        match self {
            PosMaintResult::SuccessfulCompletion => "0",
            PosMaintResult::Rejected => "1",
            PosMaintResult::Other => "99",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(PosMaintResult::SuccessfulCompletion),
            "1" => Some(PosMaintResult::Rejected),
            "99" => Some(PosMaintResult::Other),
            _ => None,
        }
    }
}


/// PosReqStatus (Tag 729) - Status of position request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PosReqStatus {
    Completed,            // 0
    CompletedWithWarnings, // 1
    Rejected,             // 2
}

impl PosReqStatus {
    pub fn to_fix(&self) -> char {
        match self {
            PosReqStatus::Completed => '0',
            PosReqStatus::CompletedWithWarnings => '1',
            PosReqStatus::Rejected => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(PosReqStatus::Completed),
            '1' => Some(PosReqStatus::CompletedWithWarnings),
            '2' => Some(PosReqStatus::Rejected),
            _ => None,
        }
    }
}


/// PosReqResult (Tag 728) - Result of position request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PosReqResult {
    ValidRequest,                // 0
    InvalidOrUnsupportedRequest, // 1
    NoPositionsFound,            // 2
    NotAuthorized,               // 3
    RequestNotSupported,         // 4
    Other,                       // 99
}

impl PosReqResult {
    pub fn to_fix(&self) -> &str {
        match self {
            PosReqResult::ValidRequest => "0",
            PosReqResult::InvalidOrUnsupportedRequest => "1",
            PosReqResult::NoPositionsFound => "2",
            PosReqResult::NotAuthorized => "3",
            PosReqResult::RequestNotSupported => "4",
            PosReqResult::Other => "99",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(PosReqResult::ValidRequest),
            "1" => Some(PosReqResult::InvalidOrUnsupportedRequest),
            "2" => Some(PosReqResult::NoPositionsFound),
            "3" => Some(PosReqResult::NotAuthorized),
            "4" => Some(PosReqResult::RequestNotSupported),
            "99" => Some(PosReqResult::Other),
            _ => None,
        }
    }
}


/// PosType (Tag 703) - Type of position quantity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PosType {
    AllocationTradeQty,           // ALC
    OptionAssignment,             // AS
    AsOfTradeQty,                 // ASF
    DeliveryQty,                  // DLV
    ElectronicTradeQty,           // ETR
    OptionExercise,               // EX
    EndOfDayQty,                  // FIN
    IntraSpreadQty,               // IAS
    InterSpreadQty,               // IES
    AdjustmentQty,                // PA
    PitTradeQty,                  // PIT
    StartOfDayQty,                // SOD
    IntegralSplit,                // SPL
    TransactionFromAssignment,    // TA
    TotalTransactionQty,          // TOT
    TransactionQuantity,          // TQ
    TransferTradeQty,             // TRF
    TransactionFromExercise,      // TX
    CrossMarginQty,               // XM
    ReceiveQuantity,              // RCV
    CorporateActionAdjustment,    // CAA
    DeliveryNoticeQty,            // DN
    ExchangeForPhysicalQty,       // EP
    PrivatelyNegotiatedTradeQty,  // PNTN
    NetDeltaQty,                  // DLT
    CreditEventAdjustment,        // CEA
    SuccessionEventAdjustment,    // SEA
}

impl PosType {
    pub fn to_fix(&self) -> &str {
        match self {
            PosType::AllocationTradeQty => "ALC",
            PosType::OptionAssignment => "AS",
            PosType::AsOfTradeQty => "ASF",
            PosType::DeliveryQty => "DLV",
            PosType::ElectronicTradeQty => "ETR",
            PosType::OptionExercise => "EX",
            PosType::EndOfDayQty => "FIN",
            PosType::IntraSpreadQty => "IAS",
            PosType::InterSpreadQty => "IES",
            PosType::AdjustmentQty => "PA",
            PosType::PitTradeQty => "PIT",
            PosType::StartOfDayQty => "SOD",
            PosType::IntegralSplit => "SPL",
            PosType::TransactionFromAssignment => "TA",
            PosType::TotalTransactionQty => "TOT",
            PosType::TransactionQuantity => "TQ",
            PosType::TransferTradeQty => "TRF",
            PosType::TransactionFromExercise => "TX",
            PosType::CrossMarginQty => "XM",
            PosType::ReceiveQuantity => "RCV",
            PosType::CorporateActionAdjustment => "CAA",
            PosType::DeliveryNoticeQty => "DN",
            PosType::ExchangeForPhysicalQty => "EP",
            PosType::PrivatelyNegotiatedTradeQty => "PNTN",
            PosType::NetDeltaQty => "DLT",
            PosType::CreditEventAdjustment => "CEA",
            PosType::SuccessionEventAdjustment => "SEA",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "ALC" => Some(PosType::AllocationTradeQty),
            "AS" => Some(PosType::OptionAssignment),
            "ASF" => Some(PosType::AsOfTradeQty),
            "DLV" => Some(PosType::DeliveryQty),
            "ETR" => Some(PosType::ElectronicTradeQty),
            "EX" => Some(PosType::OptionExercise),
            "FIN" => Some(PosType::EndOfDayQty),
            "IAS" => Some(PosType::IntraSpreadQty),
            "IES" => Some(PosType::InterSpreadQty),
            "PA" => Some(PosType::AdjustmentQty),
            "PIT" => Some(PosType::PitTradeQty),
            "SOD" => Some(PosType::StartOfDayQty),
            "SPL" => Some(PosType::IntegralSplit),
            "TA" => Some(PosType::TransactionFromAssignment),
            "TOT" => Some(PosType::TotalTransactionQty),
            "TQ" => Some(PosType::TransactionQuantity),
            "TRF" => Some(PosType::TransferTradeQty),
            "TX" => Some(PosType::TransactionFromExercise),
            "XM" => Some(PosType::CrossMarginQty),
            "RCV" => Some(PosType::ReceiveQuantity),
            "CAA" => Some(PosType::CorporateActionAdjustment),
            "DN" => Some(PosType::DeliveryNoticeQty),
            "EP" => Some(PosType::ExchangeForPhysicalQty),
            "PNTN" => Some(PosType::PrivatelyNegotiatedTradeQty),
            "DLT" => Some(PosType::NetDeltaQty),
            "CEA" => Some(PosType::CreditEventAdjustment),
            "SEA" => Some(PosType::SuccessionEventAdjustment),
            _ => None,
        }
    }
}


/// PosQtyStatus (Tag 706) - Status of position quantity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PosQtyStatus {
    Submitted, // 0
    Accepted,  // 1
    Rejected,  // 2
}

impl PosQtyStatus {
    pub fn to_fix(&self) -> char {
        match self {
            PosQtyStatus::Submitted => '0',
            PosQtyStatus::Accepted => '1',
            PosQtyStatus::Rejected => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(PosQtyStatus::Submitted),
            '1' => Some(PosQtyStatus::Accepted),
            '2' => Some(PosQtyStatus::Rejected),
            _ => None,
        }
    }
}


/// SettlPriceType (Tag 731) - Type of settlement price
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlPriceType {
    Final,       // 1
    Theoretical, // 2
}

impl SettlPriceType {
    pub fn to_fix(&self) -> char {
        match self {
            SettlPriceType::Final => '1',
            SettlPriceType::Theoretical => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(SettlPriceType::Final),
            '2' => Some(SettlPriceType::Theoretical),
            _ => None,
        }
    }
}


/// AdjustmentType (Tag 718) - Type of adjustment
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdjustmentType {
    ProcessRequestAsMarginDisposition, // 0
    DeltaPlus,                         // 1
    DeltaMinus,                        // 2
    Final,                             // 3
}

impl AdjustmentType {
    pub fn to_fix(&self) -> char {
        match self {
            AdjustmentType::ProcessRequestAsMarginDisposition => '0',
            AdjustmentType::DeltaPlus => '1',
            AdjustmentType::DeltaMinus => '2',
            AdjustmentType::Final => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(AdjustmentType::ProcessRequestAsMarginDisposition),
            '1' => Some(AdjustmentType::DeltaPlus),
            '2' => Some(AdjustmentType::DeltaMinus),
            '3' => Some(AdjustmentType::Final),
            _ => None,
        }
    }
}


/// PosAmtType (Tag 707) - Type of position amount
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PosAmtType {
    CashAmount,                       // CASH
    CashResidualAmount,               // CRES
    FinalMarkToMarket,                // FMTM
    IncrementalMarkToMarket,          // IMTM
    PremiumAmount,                    // PREM
    StartOfDayMarkToMarket,           // SMTM
    TradeVariationAmount,             // TVAR
    ValueAdjustedAmount,              // VADJ
    SettlementValue,                  // SETL
    InitialTradeCouponAmount,         // ICPN
    AccruedCouponAmount,              // ACPN
    CouponAmount,                     // CPN
    IncrementalAccruedCoupon,         // IACPN
    CollateralizedMarkToMarket,       // CMTM
    IncrementalCollateralizedMTM,     // ICMTM
    CompensationAmount,               // DLV
    TotalBankedAmount,                // BANK
    TotalCollateralizedAmount,        // COLAT
}

impl PosAmtType {
    pub fn to_fix(&self) -> &str {
        match self {
            PosAmtType::CashAmount => "CASH",
            PosAmtType::CashResidualAmount => "CRES",
            PosAmtType::FinalMarkToMarket => "FMTM",
            PosAmtType::IncrementalMarkToMarket => "IMTM",
            PosAmtType::PremiumAmount => "PREM",
            PosAmtType::StartOfDayMarkToMarket => "SMTM",
            PosAmtType::TradeVariationAmount => "TVAR",
            PosAmtType::ValueAdjustedAmount => "VADJ",
            PosAmtType::SettlementValue => "SETL",
            PosAmtType::InitialTradeCouponAmount => "ICPN",
            PosAmtType::AccruedCouponAmount => "ACPN",
            PosAmtType::CouponAmount => "CPN",
            PosAmtType::IncrementalAccruedCoupon => "IACPN",
            PosAmtType::CollateralizedMarkToMarket => "CMTM",
            PosAmtType::IncrementalCollateralizedMTM => "ICMTM",
            PosAmtType::CompensationAmount => "DLV",
            PosAmtType::TotalBankedAmount => "BANK",
            PosAmtType::TotalCollateralizedAmount => "COLAT",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "CASH" => Some(PosAmtType::CashAmount),
            "CRES" => Some(PosAmtType::CashResidualAmount),
            "FMTM" => Some(PosAmtType::FinalMarkToMarket),
            "IMTM" => Some(PosAmtType::IncrementalMarkToMarket),
            "PREM" => Some(PosAmtType::PremiumAmount),
            "SMTM" => Some(PosAmtType::StartOfDayMarkToMarket),
            "TVAR" => Some(PosAmtType::TradeVariationAmount),
            "VADJ" => Some(PosAmtType::ValueAdjustedAmount),
            "SETL" => Some(PosAmtType::SettlementValue),
            "ICPN" => Some(PosAmtType::InitialTradeCouponAmount),
            "ACPN" => Some(PosAmtType::AccruedCouponAmount),
            "CPN" => Some(PosAmtType::CouponAmount),
            "IACPN" => Some(PosAmtType::IncrementalAccruedCoupon),
            "CMTM" => Some(PosAmtType::CollateralizedMarkToMarket),
            "ICMTM" => Some(PosAmtType::IncrementalCollateralizedMTM),
            "DLV" => Some(PosAmtType::CompensationAmount),
            "BANK" => Some(PosAmtType::TotalBankedAmount),
            "COLAT" => Some(PosAmtType::TotalCollateralizedAmount),
            _ => None,
        }
    }
}


/// SettlInstMode (Tag 160) - Settlement instruction mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlInstMode {
    /// Standing Instructions Provided
    StandingInstructionsProvided,
    /// Specific Order for a single account (for CIV)
    SpecificOrderForSingleAccount,
    /// Request reject
    RequestReject,
}

impl SettlInstMode {
    pub fn to_fix(&self) -> char {
        match self {
            SettlInstMode::StandingInstructionsProvided => '1',
            SettlInstMode::SpecificOrderForSingleAccount => '4',
            SettlInstMode::RequestReject => '5',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(SettlInstMode::StandingInstructionsProvided),
            '4' => Some(SettlInstMode::SpecificOrderForSingleAccount),
            '5' => Some(SettlInstMode::RequestReject),
            _ => None,
        }
    }
}


/// SettlInstTransType (Tag 163) - Settlement instruction transaction type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlInstTransType {
    /// New
    New,
    /// Replace
    Replace,
    /// Cancel
    Cancel,
    /// Restate
    Restate,
}

impl SettlInstTransType {
    pub fn to_fix(&self) -> char {
        match self {
            SettlInstTransType::New => 'N',
            SettlInstTransType::Replace => 'R',
            SettlInstTransType::Cancel => 'C',
            SettlInstTransType::Restate => 'T',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'N' => Some(SettlInstTransType::New),
            'R' => Some(SettlInstTransType::Replace),
            'C' => Some(SettlInstTransType::Cancel),
            'T' => Some(SettlInstTransType::Restate),
            _ => None,
        }
    }
}


/// SettlInstSource (Tag 165) - Settlement instruction source
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlInstSource {
    /// Broker's Instructions
    BrokersInstructions,
    /// Institution's Instructions
    InstitutionsInstructions,
    /// Investor (e.g. CIV use)
    Investor,
}

impl SettlInstSource {
    pub fn to_fix(&self) -> char {
        match self {
            SettlInstSource::BrokersInstructions => '1',
            SettlInstSource::InstitutionsInstructions => '2',
            SettlInstSource::Investor => '3',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(SettlInstSource::BrokersInstructions),
            '2' => Some(SettlInstSource::InstitutionsInstructions),
            '3' => Some(SettlInstSource::Investor),
            _ => None,
        }
    }
}


/// StandInstDbType (Tag 169) - Standing instruction database type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum StandInstDbType {
    /// Other
    Other,
    /// DTC SID
    DtcSid,
    /// Thomson ALERT
    ThomsonAlert,
    /// A Global Custodian
    GlobalCustodian,
    /// AccountNet
    AccountNet,
}

impl StandInstDbType {
    pub fn to_fix(&self) -> char {
        match self {
            StandInstDbType::Other => '0',
            StandInstDbType::DtcSid => '1',
            StandInstDbType::ThomsonAlert => '2',
            StandInstDbType::GlobalCustodian => '3',
            StandInstDbType::AccountNet => '4',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(StandInstDbType::Other),
            '1' => Some(StandInstDbType::DtcSid),
            '2' => Some(StandInstDbType::ThomsonAlert),
            '3' => Some(StandInstDbType::GlobalCustodian),
            '4' => Some(StandInstDbType::AccountNet),
            _ => None,
        }
    }
}


/// SettlInstReqRejCode (Tag 792) - Settlement instruction request rejection reason code
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlInstReqRejCode {
    /// Unable to process request
    UnableToProcess,
    /// Unknown account
    UnknownAccount,
    /// No matching settlement instructions found
    NoMatchingInstructions,
    /// Other
    Other,
}

impl SettlInstReqRejCode {
    pub fn to_fix(&self) -> &str {
        match self {
            SettlInstReqRejCode::UnableToProcess => "0",
            SettlInstReqRejCode::UnknownAccount => "1",
            SettlInstReqRejCode::NoMatchingInstructions => "2",
            SettlInstReqRejCode::Other => "99",
        }
    }

    pub fn from_fix(value: &str) -> Option<Self> {
        match value {
            "0" => Some(SettlInstReqRejCode::UnableToProcess),
            "1" => Some(SettlInstReqRejCode::UnknownAccount),
            "2" => Some(SettlInstReqRejCode::NoMatchingInstructions),
            "99" => Some(SettlInstReqRejCode::Other),
            _ => None,
        }
    }
}


/// SettlObligMode (Tag 1159) - Settlement obligation mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SettlObligMode {
    /// Preliminary
    Preliminary,
    /// Final
    Final,
}

impl SettlObligMode {
    pub fn to_fix(&self) -> char {
        match self {
            SettlObligMode::Preliminary => '1',
            SettlObligMode::Final => '2',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '1' => Some(SettlObligMode::Preliminary),
            '2' => Some(SettlObligMode::Final),
            _ => None,
        }
    }
}


/// TradeRequestType (Tag 569)
/// Identifies the type of Trade Capture Report Request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeRequestType {
    AllTrades,                    // 0
    MatchedTradesMatchingCriteria, // 1
    UnmatchedTradesThatMatchCriteria, // 2
    UnreportedTradesThatMatchCriteria, // 3
    AdvisorysThatMatchCriteria,   // 4
}

impl TradeRequestType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            TradeRequestType::AllTrades => "0",
            TradeRequestType::MatchedTradesMatchingCriteria => "1",
            TradeRequestType::UnmatchedTradesThatMatchCriteria => "2",
            TradeRequestType::UnreportedTradesThatMatchCriteria => "3",
            TradeRequestType::AdvisorysThatMatchCriteria => "4",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(TradeRequestType::AllTrades),
            "1" => Some(TradeRequestType::MatchedTradesMatchingCriteria),
            "2" => Some(TradeRequestType::UnmatchedTradesThatMatchCriteria),
            "3" => Some(TradeRequestType::UnreportedTradesThatMatchCriteria),
            "4" => Some(TradeRequestType::AdvisorysThatMatchCriteria),
            _ => None,
        }
    }
}


/// TradeRequestResult (Tag 749)
/// Result of Trade Capture Report Request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeRequestResult {
    Successful,                  // 0
    InvalidOrUnknownInstrument,  // 1
    InvalidTypeOfTradeRequested, // 2
    InvalidParties,              // 3
    InvalidTransportTypeRequested, // 4
    InvalidDestinationRequested, // 5
    TradeRequestTypeNotSupported, // 8
    Unauthorized,                // 9
    Other,                       // 99
}

impl TradeRequestResult {
    pub fn to_fix(&self) -> &'static str {
        match self {
            TradeRequestResult::Successful => "0",
            TradeRequestResult::InvalidOrUnknownInstrument => "1",
            TradeRequestResult::InvalidTypeOfTradeRequested => "2",
            TradeRequestResult::InvalidParties => "3",
            TradeRequestResult::InvalidTransportTypeRequested => "4",
            TradeRequestResult::InvalidDestinationRequested => "5",
            TradeRequestResult::TradeRequestTypeNotSupported => "8",
            TradeRequestResult::Unauthorized => "9",
            TradeRequestResult::Other => "99",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(TradeRequestResult::Successful),
            "1" => Some(TradeRequestResult::InvalidOrUnknownInstrument),
            "2" => Some(TradeRequestResult::InvalidTypeOfTradeRequested),
            "3" => Some(TradeRequestResult::InvalidParties),
            "4" => Some(TradeRequestResult::InvalidTransportTypeRequested),
            "5" => Some(TradeRequestResult::InvalidDestinationRequested),
            "8" => Some(TradeRequestResult::TradeRequestTypeNotSupported),
            "9" => Some(TradeRequestResult::Unauthorized),
            "99" => Some(TradeRequestResult::Other),
            _ => None,
        }
    }
}


/// TradeRequestStatus (Tag 750)
/// Status of Trade Capture Report Request
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeRequestStatus {
    Accepted,  // 0
    Completed, // 1
    Rejected,  // 2
}

impl TradeRequestStatus {
    pub fn to_fix(&self) -> &'static str {
        match self {
            TradeRequestStatus::Accepted => "0",
            TradeRequestStatus::Completed => "1",
            TradeRequestStatus::Rejected => "2",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(TradeRequestStatus::Accepted),
            "1" => Some(TradeRequestStatus::Completed),
            "2" => Some(TradeRequestStatus::Rejected),
            _ => None,
        }
    }
}


/// TradeReportType (Tag 856)
/// Type of Trade Capture Report
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TradeReportType {
    Submit,                      // 0
    Alleged,                     // 1
    Accept,                      // 2
    Decline,                     // 3
    Addendum,                    // 4
    No,                          // 5
    TradeReportCancel,           // 6
    LockedIn,                    // 7
    Defaulted,                   // 8
    InvalidCMTA,                 // 9
    Pended,                      // 10
    AllegedNew,                  // 11
    AllegedAddendum,             // 12
    AllegedNo,                   // 13
    AllegedTradeReportCancel,    // 14
    AllegedTradeBreak,           // 15
}

impl TradeReportType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            TradeReportType::Submit => "0",
            TradeReportType::Alleged => "1",
            TradeReportType::Accept => "2",
            TradeReportType::Decline => "3",
            TradeReportType::Addendum => "4",
            TradeReportType::No => "5",
            TradeReportType::TradeReportCancel => "6",
            TradeReportType::LockedIn => "7",
            TradeReportType::Defaulted => "8",
            TradeReportType::InvalidCMTA => "9",
            TradeReportType::Pended => "10",
            TradeReportType::AllegedNew => "11",
            TradeReportType::AllegedAddendum => "12",
            TradeReportType::AllegedNo => "13",
            TradeReportType::AllegedTradeReportCancel => "14",
            TradeReportType::AllegedTradeBreak => "15",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(TradeReportType::Submit),
            "1" => Some(TradeReportType::Alleged),
            "2" => Some(TradeReportType::Accept),
            "3" => Some(TradeReportType::Decline),
            "4" => Some(TradeReportType::Addendum),
            "5" => Some(TradeReportType::No),
            "6" => Some(TradeReportType::TradeReportCancel),
            "7" => Some(TradeReportType::LockedIn),
            "8" => Some(TradeReportType::Defaulted),
            "9" => Some(TradeReportType::InvalidCMTA),
            "10" => Some(TradeReportType::Pended),
            "11" => Some(TradeReportType::AllegedNew),
            "12" => Some(TradeReportType::AllegedAddendum),
            "13" => Some(TradeReportType::AllegedNo),
            "14" => Some(TradeReportType::AllegedTradeReportCancel),
            "15" => Some(TradeReportType::AllegedTradeBreak),
            _ => None,
        }
    }
}


/// TrdType (Tag 828)
/// Type of Trade
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrdType {
    RegularTrade,               // 0
    BlockTrade,                 // 1
    EFP,                        // 2 - Exchange for Physical
    Transfer,                   // 3
    LateTrade,                  // 4
    TTrade,                     // 5
    WeightedAveragePriceTrade,  // 6
    BunchedTrade,               // 7
    LateBunchedTrade,           // 8
    PriorReferencePriceTrade,   // 9
    AfterHoursTrade,            // 10
}

impl TrdType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            TrdType::RegularTrade => "0",
            TrdType::BlockTrade => "1",
            TrdType::EFP => "2",
            TrdType::Transfer => "3",
            TrdType::LateTrade => "4",
            TrdType::TTrade => "5",
            TrdType::WeightedAveragePriceTrade => "6",
            TrdType::BunchedTrade => "7",
            TrdType::LateBunchedTrade => "8",
            TrdType::PriorReferencePriceTrade => "9",
            TrdType::AfterHoursTrade => "10",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(TrdType::RegularTrade),
            "1" => Some(TrdType::BlockTrade),
            "2" => Some(TrdType::EFP),
            "3" => Some(TrdType::Transfer),
            "4" => Some(TrdType::LateTrade),
            "5" => Some(TrdType::TTrade),
            "6" => Some(TrdType::WeightedAveragePriceTrade),
            "7" => Some(TrdType::BunchedTrade),
            "8" => Some(TrdType::LateBunchedTrade),
            "9" => Some(TrdType::PriorReferencePriceTrade),
            "10" => Some(TrdType::AfterHoursTrade),
            _ => None,
        }
    }
}


/// TrdSubType (Tag 829)
/// Sub-type of Trade
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrdSubType {
    CMTA,                       // 0
    InternalTransferOrAdjustment, // 1
    ExternalTransferOrAdjustment, // 2
    RejectForSubmittingTrade,   // 3
    AdvisoryForContraSide,      // 4
    OffsetDueToAnAllocation,    // 5
    OnsetDueToAnAllocation,     // 6
    DifferentialSpread,         // 7
    ImpliedSpreadLegExecuted,   // 8
    TransactionFromExercise,    // 9
    TransactionFromAssignment,  // 10
}

impl TrdSubType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            TrdSubType::CMTA => "0",
            TrdSubType::InternalTransferOrAdjustment => "1",
            TrdSubType::ExternalTransferOrAdjustment => "2",
            TrdSubType::RejectForSubmittingTrade => "3",
            TrdSubType::AdvisoryForContraSide => "4",
            TrdSubType::OffsetDueToAnAllocation => "5",
            TrdSubType::OnsetDueToAnAllocation => "6",
            TrdSubType::DifferentialSpread => "7",
            TrdSubType::ImpliedSpreadLegExecuted => "8",
            TrdSubType::TransactionFromExercise => "9",
            TrdSubType::TransactionFromAssignment => "10",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "0" => Some(TrdSubType::CMTA),
            "1" => Some(TrdSubType::InternalTransferOrAdjustment),
            "2" => Some(TrdSubType::ExternalTransferOrAdjustment),
            "3" => Some(TrdSubType::RejectForSubmittingTrade),
            "4" => Some(TrdSubType::AdvisoryForContraSide),
            "5" => Some(TrdSubType::OffsetDueToAnAllocation),
            "6" => Some(TrdSubType::OnsetDueToAnAllocation),
            "7" => Some(TrdSubType::DifferentialSpread),
            "8" => Some(TrdSubType::ImpliedSpreadLegExecuted),
            "9" => Some(TrdSubType::TransactionFromExercise),
            "10" => Some(TrdSubType::TransactionFromAssignment),
            _ => None,
        }
    }
}


/// MatchType (Tag 574)
/// Type of match algorithm
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MatchType {
    ExactMatchOnTradeDate,                      // M3
    ExactMatchOnTradeDateTime,                  // M4
    ExactMatchOnTradeDateTime1MinuteWindow,     // M5
    ExactMatchOnTradeDateTime2MinuteWindow,     // M6
    ACTAcceptedTrade,                           // A1
    ACTDefaultTrade,                            // A2
    ACTDefaultAfterM2,                          // A3
    ACTAcceptedAfterM1,                         // A4
    ACTAcceptedAfterM2,                         // A5
}

impl MatchType {
    pub fn to_fix(&self) -> &'static str {
        match self {
            MatchType::ExactMatchOnTradeDate => "M3",
            MatchType::ExactMatchOnTradeDateTime => "M4",
            MatchType::ExactMatchOnTradeDateTime1MinuteWindow => "M5",
            MatchType::ExactMatchOnTradeDateTime2MinuteWindow => "M6",
            MatchType::ACTAcceptedTrade => "A1",
            MatchType::ACTDefaultTrade => "A2",
            MatchType::ACTDefaultAfterM2 => "A3",
            MatchType::ACTAcceptedAfterM1 => "A4",
            MatchType::ACTAcceptedAfterM2 => "A5",
        }
    }

    pub fn from_fix(s: &str) -> Option<Self> {
        match s {
            "M3" => Some(MatchType::ExactMatchOnTradeDate),
            "M4" => Some(MatchType::ExactMatchOnTradeDateTime),
            "M5" => Some(MatchType::ExactMatchOnTradeDateTime1MinuteWindow),
            "M6" => Some(MatchType::ExactMatchOnTradeDateTime2MinuteWindow),
            "A1" => Some(MatchType::ACTAcceptedTrade),
            "A2" => Some(MatchType::ACTDefaultTrade),
            "A3" => Some(MatchType::ACTDefaultAfterM2),
            "A4" => Some(MatchType::ACTAcceptedAfterM1),
            "A5" => Some(MatchType::ACTAcceptedAfterM2),
            _ => None,
        }
    }
}
