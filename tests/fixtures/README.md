# FIX Test Fixtures

This directory contains test fixtures for validating FIX message parsing.

## Structure

Each test case consists of two files:

1. `<test_name>.fix` - Raw FIX message (pipe-delimited for readability)
2. `<test_name>.json` - Expected parsed output in JSON format

## Coverage Categories

### Session Messages
- [ ] logon_basic.fix - Basic logon message
- [ ] logout_with_text.fix - Logout with reason text
- [ ] heartbeat_response.fix - Heartbeat responding to TestRequest

### Order Messages
- [ ] new_order_market.fix - Market order
- [ ] new_order_limit.fix - Limit order
- [ ] order_cancel_request.fix - Cancel request

### Execution Reports
- [ ] exec_report_new.fix - Order acknowledged
- [ ] exec_report_partial_fill.fix - Partial fill
- [ ] exec_report_full_fill.fix - Complete fill
- [ ] exec_report_rejected.fix - Order rejected
- [ ] exec_report_cancelled.fix - Order cancelled

### Market Data
- [ ] market_data_request.fix - Subscribe to market data
- [ ] market_data_snapshot.fix - Quote update

### Edge Cases
- [ ] minimal_message.fix - Minimal valid message
- [ ] unknown_tags.fix - Message with unknown/custom tags
- [ ] message_with_repeating_groups.fix - Complex repeating group
- [ ] unicode_in_text.fix - Unicode characters in text field

## JSON Format

The JSON format should mirror the Rust struct, with special handling for:
- Enums: represented as strings (e.g., `"side": "Buy"`)
- DateTimes: ISO 8601 format
- Optional fields: `null` or omitted

Example:
```json
{
  "msg_type": "NewOrderSingle",
  "fields": {
    "cl_ord_id": "ORDER123",
    "symbol": "AAPL",
    "side": "Buy",
    "order_qty": 100.0,
    "ord_type": "Limit",
    "price": 150.25,
    "transact_time": "2025-10-06T15:30:00.000Z"
  }
}
```
