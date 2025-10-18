use serde::{Deserialize, Serialize};


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
