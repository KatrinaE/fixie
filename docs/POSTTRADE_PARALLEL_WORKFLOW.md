# Post-Trade Messages - Parallel Implementation Workflow

This document outlines a parallelizable workflow for implementing Post-Trade messages from the FIX 5.0 SP2 specification.

## Overview

The Post-Trade business area contains **31 messages** across **6 categories**:
- Account Reporting (1 message)
- Position Maintenance (11 messages)
- Allocation (7 messages)
- Confirmation (3 messages)
- Settlement Instruction (3 messages)
- Trade Capture Reporting (6 messages)

## Workflow Structure

The implementation is divided into phases:
- **Phase 0**: Foundation (SERIAL) - Pre-allocate sections to avoid merge conflicts
- **Phases 1-6**: Module Implementation (PARALLEL) - 6 agents work simultaneously
- **Phase 7**: Integration (SERIAL) - Integrate all messages into central enums

---

  File Ownership Map

  | File                       | Phase 0                 | Phase 1-6                  | Phase 7       |
  |----------------------------|-------------------------|----------------------------|---------------|
  | lib.rs                     | ✅ Uncomment all         | ❌ No touch                 | ❌ No touch    |
  | mod.rs                     | ✅ Add all re-exports    | ❌ No touch                 | ❌ No touch    |
  | types.rs                   | ✅ Pre-allocate sections | ✅ Each agent → own section | ❌ No touch    |
  | groups.rs                  | ✅ Pre-allocate sections | ✅ Each agent → own section | ❌ No touch    |
  | account_reporting.rs       | ✅ Create empty          | ✅ Agent 1 only             | ❌ No touch    |
  | position_maintenance.rs    | ✅ Create empty          | ✅ Agent 2 only             | ❌ No touch    |
  | allocation.rs              | ✅ Create empty          | ✅ Agent 3 only             | ❌ No touch    |
  | confirmation.rs            | ✅ Create empty          | ✅ Agent 4 only             | ❌ No touch    |
  | settlement_instruction.rs  | ✅ Create empty          | ✅ Agent 5 only             | ❌ No touch    |
  | trade_capture_reporting.rs | ✅ Create empty          | ✅ Agent 6 only             | ❌ No touch    |
  | messages.rs                | ❌ No touch              | ❌ No touch                 | ✅ Integration |
  | fixie.rs                   | ❌ No touch              | ❌ No touch                 | ✅ Integration |


## Phase 0: Foundation (SERIAL)

**Branch**: `feature/posttrade-foundation`
**Agent**: Single agent
**Goal**: Pre-allocate sections in shared files to prevent merge conflicts

### Tasks:

#### 1. Pre-allocate sections in `src/types.rs`

Add placeholder comments for each category's enums (alphabetically ordered):

```rust
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

```

#### 2. Pre-allocate sections in `src/groups.rs`

Add placeholder comments for each category's repeating groups:

```rust
// ============================================================================
// Post-Trade: Account Reporting Groups
// Reserved for account reporting repeating groups
// ============================================================================


// ============================================================================
// Post-Trade: Allocation Groups
// Reserved for AllocGrp, ExecAllocGrp, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Confirmation Groups
// Reserved for CpctyConfGrp, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Position Maintenance Groups
// Reserved for PosUndInstrmtGrp, PosAmtGrp, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Settlement Instruction Groups
// Reserved for SettlInstGrp, SettlPartyGrp, etc.
// ============================================================================


// ============================================================================
// Post-Trade: Trade Capture Reporting Groups
// Reserved for TrdCapRptAckSideGrp, TrdInstrmtLegGrp, etc.
// ============================================================================

```

#### 3. Create module files and declare them in `src/lib.rs`

**IMPORTANT**: To prevent merge conflicts in Phases 1-6, we will uncomment ALL module declarations in Phase 0, even though the modules are empty.

Create empty module files with basic structure:

**`src/message_types/account_reporting.rs`**:
```rust
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Account Reporting Messages
// Implementation of FIX 5.0 SP2 Post-Trade Account Reporting messages
// ============================================================================

// Messages will be implemented in Phase 1
```

**`src/message_types/allocation.rs`**:
```rust
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Allocation Messages
// Implementation of FIX 5.0 SP2 Post-Trade Allocation messages
// ============================================================================

// Messages will be implemented in Phase 3
```

