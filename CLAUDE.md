# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Fixie is a Rust library and CLI tool for parsing and encoding FIX (Financial Information eXchange) protocol messages. It supports FIXT 1.1 (session layer) and FIX 5.0 SP2 (application layer).

**Status**: Pre-release, not production ready

**Current Support**:
- Infrastructure, pre-trade, and trade messages
- Post-trade messages are not yet supported (see FIX spec)

## Development Commands

### Building
```bash
# Build the library
cargo build

# Build the CLI tool (release mode)
cargo build --release -p fixie

# The binary will be in ./target/release/fixie
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test by name
cargo test test_parse_new_order_limit

# Run with output (helpful for debugging)
cargo test -- --nocapture

# Run only integration tests
cargo test --test integration_tests

# Run tests for entire workspace
cargo test --workspace
```

### Running the CLI
```bash
# Display FIX message as JSON (default)
fixie '<message>'

# Display FIX message pretty-printed
fixie --pp '<message>'

# Display FIX message as raw values
fixie --raw '<message>'
```

Note: Use single quotes around FIX messages to prevent shell interpretation of pipe characters.

### Linting and Formatting
```bash
# Format code
cargo fmt

# Check code with clippy
cargo clippy
```

## Code Architecture

### Module Organization

The codebase is organized into clear module boundaries:

```
src/
├── lib.rs              # Main library entry point with re-exports
├── parser.rs           # RawFixMessage parsing and encoding
├── groups.rs           # Repeating group registry and nested group support
├── types.rs            # FIX data types (Side, OrdType, etc.)
├── components.rs       # Reusable FIX components
├── messages.rs         # MsgType enum and top-level FixMessage enum
├── message_types/      # Message type implementations (organized by FIX spec category)
│   ├── infrastructure.rs
│   ├── single_general_order_handling.rs
│   ├── order_mass_handling.rs
│   ├── program_trading.rs
│   ├── quotation.rs
│   ├── market_data.rs
│   └── ...
└── bin/
    └── fixie.rs        # CLI tool implementation
```

### Key Architectural Patterns

#### 1. Two-Phase Parsing: Raw → Typed

FIX messages are parsed in two phases:

**Phase 1: Wire Format → RawFixMessage** (`parser.rs`)
- Parses wire format (SOH or pipe-delimited) into `RawFixMessage`
- `RawFixMessage` contains:
  - `fields: HashMap<u32, String>` - Top-level tag-value pairs
  - `groups: HashMap<u32, Vec<GroupEntryId>>` - Top-level repeating groups
  - `group_arena: Vec<GroupEntry>` - Flat arena for all group entries (for cache efficiency)
- Handles repeating groups with context-specific parsing based on `MsgType`

**Phase 2: RawFixMessage → Typed Messages** (`message_types/`)
- Each message type has a `from_raw()` method to convert from `RawFixMessage`
- Each message type has a `to_raw()` method to convert to `RawFixMessage`
- Type-safe representations with proper Rust types (enums, structs, etc.)

Example:
```rust
let raw = RawFixMessage::parse(fix_msg)?;
let order = NewOrderSingle::from_raw(&raw)?;
// ... work with typed data ...
let raw_again = order.to_raw();
let wire_format = raw_again.encode();
```

#### 2. Nested Repeating Groups with Arena Storage

Fixie supports multi-level nested repeating groups (up to 4 levels). The architecture uses:

**Arena-Based Storage**:
- All group entries (nested or not) are stored in a flat `Vec<GroupEntry>` arena
- Groups reference entries by index (`GroupEntryId = usize`)
- This provides cache-friendly memory layout and avoids deep nesting

**GroupEntry Structure**:
```rust
struct GroupEntry {
    fields: HashMap<u32, String>,
    nested_groups: HashMap<u32, Vec<GroupEntryId>>,  // References to nested entries
}
```

