# Testing Strategy for fixie

This document outlines the comprehensive testing approach for the fixie FIX parser library.

## Quickstart

 # Run a specific test
  cargo test -p fixie test_parse_new_order_limit

  # Run with verbose output
  cargo test -p fixie -- --nocapture

  # Run only integration tests
  cargo test -p fixie --test integration_tests

  # Run tests for entire workspace
  cargo test --workspace

## Testing Philosophy

Our testing strategy uses multiple complementary approaches to ensure correctness:

1. **Fixture-based tests**: Compare parsed output against known-good JSON representations
2. **Round-trip tests**: Ensure parse → encode → parse produces identical results
3. **Property-based tests**: Verify invariants hold for all valid inputs
4. **Edge case tests**: Handle malformed, minimal, and unusual messages
5. **Unit tests**: Test individual components in isolation

## Test Coverage Matrix

### Message Types (by FIX spec)

| Category | Message Type | Test Fixture | Status |
|----------|-------------|--------------|--------|
| Session | Logon (A) | `logon_basic.fix` | ✅ TODO |
| Session | Logout (5) | `logout_with_text.fix` | ✅ TODO |
| Session | Heartbeat (0) | `heartbeat_response.fix` | ✅ TODO |
| Session | TestRequest (1) | `test_request.fix` | ❌ TODO |
| Session | ResendRequest (2) | `resend_request.fix` | ❌ TODO |
| Session | Reject (3) | `reject_message.fix` | ❌ TODO |
| Order | NewOrderSingle (D) Market | `new_order_market.fix` | ❌ TODO |
| Order | NewOrderSingle (D) Limit | `new_order_limit.fix` | ✅ DONE |
| Order | OrderCancelRequest (F) | `order_cancel_request.fix` | ❌ TODO |
| Execution | ExecutionReport - New (8) | `exec_report_new.fix` | ❌ TODO |
| Execution | ExecutionReport - Partial Fill | `exec_report_partial.fix` | ❌ TODO |
| Execution | ExecutionReport - Full Fill | `exec_report_full.fix` | ❌ TODO |
| Execution | ExecutionReport - Rejected | `exec_report_rejected.fix` | ❌ TODO |
| Execution | ExecutionReport - Cancelled | `exec_report_cancelled.fix` | ❌ TODO |
| Execution | OrderCancelReject (9) | `order_cancel_reject.fix` | ❌ TODO |
| Market Data | MarketDataRequest (V) | `market_data_request.fix` | ❌ TODO |
| Market Data | MarketDataSnapshot (W) | `market_data_snapshot.fix` | ❌ TODO |

### Edge Cases

| Scenario | Test Name | Status |
|----------|-----------|--------|
| Minimal valid message | `test_minimal_message` | ❌ TODO |
| Unknown/custom tags | `test_unknown_tags_allowed` | ✅ DONE |
| Empty field values | `test_empty_field_value` | ✅ DONE |
| Missing required fields | `test_missing_required_field` | ✅ DONE |
| Invalid tag format | `test_invalid_tag_format` | ✅ DONE |
| Pipe vs SOH delimiters | `test_pipe_vs_soh_delimiter` | ✅ DONE |
| Unicode in text fields | `test_unicode_in_text` | ❌ TODO |
| Very large field values | `test_large_field_values` | ❌ TODO |
| Repeating groups | `test_repeating_groups` | ❌ TODO |
| Messages with 100+ tags | `test_large_message` | ❌ TODO |

## How to Add a New Test Case

### Step 1: Create Fixture Files

Create two files in `tests/fixtures/`:

**1. `<test_name>.fix`** - Raw FIX message (pipe-delimited for readability)
```
8=FIXT.1.1|35=D|55=AAPL|54=1|38=100|40=2|44=150.25|60=20250106-15:30:00.123|10=123|
```

