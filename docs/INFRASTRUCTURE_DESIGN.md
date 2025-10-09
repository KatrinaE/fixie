# FIX Infrastructure Messages - Implementation Plans

This document provides implementation plans for all four categories of FIX Infrastructure messages from the Business Area: Infrastructure specification.

**Reference**: https://www.fixtrading.org/online-specification/business-area-infrastructure/

---

## 1. Application Sequencing Messages

### Overview
Application sequencing messages provide message gap detection and recovery at the application level, independent of session-level sequencing. These messages allow applications to request retransmission of specific messages and reset application sequence numbers.

### Message Types
- **ApplicationMessageRequest (BW)** - Request retransmission of application messages
- **ApplicationMessageRequestAck (BX)** - Acknowledge application message request
- **ApplicationMessageReport (BY)** - Report application message status or reset sequences

### Key Use Cases
1. **Gap Detection**: Application detects missing messages and requests retransmission
2. **Sequence Reset**: Reset application sequence numbers after maintenance or failover
3. **Message Recovery**: Recover specific application messages after network issues
4. **Last Message Notification**: Indicate completion of message retransmission

### Implementation Plan

#### Phase 1: Enum Types and Field Definitions (1 day)
Add application sequencing enums to `src/types.rs`:

```rust
/// ApplReqType (Tag 1347) - Type of application request
pub enum ApplReqType {
    Retransmission,              // 0
    Subscription,                // 1
    RequestLastSeqNum,           // 2
    RequestApplications,         // 3
    Unsubscribe,                 // 4
    CancelRetransmission,        // 5
    CancelRetransmissionAndUnsubscribe, // 6
}

/// ApplResponseType (Tag 1348) - Type of response
pub enum ApplResponseType {
    RequestSuccessfullyProcessed, // 0
    ApplicationDoesNotExist,      // 1
    MessagesNotAvailable,         // 2
}

/// ApplReportType (Tag 1426) - Type of application report
pub enum ApplReportType {
    ApplSeqNumReset,             // 0
    LastMessageSent,             // 1
    ApplicationAlive,            // 2
    ApplicationMessageRestart,   // 3
}
```

All enums implement `to_fix() -> char` and `from_fix(char) -> Option<Self>` with unit tests.

**Estimated effort**: 1 day
**Tests**: 3 unit tests (one per enum)

#### Phase 2: Repeating Group Definitions (1 day)
Add application ID groups to `src/groups.rs`:

```rust
// ApplIDRequestGrp (Tag 1351 = NoApplIDs) - for BW message
GroupConfig {
    num_in_group_tag: 1351,
    delimiter_tag: 1355,    // RefApplID
    member_tags: vec![
        1355,  // RefApplID
        1430,  // RefApplReqID
        1182,  // ApplBegSeqNum
        1183,  // ApplEndSeqNum
    ],
    nested_groups: vec![],
}

// ApplIDReportGrp (Tag 1351 = NoApplIDs) - for BY message
// Same structure but context-dependent usage
```

**Estimated effort**: 1 day
**Tests**: 2 unit tests for group configurations

#### Phase 3: Message Structures (2 days)
Create `src/application_sequencing.rs`:

```rust
pub struct ApplicationMessageRequest {
    pub appl_req_id: String,              // Required - Tag 1346
    pub appl_req_type: ApplReqType,       // Required - Tag 1347
    pub appl_id: Option<String>,          // Tag 1180
    pub appl_begin_seq_num: Option<u64>,  // Tag 1182
    pub appl_end_seq_num: Option<u64>,    // Tag 1183
    // NoApplIDs group accessed via RawFixMessage.groups.get(&1351)
}

pub struct ApplicationMessageRequestAck {
    pub appl_response_id: String,           // Required - Tag 1353
    pub appl_req_id: Option<String>,        // Tag 1346
    pub appl_response_type: Option<ApplResponseType>, // Tag 1348
    pub appl_response_error: Option<u32>,   // Tag 1354
    pub text: Option<String>,               // Tag 58
}

pub struct ApplicationMessageReport {
    pub appl_report_id: String,             // Required - Tag 1356
    pub appl_report_type: ApplReportType,   // Required - Tag 1426
    pub appl_req_id: Option<String>,        // Tag 1346
    pub appl_begin_seq_num: Option<u64>,    // Tag 1182
    pub appl_end_seq_num: Option<u64>,      // Tag 1183
    pub text: Option<String>,               // Tag 58
    // NoApplIDs group accessed via RawFixMessage.groups.get(&1351)
}
```

