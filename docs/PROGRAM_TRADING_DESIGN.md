# Program Trading Implementation Design

## Overview

This document outlines the design and implementation plan for FIX 5.0 SP2 Program Trading / List Trading / Basket Trading messages in fixie. Program Trading messages are significantly more complex than single order messages, featuring multiple levels of repeating groups, nested structures, and extensive field sets.

## Background

Program Trading in FIX allows institutions to:
- Submit multiple orders as a single atomic unit
- Request bids for baskets of securities
- Execute, monitor, and cancel entire lists of orders
- Support both "Disclosed" and "Non-Disclosed" trading conventions

### Trading Conventions

**Non-Disclosed Model** (US/European):
- BidRequest contains sector, country, index, liquidity info
- No specific order details disclosed upfront
- Broker responds based on aggregate characteristics

**Disclosed Model** (Japanese):
- NewOrderList sent first with specific orders
- BidRequest references the list
- Broker responds with bids for specific orders

## Message Types

| Message | Type | Description |
|---------|------|-------------|
| NewOrderList | E | Submit multiple orders as a list/basket |
| BidRequest | k | Request bids for a program trade |
| BidResponse | l | Provide bid responses |
| ListExecute | L | Execute a previously submitted list |
| ListCancelRequest | K | Cancel an entire list |
| ListStatusRequest | M | Request status of a list |
| ListStatus | N | Report status of list orders |
| ListStrikePrice | m | Exchange strike prices for principal trades |

## Required Infrastructure Enhancements

### 1. Nested Repeating Groups Support

Currently, fixie supports one level of repeating groups. Program Trading requires multi-level nesting:

```
NewOrderList
  └─ ListOrdGrp (NoOrders = 73)
       ├─ Order 1
       │    ├─ Instrument component
       │    ├─ OrderQtyData component
       │    ├─ Parties component
       │    │    └─ PartySubIDsGrp (nested!)
       │    └─ PreAllocGrp component
       │         └─ NestedParties2 (nested!)
       ├─ Order 2
       │    └─ ...
       └─ Order N
```

**Required Changes:**
- Extend `GroupConfig` to support parent/child relationships
- Update parser to track nesting depth
- Add recursive parsing for nested groups
- Update encoder to serialize nested structures

### 2. New Repeating Group Definitions

The following repeating groups need to be defined in `groups.rs`:

#### ListOrdGrp (Tag 73 = NoOrders)
Used in: NewOrderList, ListStatus

**Delimiter:** Tag 11 (ClOrdID)

**Core Fields:**
- ClOrdID (11) - Required
- ListSeqNo (67) - Required
- SettlInstMode (160)
- Instrument component (Symbol, SecurityID, etc.)
- OrderQtyData component (OrderQty, CashOrderQty, etc.)
- Side (54)
- OrdType (40)
- Price (44)
- TimeInForce (59)
- TransactTime (60)
- Parties component (nested group!)
- PreAllocGrp component (nested group!)
- CommissionData component
- And 50+ more optional fields...

#### BidDescrReqGrp (Tag 398 = NoBidDescriptors)
Used in: BidRequest (Non-Disclosed convention)

**Delimiter:** Tag 399 (BidDescriptorType)

**Fields:**
- BidDescriptorType (399) - Required
- BidDescriptor (400)
- SideValueInd (401)
- LiquidityValue (404)
- LiquidityNumSecurities (441)
- LiquidityPctLow (402)
- LiquidityPctHigh (403)
- EFPTrackingError (405)
- FairValue (406)
- OutsideIndexPct (407)
- ValueOfFutures (408)

#### BidCompReqGrp (Tag 420 = NoBidComponents)
Used in: BidRequest (Disclosed convention)

**Delimiter:** Tag 66 (ListID)

**Fields:**
- ListID (66) - Required
- Side (54)
- TradingSessionID (336)
- TradingSessionSubID (625)
- NetGrossInd (430)
- SettlType (63)
- SettlDate (64)
- Account (1)

#### BidCompRspGrp (Tag 420 = NoBidComponents)
Used in: BidResponse

**Delimiter:** Tag 66 (ListID)

