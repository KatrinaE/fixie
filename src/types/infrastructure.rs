use serde::{Deserialize, Serialize};


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