Each struct implements:
- `from_raw(raw: &RawFixMessage) -> Result<Self, String>`
- `to_raw(&self) -> RawFixMessage`
- Round-trip unit tests

**Estimated effort**: 2 days
**Tests**: 3 round-trip tests

#### Phase 4: Integration (1 day)
- Add to `MsgType` enum: `ApplicationMessageRequest`, `ApplicationMessageRequestAck`, `ApplicationMessageReport`
- Update `to_fix()`: return "BW", "BX", "BY"
- Update `from_fix()`: parse "BW", "BX", "BY"
- Add to `FixMessage` enum
- Update `msg_type()` method
- Add CLI field mappings for tags 1346-1430
- Add message type names to CLI

**Estimated effort**: 1 day
**Tests**: Verify all existing tests pass

#### Phase 5: Testing (1 day)
Add integration tests to `tests/integration_tests.rs`:
- Test gap detection scenario
- Test sequence reset
- Test message retransmission request
- Test error responses
- Test round-trip conversions

**Estimated effort**: 1 day
**Tests**: 5+ integration tests

**Total Timeline**: 6 days

---

## 2. Business Message Rejects

### Overview
BusinessMessageReject (j) provides application-level rejection of messages that pass session-level validation but fail business rules. Unlike session-level Reject (3), this message handles semantic validation failures.

### Message Types
- **BusinessMessageReject (j)** - Reject application-level messages

### Key Use Cases
1. **Invalid Message Type**: Message type not supported by application
2. **Conditional Required Field Missing**: Field required by business logic is missing
3. **Field Value Out of Range**: Value doesn't meet business constraints
4. **Unknown Security**: Referenced security doesn't exist
5. **Duplicate Message**: Message with same ID already processed

### Implementation Plan

#### Phase 1: Enum Types and Field Definitions (1 day)
Add business reject enums to `src/types.rs`:

```rust
/// BusinessRejectReason (Tag 380) - Reason for business-level rejection
pub enum BusinessRejectReason {
    Other,                                  // 0
    UnknownID,                             // 1
    UnknownSecurity,                       // 2
    UnsupportedMessageType,                // 3
    ApplicationNotAvailable,               // 4
    ConditionallyRequiredFieldMissing,     // 5
    NotAuthorized,                         // 6
    DeliverToFirmNotAvailableAtThisTime,   // 7
}
```

Implement `to_fix() -> char` and `from_fix(char) -> Option<Self>` with unit test.

**Estimated effort**: 1 day
**Tests**: 1 unit test

#### Phase 2: Message Structure (1 day)
Add to `src/messages.rs` (simple message, no separate module needed):

```rust
pub struct BusinessMessageReject {
    pub ref_seq_num: Option<u32>,              // Tag 45 - MsgSeqNum of rejected message
    pub ref_msg_type: String,                  // Required - Tag 372
    pub business_reject_ref_id: Option<String>, // Tag 379 - ID of rejected message
    pub business_reject_reason: BusinessRejectReason, // Required - Tag 380
    pub text: Option<String>,                  // Tag 58
    pub encoded_text_len: Option<u32>,         // Tag 354
    pub encoded_text: Option<Vec<u8>>,         // Tag 355
}
```

Implement `from_raw()`, `to_raw()`, and round-trip test.

**Estimated effort**: 1 day
**Tests**: 1 round-trip test