**Fields:**
- Commission (12)
- CommType (13)
- CommCurrency (479)
- FundRenewWaiv (497)
- ListID (66) - Required
- Country (421)
- Side (54)
- Price (44)
- PriceType (423)
- FairValue (406)
- NetGrossInd (430)
- SettlType (63)
- SettlDate (64)
- TradingSessionID (336)
- TradingSessionSubID (625)
- Text (58)

#### OrdListStatGrp (Tag 73 = NoOrders)
Used in: ListStatus

**Delimiter:** Tag 11 (ClOrdID)

**Fields:**
- ClOrdID (11) - Required
- CumQty (14) - Required
- OrdStatus (39) - Required
- WorkingIndicator (636)
- LeavesQty (151) - Required
- CxlQty (84) - Required
- AvgPx (6) - Required
- OrdRejReason (103)
- Text (58)
- Instrument component
- OrderQtyData component
- And many more...

#### StrikeRules (Tag 1201 = NoStrikeRules)
Used in: ListStrikePrice

**Delimiter:** Tag 1223 (StrikeRuleID)

**Fields:**
- StrikeRuleID (1223) - Required
- StartStrikePxRange (1202)
- EndStrikePxRange (1203)
- StrikeIncrement (1204)
- StrikeExerciseStyle (1304)
- MaturityMonthYearIncrementUnits (1302)
- MaturityMonthYearIncrement (1303)

### 3. New Enum Types

Add to `src/types.rs`:

```rust
/// BidType (Tag 394)
pub enum BidType {
    NonDisclosed,      // 1
    Disclosed,         // 2
    NoBiddingProcess,  // 3
}

/// ProgRptReqs (Tag 414)
pub enum ProgRptReqs {
    BuySideRequests,     // 1
    SellSideRequests,    // 2
    RealTimeExecutions,  // 3
}

/// ListExecInstType (Tag 433)
pub enum ListExecInstType {
    Immediate,              // 1
    WaitForInstruction,     // 2
    ExchangeSwitchCIVOrder, // 3
    SellDriven,             // 4
    BuyDrivenCash,          // 5
}

/// ListStatusType (Tag 429)
pub enum ListStatusType {
    Ack,         // 1
    Response,    // 2
    Timed,       // 3
    ExecStarted, // 4
    AllDone,     // 5
    Alert,       // 6
}

/// ListOrderStatus (Tag 431)
pub enum ListOrderStatus {
    InBiddingProcess,   // 1
    ReceivedForExecution, // 2
    Executing,          // 3
    Cancelling,         // 4
    Alert,              // 5
    AllDone,            // 6
    Reject,             // 7
}

/// BidDescriptorType (Tag 399)
pub enum BidDescriptorType {
    Sector,         // 1
    Country,        // 2
    Index,          // 3
}

/// SideValueInd (Tag 401)
pub enum SideValueInd {
    SideValue1,  // 1
    SideValue2,  // 2
}

/// NetGrossInd (Tag 430)
pub enum NetGrossInd {
    Net,   // 1
    Gross, // 2
}

/// PriceType (Tag 423)
pub enum PriceType {
    Percentage,      // 1
    PerUnit,         // 2
    FixedAmount,     // 3
    Discount,        // 4
    Premium,         // 5
    Spread,          // 6
    TEDPrice,        // 7
    TEDYield,        // 8
    Yield,           // 9
    FixedCabinetPrice, // 10
    VariableCabinetPrice, // 11
    // ... many more
}
```

### 4. Component Blocks

Several component blocks are reused across messages:

#### Instrument Component
Already partially implemented, but needs full field set:
- Symbol (55)
- SymbolSfx (65)
- SecurityID (48)
- SecurityIDSource (22)
- SecurityType (167)
- MaturityMonthYear (200)
- MaturityDate (541)
- CouponPaymentDate (224)
- IssueDate (225)
- RepoCollateralSecurityType (239)
- RepurchaseTerm (226)
- RepurchaseRate (227)
- Factor (228)
- CreditRating (255)
- InstrRegistry (543)
- CountryOfIssue (470)
- StateOrProvinceOfIssue (471)
- LocaleOfIssue (472)
- RedemptionDate (240)
- StrikePrice (202)
- StrikeCurrency (947)
- OptAttribute (206)
- ContractMultiplier (231)
- CouponRate (223)
- SecurityExchange (207)
- Issuer (106)
- SecurityDesc (107)
- Pool (691)
- And more...

