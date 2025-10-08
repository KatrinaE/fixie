# Nested Repeating Groups in Fixie

## Overview

Fixie supports multi-level nested repeating groups as defined in the FIX 5.0 SP2 specification. This allows for complex hierarchical structures like:

```
NewOrderCross (message 's')
  └─ SideCrossOrdModGrp (NoSides = 552)
       ├─ Side 1
       │    └─ Parties (NoPartyIDs = 453)
       │         ├─ Party 1
       │         │    └─ PartySubIDsGrp (NoPartySubIDs = 802)
       │         │         ├─ PartySubID 1
       │         │         └─ PartySubID 2
       │         └─ Party 2
       └─ Side 2
```

## Architecture

### Data Structures

#### NestedGroupInfo
Describes a nested group within a parent group:

```rust
pub struct NestedGroupInfo {
    /// The NoXXX tag that indicates the number of nested group entries
    pub num_in_group_tag: u32,
    /// Optional: specific parent field tag that contains this nested group
    /// None means it can appear anywhere in the parent group
    pub parent_tag: Option<u32>,
}
```

#### GroupConfig
Extended to include nested group information:

```rust
pub struct GroupConfig {
    pub num_in_group_tag: u32,
    pub delimiter_tag: u32,
    pub member_tags: Vec<u32>,
    pub nested_groups: Vec<NestedGroupInfo>,  // NEW
}
```

### Registry Structure

The `GROUP_REGISTRY` maps `GroupKey` to `GroupConfig`:

```rust
pub struct GroupKey {
    pub num_in_group_tag: u32,
    pub msg_type: Option<String>,  // Context-specific groups
}
```

## Examples

### Parties Component with Nested PartySubIDsGrp

```rust
// Parent group
registry.insert(
    GroupKey { num_in_group_tag: 453, msg_type: None },
    GroupConfig {
        num_in_group_tag: 453,  // NoPartyIDs
        delimiter_tag: 448,     // PartyID
        member_tags: vec![448, 447, 452],
        nested_groups: vec![
            NestedGroupInfo {
                num_in_group_tag: 802,  // NoPartySubIDs
                parent_tag: None,       // Can appear anywhere
            },
        ],
    },
);

// Nested group
registry.insert(
    GroupKey { num_in_group_tag: 802, msg_type: None },
    GroupConfig {
        num_in_group_tag: 802,  // NoPartySubIDs
        delimiter_tag: 523,     // PartySubID
        member_tags: vec![523, 803],
        nested_groups: vec![],  // No further nesting
    },
);
```

### Message-Specific Groups with Nesting

```rust
// SideCrossOrdModGrp for NewOrderCross ('s')
registry.insert(
    GroupKey { num_in_group_tag: 552, msg_type: Some("s".to_string()) },
    GroupConfig {
        num_in_group_tag: 552,
        delimiter_tag: 54,
        member_tags: vec![54, 2102, 41, 11, /* ... */],
        nested_groups: vec![
            NestedGroupInfo {
                num_in_group_tag: 453,  // Parties
                parent_tag: None,
            },
        ],
    },
);
```

## API Functions

### Query Functions

```rust
/// Get nested group information for a parent group
pub fn get_nested_groups(
    num_in_group_tag: u32,
    msg_type: Option<&str>
) -> Option<&'static Vec<NestedGroupInfo>>

/// Check if a tag is a nested group within a parent
pub fn is_nested_group(
    parent_num_in_group_tag: u32,
    child_num_in_group_tag: u32,
    msg_type: Option<&str>
) -> bool
```

### Usage Examples

```rust
// Check if PartySubIDsGrp (802) is nested within Parties (453)
assert!(is_nested_group(453, 802, None));

// Get nested groups for SideCrossOrdModGrp in message 's'
let nested = get_nested_groups(552, Some("s")).unwrap();
assert_eq!(nested[0].num_in_group_tag, 453);  // Parties

// Check two-level nesting
assert!(is_nested_group(552, 453, Some("s")));  // Level 1
assert!(is_nested_group(453, 802, None));       // Level 2
```

## Implementation Status

### Phase 1: Configuration (✅ Complete)
- [x] Add `NestedGroupInfo` struct
- [x] Extend `GroupConfig` with `nested_groups` field
- [x] Update existing group configurations
- [x] Add query helper functions
- [x] Write unit tests

### Phase 2: Parsing (Planned)
- [ ] Recursive group parsing in parser
- [ ] Parse nested groups within parent groups
- [ ] Maintain parent-child relationships
- [ ] Handle multiple nesting levels

### Phase 3: Encoding (Planned)
- [ ] Recursive group encoding
- [ ] Maintain proper field ordering
- [ ] Handle nested group serialization

## Testing

Unit tests cover:
- Nested group registration
- Message-specific nested groups
- Two-level nesting queries
- Non-existent group handling
- Context fallback behavior

Run tests:
```bash
cargo test --lib groups::tests
```

## References

- FIX 5.0 SP2 Specification
- FIX Trading Community Trade Appendix
- PROGRAM_TRADING_DESIGN.md (Phase 1, Goal 1)