#### Phase 3: Integration (1 day)
- Add to `MsgType` enum: `BusinessMessageReject`
- Update `to_fix()`: return "j"
- Update `from_fix()`: parse "j"
- Add to `FixMessage` enum
- Update `msg_type()` method
- Add CLI field mappings for tags 45, 372, 379, 380
- Add message type name to CLI

**Estimated effort**: 1 day
**Tests**: Verify all existing tests pass

#### Phase 4: Testing (1 day)
Add integration tests:
- Test rejection scenarios for each BusinessRejectReason
- Test with and without optional fields
- Test round-trip conversions
- Test text encoding

**Estimated effort**: 1 day
**Tests**: 5+ integration tests

**Total Timeline**: 4 days

---

## 3. Network Status Communication Messages

### Overview
Network status messages provide visibility into counterparty system availability in a network. These messages support multi-party trading networks where participants need to know which counterparties are available.

### Message Types
- **NetworkCounterpartySystemStatusRequest (BC)** - Request status of counterparties
- **NetworkCounterpartySystemStatusResponse (BD)** - Report status of counterparties

### Key Use Cases
1. **Network Health Monitoring**: Track which counterparties are online
2. **Failover Planning**: Identify available backup counterparties
3. **Trading Decisions**: Route orders to available counterparties
4. **Network Topology**: Understand network connectivity

### Implementation Plan

#### Phase 1: Enum Types and Field Definitions (1 day)
Add network status enums to `src/types.rs`:

```rust
/// NetworkRequestType (Tag 935) - Type of network request
pub enum NetworkRequestType {
    Snapshot,                    // 1
    Subscribe,                   // 2
    StopSubscribing,             // 4
    LevelOfDetail,               // 8
}

/// NetworkStatusResponseType (Tag 937) - Type of network response
pub enum NetworkStatusResponseType {
    Full,                        // 1
    IncrementalUpdate,           // 2
}

/// NetworkSystemStatus (Tag 934) - Status of a counterparty system
pub enum NetworkSystemStatus {
    Online,                      // 0
    TemporarilyUnavailable,      // 1
    Offline,                     // 2
}
```

All enums implement `to_fix()` and `from_fix()` with unit tests.

**Estimated effort**: 1 day
**Tests**: 3 unit tests

#### Phase 2: Repeating Group Definitions (1 day)
Add network status groups to `src/groups.rs`:

```rust
// CompIDReqGrp (Tag 936 = NoCompIDs) - for BC message
GroupConfig {
    num_in_group_tag: 936,
    delimiter_tag: 930,    // RefCompID
    member_tags: vec![
        930,  // RefCompID
        931,  // RefSubID
        283,  // LocationID
        284,  // DeskID
    ],
    nested_groups: vec![],
}

// CompIDStatGrp (Tag 936 = NoCompIDs) - for BD message
GroupConfig {
    num_in_group_tag: 936,
    delimiter_tag: 930,    // RefCompID
    member_tags: vec![
        930,  // RefCompID
        931,  // RefSubID
        283,  // LocationID
        284,  // DeskID
        928,  // StatusValue (NetworkSystemStatus)
        929,  // StatusText
    ],
    nested_groups: vec![],
}
```

**Estimated effort**: 1 day
**Tests**: 2 unit tests

#### Phase 3: Message Structures (2 days)
Create `src/network_status.rs`:

```rust
pub struct NetworkCounterpartySystemStatusRequest {
    pub network_req_id: String,                // Required - Tag 933
    pub network_request_type: NetworkRequestType, // Required - Tag 935
    // NoCompIDs group accessed via RawFixMessage.groups.get(&936)
}

pub struct NetworkCounterpartySystemStatusResponse {
    pub network_resp_id: String,               // Required - Tag 932
    pub network_request_id: Option<String>,    // Tag 933
    pub network_response_type: NetworkStatusResponseType, // Required - Tag 937
    pub last_network_resp_id: Option<String>,  // Tag 934
    // NoCompIDs group accessed via RawFixMessage.groups.get(&936)
}
```

Each struct implements `from_raw()`, `to_raw()`, and round-trip tests.

