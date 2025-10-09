# Multileg Order Messages Implementation Plan

## Overview

Multileg Order messages enable trading of complex securities composed of multiple legs (contracts) that must be executed atomically. These messages are essential for:
- Options strategies (spreads, straddles, strangles, butterflies, condors, etc.)
- Futures calendar spreads and inter-commodity spreads
- Swaps and other multi-component derivatives
- Complex combination orders

Unlike Program Trading (which groups unrelated orders) or Mass Orders (which act on multiple orders), Multileg Orders represent a single trade of a multi-component security.

## Message Types to Implement

### 1. NewOrderMultileg (AB)
**Purpose**: Submit a new order for a multileg security

**Key Characteristics**:
- All legs execute atomically (all-or-none across legs)
- Each leg has its own instrument specification
- Pricing can be net across all legs or leg-specific
- Used for options spreads, futures spreads, swaps

**Key Fields**:
- ClOrdID (11) - Required - Client order ID
- Symbol (55) - Optional - Top-level symbol (strategy name)
- Side (54) - Required - Overall side (Buy/Sell)
- TransactTime (60) - Required - Time of order creation
- OrdType (40) - Required - Order type
- OrderQty (38) - Optional - Overall quantity (ratio applies to legs)
- Price (44) - Optional - Net price across all legs
- TimeInForce (59) - Optional
- ExecInst (18) - Optional - Execution instructions

**Repeating Groups**:
- LegOrdGrp (Tag 555 = NoLegs) - **Most important** - Defines each leg
  * Each leg contains:
    - Instrument component (Symbol, SecurityID, etc.)
    - LegSide (624) - Side for this leg
    - LegRatioQty (623) - Ratio quantity
    - LegPrice (566) - Leg-specific price (optional)
    - LegRefID (654) - Leg reference identifier
- Parties (Tag 453) - Optional
- PreAllocMlegGrp (Tag 78) - Optional pre-allocations

**Response**: ExecutionReport (8) with ExecType indicating new order

### 2. MultilegOrderCancelReplace (AC)
**Purpose**: Modify an existing multileg order

**Key Characteristics**:
- Must reference original ClOrdID
- Can modify price, quantity, or other parameters
- Leg structure typically remains the same (but can be changed)
- Atomic modification across all legs

**Key Fields**:
- ClOrdID (11) - Required - New client order ID
- OrigClOrdID (41) - Required - Original order to replace
- Symbol (55) - Optional - Strategy symbol
- Side (54) - Required - Overall side
- TransactTime (60) - Required - Time of modification
- OrdType (40) - Required - Order type
- OrderQty (38) - Optional - New quantity
- Price (44) - Optional - New net price

**Repeating Groups**:
- LegOrdGrp (Tag 555 = NoLegs) - Leg specifications
- Parties (Tag 453) - Optional
- PreAllocMlegGrp (Tag 78) - Optional

**Response**: ExecutionReport (8) or OrderCancelReject (9)

## Implementation Phases

### Phase 1: Enum Types and Field Definitions (Week 1)
**Goal**: Add multileg-specific enums and field types

**Tasks**:
1. Add to `src/types.rs`:
   ```rust
   /// MultilegReportingType (Tag 442)
   pub enum MultilegReportingType {
       SingleSecurity,              // 1
       IndividualLegOfMultilegSec,  // 2
       MultilegSecurity,            // 3
   }

   /// LegSide (Tag 624) - Side for individual leg
   /// Note: Reuses Side enum values but in leg context
   /// This might just be an alias: pub type LegSide = Side;

   /// MultilegModel (Tag 1377)
   pub enum MultilegModel {
       PredefinedMultilegSecurity,  // 0
       UserDefinedMultileg,         // 1
   }

   /// MultilegPriceMethod (Tag 1378)
   pub enum MultilegPriceMethod {
       NetPrice,                    // 0
       ReversedNetPrice,            // 1
       YieldDifference,             // 2
       Individual,                  // 3
       ContractWeightedAveragePrice, // 4
       MultipliedPrice,             // 5
   }
   ```

2. Add conversion methods (to_fix/from_fix) for all enums
3. Add unit tests for all enum conversions

**Estimated Time**: 1-2 days

### Phase 2: Repeating Group Definitions (Week 1-2)
**Goal**: Define LegOrdGrp and related groups in `src/groups.rs`

**Groups to Add**:

1. **LegOrdGrp (Tag 555 = NoLegs)**
   - Delimiter: LegSymbol (600)
   - Used in: NewOrderMultileg (AB), MultilegOrderCancelReplace (AC)
   - Fields: Complete leg specification
   - Nested groups: LegStipulations (683), NestedPartyIDs (539)

   ```rust
   // LegOrdGrp (Tag 555 = NoLegs)
   // Used in: NewOrderMultileg (AB), MultilegOrderCancelReplace (AC)
   GroupConfig {
       num_in_group_tag: 555,  // NoLegs
       delimiter_tag: 600,     // LegSymbol
       member_tags: vec![
           600,  // LegSymbol
           601,  // LegSymbolSfx
           602,  // LegSecurityID
           603,  // LegSecurityIDSource
           604,  // LegProduct
           605,  // LegCFICode
           606,  // LegSecurityType
           607,  // LegMaturityMonthYear
           608,  // LegMaturityDate
           609,  // LegStrikePrice
           610,  // LegOptAttribute
           611,  // LegContractMultiplier
           612,  // LegCouponRate
           613,  // LegSecurityExchange
           614,  // LegIssuer
           615,  // LegSecurityDesc
           616,  // LegRatioQty
           624,  // LegSide
           566,  // LegPrice
           654,  // LegRefID
           687,  // LegQty
           690,  // LegSwapType
           564,  // LegPositionEffect
           565,  // LegCoveredOrUncovered
       ],
       nested_groups: vec![
           NestedGroupInfo {
               num_in_group_tag: 683,  // LegStipulations (NoLegStipulations)
               parent_tag: None,
           },
           NestedGroupInfo {
               num_in_group_tag: 539,  // NestedPartyIDs (NoNestedPartyIDs)
               parent_tag: None,
           },
       ],
   }
   ```

2. **LegStipulations (Tag 683 = NoLegStipulations)**
   - Delimiter: LegStipulationType (688)
   - Fields: Leg-specific stipulations

3. **NestedPartyIDs (Tag 539 = NoNestedPartyIDs)**
   - Delimiter: NestedPartyID (524)
   - Used for leg-level party information
   - Similar to Parties but in nested context

4. **PreAllocMlegGrp (Tag 78 = NoAllocs)**
   - Similar to PreAllocGrp but for multileg context
   - May have leg-specific allocation details

**Estimated Time**: 2-3 days

### Phase 3: Message Structures (Week 2-3)
**Goal**: Implement typed message structs

**Order of Implementation**:
1. **NewOrderMultileg (AB)** - Core multileg order
2. **MultilegOrderCancelReplace (AC)** - Modification message

**Implementation Pattern**:
```rust
pub struct NewOrderMultileg {
    // Required fields
    pub cl_ord_id: String,                // Tag 11
    pub side: Side,                       // Tag 54
    pub transact_time: String,            // Tag 60
    pub ord_type: OrdType,                // Tag 40

    // Optional fields
    pub symbol: Option<String>,           // Tag 55 (strategy name)
    pub order_qty: Option<f64>,           // Tag 38
    pub price: Option<f64>,               // Tag 44 (net price)
    pub time_in_force: Option<TimeInForce>, // Tag 59
    pub exec_inst: Option<String>,        // Tag 18
    pub multileg_reporting_type: Option<MultilegReportingType>, // Tag 442
    pub multileg_price_method: Option<MultilegPriceMethod>, // Tag 1378

    // Component groups
    pub account: Option<String>,          // Tag 1
    pub currency: Option<String>,         // Tag 15
    pub text: Option<String>,             // Tag 58

    // Note: LegOrdGrp - Repeating group (Tag 555 = NoLegs)
    // This is stored in RawFixMessage.groups field with key 555
    // Each entry contains full leg specification with nested groups
}

impl NewOrderMultileg {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, FixParseError> { ... }
    pub fn to_raw(&self) -> RawFixMessage { ... }
}
```

**Estimated Time**: 4-5 days

### Phase 4: Integration (Week 3)
**Goal**: Wire into existing infrastructure

**Tasks**:
1. Add to `MsgType` enum in `src/messages.rs`:
   ```rust
   pub enum MsgType {
       // ... existing variants
       NewOrderMultileg,              // AB
       MultilegOrderCancelReplace,    // AC
   }
   ```

2. Update `to_fix()` and `from_fix()` methods

3. Add to `FixMessage` enum:
   ```rust
   pub enum FixMessage {
       // ... existing variants
       NewOrderMultileg(crate::multileg_orders::NewOrderMultileg),
       MultilegOrderCancelReplace(crate::multileg_orders::MultilegOrderCancelReplace),
   }
   ```