#### OrderQtyData Component
- OrderQty (38)
- CashOrderQty (152)
- OrderPercent (516)
- RoundingDirection (468)
- RoundingModulus (469)

#### PreAllocGrp Component (Nested Repeating Group!)
Tag 78 = NoAllocs

**Delimiter:** Tag 79 (AllocAccount)

**Fields:**
- AllocAccount (79) - Required
- AllocAcctIDSource (661)
- AllocSettlCurrency (736)
- IndividualAllocID (467)
- NestedParties2 (nested group with Tag 756!)
- AllocQty (80)

#### NestedParties2 (Doubly-Nested!)
Tag 756 = NoNested2PartyIDs

**Delimiter:** Tag 757 (Nested2PartyID)

This demonstrates the need for multi-level nesting support.

## Implementation Plan

### Phase 1: Infrastructure (Week 1-2)

**Goal:** Support nested repeating groups

1. **Extend GroupConfig Structure**
   ```rust
   pub struct GroupConfig {
       pub num_in_group_tag: u32,
       pub delimiter_tag: u32,
       pub member_tags: Vec<u32>,
       pub nested_groups: Vec<NestedGroupInfo>, // NEW
   }

   pub struct NestedGroupInfo {
       pub num_in_group_tag: u32,
       pub parent_tag: Option<u32>, // Which parent field contains this
   }
   ```

2. **Update RawFixMessage Structure**
   ```rust
   pub struct RawFixMessage {
       pub fields: HashMap<u32, String>,
       pub groups: HashMap<u32, Vec<GroupEntry>>, // Changed
   }

   pub struct GroupEntry {
       pub fields: HashMap<u32, String>,
       pub nested_groups: HashMap<u32, Vec<GroupEntry>>, // Recursive!
   }
   ```

3. **Implement Recursive Parser**
   - Modify `parse()` to track nesting depth
   - Add `parse_group_entry()` recursive function
   - Handle parent context in group lookups

4. **Implement Recursive Encoder**
   - Update `encode()` to serialize nested structures
   - Maintain proper field ordering in nested contexts

5. **Testing**
   - Unit tests for single-level groups (existing)
   - Unit tests for nested groups
   - Unit tests for doubly-nested groups
   - Round-trip tests

### Phase 2: Enum Types (Week 2)

**Goal:** Add all Program Trading enums

1. Add enums to `src/types.rs`:
   - BidType
   - ProgRptReqs
   - ListExecInstType
   - ListStatusType
   - ListOrderStatus
   - BidDescriptorType
   - SideValueInd
   - NetGrossInd
   - PriceType

2. Implement `to_fix()` and `from_fix()` for each

3. Add unit tests for enum conversions

### Phase 3: Component Blocks (Week 3)

**Goal:** Implement reusable component structures

1. **Create `src/components.rs`**
   ```rust
   pub struct Instrument {
       pub symbol: Option<String>,
       pub security_id: Option<String>,
       pub security_type: Option<String>,
       // ... 40+ more fields
   }

   pub struct OrderQtyData {
       pub order_qty: Option<f64>,
       pub cash_order_qty: Option<f64>,
       // ...
   }

   pub struct CommissionData {
       pub commission: Option<f64>,
       pub comm_type: Option<char>,
       // ...
   }
   ```

2. Implement serialization/deserialization helpers

3. Add validation logic

### Phase 4: Repeating Group Definitions (Week 3-4)

**Goal:** Define all Program Trading groups

1. Add to `src/groups.rs`:
   - ListOrdGrp configuration
   - BidDescrReqGrp configuration
   - BidCompReqGrp configuration
   - BidCompRspGrp configuration
   - OrdListStatGrp configuration
   - StrikeRules configuration
   - PreAllocGrp configuration (with nesting)
   - NestedParties2 configuration (doubly-nested)

2. Include message-specific contexts where needed

3. Document field purposes and requirements

### Phase 5: Message Structures (Week 4-5)

**Goal:** Implement all 8 message types

