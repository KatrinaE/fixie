# Pre-Trade Messages - Parallel Development Workflow

This document describes the workflow for implementing Pre-Trade messages in parallel with minimal merge conflicts.

## Overview

The Pre-Trade message categories will be implemented in parallel by multiple agents, each working on a separate branch. To avoid merge conflicts, we use pre-allocated sections in shared files and implement Phase 4 (integration) separately after all module PRs are merged.

## Pre-Allocated Sections

Three shared files have been pre-allocated with section markers:

### 1. `src/types.rs`
Sections 100-600 for Pre-Trade enum types:
- **[SECTION 100]** Indication Messages
- **[SECTION 200]** Event Communication Messages
- **[SECTION 300]** Quotation/Negotiation Messages
- **[SECTION 400]** Market Data Messages
- **[SECTION 500]** Market Structure Reference Data Messages
- **[SECTION 600]** Securities Reference Data Messages

### 2. `src/groups.rs`
Sections 100-600 for Pre-Trade repeating group configurations (added before `registry` return statement)

### 3. `src/lib.rs`
Commented module declarations for all Pre-Trade categories

## Implementation Phases

### Phase 0: Foundation (COMPLETE)
**Branch:** `feature/pretrade-foundation`
**Status:** Merged to main

Added section markers to:
- `src/types.rs` - Pre-allocated enum sections
- `src/groups.rs` - Pre-allocated group sections
- `src/lib.rs` - Commented module declarations

### Phases 1-3: Module Implementation (PARALLEL)

Each category implements **Phases 1-3 ONLY** in their own branch:

| Category | Branch | Messages | Phases |
|----------|--------|----------|--------|
| Indication | `feature/pretrade-indication` | IOI, Advertisement, CrossRequest, CrossRequestAck | 1-3 |
| Event Communication | `feature/pretrade-event-communication` | Email, News | 1-3 |
| Quotation/Negotiation | `feature/pretrade-quotation` | Quote, QuoteRequest, etc. (11 msgs) | 1-3 |
| Market Data | `feature/pretrade-market-data` | MarketDataRequest, etc. (3 msgs) | 1-3 |
| Market Structure | `feature/pretrade-market-structure` | MarketDefinition, TradingSessionStatus, etc. (8 msgs) | 1-3 |
| Securities Reference | `feature/pretrade-securities-reference` | SecurityDefinition, SecurityList, etc. (15 msgs) | 1-3 |

**What each PR contains (Phases 1-3):**

1. **Phase 1**: Add enums to their pre-allocated section in `src/types.rs`
2. **Phase 2**: Add group configs to their pre-allocated section in `src/groups.rs`
3. **Phase 3**: Create module file (e.g., `src/indication.rs`) with message structs
4. **Tests**: Create separate test file (e.g., `tests/indication_tests.rs`)
5. **Docs**: Keep implementation plan in `docs/` (e.g., `docs/INDICATION_DESIGN.md`)
6. Uncomment module declaration in `src/lib.rs`

**What each PR DOES NOT contain:**
- ❌ NO changes to `src/messages.rs` (`MsgType` or `FixMessage` enums)
- ❌ NO Phase 4 integration
- ❌ NO CLI field mappings

### Phase 4: Integration (SEQUENTIAL - After All Modules Merged)

After **ALL** module PRs (Phases 1-3) are merged, create separate integration PRs:

| Integration PR | Branch | Purpose |
|----------------|--------|---------|
| Indication Integration | `feature/pretrade-indication-integration` | Add to `MsgType`, `FixMessage`, CLI |
| Event Comm Integration | `feature/pretrade-event-comm-integration` | Add to `MsgType`, `FixMessage`, CLI |
| Quotation Integration | `feature/pretrade-quotation-integration` | Add to `MsgType`, `FixMessage`, CLI |
| Market Data Integration | `feature/pretrade-market-data-integration` | Add to `MsgType`, `FixMessage`, CLI |
| Market Structure Integration | `feature/pretrade-market-structure-integration` | Add to `MsgType`, `FixMessage`, CLI |
| Securities Ref Integration | `feature/pretrade-securities-ref-integration` | Add to `MsgType`, `FixMessage`, CLI |