4. Update CLI tool in `src/bin/fixie.rs`:
   - Add message type names
   - Add field name mappings for new tags:
     * 555 (NoLegs)
     * 600 (LegSymbol)
     * 602 (LegSecurityID)
     * 604 (LegProduct)
     * 606 (LegSecurityType)
     * 616 (LegRatioQty)
     * 623 (LegRatioQty - alternative tag)
     * 624 (LegSide)
     * 566 (LegPrice)
     * 654 (LegRefID)
     * 687 (LegQty)
     * 442 (MultilegReportingType)
     * 1377 (MultilegModel)
     * 1378 (MultilegPriceMethod)
     * 539 (NoNestedPartyIDs)
     * 524 (NestedPartyID)
     * 683 (NoLegStipulations)
     * 688 (LegStipulationType)

**Estimated Time**: 2-3 days

### Phase 5: Testing (Week 4)
**Goal**: Comprehensive test coverage

**Test Categories**:

1. **Unit Tests** (in message structs):
   - Round-trip conversion for each message type
   - Enum conversions
   - Required field validation

2. **Integration Tests** (in `tests/integration_tests.rs`):
   - Parse NewOrderMultileg with basic fields
   - Parse NewOrderMultileg with LegOrdGrp (2-leg spread)
   - Parse NewOrderMultileg with complex 4-leg strategy
   - Parse NewOrderMultileg with nested leg stipulations
   - Parse MultilegOrderCancelReplace
   - Round-trip all messages
   - Test LegOrdGrp parsing (verify each leg)
   - Test nested groups within legs

3. **Real-World Scenarios**:
   - Vertical call spread (2 legs)
   - Iron condor (4 legs)
   - Calendar spread (2 legs, different expirations)
   - Futures inter-commodity spread

**Example Test**:
```rust
#[test]
fn test_new_order_multileg_vertical_spread() {
    // Vertical call spread: Buy 100 lower strike, Sell 100 higher strike
    let msg = "8=FIXT.1.1|9=300|35=AB|11=ML001|54=1|40=2|44=2.50|60=20251008-12:00:00.000|\
555=2|\
600=AAPL 250315C00150000|624=1|623=1|566=10.00|606=OPT|609=150|\
600=AAPL 250315C00160000|624=2|623=1|566=7.50|606=OPT|609=160|\
10=000|";

    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");

    // Verify message-level fields
    assert_eq!(parsed.get_field(35), Some("AB"));
    assert_eq!(parsed.get_field(11), Some("ML001"));
    assert_eq!(parsed.get_field(44), Some("2.50")); // Net debit

    // Verify LegOrdGrp
    let legs = parsed.groups.get(&555).expect("Should have LegOrdGrp");
    assert_eq!(legs.len(), 2);

    let leg1 = &parsed.group_arena[legs[0]];
    assert_eq!(leg1.fields.get(&600), Some(&"AAPL 250315C00150000".to_string()));
    assert_eq!(leg1.fields.get(&624), Some(&"1".to_string())); // Buy
    assert_eq!(leg1.fields.get(&566), Some(&"10.00".to_string()));

    let leg2 = &parsed.group_arena[legs[1]];
    assert_eq!(leg2.fields.get(&600), Some(&"AAPL 250315C00160000".to_string()));
    assert_eq!(leg2.fields.get(&624), Some(&"2".to_string())); // Sell
    assert_eq!(leg2.fields.get(&566), Some(&"7.50".to_string()));
}
```

**Estimated Time**: 4-5 days

## Key Design Decisions

### 1. Reuse Existing Infrastructure
- Use same index-based arena for LegOrdGrp as we do for ListOrdGrp and OrderEntryGrp
- Support nested groups within legs (LegStipulations, NestedPartyIDs)
- Follow same to_raw/from_raw pattern as other complex messages

### 2. Leg Representation
Each leg is essentially a mini-order with:
- Its own instrument specification
- Its own side (can differ from parent)
- Its own quantity ratio
- Optional leg-specific price
- Nested party and stipulation information

### 3. Atomic Execution Semantics
- All legs must execute together or not at all
- No partial fills across legs (though within a leg is possible)
- Execution reports should indicate multileg context

### 4. Pricing Models
Support multiple pricing methods:
- **Net Price**: Single price for the entire strategy (most common)
- **Individual**: Each leg has separate price
- **Yield Difference**: For fixed income spreads
- **Multiplied**: Product of leg prices

### 5. Common Use Cases to Support

**Options Spreads**:
- Vertical spreads (same expiration, different strikes)
- Calendar spreads (same strike, different expirations)
- Diagonal spreads (different strikes and expirations)
- Iron condor (4 legs: buy OTM put, sell ATM put, sell ATM call, buy OTM call)
- Butterfly spreads