**`src/message_types/confirmation.rs`**:
```rust
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Confirmation Messages
// Implementation of FIX 5.0 SP2 Post-Trade Confirmation messages
// ============================================================================

// Messages will be implemented in Phase 4
```

**`src/message_types/position_maintenance.rs`**:
```rust
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Position Maintenance Messages
// Implementation of FIX 5.0 SP2 Post-Trade Position Maintenance messages
// ============================================================================

// Messages will be implemented in Phase 2
```

**`src/message_types/settlement_instruction.rs`**:
```rust
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Settlement Instruction Messages
// Implementation of FIX 5.0 SP2 Post-Trade Settlement Instruction messages
// ============================================================================

// Messages will be implemented in Phase 5
```

**`src/message_types/trade_capture_reporting.rs`**:
```rust
use crate::types::*;
use serde::{Deserialize, Serialize};

// ============================================================================
// Trade Capture Reporting Messages
// Implementation of FIX 5.0 SP2 Post-Trade Trade Capture Reporting messages
// ============================================================================

// Messages will be implemented in Phase 6
```

Then, **uncomment** the module declarations in `src/lib.rs`:

```rust
// Post-Trade message types (in message_types/mod.rs)
pub mod account_reporting;
pub mod allocation;
pub mod confirmation;
pub mod position_maintenance;
pub mod settlement_instruction;
pub mod trade_capture_reporting;
```

#### 4. Add re-export declarations in `src/message_types/mod.rs`

Add the following to `src/message_types/mod.rs`:

```rust
// Post-Trade message types
pub mod account_reporting;
pub mod allocation;
pub mod confirmation;
pub mod position_maintenance;
pub mod settlement_instruction;
pub mod trade_capture_reporting;

// Re-export all Post-Trade types
pub use account_reporting::*;
pub use allocation::*;
pub use confirmation::*;
pub use position_maintenance::*;
pub use settlement_instruction::*;
pub use trade_capture_reporting::*;
```

**Why do this in Phase 0?**
- Prevents all 6 agents from modifying the same lines in `lib.rs` and `mod.rs`
- Empty modules compile fine in Rust
- Each agent in Phases 1-6 only modifies their own module file
- **Zero merge conflicts** in shared files!

### Deliverables:
- Commit: "Post-Trade Foundation: Pre-allocate sections in types.rs, groups.rs, and lib.rs"
- PR: "Post-Trade Phase 0: Foundation"
- **Merge this PR before starting Phases 1-6**

---

## Phases 1-6: Module Implementation (PARALLEL)

These phases can run **simultaneously** on separate branches. Each agent works on one category.

### Phase 1: Account Reporting (PARALLEL)

**Branch**: `feature/posttrade-account-reporting`
**Agent**: Agent 1
**Goal**: Implement 1 Account Reporting message

#### Messages to Implement:
1. AccountSummaryReport (CQ)

#### Tasks:

1. **Add enums to `src/types.rs`** (in Section 700):
   - Any account reporting-specific enums needed

2. **Add groups to `src/groups.rs`** (in Account Reporting section):
   - Any repeating groups needed for account reporting messages

3. **Implement messages in `src/message_types/account_reporting.rs`**:
   - AccountSummaryReport struct with all fields
   - Builder methods (new(), with_*() methods)
   - Comprehensive unit tests

**Note**: Module is already declared in `lib.rs` and `mod.rs` from Phase 0, so no changes needed there.

#### Deliverables:
- Commit: "Account Reporting - Complete"
- PR: "Post-Trade Phase 1: Account Reporting Messages"
- **Merge to main after approval** (conflict-free by design!)

---

### Phase 2: Position Maintenance (PARALLEL)

**Branch**: `feature/posttrade-position-maintenance`
**Agent**: Agent 2
**Goal**: Implement 11 Position Maintenance messages

#### Messages to Implement:
1. AdjustedPositionReport (BL)
2. AssignmentReport (AW)
3. ContraryIntentionReport (BO)
4. PositionMaintenanceReport (AM)
5. PositionMaintenanceRequest (AL)
6. PositionReport (AP)
7. PositionTransferInstruction (DL)
8. PositionTransferInstructionAck (DM)
9. PositionTransferReport (DN)
10. RequestForPositions (AN)
11. RequestForPositionsAck (AO)

#### Tasks:

1. **Add enums to `src/types.rs`** (in Section 710):
   - PosReqType, PosTransType, PosMaintAction, PosMaintResult, etc.

