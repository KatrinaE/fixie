# FIX Indication Messages - Implementation Plan

This document provides the implementation plan for the Indication category of FIX Pre-Trade messages from the Business Area: Pre-Trade specification.

**Reference**: https://www.fixtrading.org/online-specification/business-area-pretrade/

---

## Overview

The Indication category contains messages used to communicate trading interest and cross order intentions before trade execution. These messages enable market participants to:

- **Advertise completed transactions** to the market
- **Indicate interest** in buying or selling securities
- **Request permission** to cross orders between parties
- **Acknowledge cross requests** from counterparties

### Message Types

- **Advertisement (7)** - Announce completed transactions
- **IOI (6)** - Market merchandise the broker is buying or selling
- **CrossRequest (DS)** - Indicate submission of orders that may result in a crossed trade
- **CrossRequestAck (DT)** - Confirm receipt of a CrossRequest

### Key Use Cases

1. **Post-Trade Advertisement**: Announce completed transactions to the market after execution
2. **Pre-Trade Indication**: Market merchandise to potential counterparties before formal quotes
3. **Cross Order Management**: Request permission to cross orders within the same firm/account
4. **Regulatory Compliance**: Satisfy requirements to announce cross intentions before execution

---

## Implementation Plan

### Phase 1: Enum Types and Field Definitions (2 days)

Add indication message enums to `src/types.rs`:

```rust
/// IOITransType (Tag 28) - Type of IOI transaction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IOITransType {
    New,       // N - New IOI
    Cancel,    // C - Cancel IOI
    Replace,   // R - Replace IOI
}

impl IOITransType {
    pub fn to_fix(&self) -> char {
        match self {
            IOITransType::New => 'N',
            IOITransType::Cancel => 'C',
            IOITransType::Replace => 'R',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'N' => Some(IOITransType::New),
            'C' => Some(IOITransType::Cancel),
            'R' => Some(IOITransType::Replace),
            _ => None,
        }
    }
}

/// IOIQltyInd (Tag 25) - Quality indicator for IOI
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IOIQltyInd {
    Low,      // L - Low quality
    Medium,   // M - Medium quality
    High,     // H - High quality
}

impl IOIQltyInd {
    pub fn to_fix(&self) -> char {
        match self {
            IOIQltyInd::Low => 'L',
            IOIQltyInd::Medium => 'M',
            IOIQltyInd::High => 'H',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'L' => Some(IOIQltyInd::Low),
            'M' => Some(IOIQltyInd::Medium),
            'H' => Some(IOIQltyInd::High),
            _ => None,
        }
    }
}

/// AdvSide (Tag 4) - Side of advertisement
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdvSide {
    Buy,   // B - Buy
    Sell,  // S - Sell
    Cross, // X - Cross
    Trade, // T - Trade
}

impl AdvSide {
    pub fn to_fix(&self) -> char {
        match self {
            AdvSide::Buy => 'B',
            AdvSide::Sell => 'S',
            AdvSide::Cross => 'X',
            AdvSide::Trade => 'T',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'B' => Some(AdvSide::Buy),
            'S' => Some(AdvSide::Sell),
            'X' => Some(AdvSide::Cross),
            'T' => Some(AdvSide::Trade),
            _ => None,
        }
    }
}

/// AdvTransType (Tag 5) - Type of advertisement transaction
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdvTransType {
    New,     // N - New
    Cancel,  // C - Cancel
    Replace, // R - Replace
}

impl AdvTransType {
    pub fn to_fix(&self) -> char {
        match self {
            AdvTransType::New => 'N',
            AdvTransType::Cancel => 'C',
            AdvTransType::Replace => 'R',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            'N' => Some(AdvTransType::New),
            'C' => Some(AdvTransType::Cancel),
            'R' => Some(AdvTransType::Replace),
            _ => None,
        }
    }
}

/// QtyType (Tag 854) - Type of quantity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum QtyType {
    Units,          // 0 - Units
    Contracts,      // 1 - Contracts
}

impl QtyType {
    pub fn to_fix(&self) -> char {
        match self {
            QtyType::Units => '0',
            QtyType::Contracts => '1',
        }
    }

    pub fn from_fix(c: char) -> Option<Self> {
        match c {
            '0' => Some(QtyType::Units),
            '1' => Some(QtyType::Contracts),
            _ => None,
        }
    }
}
```