**Estimated effort**: 2 days
**Tests**: 2 round-trip tests

#### Phase 4: Integration (1 day)
- Add to `MsgType` enum: `NetworkCounterpartySystemStatusRequest`, `NetworkCounterpartySystemStatusResponse`
- Update `to_fix()`: return "BC", "BD"
- Update `from_fix()`: parse "BC", "BD"
- Add to `FixMessage` enum
- Update `msg_type()` method
- Add CLI field mappings for tags 928-937
- Add message type names to CLI

**Estimated effort**: 1 day
**Tests**: Verify all existing tests pass

#### Phase 5: Testing (1 day)
Add integration tests:
- Test snapshot request/response
- Test subscription updates
- Test multiple counterparties
- Test status changes
- Test round-trip conversions

**Estimated effort**: 1 day
**Tests**: 5+ integration tests

**Total Timeline**: 6 days

---

## 4. User Management Messages

### Overview
User management messages handle user authentication, session management, and notifications. These messages support multi-user systems where individual users need to be authenticated and managed separately from system-level sessions.

### Message Types
- **UserRequest (BE)** - Request user actions (logon, logout, password change)
- **UserResponse (BF)** - Respond to user requests
- **UserNotification (CB)** - Notify users of events

### Key Use Cases
1. **User Logon**: Authenticate individual user within FIX session
2. **User Logout**: Terminate user access
3. **Password Change**: Update user credentials
4. **User Management**: Create, modify, or disable user accounts
5. **Event Notification**: Notify users of important events

### Implementation Plan

#### Phase 1: Enum Types and Field Definitions (2 days)
Add user management enums to `src/types.rs`:

```rust
/// UserRequestType (Tag 924) - Type of user request
pub enum UserRequestType {
    LogOnUser,                   // 1
    LogOffUser,                  // 2
    ChangePasswordForUser,       // 3
    RequestIndividualUserStatus, // 4
    RequestThrottleLimit,        // 5
    ChangeThrottleLimit,         // 6
}

/// UserStatus (Tag 926) - Status of user
pub enum UserStatus {
    LoggedIn,                    // 1
    NotLoggedIn,                 // 2
    UserNotRecognized,           // 3
    PasswordIncorrect,           // 4
    PasswordChanged,             // 5
    Other,                       // 6
    ForcedUserLogoutByExchange,  // 7
    SessionShutdownWarning,      // 8
}

/// ThrottleInst (Tag 1685) - Throttle instruction
pub enum ThrottleInst {
    RejectIfThrottleLimitExceeded, // 0
    QueueIfThrottleLimitExceeded,  // 1
}

/// UserStatusText (Tag 927) - free-form text describing user status (String field)
```

All enums implement `to_fix()` and `from_fix()` with unit tests.

**Estimated effort**: 2 days
**Tests**: 3 unit tests

#### Phase 2: Message Structures (3 days)
Create `src/user_management.rs`:

```rust
pub struct UserRequest {
    pub user_request_id: String,              // Required - Tag 923
    pub user_request_type: UserRequestType,   // Required - Tag 924
    pub username: String,                     // Required - Tag 553
    pub password: Option<String>,             // Tag 554
    pub new_password: Option<String>,         // Tag 925
    pub raw_data_length: Option<u32>,         // Tag 95
    pub raw_data: Option<Vec<u8>>,            // Tag 96
    pub user_status: Option<UserStatus>,      // Tag 926
    pub throttle_limit: Option<u32>,          // Tag 1686
    pub throttle_time_unit: Option<u32>,      // Tag 1690
    pub throttle_inst: Option<ThrottleInst>,  // Tag 1685
}

pub struct UserResponse {
    pub user_request_id: String,              // Required - Tag 923
    pub username: String,                     // Required - Tag 553
    pub user_status: Option<UserStatus>,      // Tag 926
    pub user_status_text: Option<String>,     // Tag 927
    pub throttle_limit: Option<u32>,          // Tag 1686
    pub throttle_time_unit: Option<u32>,      // Tag 1690
}

pub struct UserNotification {
    pub username: String,                     // Required - Tag 553
    pub user_status: Option<UserStatus>,      // Tag 926
    pub user_status_text: Option<String>,     // Tag 927
    pub throttle_limit: Option<u32>,          // Tag 1686
    pub throttle_time_unit: Option<u32>,      // Tag 1690
}
```