2. **Add groups to `src/groups.rs`** (in Position Maintenance section):
   - PosUndInstrmtGrp, PosAmtGrp, etc.

3. **Implement messages in `src/message_types/position_maintenance.rs`**:
   - All 11 message structs with fields
   - Builder methods for each
   - Comprehensive unit tests

**Note**: Module is already declared in `lib.rs` and `mod.rs` from Phase 0, so no changes needed there.

#### Deliverables:
- Commit: "Position Maintenance - Complete (All 11 messages)"
- PR: "Post-Trade Phase 2: Position Maintenance Messages"
- **Merge to main after approval** (conflict-free by design!)

---

### Phase 3: Allocation (PARALLEL)

**Branch**: `feature/posttrade-allocation`
**Agent**: Agent 3
**Goal**: Implement 7 Allocation messages

#### Messages to Implement:
1. AllocationInstruction (J)
2. AllocationInstructionAck (P)
3. AllocationInstructionAlert (BM)
4. AllocationInstructionAlertRequest (DU)
5. AllocationInstructionAlertRequestAck (DV)
6. AllocationReport (AS)
7. AllocationReportAck (AT)

#### Tasks:

1. **Add enums to `src/types.rs`** (in Section 720):
   - AllocType, AllocTransType, AllocStatus, AllocRejCode, etc.

2. **Add groups to `src/groups.rs`** (in Allocation section):
   - AllocGrp, ExecAllocGrp, etc.

3. **Implement messages in `src/message_types/allocation.rs`**:
   - All 7 message structs with fields
   - Builder methods for each
   - Comprehensive unit tests

**Note**: Module is already declared in `lib.rs` and `mod.rs` from Phase 0, so no changes needed there.

#### Deliverables:
- Commit: "Allocation - Complete (All 7 messages)"
- PR: "Post-Trade Phase 3: Allocation Messages"
- **Merge to main after approval** (conflict-free by design!)

---

### Phase 4: Confirmation (PARALLEL)

**Branch**: `feature/posttrade-confirmation`
**Agent**: Agent 4
**Goal**: Implement 3 Confirmation messages

#### Messages to Implement:
1. Confirmation (AK)
2. ConfirmationAck (AU)
3. ConfirmationRequest (BH)

#### Tasks:

1. **Add enums to `src/types.rs`** (in Section 730):
   - ConfirmType, ConfirmStatus, ConfirmTransType, etc.

2. **Add groups to `src/groups.rs`** (in Confirmation section):
   - CpctyConfGrp, etc.

3. **Implement messages in `src/message_types/confirmation.rs`**:
   - All 3 message structs with fields
   - Builder methods for each
   - Comprehensive unit tests

**Note**: Module is already declared in `lib.rs` and `mod.rs` from Phase 0, so no changes needed there.

#### Deliverables:
- Commit: "Confirmation - Complete (All 3 messages)"
- PR: "Post-Trade Phase 4: Confirmation Messages"
- **Merge to main after approval** (conflict-free by design!)

---

### Phase 5: Settlement Instruction (PARALLEL)

**Branch**: `feature/posttrade-settlement-instruction`
**Agent**: Agent 5
**Goal**: Implement 3 Settlement Instruction messages

#### Messages to Implement:
1. SettlementInstructionRequest (AV)
2. SettlementInstructions (T)
3. SettlementObligationReport (BQ)

#### Tasks:

1. **Add enums to `src/types.rs`** (in Section 740):
   - SettlInstSource, SettlInstTransType, SettlInstReqRejCode, etc.

2. **Add groups to `src/groups.rs`** (in Settlement Instruction section):
   - SettlInstGrp, SettlPartyGrp, etc.

3. **Implement messages in `src/message_types/settlement_instruction.rs`**:
   - All 3 message structs with fields
   - Builder methods for each
   - Comprehensive unit tests

**Note**: Module is already declared in `lib.rs` and `mod.rs` from Phase 0, so no changes needed there.

#### Deliverables:
- Commit: "Settlement Instruction - Complete (All 3 messages)"
- PR: "Post-Trade Phase 5: Settlement Instruction Messages"
- **Merge to main after approval** (conflict-free by design!)

---

### Phase 6: Trade Capture Reporting (PARALLEL)

**Branch**: `feature/posttrade-trade-capture-reporting`
**Agent**: Agent 6
**Goal**: Implement 6 Trade Capture Reporting messages

