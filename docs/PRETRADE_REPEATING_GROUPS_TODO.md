# Pre-Trade Messages - Repeating Groups Enhancement

This document tracks Pre-Trade messages that currently have placeholder comments instead of full repeating group implementations.

## Overview

During the parallel Pre-Trade implementation (Phases 1-6), some messages were implemented with simplified placeholders for repeating groups to accelerate development. This document tracks which messages need to be enhanced with full repeating group support.

## Messages Requiring Enhancement

All of these messages are in **Securities Reference** category (`src/message_types/securities_reference.rs`):

### 1. SecurityList (MsgType=y)

**Location**: Line 485
**Current State**: Placeholder comment
**Missing Group**: NoRelatedSym (146)

**Required Implementation**:
```rust
/// Entry in the SecListGrp repeating group (NoRelatedSym=146)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityListEntry {
    /// Symbol (55) - Ticker symbol
    pub symbol: String,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// SecurityDesc (107) - Description of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_desc: Option<String>,

    /// SecurityType (167) - Type of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// MaturityMonthYear (200) - Month and year of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_month_year: Option<String>,

    /// StrikePrice (202) - Strike price for options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,

    /// Currency (15) - Currency of the security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    // ... additional fields as per FIX spec
}

pub struct SecurityList {
    // ... existing fields ...

    /// SecListGrp - Repeating group of securities (NoRelatedSym=146)
    pub securities: Vec<SecurityListEntry>,
}
```

**groups.rs Entry Needed**:
```rust
// SecListGrp - Securities in a list (NoRelatedSym = 146)
registry.insert(146, GroupConfig {
    count_tag: 146,
    first_tag: 55,  // Symbol
    member_tags: vec![55, 48, 22, 107, 167, 200, 202, 15, /* ... */],
    nested_groups: vec![],
    message_specific: Some(MessageTypeContext {
        msg_type: "y",
        additional_tags: vec![],
    }),
});
```

---

### 2. SecurityListUpdateReport (MsgType=BK)

**Location**: Line 578
**Current State**: Placeholder comment
**Missing Group**: NoRelatedSym (146)

**Required Implementation**:
```rust
/// Entry in the SecListUpdRelSymGrp repeating group (NoRelatedSym=146)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityListUpdateEntry {
    /// Symbol (55) - Ticker symbol
    pub symbol: String,

    /// SecurityUpdateAction (980) - Action to take for this security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_update_action: Option<SecurityUpdateAction>,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// SecurityDesc (107) - Description of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_desc: Option<String>,

    // ... additional fields
}

pub struct SecurityListUpdateReport {
    // ... existing fields ...

    /// SecListUpdRelSymGrp - Repeating group of security updates (NoRelatedSym=146)
    pub security_updates: Vec<SecurityListUpdateEntry>,
}
```

**groups.rs Entry Needed**: Similar to SecurityList but for message type "BK"

---

### 3. SecurityTypes (MsgType=w)

**Location**: Line 891
**Current State**: Placeholder comment
**Missing Group**: NoSecurityTypes (558)

**Required Implementation**:
```rust
/// Entry in the SecTypesGrp repeating group (NoSecurityTypes=558)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTypeEntry {
    /// SecurityType (167) - Type of security
    pub security_type: String,

    /// SecuritySubType (762) - Sub-type qualification/classification of security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_sub_type: Option<String>,

    /// Product (460) - Product classification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<u32>,

    /// CFICode (461) - Classification of Financial Instruments code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,

    /// Text (58) - Free format text string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

pub struct SecurityTypes {
    // ... existing fields ...

    /// SecTypesGrp - Repeating group of security types (NoSecurityTypes=558)
    pub security_types: Vec<SecurityTypeEntry>,
}
```

**groups.rs Entry Needed**:
```rust
// SecTypesGrp - Security types (NoSecurityTypes = 558)
registry.insert(558, GroupConfig {
    count_tag: 558,
    first_tag: 167,  // SecurityType
    member_tags: vec![167, 762, 460, 461, 58],
    nested_groups: vec![],
    message_specific: Some(MessageTypeContext {
        msg_type: "w",
        additional_tags: vec![],
    }),
});
```

---