**2. `<test_name>.json`** - Expected parsed output
```json
{
  "header": {
    "begin_string": "FIXT.1.1",
    "msg_type": "D",
    ...
  },
  "body": {
    "symbol": "AAPL",
    "side": "1",
    ...
  },
  "trailer": {
    "check_sum": "123"
  }
}
```

### Step 2: Add Integration Test

Add a test function in `tests/integration_tests.rs`:

```rust
#[test]
fn test_parse_<your_message_type>() {
    let fix_msg = load_fixture("<test_name>", "fix");
    let expected_json: Value = serde_json::from_str(
        &load_fixture("<test_name>", "json")
    ).unwrap();

    let parsed = RawFixMessage::parse(&fix_msg).unwrap();

    // Assert key fields match
    assert_eq!(parsed.get_field(35), Some("D"));
    assert_eq!(parsed.get_field(55), expected_json["body"]["symbol"].as_str());
    // ... more assertions
}
```

### Step 3: Update Coverage Matrix

Update the table in this document to mark your test as ✅ DONE.

## Running Tests

```bash
# Run all tests
cargo test -p fixie

# Run specific test
cargo test -p fixie test_parse_new_order_limit

# Run with output
cargo test -p fixie -- --nocapture

# Run integration tests only
cargo test -p fixie --test integration_tests
```

## Test Categories Explained

### 1. Fixture-Based Tests
**Purpose**: Verify correct parsing of real-world messages

**Advantages**:
- Easy to add new test cases
- Human-readable expected output (JSON)
- Can test complete message structures
- Good for regression testing

**Example**: `test_parse_new_order_limit`

### 2. Round-Trip Tests
**Purpose**: Ensure encoding and decoding are inverses

**Advantages**:
- Catches asymmetries in parse/encode logic
- No need to manually verify encoded format
- Tests actual wire format compatibility

**Example**: `test_round_trip_encoding`

### 3. Property-Based Tests
**Purpose**: Verify invariants hold for all valid inputs

**Properties we test**:
- All parsed messages have required fields (8, 35)
- Round-trip property: `parse(encode(parse(x))) == parse(x)`
- Field values are preserved (except calculated fields like checksum)
- Unknown tags are allowed and preserved

**Example**: `test_property_all_messages_round_trip`

### 4. Edge Case Tests
**Purpose**: Handle boundary conditions and error cases

**Covers**:
- Malformed messages
- Missing required fields
- Invalid tag formats
- Empty values
- Unknown tags
- Different delimiters

**Examples**: `test_missing_required_field`, `test_unknown_tags_allowed`

## Best Practices

1. **Test Names**: Use descriptive names like `test_parse_<message_type>_<variant>`
2. **Fixtures**: Use pipe delimiters (`|`) in `.fix` files for readability
3. **JSON Format**: Keep JSON structure consistent (header/body/trailer)
4. **Assertions**: Test the most important fields, not every single field
5. **Error Messages**: Use `.expect()` with clear messages for debugging
6. **Real Examples**: Base fixtures on actual FIX messages when possible

## Coverage Goals

- [ ] 100% of supported message types have fixture tests
- [ ] All enum values (Side, OrdType, OrdStatus, ExecType) are tested
- [ ] All error conditions have negative tests
- [ ] Round-trip property verified for all message types
- [ ] CLI tool has end-to-end tests

## Future Enhancements

1. **Snapshot Testing**: Use `insta` crate for automatic snapshot generation
2. **QuickCheck Integration**: Generate random valid FIX messages
3. **Benchmark Tests**: Track parsing performance over time
4. **FIX Spec Compliance**: Test against official FIX specification examples
5. **Fuzzing**: Use cargo-fuzz to find edge cases
6. **Performance Tests**: Ensure parser handles high-throughput scenarios

## Resources

- [FIX Protocol Specification](https://www.fixtrading.org/standards/)
- [FIX 5.0 SP2 Documentation](https://www.fixtrading.org/standards/fix-5-0-sp-2/)
- [Chrono DateTime Parsing](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)