#### Messages to Implement:
1. TradeCaptureReport (AE)
2. TradeCaptureReportAck (AR)
3. TradeCaptureReportRequest (AD)
4. TradeCaptureReportRequestAck (AQ)
5. TradeMatchReport (DC)
6. TradeMatchReportAck (DD)

#### Tasks:

1. **Add enums to `src/types.rs`** (in Section 750):
   - TradeRequestType, TradeReportType, TrdType, etc.

2. **Add groups to `src/groups.rs`** (in Trade Capture Reporting section):
   - TrdCapRptAckSideGrp, TrdInstrmtLegGrp, etc.

3. **Implement messages in `src/message_types/trade_capture_reporting.rs`**:
   - All 6 message structs with fields
   - Builder methods for each
   - Comprehensive unit tests

**Note**: Module is already declared in `lib.rs` and `mod.rs` from Phase 0, so no changes needed there.

#### Deliverables:
- Commit: "Trade Capture Reporting - Complete (All 6 messages)"
- PR: "Post-Trade Phase 6: Trade Capture Reporting Messages"
- **Merge to main after approval** (conflict-free by design!)

---

## Phase 7: Integration (SERIAL)

**Branch**: `feature/posttrade-integration`
**Agent**: Single agent
**Goal**: Integrate all 31 Post-Trade messages into central message enums and CLI

**Prerequisites**:
- Phase 0 must be merged to main ✅
- All Phases 1-6 must be merged to main ✅
- This branch should be based on main (which already has all phase implementations)

**Note**: Since Phases 1-6 are already merged to main, Phase 7 only needs to add integration code - no branch merging required!

### Tasks:

#### 1. Create integration branch from main

```bash
git checkout -b feature/posttrade-integration main
```

Main already contains all Phase 1-6 implementations, so no merging needed!

#### 2. Add all messages to `src/messages.rs`

**Add to MsgType enum** (alphabetically):
```rust
// Post-Trade Messages
AccountSummaryReport,                      // CQ
AdjustedPositionReport,                    // BL
AllocationInstruction,                     // J
AllocationInstructionAck,                  // P
AllocationInstructionAlert,                // BM
AllocationInstructionAlertRequest,         // DU
AllocationInstructionAlertRequestAck,      // DV
AllocationReport,                          // AS
AllocationReportAck,                       // AT
AssignmentReport,                          // AW
Confirmation,                              // AK
ConfirmationAck,                           // AU
ConfirmationRequest,                       // BH
ContraryIntentionReport,                   // BO
PositionMaintenanceReport,                 // AM
PositionMaintenanceRequest,                // AL
PositionReport,                            // AP
PositionTransferInstruction,               // DL
PositionTransferInstructionAck,            // DM
PositionTransferReport,                    // DN
RequestForPositions,                       // AN
RequestForPositionsAck,                    // AO
SettlementInstructionRequest,              // AV
SettlementInstructions,                    // T
SettlementObligationReport,                // BQ
TradeCaptureReport,                        // AE
TradeCaptureReportAck,                     // AR
TradeCaptureReportRequest,                 // AD
TradeCaptureReportRequestAck,              // AQ
TradeMatchReport,                          // DC
TradeMatchReportAck,                       // DD
```

**Update to_fix() method** with all mappings

**Update from_fix() method** with all mappings

**Add to FixMessage enum**:
```rust
// Post-Trade Messages
AccountSummaryReport(crate::message_types::account_reporting::AccountSummaryReport),
AdjustedPositionReport(crate::message_types::position_maintenance::AdjustedPositionReport),
// ... all 31 messages
```

**Update msg_type() method** with all mappings

#### 3. Update CLI in `src/bin/fixie.rs`

**Add message type names to msg_type_name()** for all 31 messages

**Add field tag names to tag_name()** for common Post-Trade fields:
- Account fields (1, 660, 581, etc.)
- Position fields (702, 703, 704, etc.)
- Allocation fields (70, 71, 72, 73, etc.)
- Confirmation fields (664, 665, 666, etc.)
- Settlement fields (162, 163, 214, etc.)
- Trade capture fields (568, 569, 570, etc.)

#### 4. Run tests

```bash
cargo test --all
```

Ensure all tests pass (should be 197 + new tests from phases 1-6).