All enums implement `to_fix()` and `from_fix()` methods with unit tests.

**Estimated effort**: 2 days
**Tests**: 5 unit tests (one per enum)

---

### Phase 2: Repeating Group Definitions (1 day)

Add indication message groups to `src/groups.rs`:

```rust
// IOIQualGrp (Tag 199 = NoIOIQualifiers) - for IOI message
GroupConfig {
    num_in_group_tag: 199,
    delimiter_tag: 104,    // IOIQualifier
    member_tags: vec![
        104,  // IOIQualifier
    ],
    nested_groups: vec![],
}

// RoutingGrp (Tag 215 = NoRoutingIDs) - for IOI message
GroupConfig {
    num_in_group_tag: 215,
    delimiter_tag: 216,    // RoutingType
    member_tags: vec![
        216,  // RoutingType
        217,  // RoutingID
    ],
    nested_groups: vec![],
}

// InstrmtLegIOIGrp (Tag 555 = NoLegs) - for IOI message
// Reuse existing InstrmtLegGrp configuration if already defined

// UndInstrmtGrp (Tag 711 = NoUnderlyings) - for IOI and Advertisement
// Reuse existing underlyings group configuration if already defined
```

**Estimated effort**: 1 day
**Tests**: 2 unit tests for new group configurations

---

### Phase 3: Message Structures (3 days)

Create `src/indication.rs`:

```rust
use crate::parser::RawFixMessage;
use crate::types::{IOITransType, IOIQltyInd, AdvSide, AdvTransType, QtyType, Side};

/// IOI (Indication of Interest) - MsgType 6
/// Market merchandise which the broker is buying or selling
pub struct IOI {
    pub ioi_id: String,                     // Required - Tag 23
    pub ioi_trans_type: IOITransType,       // Required - Tag 28
    pub side: Side,                         // Required - Tag 54
    pub ioi_qty: String,                    // Required - Tag 27 (e.g., "S" for small, "M" for medium, "L" for large, or numeric)

    // Symbol/Instrument (from Instrument component)
    pub symbol: Option<String>,             // Tag 55
    pub security_id: Option<String>,        // Tag 48
    pub security_id_source: Option<String>, // Tag 22

    // Optional fields
    pub ioi_ref_id: Option<String>,         // Tag 26
    pub currency: Option<String>,           // Tag 15
    pub price: Option<f64>,                 // Tag 44
    pub valid_until_time: Option<String>,   // Tag 62
    pub ioi_qlty_ind: Option<IOIQltyInd>,   // Tag 25
    pub text: Option<String>,               // Tag 58
    pub transact_time: Option<String>,      // Tag 60
    pub url_link: Option<String>,           // Tag 149

    // NoIOIQualifiers(199), NoRoutingIDs(215), NoLegs(555), NoUnderlyings(711)
    // accessed via RawFixMessage.groups
}

/// Advertisement - MsgType 7
/// Announce completed transactions
pub struct Advertisement {
    pub adv_id: String,                     // Required - Tag 2
    pub adv_trans_type: AdvTransType,       // Required - Tag 5
    pub adv_side: AdvSide,                  // Required - Tag 4
    pub quantity: f64,                      // Required - Tag 53

    // Symbol/Instrument (from Instrument component)
    pub symbol: Option<String>,             // Tag 55
    pub security_id: Option<String>,        // Tag 48
    pub security_id_source: Option<String>, // Tag 22

    // Optional fields
    pub adv_ref_id: Option<String>,         // Tag 3
    pub qty_type: Option<QtyType>,          // Tag 854
    pub price: Option<f64>,                 // Tag 44
    pub currency: Option<String>,           // Tag 15
    pub trade_date: Option<String>,         // Tag 75
    pub transact_time: Option<String>,      // Tag 60
    pub text: Option<String>,               // Tag 58
    pub url_link: Option<String>,           // Tag 149
    pub last_mkt: Option<String>,           // Tag 30
    pub trading_session_id: Option<String>, // Tag 336
    pub trading_session_sub_id: Option<String>, // Tag 625

    // NoLegs(555), NoUnderlyings(711) accessed via RawFixMessage.groups
}

/// CrossRequest - MsgType DS
/// Indicate submission of orders that may result in a crossed trade
pub struct CrossRequest {
    pub cross_request_id: String,           // Required - Tag 2672

    // Symbol/Instrument (from Instrument component)
    pub symbol: Option<String>,             // Tag 55
    pub security_id: Option<String>,        // Tag 48
    pub security_id_source: Option<String>, // Tag 22

    // Optional fields
    pub market_id: Option<String>,          // Tag 1301
    pub market_segment_id: Option<String>,  // Tag 1300
    pub order_qty: Option<f64>,             // Tag 38
    pub compliance_id: Option<String>,      // Tag 376
    pub compliance_text: Option<String>,    // Tag 2404
}

/// CrossRequestAck - MsgType DT
/// Confirm receipt of a CrossRequest
pub struct CrossRequestAck {
    pub cross_request_id: String,           // Required - Tag 2672

    // Symbol/Instrument (from Instrument component)
    pub symbol: Option<String>,             // Tag 55
    pub security_id: Option<String>,        // Tag 48
    pub security_id_source: Option<String>, // Tag 22

    // Optional fields
    pub market_id: Option<String>,          // Tag 1301
    pub market_segment_id: Option<String>,  // Tag 1300

    // NoHops, NoSecurityAltID, NoInstrumentParties accessed via RawFixMessage.groups
}

impl IOI {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, String> {
        // Parse required fields
        let ioi_id = raw.get_field_value(23)
            .ok_or("Missing required field: IOIID(23)")?.to_string();

        let ioi_trans_type_str = raw.get_field_value(28)
            .ok_or("Missing required field: IOITransType(28)")?;
        let ioi_trans_type = IOITransType::from_fix(
            ioi_trans_type_str.chars().next()
                .ok_or("Invalid IOITransType")?
        ).ok_or("Invalid IOITransType value")?;

        let side_str = raw.get_field_value(54)
            .ok_or("Missing required field: Side(54)")?;
        let side = Side::from_fix(
            side_str.chars().next()
                .ok_or("Invalid Side")?
        ).ok_or("Invalid Side value")?;

        let ioi_qty = raw.get_field_value(27)
            .ok_or("Missing required field: IOIQty(27)")?.to_string();

        // Parse optional fields
        let ioi_ref_id = raw.get_field_value(26).map(String::from);
        let symbol = raw.get_field_value(55).map(String::from);
        let security_id = raw.get_field_value(48).map(String::from);
        let security_id_source = raw.get_field_value(22).map(String::from);
        let currency = raw.get_field_value(15).map(String::from);
        let price = raw.get_field_value(44).and_then(|v| v.parse().ok());
        let valid_until_time = raw.get_field_value(62).map(String::from);

        let ioi_qlty_ind = raw.get_field_value(25)
            .and_then(|v| v.chars().next())
            .and_then(IOIQltyInd::from_fix);

        let text = raw.get_field_value(58).map(String::from);
        let transact_time = raw.get_field_value(60).map(String::from);
        let url_link = raw.get_field_value(149).map(String::from);

        Ok(IOI {
            ioi_id,
            ioi_trans_type,
            side,
            ioi_qty,
            symbol,
            security_id,
            security_id_source,
            ioi_ref_id,
            currency,
            price,
            valid_until_time,
            ioi_qlty_ind,
            text,
            transact_time,
            url_link,
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        // Required fields
        raw.add_field(23, &self.ioi_id);
        raw.add_field(28, &self.ioi_trans_type.to_fix().to_string());
        raw.add_field(54, &self.side.to_fix().to_string());
        raw.add_field(27, &self.ioi_qty);

        // Optional fields
        if let Some(ref v) = self.ioi_ref_id { raw.add_field(26, v); }
        if let Some(ref v) = self.symbol { raw.add_field(55, v); }
        if let Some(ref v) = self.security_id { raw.add_field(48, v); }
        if let Some(ref v) = self.security_id_source { raw.add_field(22, v); }
        if let Some(ref v) = self.currency { raw.add_field(15, v); }
        if let Some(v) = self.price { raw.add_field(44, &v.to_string()); }
        if let Some(ref v) = self.valid_until_time { raw.add_field(62, v); }
        if let Some(v) = self.ioi_qlty_ind { raw.add_field(25, &v.to_fix().to_string()); }
        if let Some(ref v) = self.text { raw.add_field(58, v); }
        if let Some(ref v) = self.transact_time { raw.add_field(60, v); }
        if let Some(ref v) = self.url_link { raw.add_field(149, v); }

        raw
    }
}

impl Advertisement {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, String> {
        // Parse required fields
        let adv_id = raw.get_field_value(2)
            .ok_or("Missing required field: AdvId(2)")?.to_string();

        let adv_trans_type_str = raw.get_field_value(5)
            .ok_or("Missing required field: AdvTransType(5)")?;
        let adv_trans_type = AdvTransType::from_fix(
            adv_trans_type_str.chars().next()
                .ok_or("Invalid AdvTransType")?
        ).ok_or("Invalid AdvTransType value")?;

        let adv_side_str = raw.get_field_value(4)
            .ok_or("Missing required field: AdvSide(4)")?;
        let adv_side = AdvSide::from_fix(
            adv_side_str.chars().next()
                .ok_or("Invalid AdvSide")?
        ).ok_or("Invalid AdvSide value")?;

        let quantity = raw.get_field_value(53)
            .ok_or("Missing required field: Quantity(53)")?
            .parse()
            .map_err(|_| "Invalid Quantity value")?;

        // Parse optional fields
        let adv_ref_id = raw.get_field_value(3).map(String::from);
        let symbol = raw.get_field_value(55).map(String::from);
        let security_id = raw.get_field_value(48).map(String::from);
        let security_id_source = raw.get_field_value(22).map(String::from);

        let qty_type = raw.get_field_value(854)
            .and_then(|v| v.chars().next())
            .and_then(QtyType::from_fix);

        let price = raw.get_field_value(44).and_then(|v| v.parse().ok());
        let currency = raw.get_field_value(15).map(String::from);
        let trade_date = raw.get_field_value(75).map(String::from);
        let transact_time = raw.get_field_value(60).map(String::from);
        let text = raw.get_field_value(58).map(String::from);
        let url_link = raw.get_field_value(149).map(String::from);
        let last_mkt = raw.get_field_value(30).map(String::from);
        let trading_session_id = raw.get_field_value(336).map(String::from);
        let trading_session_sub_id = raw.get_field_value(625).map(String::from);

        Ok(Advertisement {
            adv_id,
            adv_trans_type,
            adv_side,
            quantity,
            symbol,
            security_id,
            security_id_source,
            adv_ref_id,
            qty_type,
            price,
            currency,
            trade_date,
            transact_time,
            text,
            url_link,
            last_mkt,
            trading_session_id,
            trading_session_sub_id,
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        // Required fields
        raw.add_field(2, &self.adv_id);
        raw.add_field(5, &self.adv_trans_type.to_fix().to_string());
        raw.add_field(4, &self.adv_side.to_fix().to_string());
        raw.add_field(53, &self.quantity.to_string());

        // Optional fields
        if let Some(ref v) = self.adv_ref_id { raw.add_field(3, v); }
        if let Some(ref v) = self.symbol { raw.add_field(55, v); }
        if let Some(ref v) = self.security_id { raw.add_field(48, v); }
        if let Some(ref v) = self.security_id_source { raw.add_field(22, v); }
        if let Some(v) = self.qty_type { raw.add_field(854, &v.to_fix().to_string()); }
        if let Some(v) = self.price { raw.add_field(44, &v.to_string()); }
        if let Some(ref v) = self.currency { raw.add_field(15, v); }
        if let Some(ref v) = self.trade_date { raw.add_field(75, v); }
        if let Some(ref v) = self.transact_time { raw.add_field(60, v); }
        if let Some(ref v) = self.text { raw.add_field(58, v); }
        if let Some(ref v) = self.url_link { raw.add_field(149, v); }
        if let Some(ref v) = self.last_mkt { raw.add_field(30, v); }
        if let Some(ref v) = self.trading_session_id { raw.add_field(336, v); }
        if let Some(ref v) = self.trading_session_sub_id { raw.add_field(625, v); }

        raw
    }
}

impl CrossRequest {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, String> {
        let cross_request_id = raw.get_field_value(2672)
            .ok_or("Missing required field: CrossRequestID(2672)")?.to_string();

        let symbol = raw.get_field_value(55).map(String::from);
        let security_id = raw.get_field_value(48).map(String::from);
        let security_id_source = raw.get_field_value(22).map(String::from);
        let market_id = raw.get_field_value(1301).map(String::from);
        let market_segment_id = raw.get_field_value(1300).map(String::from);
        let order_qty = raw.get_field_value(38).and_then(|v| v.parse().ok());
        let compliance_id = raw.get_field_value(376).map(String::from);
        let compliance_text = raw.get_field_value(2404).map(String::from);

        Ok(CrossRequest {
            cross_request_id,
            symbol,
            security_id,
            security_id_source,
            market_id,
            market_segment_id,
            order_qty,
            compliance_id,
            compliance_text,
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        raw.add_field(2672, &self.cross_request_id);

        if let Some(ref v) = self.symbol { raw.add_field(55, v); }
        if let Some(ref v) = self.security_id { raw.add_field(48, v); }
        if let Some(ref v) = self.security_id_source { raw.add_field(22, v); }
        if let Some(ref v) = self.market_id { raw.add_field(1301, v); }
        if let Some(ref v) = self.market_segment_id { raw.add_field(1300, v); }
        if let Some(v) = self.order_qty { raw.add_field(38, &v.to_string()); }
        if let Some(ref v) = self.compliance_id { raw.add_field(376, v); }
        if let Some(ref v) = self.compliance_text { raw.add_field(2404, v); }

        raw
    }
}

impl CrossRequestAck {
    pub fn from_raw(raw: &RawFixMessage) -> Result<Self, String> {
        let cross_request_id = raw.get_field_value(2672)
            .ok_or("Missing required field: CrossRequestID(2672)")?.to_string();

        let symbol = raw.get_field_value(55).map(String::from);
        let security_id = raw.get_field_value(48).map(String::from);
        let security_id_source = raw.get_field_value(22).map(String::from);
        let market_id = raw.get_field_value(1301).map(String::from);
        let market_segment_id = raw.get_field_value(1300).map(String::from);

        Ok(CrossRequestAck {
            cross_request_id,
            symbol,
            security_id,
            security_id_source,
            market_id,
            market_segment_id,
        })
    }

    pub fn to_raw(&self) -> RawFixMessage {
        let mut raw = RawFixMessage::new();

        raw.add_field(2672, &self.cross_request_id);

        if let Some(ref v) = self.symbol { raw.add_field(55, v); }
        if let Some(ref v) = self.security_id { raw.add_field(48, v); }
        if let Some(ref v) = self.security_id_source { raw.add_field(22, v); }
        if let Some(ref v) = self.market_id { raw.add_field(1301, v); }
        if let Some(ref v) = self.market_segment_id { raw.add_field(1300, v); }

        raw
    }
}
```