### 4. DerivativeSecurityList (MsgType=AA)

**Location**: Line 1138
**Current State**: Placeholder comment
**Missing Group**: NoRelatedSym (146)

**Required Implementation**:
```rust
/// Entry in the DerivSecListGrp repeating group (NoRelatedSym=146)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivativeSecurityListEntry {
    /// Symbol (55) - Ticker symbol
    pub symbol: String,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// SecurityType (167) - Type of derivative security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_type: Option<String>,

    /// MaturityMonthYear (200) - Month and year of maturity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maturity_month_year: Option<String>,

    /// StrikePrice (202) - Strike price for options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,

    /// PutOrCall (201) - Indicates whether an option is a put or call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_or_call: Option<u8>,

    // ... additional derivative-specific fields
}

pub struct DerivativeSecurityList {
    // ... existing fields ...

    /// DerivSecListGrp - Repeating group of derivatives (NoRelatedSym=146)
    pub derivatives: Vec<DerivativeSecurityListEntry>,
}
```

**groups.rs Entry Needed**: Similar to SecurityList but for message type "AA" and with derivative-specific fields

---

### 5. SecurityMassStatus (MsgType=CO)

**Location**: Line 1395
**Current State**: Placeholder comment
**Missing Group**: NoSecurityStatus (146) - Note: This may use a different count tag

**Required Implementation**:
```rust
/// Entry in the SecMassStatGrp repeating group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMassStatusEntry {
    /// Symbol (55) - Ticker symbol
    pub symbol: String,

    /// SecurityID (48) - Security identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id: Option<String>,

    /// SecurityIDSource (22) - Identifies the type of SecurityID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_id_source: Option<String>,

    /// SecurityTradingStatus (326) - Trading status of the security
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_trading_status: Option<SecurityTradingStatus>,

    /// HaltReason (327) - Reason for trading halt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub halt_reason: Option<HaltReason>,

    /// TradeDate (75) - Trade date for which status applies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<String>,

    /// TransactTime (60) - Time of status update
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<String>,

    // ... additional fields
}

pub struct SecurityMassStatus {
    // ... existing fields ...

    /// SecMassStatGrp - Repeating group of security statuses
    pub security_statuses: Vec<SecurityMassStatusEntry>,
}
```

**groups.rs Entry Needed**: Verify the correct count tag from FIX spec (may not be 146)

---

## Implementation Plan

### Option A: Standalone Enhancement (Recommended)
Create a separate branch to enhance these 5 messages:
- Branch: `feature/pretrade-securities-repeating-groups`
- Modify only `securities_reference.rs` and `groups.rs`
- No changes to `messages.rs` or `fixie.rs` (message types already integrated)
- Add comprehensive tests for repeating group functionality

### Option B: Include in Post-Trade Phase 0
Add this work to the Post-Trade Phase 0 foundation:
- Would delay Post-Trade start
- But ensures all Pre-Trade messages are complete before starting Post-Trade

### Option C: Separate Phase 8
Do this after Post-Trade Phase 7 is complete:
- Allows Post-Trade work to proceed
- Can be done by a single agent
- Lower priority since current implementation works for simple cases

## Recommendation

**Option A** is recommended because:
1. Can be done in parallel with Post-Trade work
2. Relatively small scope (5 messages, same module)
3. No integration conflicts (messages already integrated)
4. Provides complete implementation before users depend on these messages

## Success Criteria

✅ All 5 messages have full repeating group implementations
✅ Group structures defined in each message file
✅ GroupConfig entries added to groups.rs
✅ Builder methods for adding/setting group entries
✅ Tests verify repeating group functionality
✅ No placeholder comments remain
✅ All existing tests still pass
✅ New tests added for repeating group scenarios

## Estimated Effort

- **Time**: 4-6 hours for one agent
- **Complexity**: Medium (requires understanding FIX spec for each group)
- **Risk**: Low (no integration work, isolated to one module)

## Notes

- Reference the FIX 5.0 SP2 specification for exact field lists
- Some groups may have nested groups (e.g., Parties within SecListGrp)
- Ensure all group entries use proper builder patterns
- Follow the patterns established in `program_trading.rs` and `mass_orders.rs` for repeating group implementation
