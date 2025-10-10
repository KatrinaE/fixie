# Post-Trade Implementation - Ready to Execute

## Summary

A comprehensive parallelizable workflow has been created for implementing **31 Post-Trade messages** across **6 categories**. The plan supports running 6 concurrent agents to maximize development efficiency.

## Documents Created

1. **`POSTTRADE_PARALLEL_WORKFLOW.md`** - Complete workflow plan
   - Phase 0: Foundation (serial)
   - Phases 1-6: Module implementation (parallel)
   - Phase 7: Integration (serial)
   - Full repeating groups implementation guide
   - All requirements documented

2. **`PRETRADE_REPEATING_GROUPS_TODO.md`** - Tracking document
   - Lists 5 Pre-Trade messages needing repeating groups
   - All in Securities Reference category
   - Can be done in parallel with Post-Trade work

## Implementation Requirements Summary

### Field Coverage
- ✅ **FULL** - All required and optional fields
- ✅ Reference: FIX 5.0 SP2 online specification
- ✅ Support custom tags (9000+ range)

### Repeating Groups
- ✅ **FULL IMPLEMENTATION** - No placeholders
- ✅ Define group structs in message files
- ✅ Register in `groups.rs`
- ✅ Implement builder methods
- ✅ Test repeating group functionality

### Documentation
- ✅ **REQUIRED** - Rustdoc comments on all structs
- ✅ Document message purpose
- ✅ Document each field with tag number

### Testing
- ✅ **MATCH PRE-TRADE LEVEL**
- ✅ Message creation tests
- ✅ Field setting tests (builder pattern)
- ✅ Round-trip tests
- ✅ Edge case tests

### Message Complexity
- ✅ **FULL** - All fields, all groups, all nesting
- ✅ Some messages may have 50+ fields
- ✅ Multiple levels of nested repeating groups

## Post-Trade Messages Breakdown

### Phase 1: Account Reporting (1 message)
- AccountSummaryReport (CQ)
- **Effort**: ~2-3 hours

### Phase 2: Position Maintenance (11 messages)
- AdjustedPositionReport (BL)
- AssignmentReport (AW)
- ContraryIntentionReport (BO)
- PositionMaintenanceReport (AM)
- PositionMaintenanceRequest (AL)
- PositionReport (AP)
- PositionTransferInstruction (DL)
- PositionTransferInstructionAck (DM)
- PositionTransferReport (DN)
- RequestForPositions (AN)
- RequestForPositionsAck (AO)
- **Effort**: ~8-12 hours

### Phase 3: Allocation (7 messages)
- AllocationInstruction (J)
- AllocationInstructionAck (P)
- AllocationInstructionAlert (BM)
- AllocationInstructionAlertRequest (DU)
- AllocationInstructionAlertRequestAck (DV)
- AllocationReport (AS)
- AllocationReportAck (AT)
- **Effort**: ~6-8 hours
- **Note**: AllocationInstruction is very complex

### Phase 4: Confirmation (3 messages)
- Confirmation (AK)
- ConfirmationAck (AU)
- ConfirmationRequest (BH)
- **Effort**: ~3-4 hours

### Phase 5: Settlement Instruction (3 messages)
- SettlementInstructionRequest (AV)
- SettlementInstructions (T)
- SettlementObligationReport (BQ)
- **Effort**: ~3-4 hours

### Phase 6: Trade Capture Reporting (6 messages)
- TradeCaptureReport (AE)
- TradeCaptureReportAck (AR)
- TradeCaptureReportRequest (AD)
- TradeCaptureReportRequestAck (AQ)
- TradeMatchReport (DC)
- TradeMatchReportAck (DD)
- **Effort**: ~5-7 hours

### Phase 7: Integration (1 agent)
- Merge all phase branches
- Add to MsgType and FixMessage enums
- Update CLI with field names
- Run full test suite
- **Effort**: ~2-3 hours

## Timeline

### Serial Execution
- Total: ~30-40 hours

### Parallel Execution (6 agents)
- Phase 0: 30 minutes
- Phases 1-6: ~12 hours (longest phase runs concurrently)
- Phase 7: 2-3 hours
- **Total: ~15 hours elapsed time**
- **Speedup: ~2.5-3x**

## Conflict Prevention Strategy - ZERO CONFLICTS ✅

Phase 0 pre-allocates sections AND uncomments all modules upfront:

**Files Modified in Phase 0**:
- `types.rs` - Pre-allocate sections 700, 710, 720, 730, 740, 750
- `groups.rs` - Pre-allocate comment-delimited sections per category
- `lib.rs` - **Uncomment ALL 6 module declarations** (empty modules compile fine!)
- `message_types/mod.rs` - **Add ALL 6 re-exports**
- Create all 6 empty module files