Each struct implements:
- `from_raw(raw: &RawFixMessage) -> Result<Self, String>`
- `to_raw(&self) -> RawFixMessage`
- Round-trip unit tests

**Estimated effort**: 3 days
**Tests**: 4 round-trip tests (one per message type)

---

### Phase 4: Integration (1 day)

- Add to `MsgType` enum in `src/messages.rs`: `IOI`, `Advertisement`, `CrossRequest`, `CrossRequestAck`
- Update `to_fix()` method: return "6", "7", "DS", "DT"
- Update `from_fix()` method: parse "6", "7", "DS", "DT"
- Add to `FixMessage` enum
- Update `msg_type()` method
- Add CLI field mappings for tags:
  - 2, 3, 4, 5: AdvId, AdvRefID, AdvSide, AdvTransType
  - 23, 25, 26, 27, 28: IOIID, IOIQltyInd, IOIRefID, IOIQty, IOITransType
  - 30: LastMkt
  - 53: Quantity
  - 104: IOIQualifier
  - 149: URLLink
  - 199: NoIOIQualifiers
  - 215, 216, 217: NoRoutingIDs, RoutingType, RoutingID
  - 376: ComplianceID
  - 854: QtyType
  - 1300, 1301: MarketSegmentID, MarketID
  - 2404: ComplianceText
  - 2672: CrossRequestID