For each message, implement:
- Struct with all required/optional fields
- `to_raw()` method
- `from_raw()` method
- Documentation

**Order of Implementation:**
1. NewOrderList (E) - Most complex, foundational
2. ListStatus (N) - Similar structure to NewOrderList
3. ListExecute (L) - Simpler, references existing list
4. ListCancelRequest (K) - Simpler
5. ListStatusRequest (M) - Simpler
6. BidRequest (k) - Medium complexity, two conventions
7. BidResponse (l) - Medium complexity
8. ListStrikePrice (m) - Specialized, different structure

### Phase 6: Integration (Week 5-6)

**Goal:** Wire everything together

1. Add message types to `MsgType` enum
2. Add to `FixMessage` enum
3. Update CLI tool with message type names
4. Add field name mappings for display
5. Update README documentation

### Phase 7: Testing (Week 6)

**Goal:** Comprehensive test coverage

1. **Test Fixtures**
   - Create .fix and .json files for each message type
   - Include examples of both trading conventions
   - Test fragmentation scenarios
   - Test nested group scenarios

2. **Integration Tests**
   - Parse tests for each message
   - Typed conversion tests
   - Round-trip tests
   - Error handling tests

3. **Property-Based Tests**
   - Random message generation
   - Invariant checking

## Challenges and Solutions

### Challenge 1: Recursive Data Structures in Rust

**Problem:** Rust doesn't allow infinitely recursive types without indirection.

**Solution:** Use `Box<GroupEntry>` for nested groups or restructure to use indices/IDs.

```rust
pub struct GroupEntry {
    pub fields: HashMap<u32, String>,
    pub nested_groups: HashMap<u32, Vec<Box<GroupEntry>>>, // Boxed!
}
```

### Challenge 2: Group Disambiguation

**Problem:** Some groups use the same NoXXX tag in different contexts (like NoOrders=73).

**Solution:** Already solved with message-type context in GroupKey. Extend to include parent group context:

```rust
pub struct GroupKey {
    pub num_in_group_tag: u32,
    pub msg_type: Option<String>,
    pub parent_context: Option<u32>, // NEW: parent group tag
}
```

### Challenge 3: Field Ordering in Nested Groups

**Problem:** FIX requires specific field ordering, especially complex with nesting.

**Solution:**
- Document canonical field order in group configs
- Encoder outputs in defined order
- Parser accepts any order (per FIX spec)

### Challenge 4: Message Size and Performance

**Problem:** Lists can contain hundreds of orders, creating large messages.

**Solution:**
- Implement fragmentation support (LastFragment tag)
- Use `String::with_capacity()` for pre-allocation
- Consider streaming parser for very large messages

### Challenge 5: Two Trading Conventions

**Problem:** BidRequest/BidResponse have mutually exclusive fields based on convention.

**Solution:**
- Use `Option<>` for all convention-specific fields
- Add validation helper to check consistency
- Document conventions clearly

```rust
impl BidRequest {
    pub fn validate_convention(&self) -> Result<(), String> {
        match self.bid_type {
            BidType::NonDisclosed => {
                if self.bid_descr_req_grp.is_empty() {
                    return Err("Non-disclosed requires BidDescrReqGrp".into());
                }
            }
            BidType::Disclosed => {
                if self.bid_comp_req_grp.is_empty() {
                    return Err("Disclosed requires BidCompReqGrp".into());
                }
            }
            _ => {}
        }
        Ok(())
    }
}
```

## API Design Examples

### NewOrderList with Nested Groups

```rust
pub struct NewOrderList {
    // Required fields
    pub list_id: String,                    // Tag 66
    pub bid_type: BidType,                  // Tag 394
    pub tot_no_orders: i32,                 // Tag 68

    // Optional fields
    pub bid_id: Option<String>,             // Tag 390
    pub client_bid_id: Option<String>,      // Tag 391
    pub prog_rpt_reqs: Option<ProgRptReqs>, // Tag 414
    pub list_exec_inst_type: Option<ListExecInstType>, // Tag 433

    // Note: Actual order details are in NoOrders(73) repeating group
    // which is accessed via raw.get_group(73) due to complexity
}

// Helper method to work with orders
impl NewOrderList {
    pub fn get_orders(&self, raw: &RawFixMessage) -> Option<Vec<ListOrder>> {
        raw.get_group(73).map(|entries| {
            entries.iter().map(|entry| {
                ListOrder::from_group_entry(entry)
            }).collect()
        })
    }
}

pub struct ListOrder {
    pub cl_ord_id: String,
    pub list_seq_no: i32,
    pub symbol: String,
    pub side: Side,
    pub order_qty: f64,
    pub ord_type: OrdType,
    pub price: Option<f64>,
    // ... many more fields
}
```