**Example Nesting**:
```
NewOrderList (E)
  └─ ListOrdGrp (NoOrders = 73)
       ├─ Order 1
       │    ├─ Parties (NoPartyIDs = 453)
       │    │    └─ Party 1
       │    │         └─ PartySubIDsGrp (NoPartySubIDs = 802)
       │    │              ├─ PartySubID 1
       │    │              └─ PartySubID 2
       │    └─ PreAllocGrp (NoAllocs = 78)
       │         └─ Alloc 1
       │              └─ NestedParties2 (NoNested2PartyIDs = 756)
       │                   └─ NestedParty 1
       │                        └─ NstdPtys2SubGrp (NoNested2PartySubIDs = 806)
```

#### 3. Group Registry System (`groups.rs`)

The `GROUP_REGISTRY` is a static registry that defines all repeating groups:

**Context-Specific Groups**:
- Groups can be message-specific or generic
- Lookup key: `(num_in_group_tag, msg_type)`
- Falls back to generic if message-specific not found

**GroupConfig**:
```rust
struct GroupConfig {
    num_in_group_tag: u32,     // The NoXXX tag
    delimiter_tag: u32,         // First tag of each entry
    member_tags: Vec<u32>,      // All tags in this group
    nested_groups: Vec<NestedGroupInfo>,  // Nested groups
}
```

**Important Functions**:
- `is_num_in_group_tag(tag, msg_type)` - Check if tag is a NoXXX tag
- `get_delimiter_tag(tag, msg_type)` - Get delimiter for group
- `get_member_tags(tag, msg_type)` - Get all member tags
- `get_nested_groups(tag, msg_type)` - Get nested group info
- `is_nested_group(parent_tag, child_tag, msg_type)` - Check nesting relationship

#### 4. Message Type Organization

Message types in `src/message_types/` mirror the FIX specification structure:

- **Infrastructure**: Logon, Logout, Heartbeat, BusinessMessageReject
- **Single General Order Handling**: NewOrderSingle, ExecutionReport, OrderCancelRequest, etc.
- **Order Mass Handling**: OrderMassCancelRequest, OrderMassCancelReport
- **Order Cross Handling**: NewOrderCross, CrossOrderCancelRequest
- **Program Trading**: NewOrderList, ListStatus
- **Mass Orders**: MassOrder, MassOrderAck, OrderMassActionRequest
- **Multileg Orders**: NewOrderMultileg, MultilegOrderCancelReplace
- **Market Data**: MarketDataRequest, MarketDataSnapshotFullRefresh
- **Quotation**: Quote, QuoteRequest, MassQuote
- **Event Communication**: Email, News
- **Market Structure**: MarketDefinition, TradingSessionStatus
- **Securities Reference**: SecurityList, SecurityDefinition

Each message type struct should:
- Derive `Debug, Clone, Serialize, Deserialize`
- Implement `from_raw()` for parsing
- Implement `to_raw()` for encoding
- Use Rust enums for FIX enum fields (e.g., `Side`, `OrdType`)

### Message Type Context in Group Parsing

When parsing repeating groups, the parser needs the `MsgType` (tag 35) for context:

```rust
// Parser captures MsgType during parsing
if tag == 35 {
    msg_type = Some(value.clone());
}

// Uses MsgType for context-specific group lookups
if is_num_in_group_tag(*tag, msg_type_str) {
    // Parse group with message-specific configuration
}
```

This allows the same NoXXX tag to have different meanings in different message types.

## Testing Strategy

### Test Organization

Tests are organized into:

1. **Unit tests** - In `src/` files with `#[cfg(test)]`
2. **Integration tests** - In `tests/` directory
3. **Fixture-based tests** - Using `.fix` and `.json` files in `tests/fixtures/`

### Fixture-Based Testing

Fixtures provide human-readable test cases:

**Structure**:
- `tests/fixtures/<test_name>.fix` - Raw FIX message (pipe-delimited)
- `tests/fixtures/<test_name>.json` - Expected parsed output

**Example**:
```bash
# new_order_limit.fix
8=FIXT.1.1|9=178|35=D|49=TRADER1|...|10=156|

# new_order_limit.json
{
  "BeginString": "FIXT.1.1",
  "MsgType": "D",
  ...
}
```

### Adding New Tests

When adding support for new message types:

1. Create fixture files in `tests/fixtures/`
2. Add integration test in appropriate `tests/*_tests.rs` file
3. Add unit tests in the message type file
4. Update test coverage matrix in `docs/TESTING.md`

### Running Specific Tests

```bash
# Run a specific test function
cargo test test_parse_new_order_limit

# Run all tests in a file
cargo test --test integration_tests

# Run tests matching a pattern
cargo test quotation
```

## Important Implementation Notes

### Delimiter Handling

FIX messages use SOH (`\x01`) as the standard delimiter, but the parser also supports pipe (`|`) for debugging:

```rust
pub const SOH: char = '\x01';

// Parser auto-detects delimiter
let delimiter = if input.contains(SOH) { SOH } else { '|' };
```

Always use SOH for wire format, pipe for human-readable examples.

### Checksum Calculation

The checksum (tag 10) is calculated as the sum of all bytes modulo 256:

```rust
let checksum: u32 = message.bytes().map(|b| b as u32).sum::<u32>() % 256;
message.push_str(&format!("10={:03}{}", checksum, SOH));
```

### Field Ordering

FIX messages have strict field ordering:
1. BeginString (8)
2. BodyLength (9)
3. Standard header: MsgType (35), SenderCompID (49), TargetCompID (56), MsgSeqNum (34), SendingTime (52), ApplVerID (1128)
4. Body fields (sorted by tag)
5. Repeating groups (NoXXX tag, then group entries)
6. CheckSum (10)

### Adding New Message Types

When implementing a new message type:

1. Add the message type to `MsgType` enum in `messages.rs`
2. Create the struct in appropriate `message_types/*.rs` file
3. Implement `from_raw()` and `to_raw()` methods
4. Add variant to `FixMessage` enum in `messages.rs`
5. Add repeating group configurations to `GROUP_REGISTRY` in `groups.rs` if needed
6. Add integration tests with fixtures
7. Update README.md supported message list

### Adding New Repeating Groups

When adding support for new repeating groups:

1. Add `GroupConfig` entry to `GROUP_REGISTRY` in `groups.rs`
2. Use message-specific `GroupKey` if the group is context-specific
3. Add nested groups to `nested_groups` vec if applicable
4. Write unit tests in `groups.rs` to verify configuration
5. Add integration tests for parsing/encoding

## Common Pitfalls

1. **Forgetting Message Context**: Some groups are message-specific. Always pass `msg_type` to group registry functions.

2. **Group Entry Delimiter**: The delimiter tag must be the first tag of each group entry. Verify against FIX spec.

3. **Nested Group Member Tags**: When a group has nested groups, the nested NoXXX tag should appear in the parent's `member_tags` list.

4. **Arena Indices**: Group entry IDs are indices into `group_arena`. Never use IDs after the arena is modified.

5. **Encoding Order**: When encoding, respect FIX field ordering rules. The `encode()` method handles this, but be careful with manual construction.

## Documentation

Key documentation files in `docs/`:
- `TESTING.md` - Testing strategy and test coverage matrix
- `NESTED_GROUPS.md` - Nested repeating groups architecture
- `NESTED_GROUPS_IMPLEMENTATION.md` - Implementation details for nested groups
- `PROGRAM_TRADING_DESIGN.md` - Program trading message design
- `MASS_ORDER_DESIGN.md` - Mass order message design
- `MULTILEG_ORDER_DESIGN.md` - Multileg order message design
- `INDICATION_DESIGN.md` - Pre-trade indication message design
- `QUOTATION_DESIGN.md` - Quotation message design (if exists)
- `PERFORMANCE.md` - Performance considerations

## Related Resources

- [FIX Protocol Specification](https://www.fixtrading.org/online-specification/)
- [FIX 5.0 SP2 Documentation](https://www.fixtrading.org/standards/fix-5-0-sp-2/)
- FIX Infrastructure: https://www.fixtrading.org/online-specification/business-area-infrastructure/
- FIX Pre-trade: https://www.fixtrading.org/online-specification/business-area-pretrade/
- FIX Trade: https://www.fixtrading.org/online-specification/business-area-trade/
