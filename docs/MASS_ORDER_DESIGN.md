# Mass Order Messages Implementation Plan

## Overview

Mass Order messages provide bulk order management capabilities in FIX 5.0 SP2. Unlike Program Trading (which deals with related orders in a list/basket), Mass Order messages handle multiple unrelated orders or actions on existing orders matching specific criteria.

## Message Types to Implement

### 1. MassOrder (DJ)
**Purpose**: Add, modify, or delete multiple unrelated orders using a single message

**Key Characteristics**:
- Individual orders treated as stand-alone
- Can vary by OrdType and TimeInForce
- More flexible than NewOrderList (orders don't need to be related)

**Key Fields**:
- OrderEntryAction (2429) - Required - Add/Modify/Delete
- OrderEntryGrp - Repeating group of orders

**Response**: MassOrderAck (DK)

### 2. MassOrderAck (DK)
**Purpose**: Acknowledge receipt and status of a MassOrder message

**Key Fields**:
- OrderResponseLevel (2427) - Level of detail in response
- OrderEntryAckGrp - Repeating group with acknowledgements

**Notes**:
- Provides order status, not immediate executions
- Individual executions come via ExecutionReport (8)

### 3. OrderMassActionRequest (CA)
**Purpose**: Request suspension, release, or cancellation of orders matching criteria

**Key Characteristics**:
- Bulk action on existing orders
- Uses criteria to match orders (not individual order IDs)
- Actions: Suspend, Release, Cancel

**Key Fields**:
- MassActionType (1373) - Required - Type of action
- ClOrdID (11) - Required - Unique ID for this request
- TargetMarketSegmentGrp - Optional repeating group

**Response**: OrderMassActionReport (BZ)

### 4. OrderMassActionReport (BZ)
**Purpose**: Acknowledge OrderMassActionRequest

**Key Fields**:
- ClOrdID (11) - From request
- MassActionReportID (1369) - Unique report ID
- MassActionType (1373) - Echo from request
- MassActionResponse (1375) - Accepted/Rejected
- TotalAffectedOrders (533) - Count of affected orders
- AffectedMarketSegmentGrp - Repeating group
- NotAffectedMarketSegmentGrp - Repeating group

**Notes**:
- Each affected order gets separate ExecutionReport (8)

### 5. OrderMassStatusRequest (AF)
**Purpose**: Request status for orders matching specific criteria

**Key Fields**:
- MassStatusReqID (584) - Required - Unique request ID
- MassStatusReqType (585) - Required - Type of status request
- ClOrdID (11) - Optional - Specific order

**Response**: ExecutionReport (8) with ExecType = I (Order Status) for each matching order

## Implementation Phases

### Phase 1: Enum Types and Field Definitions (Week 1)
**Goal**: Add all new enums and field types

**Tasks**:
1. Add to `src/types.rs`:
   ```rust
   /// OrderEntryAction (Tag 2429)
   pub enum OrderEntryAction {
       Add,      // 1
       Modify,   // 2
       Delete,   // 3
       Suspend,  // 4
       Release,  // 5
   }

   /// MassActionType (Tag 1373)
   pub enum MassActionType {
       SuspendOrders,           // 1
       ReleaseOrdersFromSuspension, // 2
       CancelOrders,            // 3
   }

   /// MassActionResponse (Tag 1375)
   pub enum MassActionResponse {
       Rejected,               // 0
       Accepted,               // 1
   }

   /// MassStatusReqType (Tag 585)
   pub enum MassStatusReqType {
       StatusForOrdersForParty,        // 1
       StatusForOrdersForSecurity,     // 2
       StatusForOrdersForUnderlyingSecurity, // 3
       StatusForOrdersForProduct,      // 4
       StatusForOrdersForCFICode,      // 5
       StatusForOrdersForSecurityType, // 6
       StatusForOrdersForTradingSession, // 7
       StatusForAllOrders,             // 8
       StatusForOrdersForPartyGroup,   // 9
   }

   /// OrderResponseLevel (Tag 2427)
   pub enum OrderResponseLevel {
       NoAck,                  // 0
       OnlyAckErrors,          // 1
       AckEachOrder,           // 2
   }
   ```

2. Add conversion methods (to_fix/from_fix) for all enums
3. Add unit tests for all enum conversions

### Phase 2: Repeating Group Definitions (Week 1-2)
**Goal**: Define repeating groups in `src/groups.rs`

**Groups to Add**:

1. **OrderEntryGrp (Tag 2430 = NoOrderEntries)**
   - Delimiter: ClOrdID (11)
   - Used in: MassOrder (DJ)
   - Fields: Full order specification (similar to NewOrderSingle)
   - Nested groups: Parties, PreAllocGrp, etc.

2. **OrderEntryAckGrp (Tag 2430 = NoOrderEntries)**
   - Delimiter: ClOrdID (11)
   - Used in: MassOrderAck (DK)
   - Fields: OrdStatus, OrdRejReason, Text, etc.

3. **TargetMarketSegmentGrp (Tag 1793 = NoTargetMarketSegments)**
   - Delimiter: MarketSegmentID (1300)
   - Used in: OrderMassActionRequest (CA)
   - Fields: MarketSegmentID, MarketID

4. **AffectedMarketSegmentGrp (Tag 1793 = NoAffectedMarketSegments)**
   - Delimiter: MarketSegmentID (1300)
   - Used in: OrderMassActionReport (BZ)
   - Fields: MarketSegmentID, AffectedOrders count

5. **NotAffectedMarketSegmentGrp (Tag 1793 = NoNotAffMarketSegments)**
   - Delimiter: MarketSegmentID (1300)
   - Used in: OrderMassActionReport (BZ)
   - Fields: MarketSegmentID, NotAffectedOrders count

### Phase 3: Message Structures (Week 2-3)
**Goal**: Implement typed message structs

**Order of Implementation**:
1. **OrderMassStatusRequest (AF)** - Simplest, good starting point
2. **OrderMassActionRequest (CA)** - Medium complexity
3. **OrderMassActionReport (BZ)** - Response to CA
4. **MassOrder (DJ)** - Most complex, has OrderEntryGrp
5. **MassOrderAck (DK)** - Response to DJ

**Implementation Pattern** (same as Program Trading):
```rust
pub struct MassOrder {
    // Required fields
    pub mass_order_id: String,              // Tag 2436
    pub order_entry_action: OrderEntryAction, // Tag 2429

    // Optional fields
    pub sender_comp_id: Option<String>,     // Tag 49
    pub target_comp_id: Option<String>,     // Tag 56

    // Repeating groups accessed via RawFixMessage.groups
    // OrderEntryGrp (2430) contains individual orders
}

impl MassOrder {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> { ... }
    pub fn to_raw(&self) -> RawFixMessage { ... }
}
```

### Phase 4: Integration (Week 3)
**Goal**: Wire into existing infrastructure

**Tasks**:
1. Add to `MsgType` enum in `src/messages.rs`:
   ```rust
   pub enum MsgType {
       // ... existing variants
       MassOrder,                // DJ
       MassOrderAck,             // DK
       OrderMassActionRequest,   // CA
       OrderMassActionReport,    // BZ
       OrderMassStatusRequest,   // AF
   }
   ```

2. Update `to_fix()` and `from_fix()` methods

3. Add to `FixMessage` enum:
   ```rust
   pub enum FixMessage {
       // ... existing variants
       MassOrder(MassOrder),
       MassOrderAck(MassOrderAck),
       OrderMassActionRequest(OrderMassActionRequest),
       OrderMassActionReport(OrderMassActionReport),
       OrderMassStatusRequest(OrderMassStatusRequest),
   }
   ```

4. Update CLI tool in `src/bin/fixie.rs`:
   - Add message type names
   - Add field name mappings for new tags:
     * 585 (MassStatusReqType)
     * 584 (MassStatusReqID)
     * 1373 (MassActionType)
     * 1375 (MassActionResponse)
     * 1369 (MassActionReportID)
     * 2427 (OrderResponseLevel)
     * 2429 (OrderEntryAction)
     * 2430 (NoOrderEntries)
     * 2436 (MassOrderID)
     * 1300 (MarketSegmentID)
     * 1793 (NoTargetMarketSegments)

### Phase 5: Testing (Week 4)
**Goal**: Comprehensive test coverage

**Test Categories**:

1. **Unit Tests** (in message structs):
   - Round-trip conversion for each message type
   - Enum conversions
   - Required field validation

2. **Integration Tests** (in `tests/integration_tests.rs`):
   - Parse OrderMassStatusRequest
   - Parse and convert OrderMassActionRequest
   - Parse OrderMassActionReport with affected/not-affected groups
   - Parse MassOrder with OrderEntryGrp
   - Parse MassOrderAck with acknowledgements
   - Round-trip all messages
   - Test repeating group parsing

3. **Workflow Tests**:
   - MassOrder → MassOrderAck flow
   - OrderMassActionRequest → OrderMassActionReport flow
   - OrderMassStatusRequest → ExecutionReport response

**Example Test**:
```rust
#[test]
fn test_mass_order_with_entries() {
    let msg = "8=FIXT.1.1|9=200|35=DJ|2436=MASS001|2429=1|\
2430=2|11=ORD1|55=AAPL|54=1|38=100|40=2|44=150.00|\
11=ORD2|55=MSFT|54=2|38=200|40=1|10=000|";

    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");

    // Verify message-level fields
    assert_eq!(parsed.get_field(35), Some("DJ"));
    assert_eq!(parsed.get_field(2436), Some("MASS001"));
    assert_eq!(parsed.get_field(2429), Some("1")); // Add

    // Verify OrderEntryGrp
    let entries = parsed.groups.get(&2430).expect("Should have OrderEntryGrp");
    assert_eq!(entries.len(), 2);

    let order1 = &parsed.group_arena[entries[0]];
    assert_eq!(order1.fields.get(&11), Some(&"ORD1".to_string()));
    assert_eq!(order1.fields.get(&55), Some(&"AAPL".to_string()));

    let order2 = &parsed.group_arena[entries[1]];
    assert_eq!(order2.fields.get(&11), Some(&"ORD2".to_string()));
    assert_eq!(order2.fields.get(&55), Some(&"MSFT".to_string()));
}
```

## Key Design Decisions

### 1. Reuse Existing Infrastructure
- Use same index-based arena for OrderEntryGrp as we do for ListOrdGrp
- Reuse nested group support (Parties, PreAllocGrp can nest within OrderEntryGrp)
- Follow same to_raw/from_raw pattern as Program Trading messages

### 2. Message Relationships
Document the request/response pairs:
- MassOrder (DJ) → MassOrderAck (DK)
- OrderMassActionRequest (CA) → OrderMassActionReport (BZ)
- OrderMassStatusRequest (AF) → ExecutionReport (8) with ExecType=I

### 3. Field Overlap with Existing Messages
Many fields overlap with existing messages:
- OrderEntryGrp entries are similar to NewOrderSingle
- Can reuse Side, OrdType, TimeInForce enums
- Can reuse Parties, Instrument, OrderQtyData components

### 4. Incremental Implementation
Start simple → complex:
1. OrderMassStatusRequest (simplest, no repeating groups)
2. OrderMassActionRequest/Report (simple repeating groups)
3. MassOrder/Ack (complex, OrderEntryGrp similar to NewOrderSingle)

## Estimated Timeline

- **Week 1**: Phase 1 (Enums) + Phase 2 (Groups) - 2-3 days
- **Week 2**: Phase 3 (Messages 1-3) - 4-5 days
- **Week 3**: Phase 3 (Messages 4-5) + Phase 4 (Integration) - 4-5 days
- **Week 4**: Phase 5 (Testing) - 3-4 days

**Total**: 3-4 weeks for complete Mass Order support

## Success Criteria

- [ ] All 5 Mass Order message types implemented
- [ ] All new enums with to_fix/from_fix conversions
- [ ] All repeating groups defined in GROUP_REGISTRY
- [ ] Integration with MsgType and FixMessage enums
- [ ] CLI displays Mass Order messages with proper field names
- [ ] Minimum 15 integration tests covering all messages
- [ ] All tests passing
- [ ] README.md updated with new message types

## Future Enhancements

After Mass Order implementation:
1. **Multileg Order Messages** (AB, AC) - For options/futures spreads
2. **Performance Optimization** - Profile parsing of large OrderEntryGrp
3. **Validation Rules** - Enforce field requirements per message type
4. **JSON Schema** - Generate JSON schemas for all message types

## Related Files

- `src/types.rs` - Enum definitions
- `src/groups.rs` - Repeating group configurations
- `src/messages.rs` - MsgType and FixMessage enums
- `src/mass_orders.rs` - NEW: Mass Order message structs
- `src/bin/fixie.rs` - CLI field name mappings
- `tests/integration_tests.rs` - Integration tests
- `README.md` - Documentation

## References

- [FIX Trading Community - Trade Business Area](https://www.fixtrading.org/online-specification/business-area-trade/)
- [FIX 5.0 SP2 Specification](https://www.fixtrading.org/online-specification/)
- Previous implementation: `docs/PROGRAM_TRADING_DESIGN.md`