### Deliverables:
- Commit: "Post-Trade Integration - Complete (All 31 messages)"
- PR: "Post-Trade Phase 7: Integration"
- **Merge after approval**

---

## Timeline Estimate

- **Phase 0** (Serial): 30 minutes
- **Phases 1-6** (Parallel): ~4-6 hours each (can run simultaneously)
- **Phase 7** (Serial): 1-2 hours

**Total elapsed time**: ~7-9 hours (if parallelized)
**Total work time**: ~30-40 hours (if done serially)

---

## Success Criteria

✅ All 31 Post-Trade messages implemented with:
- Complete struct definitions with all required and optional fields
- Builder pattern methods (new(), with_*())
- Serde serialization support
- Comprehensive unit tests for each message

✅ All enums defined in types.rs with:
- to_fix() and from_fix() conversion methods
- Unit tests for conversions

✅ All repeating groups defined in groups.rs

✅ Integration complete:
- Messages added to MsgType and FixMessage enums
- CLI support for message type names and field tags

✅ All tests passing:
- No regressions in existing tests
- New tests for all Post-Trade messages

---

## Merge Conflict Prevention Strategy

The pre-allocation in Phase 0 **eliminates** merge conflicts because:

1. **types.rs**: Each category has its own section (700, 710, 720, etc.)
   - Agents only modify their assigned section
   - No overlap between sections

2. **groups.rs**: Each category has its own comment-delimited section
   - Agents only modify their assigned section
   - No overlap between sections

3. **lib.rs**: ALL module declarations uncommented in Phase 0
   - ✅ **Critical**: Empty modules compile fine
   - ✅ Phases 1-6 don't touch lib.rs at all
   - ✅ Zero conflicts

4. **message_types/mod.rs**: ALL re-exports added in Phase 0
   - ✅ Phases 1-6 don't touch mod.rs at all
   - ✅ Zero conflicts