**What each integration PR contains:**

1. Add message types to `MsgType` enum in `src/messages.rs` (alphabetical order)
2. Add message types to `FixMessage` enum in `src/messages.rs`
3. Update `to_fix()` and `from_fix()` methods
4. Update `msg_type()` method
5. Add CLI field mappings for all tags
6. Add message type names to CLI

## Merge Order

### Step 1: Merge Foundation (DONE)
```bash
feature/pretrade-foundation → main
```

### Step 2: Merge Module PRs (Parallel - Any Order)
```bash
feature/pretrade-indication → main
feature/pretrade-event-communication → main
feature/pretrade-quotation → main
feature/pretrade-market-data → main
feature/pretrade-market-structure → main
feature/pretrade-securities-reference → main
```

**Note:** These can be merged in **any order** with zero conflicts!

### Step 3: Merge Integration PRs (Sequential - In Order)
```bash
feature/pretrade-indication-integration → main
feature/pretrade-event-comm-integration → main
feature/pretrade-quotation-integration → main
feature/pretrade-market-data-integration → main
feature/pretrade-market-structure-integration → main
feature/pretrade-securities-ref-integration → main
```

**Note:** Merge integration PRs **one at a time** to avoid conflicts in `messages.rs`. Use alphabetical ordering for `MsgType` variants to make conflicts predictable and easy to resolve if they occur.

## Conflict Resolution

### Expected Conflicts: ZERO in Phases 1-3

With pre-allocated sections, there should be **zero conflicts** during module PR merges.

### Possible Conflicts: Phase 4 Integration

If integration PRs are merged sequentially, conflicts should be minimal. If conflicts occur in `messages.rs`:

1. **In `MsgType` enum**: Maintain alphabetical order of variants
2. **In `to_fix()` method**: Add your case in alphabetical order by variant name
3. **In `from_fix()` method**: Add your case (order doesn't matter here)
4. **In `FixMessage` enum**: Maintain alphabetical order
5. **In `msg_type()` method**: Add your case

**Example merge conflict resolution:**
```rust
// If you see a conflict like this:
pub enum MsgType {
    // ... existing types ...
<<<<<<< HEAD
    Advertisement,
    IOI,
=======
    Email,
    News,
>>>>>>> feature/pretrade-event-comm-integration
}

// Resolve by alphabetical order:
pub enum MsgType {
    // ... existing types ...
    Advertisement,
    Email,
    IOI,
    News,
}
```

## Testing Strategy

Each module PR should include:
- Unit tests for enums in `src/types.rs` (in the same file)
- Unit tests for groups in `src/groups.rs` (in the same file)
- Round-trip tests in module file (e.g., `src/indication.rs`)
- Integration tests in separate test file (e.g., `tests/indication_tests.rs`)

Each integration PR should:
- Verify all existing tests still pass
- Add any additional integration tests if needed

## Timeline Estimate

**With Batch Integration (Option B):**

| Phase | Description | Wall Clock Time |
|-------|-------------|-----------------|
| Phase 0 | Foundation PR | 1 day |
| Phases 1-3 | All module PRs (parallel) | ~20 days |
| Phase 4 | All integration PRs (sequential) | ~6 days |
| **Total** | **Complete Pre-Trade** | **~27 days** |

**vs. Sequential Implementation:** ~66 days

**Time Savings:** ~59% faster

## Git Workflow Example

### Module PR Example (Indication)
```bash
# Start from updated main with foundation
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/pretrade-indication

# Implement Phases 1-3
# 1. Add enums to [SECTION 100] in src/types.rs
# 2. Add groups to [SECTION 100] in src/groups.rs
# 3. Create src/indication.rs
# 4. Create tests/indication_tests.rs
# 5. Uncomment module in src/lib.rs

# Commit
git add .
git commit -m "Pre-Trade Indication - Phases 1-3: Module implementation

- Add IOITransType, IOIQltyInd, AdvSide, AdvTransType, QtyType enums
- Add IOIQualGrp, RoutingGrp repeating groups
- Implement IOI, Advertisement, CrossRequest, CrossRequestAck messages
- Add comprehensive tests
- Uncomment indication module in lib.rs

Phase 4 (integration) will be done in a separate PR after all modules are merged."

# Push and create PR
git push origin feature/pretrade-indication
gh pr create --title "Pre-Trade Indication - Module Implementation (Phases 1-3)" \
  --body "Implements Indication category messages (IOI, Advertisement, CrossRequest, CrossRequestAck).

## Changes
- **Phase 1**: Added 5 enums to [SECTION 100] in types.rs
- **Phase 2**: Added 2 group configs to [SECTION 100] in groups.rs
- **Phase 3**: Created indication.rs with 4 message types
- **Tests**: Added indication_tests.rs with 15+ tests

## Does NOT include
- ❌ Changes to MsgType/FixMessage enums (Phase 4)
- ❌ CLI field mappings (Phase 4)

Phase 4 integration will be done in a separate PR after all module PRs are merged.

## Testing
\`\`\`bash
cargo test indication
cargo test --all
\`\`\`
"
```

### Integration PR Example (Indication)
```bash
# After all module PRs are merged
git checkout main
git pull origin main

# Create integration branch
git checkout -b feature/pretrade-indication-integration

# Implement Phase 4
# 1. Add to MsgType enum (alphabetical)
# 2. Add to FixMessage enum (alphabetical)
# 3. Update to_fix() and from_fix()
# 4. Update msg_type()
# 5. Add CLI field mappings

# Commit
git add .
git commit -m "Pre-Trade Indication - Phase 4: Integration

- Add Advertisement, CrossRequest, CrossRequestAck, IOI to MsgType (alphabetical)
- Add to FixMessage enum
- Update to_fix(), from_fix(), msg_type() methods
- Add CLI field mappings for tags 2-5, 23, 25-28, 104, 149, etc."

# Push and create PR
git push origin feature/pretrade-indication-integration
gh pr create --title "Pre-Trade Indication - Integration (Phase 4)" \
  --body "Integrates Indication messages into MsgType and FixMessage enums.

Completes the Indication category implementation started in #XX (module PR).

## Changes
- Add 4 message types to MsgType enum (alphabetical order)
- Add to FixMessage enum
- Update message type conversion methods
- Add CLI field name mappings

## Testing
\`\`\`bash
cargo test --all
\`\`\`
"
```

## Benefits of This Approach

✅ **Zero merge conflicts during parallel development** (Phases 1-3)
✅ **Clear separation of concerns** (modules vs integration)
✅ **Independent review and testing** of each category
✅ **Flexible merge order** for module PRs
✅ **Small, focused integration PRs** that are easy to review
✅ **59% faster than sequential implementation**

## Checklist for Each Module PR

- [ ] Enums added to correct [SECTION XXX] in `src/types.rs`
- [ ] Groups added to correct [SECTION XXX] in `src/groups.rs`
- [ ] Module file created (e.g., `src/indication.rs`)
- [ ] Test file created (e.g., `tests/indication_tests.rs`)
- [ ] Module uncommented in `src/lib.rs`
- [ ] Re-export uncommented in `src/lib.rs`
- [ ] Implementation plan exists in `docs/`
- [ ] All tests pass (`cargo test --all`)
- [ ] **NOT changed**: `src/messages.rs` (MsgType/FixMessage)
- [ ] **NOT added**: CLI field mappings

## Checklist for Each Integration PR

- [ ] Message types added to `MsgType` enum (alphabetical order)
- [ ] Message types added to `FixMessage` enum (alphabetical order)
- [ ] `to_fix()` method updated
- [ ] `from_fix()` method updated
- [ ] `msg_type()` method updated
- [ ] CLI field mappings added
- [ ] Message type names added to CLI
- [ ] All tests pass (`cargo test --all`)

## Questions?

See `docs/INDICATION_DESIGN.md` for a complete example implementation plan.

For questions about the parallel workflow, refer to this document or ask for clarification.