- Add message type names to CLI

**Estimated effort**: 1 day
**Tests**: Verify all existing tests pass

---

### Phase 5: Testing (2 days)

Add integration tests to `tests/integration_tests.rs`:

#### IOI Tests
- Test IOI new/cancel/replace transaction types
- Test IOI with various quality indicators
- Test IOI with instrument component fields
- Test IOI with repeating groups (qualifiers, routing, legs, underlyings)
- Test round-trip conversion

#### Advertisement Tests
- Test Advertisement new/cancel/replace transaction types
- Test Advertisement with all sides (buy, sell, cross, trade)
- Test Advertisement with optional price and currency
- Test Advertisement with trading session information
- Test round-trip conversion

#### CrossRequest Tests
- Test CrossRequest with minimal required fields
- Test CrossRequest with market identification
- Test CrossRequest with compliance information
- Test round-trip conversion

#### CrossRequestAck Tests
- Test CrossRequestAck acknowledgement
- Test CrossRequestAck with market information
- Test round-trip conversion

**Estimated effort**: 2 days
**Tests**: 15+ integration tests

---

## Summary

### Total Implementation Timeline

| Phase | Description | Duration | Deliverables |
|-------|-------------|----------|--------------|
| Phase 1 | Enum Types and Field Definitions | 2 days | 5 enums with conversions and tests |
| Phase 2 | Repeating Group Definitions | 1 day | 2-4 group configs with tests |
| Phase 3 | Message Structures | 3 days | 4 message types with parsing |
| Phase 4 | Integration | 1 day | Full integration into fixie |
| Phase 5 | Testing | 2 days | 15+ integration tests |
| **Total** | **Complete Indication Category** | **9 days** | **4 messages, 5 enums, 2+ groups** |