Each struct implements `from_raw()`, `to_raw()`, and round-trip tests.

**Note**: Password fields should be handled securely (not logged, cleared from memory).

**Estimated effort**: 3 days
**Tests**: 3 round-trip tests

#### Phase 3: Integration (1 day)
- Add to `MsgType` enum: `UserRequest`, `UserResponse`, `UserNotification`
- Update `to_fix()`: return "BE", "BF", "CB"
- Update `from_fix()`: parse "BE", "BF", "CB"
- Add to `FixMessage` enum
- Update `msg_type()` method
- Add CLI field mappings for tags 553, 554, 923-927, 1685-1690
- Add message type names to CLI

**Estimated effort**: 1 day
**Tests**: Verify all existing tests pass

#### Phase 4: Testing (2 days)
Add integration tests:
- Test user logon/logout flow
- Test password change
- Test throttle limit management
- Test user status notifications
- Test error scenarios (wrong password, unknown user)
- Test round-trip conversions
- Test security (password not in logs)

**Estimated effort**: 2 days
**Tests**: 8+ integration tests

**Total Timeline**: 8 days

---

## Summary

### Total Implementation Timeline

| Category | Timeline | Messages | Enums | Groups | Complexity |
|----------|----------|----------|-------|--------|------------|
| Application Sequencing | 6 days | 3 (BW, BX, BY) | 3 | 1 | Medium |
| Business Message Rejects | 4 days | 1 (j) | 1 | 0 | Low |
| Network Status Communication | 6 days | 2 (BC, BD) | 3 | 2 | Medium |
| User Management | 8 days | 3 (BE, BF, CB) | 3 | 0 | High |
| **Total** | **24 days** | **9 messages** | **10 enums** | **3 groups** | - |

### Implementation Order (Recommended)

1. **Business Message Rejects** (4 days) - Simplest, no repeating groups, foundational error handling
2. **Network Status Communication** (6 days) - Medium complexity, introduces network monitoring
3. **Application Sequencing** (6 days) - Medium complexity, critical for message recovery
4. **User Management** (8 days) - Most complex, security-sensitive, builds on other infrastructure

### Key Design Patterns

All categories follow the established fixie patterns:
- Enums with `to_fix()`/`from_fix()` conversions
- Structs with `from_raw()`/`to_raw()` methods
- Repeating groups in arena-based storage
- Round-trip testing for all message types
- CLI integration for field name display
- Comprehensive integration tests

### Security Considerations

**User Management** messages require special handling:
- Password fields (554, 925) should not be logged
- Sensitive data should be cleared from memory after use
- Consider adding a security audit for password handling
- Document best practices for credential management

### Dependencies

- **Phase 1-3** of each category can proceed independently
- **Phase 4** (Integration) may have minor merge conflicts if done in parallel
- Recommend sequential implementation in the order above
- Each category can be committed separately following the established pattern

### Testing Strategy

Each category includes:
- **Unit tests**: Enum conversions, group configurations
- **Round-trip tests**: Message parsing and encoding
- **Integration tests**: Real-world scenarios and edge cases
- **Regression tests**: Ensure existing functionality remains intact

---

## Next Steps

1. Choose which category to implement first (recommend: Business Message Rejects)
2. Create detailed phase-by-phase implementation for chosen category
3. Implement following the established pattern from Multileg Order messages
4. Commit after each phase as done previously
5. Move to next category

**Reference Documentation**:
- See `docs/MULTILEG_ORDER_DESIGN.md` for implementation pattern reference
- See `docs/MASS_ORDER_DESIGN.md` for similar multi-message implementation
- See `BIBLIOGRAPHY.md` for FIX specification references
