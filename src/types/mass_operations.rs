use serde::{Deserialize, Serialize};


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