**Futures Spreads**:
- Calendar spreads (same commodity, different months)
- Inter-commodity spreads (related commodities, e.g., crack spread)

**Fixed Income**:
- Bond spreads
- Curve trades

### 6. Field Overlap with Existing Messages
Many fields overlap with NewOrderSingle:
- Can reuse Side, OrdType, TimeInForce enums
- Can reuse Parties component
- Instrument fields extended with "Leg" prefix

## Estimated Timeline

- **Week 1**: Phase 1 (Enums) + Phase 2 (Groups) - 3-5 days
- **Week 2**: Phase 3 (Messages) - 4-5 days
- **Week 3**: Phase 4 (Integration) - 2-3 days
- **Week 4**: Phase 5 (Testing) - 4-5 days

**Total**: 3-4 weeks for complete Multileg Order support

## Success Criteria

- [ ] Both Multileg Order message types implemented
- [ ] All new enums with to_fix/from_fix conversions
- [ ] LegOrdGrp and nested groups defined in GROUP_REGISTRY
- [ ] Integration with MsgType and FixMessage enums
- [ ] CLI displays Multileg Order messages with proper field names
- [ ] Minimum 10 integration tests covering various strategies
- [ ] All tests passing
- [ ] README.md updated with new message types

## Future Enhancements

After Multileg Order implementation:
1. **Multileg Execution Reporting** - Enhanced ExecutionReport handling for multileg
2. **Strategy Builder** - Helper functions to construct common spreads
3. **Leg Analysis** - Calculate Greeks, profit/loss profiles
4. **Validation Rules** - Enforce valid leg combinations
5. **JSON Schema** - Generate JSON schemas for multileg messages

## Related Files

- `src/types.rs` - Enum definitions
- `src/groups.rs` - Repeating group configurations
- `src/messages.rs` - MsgType and FixMessage enums
- `src/multileg_orders.rs` - NEW: Multileg Order message structs
- `src/bin/fixie.rs` - CLI field name mappings
- `tests/integration_tests.rs` - Integration tests
- `README.md` - Documentation

## References

- [FIX Trading Community - Trade Business Area](https://www.fixtrading.org/online-specification/business-area-trade/)
- [FIX 5.0 SP2 Specification](https://www.fixtrading.org/online-specification/)
- Previous implementations:
  - `docs/PROGRAM_TRADING_DESIGN.md`
  - `docs/MASS_ORDER_DESIGN.md`

## Key Differences from Similar Message Types

### vs. NewOrderList (Program Trading)
- **NewOrderList**: Multiple unrelated orders in a basket
- **NewOrderMultileg**: Single order with multiple legs that must execute atomically
- **Key difference**: Multileg is ONE order, List is MANY orders

### vs. MassOrder
- **MassOrder**: Bulk submission of multiple unrelated orders
- **NewOrderMultileg**: Single complex order with interdependent legs
- **Key difference**: Mass = quantity, Multileg = complexity

### vs. NewOrderCross
- **NewOrderCross**: Two sides of a single security
- **NewOrderMultileg**: Multiple securities traded as one unit
- **Key difference**: Cross = one instrument two sides, Multileg = multiple instruments one strategy

## Trade-offs and Considerations

### Complexity vs. Benefit
- **High complexity**: Nested groups, leg-specific fields, multiple pricing models
- **High benefit**: Essential for derivatives trading, options market making, spread trading
- **Verdict**: Worth implementing - completes Trade message support

### Alternative Approaches Considered

1. **Simplified Leg Structure**:
   - Could limit legs to basic fields
   - **Rejected**: Real-world strategies need full instrument specs per leg

2. **Separate Leg Messages**:
   - Could send legs as separate orders, link later
   - **Rejected**: Loses atomic execution guarantee

3. **JSON-based Leg Definition**:
   - Could embed leg data as JSON in text field
   - **Rejected**: Not FIX-compliant, loses type safety

### Performance Considerations

- LegOrdGrp can have many entries (typical: 2-4, maximum: 999)
- Each leg has nested groups (parties, stipulations)
- Memory layout: Use arena structure like other repeating groups
- Parsing: Should handle 4-leg iron condor in < 1ms

## Notes for Implementation

1. **Start Simple**: Begin with 2-leg vertical spread (most common)
2. **Build Up**: Add support for more complex strategies incrementally
3. **Test Real Data**: Get sample messages from exchanges/brokers
4. **Document Examples**: Include real-world strategy examples in docs
5. **Validate Logic**: Ensure leg sides make sense (e.g., buy low strike, sell high strike)