5. **message_types/*.rs**: Each agent has their own file
   - account_reporting.rs → Agent 1
   - position_maintenance.rs → Agent 2
   - allocation.rs → Agent 3
   - confirmation.rs → Agent 4
   - settlement_instruction.rs → Agent 5
   - trade_capture_reporting.rs → Agent 6
   - ✅ Zero conflicts

6. **messages.rs**: Not touched until Phase 7 (serial)
7. **fixie.rs**: Not touched until Phase 7 (serial)

**Result**: Each agent works in completely isolated sections with **ZERO merge conflicts**.

---

## Notes for Claude Agents

### For Phase 0 Agent:
- Create clear section boundaries with distinctive comments
- Ensure alphabetical ordering of sections
- Keep section numbers consistent (700-series for Account Reporting, etc.)

### For Phase 1-6 Agents:
- Only modify files in your assigned section
- Follow existing patterns from Pre-Trade messages
- Include comprehensive tests (creation, field setting, round-trip)
- Use builder pattern for message construction
- Reference FIX specification for field definitions: https://www.fixtrading.org/online-specification/

### For Phase 7 Agent:
- Merge phase branches in numerical order (1, 2, 3, 4, 5, 6)
- Maintain alphabetical ordering in MsgType enum
- Ensure all message type names are added to CLI
- Run full test suite before committing
- Update README.md with new message counts if needed

---

## Implementation Requirements

Based on project requirements, all agents should follow these guidelines:

### 1. **Field Coverage**: FULL
- Implement ALL fields (required and optional) as documented in FIX 5.0 SP2
- Reference the online specification: https://www.fixtrading.org/online-specification/
- Support custom/proprietary tags in the 9000+ range

### 2. **Repeating Groups**: FULL IMPLEMENTATION
- All repeating groups must be fully implemented in `groups.rs`
- Define proper `GroupConfig` entries with all member tags and nested groups
- Use the arena-based structure (see existing examples in `groups.rs`)
- **Do NOT use placeholder comments** - implement the actual Vec<> fields on message structs

Example (correct approach):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInstruction {
    pub alloc_id: String,
    pub alloc_trans_type: AllocTransType,
    // ... other fields ...

    // Repeating groups (fully implemented)
    pub allocations: Vec<AllocGroup>,
    pub parties: Vec<Party>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocGroup {
    pub alloc_account: String,
    pub alloc_qty: f64,
    // ... all group fields ...
}
```

### 3. **Documentation**: REQUIRED
Every message struct must include Rustdoc comments:

```rust
/// AllocationInstruction (MsgType=J)
///
/// Provides allocation information for a trade execution. Sent from sell-side
/// to buy-side or from executing broker to give-up broker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInstruction {
    /// AllocID (70) - Unique identifier for allocation message
    pub alloc_id: String,

    /// AllocTransType (71) - Identifies allocation transaction type
    pub alloc_trans_type: AllocTransType,

    // ... etc for all fields
}
```

### 4. **Testing Level**: MATCH PRE-TRADE
All messages must have tests matching Pre-Trade level:
- Message creation tests
- Field setting tests (builder pattern)
- Round-trip tests (to_raw/from_raw if applicable)
- Edge case tests (error conditions, optional fields)

### 5. **Specification Version**: FIX 5.0 SP2
Follow the FIX 5.0 SP2 specification as documented online

### 6. **Message Complexity**: FULL
Implement complete messages with:
- All required fields
- All optional fields
- All repeating groups (fully implemented)
- All nested repeating groups

This means some messages may have 50+ fields and multiple repeating groups. That's expected and correct.

---

## Repeating Groups Implementation Guide

For agents implementing repeating groups, follow this process:

### Step 1: Define Group Structures

In your message file (e.g., `allocation.rs`), define structs for each repeating group:

```rust
/// Entry in the AllocGrp repeating group (NoAllocs=78)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocGroup {
    /// AllocAccount (79) - Account to which allocation is made
    pub alloc_account: String,

    /// AllocQty (80) - Quantity allocated to this account
    pub alloc_qty: f64,

    /// AllocPrice (366) - Price for this allocation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alloc_price: Option<f64>,

    // ... all other fields in this group
}
```

### Step 2: Add to Message Struct

Include the repeating group as a Vec field:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationInstruction {
    pub alloc_id: String,
    // ... other fields ...

    /// AllocGrp - Repeating group of allocation details (NoAllocs=78)
    pub allocations: Vec<AllocGroup>,
}
```

### Step 3: Register in groups.rs

Add the group configuration in your pre-allocated section:

```rust
// ============================================================================
// Post-Trade: Allocation Groups
// ============================================================================

// AllocGrp - Allocation details (NoAllocs = 78)
registry.insert(78, GroupConfig {
    count_tag: 78,
    first_tag: 79,  // AllocAccount
    member_tags: vec![79, 80, 366, /* ... all tags in group ... */],
    nested_groups: vec![
        // If this group has nested groups, list them here
    ],
    message_specific: None,
});
```

### Step 4: Implement Builder Methods

Add builder methods for working with the repeating group:

```rust
impl AllocationInstruction {
    pub fn new(alloc_id: String, alloc_trans_type: AllocTransType) -> Self {
        Self {
            alloc_id,
            alloc_trans_type,
            allocations: Vec::new(),  // Initialize empty
        }
    }

    /// Add an allocation to the allocations group
    pub fn add_allocation(mut self, allocation: AllocGroup) -> Self {
        self.allocations.push(allocation);
        self
    }

    /// Set all allocations at once
    pub fn with_allocations(mut self, allocations: Vec<AllocGroup>) -> Self {
        self.allocations = allocations;
        self
    }
}

impl AllocGroup {
    /// Create a new allocation entry
    pub fn new(alloc_account: String, alloc_qty: f64) -> Self {
        Self {
            alloc_account,
            alloc_qty,
            alloc_price: None,
        }
    }

    /// Set the allocation price
    pub fn with_price(mut self, price: f64) -> Self {
        self.alloc_price = Some(price);
        self
    }
}
```

### Step 5: Test Repeating Groups

Include tests that verify repeating group functionality:

```rust
#[test]
fn test_allocation_instruction_with_multiple_allocations() {
    let alloc1 = AllocGroup::new("ACCT1".to_string(), 100.0)
        .with_price(50.25);

    let alloc2 = AllocGroup::new("ACCT2".to_string(), 200.0)
        .with_price(50.26);

    let msg = AllocationInstruction::new(
        "ALLOC123".to_string(),
        AllocTransType::New,
    )
    .add_allocation(alloc1)
    .add_allocation(alloc2);

    assert_eq!(msg.allocations.len(), 2);
    assert_eq!(msg.allocations[0].alloc_account, "ACCT1");
    assert_eq!(msg.allocations[1].alloc_qty, 200.0);
}
```

---