### Key Design Patterns

All implementation follows established fixie patterns:
- Enums with `to_fix()`/`from_fix()` conversions
- Structs with `from_raw()`/`to_raw()` methods
- Repeating groups in arena-based storage (via `RawFixMessage.groups`)
- Round-trip testing for all message types
- CLI integration for field name display
- Comprehensive integration tests

### Dependencies

No new dependencies required. Uses existing:
- serde (serialization)
- chrono (timestamps)
- ahash (hash maps)

### Testing Strategy

Each message type includes:
- **Unit tests**: Enum conversions (in `types.rs`)
- **Unit tests**: Group configurations (in `groups.rs`)
- **Round-trip tests**: Message parsing and encoding (in `indication.rs`)
- **Integration tests**: Real-world scenarios and edge cases (in `tests/integration_tests.rs`)
- **Regression tests**: Ensure existing functionality remains intact

---

## Next Steps

1. Create git branch: `feature/indication-messages`
2. Implement Phase 1: Enum types (commit: "Indication - Phase 1: Add enums")
3. Implement Phase 2: Groups (commit: "Indication - Phase 2: Add repeating groups")
4. Implement Phase 3: Messages (commit: "Indication - Phase 3: Add message structures")
5. Implement Phase 4: Integration (commit: "Indication - Phase 4: Integration")
6. Implement Phase 5: Testing (commit: "Indication - Phase 5: Testing complete")
7. Merge to main after all tests pass

**Reference Documentation**:
- See `docs/INFRASTRUCTURE_DESIGN.md` for implementation pattern reference
- See `docs/PROGRAM_TRADING_DESIGN.md` for complex message examples
- See `BIBLIOGRAPHY.md` for FIX specification references

---

## Related Work

After Indication messages are complete, the remaining Pre-Trade categories can be implemented:

1. **Event Communication** (Email, News) - 2 messages, ~4 days
2. **Quotation/Negotiation** (Quote Request, Quote, etc.) - 11 messages, ~15 days
3. **Market Data** (Market Data Request, Snapshot, etc.) - 3 messages, ~6 days
4. **Market Structure Reference Data** (Market Definition, Trading Session, etc.) - 8 messages, ~12 days
5. **Securities Reference Data** (Security Definition, Security List, etc.) - 15 messages, ~20 days

**Total Pre-Trade Implementation**: ~60 days for all categories