**Files Modified in Phases 1-6**:
- Each agent ONLY modifies their own files:
  - Their section in `types.rs` (e.g., Section 700)
  - Their section in `groups.rs` (e.g., Account Reporting section)
  - Their module file (e.g., `account_reporting.rs`)
- **NO touching** of `lib.rs` or `mod.rs`

**Files Modified in Phase 7**:
- `messages.rs` - Integration only
- `fixie.rs` - Integration only

**Result**: Complete isolation = **ZERO merge conflicts**!

## Execution Steps

### 1. Merge Phase 0 to Main
- Execute Phase 0 tasks
- Create PR: "Post-Trade Phase 0: Foundation"
- Merge to main

### 2. Launch Phases 1-6 in Parallel
Create 6 git worktrees or branches:
```bash
# Option A: Worktrees (recommended for parallel agents)
git worktree add ../fixie-account-reporting feature/posttrade-account-reporting
git worktree add ../fixie-position-maintenance feature/posttrade-position-maintenance
git worktree add ../fixie-allocation feature/posttrade-allocation
git worktree add ../fixie-confirmation feature/posttrade-confirmation
git worktree add ../fixie-settlement-instruction feature/posttrade-settlement-instruction
git worktree add ../fixie-trade-capture-reporting feature/posttrade-trade-capture-reporting

# Option B: Branches (if running agents sequentially)
git checkout -b feature/posttrade-account-reporting
# ... implement, commit, create PR ...
```

### 3. Review and Merge PRs from Phases 1-6
- Create 6 PRs (one per phase)
- Review and approve each PR
- **Merge to main as approved** (in any order - conflict-free!)
- No need to wait for all 6 - merge each as it's ready

### 4. Execute Phase 7 (After All Phases 1-6 are Merged)
- Create integration branch from main (already has all implementations)
- Add messages to central enums (messages.rs)
- Update CLI (fixie.rs)
- Test
- Create PR
- Merge after approval

## Success Metrics

Upon completion:
- ✅ 31 new Post-Trade messages implemented
- ✅ All with full repeating group support
- ✅ All with comprehensive tests
- ✅ All with Rustdoc documentation
- ✅ All integrated into central enums
- ✅ All with CLI support
- ✅ All tests passing (197 + new tests)
- ✅ README.md updated

## Pre-Trade Cleanup (Optional Parallel Work)

While Post-Trade is being implemented, you can optionally fix the 5 Pre-Trade messages with placeholder repeating groups:

**Branch**: `feature/pretrade-securities-repeating-groups`

**Messages**:
1. SecurityList (y)
2. SecurityListUpdateReport (BK)
3. SecurityTypes (w)
4. DerivativeSecurityList (AA)
5. SecurityMassStatus (CO)

**Effort**: ~4-6 hours
**Risk**: Low (isolated to one module)

See `PRETRADE_REPEATING_GROUPS_TODO.md` for details.

## Agent Prompts

When ready to execute, provide agents with prompts like:

```
Execute Post-Trade Phase 1: Account Reporting from docs/POSTTRADE_PARALLEL_WORKFLOW.md.

Requirements:
- Full implementation (all fields, full repeating groups)
- Rustdoc comments on all structs and fields
- Builder methods for all messages
- Comprehensive tests (creation, field setting, repeating groups)
- Reference FIX 5.0 SP2 online specification

When complete:
- Commit with message: "Account Reporting - Complete"
- Create PR: "Post-Trade Phase 1: Account Reporting Messages"
- Do not merge (wait for Phase 7)
```

## References

- **Workflow Document**: `docs/POSTTRADE_PARALLEL_WORKFLOW.md`
- **Pre-Trade Cleanup**: `docs/PRETRADE_REPEATING_GROUPS_TODO.md`
- **FIX Specification**: https://www.fixtrading.org/online-specification/
- **Existing Examples**:
  - `src/message_types/program_trading.rs` - Nested groups
  - `src/message_types/mass_orders.rs` - Repeating groups
  - `src/groups.rs` - Group registry

## Questions?

All implementation questions have been answered in the workflow document. If agents encounter ambiguity:
1. Check the repeating groups implementation guide
2. Reference existing Pre-Trade message implementations
3. Consult FIX 5.0 SP2 specification
4. Follow the pattern established in similar messages

## Ready to Execute

The plan is complete and ready for execution. You can start Phase 0 immediately, then launch all 6 parallel agents once Phase 0 is merged.