### BidRequest with Convention Support

```rust
pub struct BidRequest {
    // Common fields
    pub bid_id: String,                     // Tag 390
    pub client_bid_id: String,              // Tag 391
    pub bid_request_trans_type: char,       // Tag 374
    pub total_num_securities: i32,          // Tag 393
    pub bid_type: BidType,                  // Tag 394
    pub trade_date: String,                 // Tag 75

    // Convention-specific (mutually exclusive groups)
    // Non-Disclosed: Uses NoBidDescriptors(398) group
    // Disclosed: Uses NoBidComponents(420) group
    // Access via raw.get_group(398) or raw.get_group(420)
}
```

## File Structure

```
fixie/
├── src/
│   ├── lib.rs
│   ├── parser.rs         # Updated with nested group support
│   ├── types.rs          # Add Program Trading enums
│   ├── messages.rs       # Add 8 new message structs
│   ├── groups.rs         # Add group configs
│   ├── components.rs     # NEW: Reusable components
│   └── bin/
│       └── fixie.rs      # Update CLI
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/
│       ├── new_order_list.fix
│       ├── new_order_list.json
│       ├── bid_request_nondisclosed.fix
│       ├── bid_request_disclosed.fix
│       └── ... (16+ more fixtures)
├── docs/
│   ├── PROGRAM_TRADING_DESIGN.md  # This file
│   └── REPEATING_GROUPS.md        # NEW: Nesting documentation
└── BIBLIOGRAPHY.md       # Update with Program Trading refs
```

## Success Criteria

- [ ] All 8 Program Trading messages implemented
- [ ] Support for nested repeating groups (2+ levels deep)
- [ ] All required enums defined
- [ ] Parse and round-trip tests for each message
- [ ] Both trading conventions tested
- [ ] Fragmentation support tested
- [ ] Documentation complete
- [ ] All tests passing (estimate: 80+ tests total)
- [ ] Performance acceptable for 100+ order lists

## Estimated Effort

- **Infrastructure (nested groups):** 40-60 hours
- **Enums and components:** 20-30 hours
- **Group definitions:** 30-40 hours
- **Message implementations:** 60-80 hours
- **Testing and fixtures:** 40-50 hours
- **Documentation:** 10-15 hours

**Total:** 200-275 hours (5-7 weeks full-time)

## Dependencies

- chrono (already in use)
- serde (already in use)
- ahash (already in use)

No new dependencies required.

## Risks

1. **Complexity Underestimation** - Nested groups may reveal unforeseen parsing challenges
2. **Specification Ambiguity** - Some field interactions may be unclear
3. **Testing Coverage** - Comprehensive testing of all combinations is challenging
4. **Performance** - Large lists may stress current architecture

## Mitigation Strategies

1. Implement infrastructure incrementally with extensive testing at each step
2. Consult official FIX specification and community resources
3. Use property-based testing to increase coverage
4. Profile and optimize if performance issues arise

## Future Enhancements

After Program Trading is complete:

1. **Streaming Parser** - For very large messages
2. **Message Fragmentation Helpers** - Automatic splitting/reassembly
3. **Validation Framework** - Business rule validation
4. **Schema Generation** - Generate type-safe builders
5. **Performance Optimization** - Zero-copy parsing where possible

## References

- [FIX Trading Community - Trade Messages](https://www.fixtrading.org/online-specification/business-area-trade/)
- [OnixS FIX Dictionary 5.0 SP2](https://www.onixs.biz/fix-dictionary/5.0.sp2/index.html)
- [FIX Protocol Specification](https://www.fixtrading.org/standards/)
- fixie BIBLIOGRAPHY.md (to be updated)
