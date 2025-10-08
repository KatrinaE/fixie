use fixie::{RawFixMessage, NewOrderSingle, OrderCancelRequest, OrderCancelReplaceRequest, OrderCancelReject, OrderStatusRequest, DontKnowTrade, ExecutionAcknowledgement, OrderMassCancelRequest, OrderMassCancelReport, Side, OrdType, OrdStatus, DKReason, ExecAckStatus, MassCancelRequestType, MassCancelResponse};
use std::fs;
use std::path::PathBuf;
use serde_json::Value;

/// Helper to load fixture files
fn load_fixture(name: &str, ext: &str) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/fixtures");
    path.push(format!("{}.{}", name, ext));
    fs::read_to_string(path).expect(&format!("Failed to load fixture: {}.{}", name, ext))
}

#[test]
fn test_parse_new_order_limit() {
    let fix_msg = load_fixture("new_order_limit", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("new_order_limit", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify header fields
    assert_eq!(parsed.get_field(8), Some("FIXT.1.1"));
    assert_eq!(parsed.get_field(35), Some("D")); // NewOrderSingle
    assert_eq!(parsed.get_field(49), Some("BUYER"));
    assert_eq!(parsed.get_field(56), Some("SELLER"));

    // Verify body fields match expected JSON
    let expected_body = expected_json["body"].as_object().unwrap();
    assert_eq!(parsed.get_field(11).unwrap(), expected_body["cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(55).unwrap(), expected_body["symbol"].as_str().unwrap());
    assert_eq!(parsed.get_field(54).unwrap(), expected_body["side"].as_str().unwrap());
    assert_eq!(parsed.get_field(38).unwrap(), expected_body["order_qty"].as_str().unwrap());
    assert_eq!(parsed.get_field(40).unwrap(), expected_body["ord_type"].as_str().unwrap());
    assert_eq!(parsed.get_field(44).unwrap(), expected_body["price"].as_str().unwrap());
}

#[test]
fn test_parse_and_convert_to_typed_message() {
    let fix_msg = load_fixture("new_order_limit", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let order = NewOrderSingle::from_raw(&parsed).expect("Failed to convert to NewOrderSingle");

    // Verify typed fields
    assert_eq!(order.cl_ord_id, "ORD0001");
    assert_eq!(order.symbol, "AAPL");
    assert_eq!(order.side, Side::Buy);
    assert_eq!(order.order_qty, 100.0);
    assert_eq!(order.ord_type, OrdType::Limit);
    assert_eq!(order.price, Some(150.25));
}

#[test]
fn test_round_trip_encoding() {
    let fix_msg = load_fixture("new_order_limit", "fix");

    // Parse
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Encode
    let encoded = parsed1.encode();

    // Parse again
    let parsed2 = RawFixMessage::parse(&encoded).expect("Failed to parse encoded message");

    // Compare all fields (except BodyLength and CheckSum which are recalculated)
    for (tag, value) in &parsed1.fields {
        if *tag != 9 && *tag != 10 {  // Skip BodyLength and CheckSum
            assert_eq!(parsed2.get_field(*tag), Some(value.as_str()),
                "Field {} mismatch after round-trip", tag);
        }
    }
}

#[test]
fn test_pipe_vs_soh_delimiter() {
    let pipe_msg = "8=FIXT.1.1|35=D|55=AAPL|54=1|";
    let soh_msg = "8=FIXT.1.1\x0135=D\x0155=AAPL\x0154=1\x01";

    let parsed_pipe = RawFixMessage::parse(pipe_msg).expect("Failed to parse pipe message");
    let parsed_soh = RawFixMessage::parse(soh_msg).expect("Failed to parse SOH message");

    // Both should parse to the same fields
    assert_eq!(parsed_pipe.get_field(8), parsed_soh.get_field(8));
    assert_eq!(parsed_pipe.get_field(35), parsed_soh.get_field(35));
    assert_eq!(parsed_pipe.get_field(55), parsed_soh.get_field(55));
    assert_eq!(parsed_pipe.get_field(54), parsed_soh.get_field(54));
}

#[test]
fn test_missing_required_field() {
    // Missing MsgType (35)
    let invalid_msg = "8=FIXT.1.1|55=AAPL|";
    let result = RawFixMessage::parse(invalid_msg);
    assert!(result.is_err());
}

#[test]
fn test_invalid_tag_format() {
    let invalid_msg = "8=FIXT.1.1|35=D|abc=123|";
    let result = RawFixMessage::parse(invalid_msg);
    assert!(result.is_err());
}

#[test]
fn test_unknown_tags_allowed() {
    // Custom/unknown tags should be allowed
    let msg_with_custom = "8=FIXT.1.1|35=D|55=AAPL|9999=CUSTOM_VALUE|";
    let parsed = RawFixMessage::parse(msg_with_custom).expect("Should allow unknown tags");
    assert_eq!(parsed.get_field(9999), Some("CUSTOM_VALUE"));
}

#[test]
fn test_empty_field_value() {
    // Empty values should be allowed
    let msg = "8=FIXT.1.1|35=D|55=AAPL|58=|";
    let parsed = RawFixMessage::parse(msg).expect("Should allow empty values");
    assert_eq!(parsed.get_field(58), Some(""));
}

/// Property-based test: Any valid parsed message should round-trip correctly
#[test]
fn test_property_all_messages_round_trip() {
    let test_messages = vec![
        "8=FIXT.1.1|35=D|55=AAPL|54=1|",
        "8=FIXT.1.1|35=8|37=12345|11=ORD001|17=EXEC001|150=0|39=0|55=MSFT|54=2|151=100|14=0|6=0|60=20250101-12:00:00.000|",
        "8=FIXT.1.1|35=A|98=0|108=30|",
    ];

    for msg in test_messages {
        let parsed1 = RawFixMessage::parse(msg).expect("Should parse");
        let encoded = parsed1.encode();
        let parsed2 = RawFixMessage::parse(&encoded).expect("Should parse encoded");

        // All non-calculated fields should match
        for (tag, value) in &parsed1.fields {
            if *tag != 9 && *tag != 10 {
                assert_eq!(parsed2.get_field(*tag), Some(value.as_str()));
            }
        }
    }
}

#[test]
fn test_parse_order_cancel_replace() {
    let fix_msg = load_fixture("order_cancel_replace", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("order_cancel_replace", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify message type
    assert_eq!(parsed.get_field(35), Some("G")); // OrderCancelReplaceRequest

    // Verify body fields match expected JSON
    assert_eq!(parsed.get_field(41).unwrap(), expected_json["orig_cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(11).unwrap(), expected_json["cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(55).unwrap(), expected_json["symbol"].as_str().unwrap());
    assert_eq!(parsed.get_field(54).unwrap(), "1"); // Side::Buy
    assert_eq!(parsed.get_field(40).unwrap(), "2"); // OrdType::Limit
    assert_eq!(parsed.get_field(38).unwrap(), "200"); // order_qty
    assert_eq!(parsed.get_field(44).unwrap(), "155.75"); // price
}

#[test]
fn test_order_cancel_replace_typed_conversion() {
    let fix_msg = load_fixture("order_cancel_replace", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let order = OrderCancelReplaceRequest::from_raw(&parsed)
        .expect("Failed to convert to OrderCancelReplaceRequest");

    // Verify typed fields
    assert_eq!(order.orig_cl_ord_id, "ORD0001");
    assert_eq!(order.cl_ord_id, "ORD0002");
    assert_eq!(order.symbol, "AAPL");
    assert_eq!(order.side, Side::Buy);
    assert_eq!(order.ord_type, OrdType::Limit);
    assert_eq!(order.order_qty, 200.0);
    assert_eq!(order.price, Some(155.75));
}

#[test]
fn test_order_cancel_replace_round_trip() {
    let fix_msg = load_fixture("order_cancel_replace", "fix");

    // Parse to typed message
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");
    let order = OrderCancelReplaceRequest::from_raw(&parsed1)
        .expect("Failed to convert to OrderCancelReplaceRequest");

    // Convert back to raw
    let raw = order.to_raw();

    // Verify critical fields are preserved
    assert_eq!(raw.get_field(41), Some("ORD0001"));
    assert_eq!(raw.get_field(11), Some("ORD0002"));
    assert_eq!(raw.get_field(55), Some("AAPL"));
    assert_eq!(raw.get_field(54), Some("1"));
    assert_eq!(raw.get_field(40), Some("2"));
    assert_eq!(raw.get_field(38), Some("200"));
    assert_eq!(raw.get_field(44), Some("155.75"));
}

#[test]
fn test_parse_order_status_request() {
    let fix_msg = load_fixture("order_status_request", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("order_status_request", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify message type
    assert_eq!(parsed.get_field(35), Some("H")); // OrderStatusRequest

    // Verify body fields match expected JSON
    assert_eq!(parsed.get_field(11).unwrap(), expected_json["cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(55).unwrap(), expected_json["symbol"].as_str().unwrap());
    assert_eq!(parsed.get_field(54).unwrap(), "1"); // Side::Buy
    assert_eq!(parsed.get_field(37).unwrap(), expected_json["order_id"].as_str().unwrap());
}

#[test]
fn test_order_status_request_typed_conversion() {
    let fix_msg = load_fixture("order_status_request", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let request = OrderStatusRequest::from_raw(&parsed)
        .expect("Failed to convert to OrderStatusRequest");

    // Verify typed fields
    assert_eq!(request.cl_ord_id, "ORD0001");
    assert_eq!(request.symbol, "AAPL");
    assert_eq!(request.side, Side::Buy);
    assert_eq!(request.order_id, Some("12345".to_string()));
}

#[test]
fn test_order_status_request_round_trip() {
    let fix_msg = load_fixture("order_status_request", "fix");

    // Parse to typed message
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");
    let request = OrderStatusRequest::from_raw(&parsed1)
        .expect("Failed to convert to OrderStatusRequest");

    // Convert back to raw
    let raw = request.to_raw();

    // Verify critical fields are preserved
    assert_eq!(raw.get_field(11), Some("ORD0001"));
    assert_eq!(raw.get_field(55), Some("AAPL"));
    assert_eq!(raw.get_field(54), Some("1"));
    assert_eq!(raw.get_field(37), Some("12345"));
}

#[test]
fn test_parse_dont_know_trade() {
    let fix_msg = load_fixture("dont_know_trade", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("dont_know_trade", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify message type
    assert_eq!(parsed.get_field(35), Some("Q")); // DontKnowTrade

    // Verify body fields match expected JSON
    assert_eq!(parsed.get_field(37).unwrap(), expected_json["order_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(17).unwrap(), expected_json["exec_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(127).unwrap(), "D"); // DKReason::NoMatchingOrder
    assert_eq!(parsed.get_field(55).unwrap(), expected_json["symbol"].as_str().unwrap());
    assert_eq!(parsed.get_field(54).unwrap(), "2"); // Side::Sell
    assert_eq!(parsed.get_field(38).unwrap(), "100"); // order_qty
    assert_eq!(parsed.get_field(32).unwrap(), "100"); // last_qty
    // Note: Float formatting may not preserve trailing zeros
    assert!(parsed.get_field(31).unwrap().starts_with("250.5")); // last_px
}

#[test]
fn test_dont_know_trade_typed_conversion() {
    let fix_msg = load_fixture("dont_know_trade", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let dk = DontKnowTrade::from_raw(&parsed)
        .expect("Failed to convert to DontKnowTrade");

    // Verify typed fields
    assert_eq!(dk.order_id, "ORD123");
    assert_eq!(dk.exec_id, "EXEC456");
    assert_eq!(dk.dk_reason, DKReason::NoMatchingOrder);
    assert_eq!(dk.symbol, "MSFT");
    assert_eq!(dk.side, Side::Sell);
    assert_eq!(dk.order_qty, 100.0);
    assert_eq!(dk.last_qty, Some(100.0));
    assert_eq!(dk.last_px, Some(250.50));
    assert_eq!(dk.text, Some("No matching order found".to_string()));
}

#[test]
fn test_dont_know_trade_round_trip() {
    let fix_msg = load_fixture("dont_know_trade", "fix");

    // Parse to typed message
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");
    let dk = DontKnowTrade::from_raw(&parsed1)
        .expect("Failed to convert to DontKnowTrade");

    // Convert back to raw
    let raw = dk.to_raw();

    // Verify critical fields are preserved
    assert_eq!(raw.get_field(37), Some("ORD123"));
    assert_eq!(raw.get_field(17), Some("EXEC456"));
    assert_eq!(raw.get_field(127), Some("D"));
    assert_eq!(raw.get_field(55), Some("MSFT"));
    assert_eq!(raw.get_field(54), Some("2"));
    assert_eq!(raw.get_field(38), Some("100"));
}

#[test]
fn test_parse_execution_ack() {
    let fix_msg = load_fixture("execution_ack", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("execution_ack", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify message type
    assert_eq!(parsed.get_field(35), Some("BN")); // ExecutionAcknowledgement

    // Verify body fields match expected JSON
    assert_eq!(parsed.get_field(37).unwrap(), expected_json["order_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(17).unwrap(), expected_json["exec_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(1036).unwrap(), "1"); // ExecAckStatus::Accepted
    assert_eq!(parsed.get_field(55).unwrap(), expected_json["symbol"].as_str().unwrap());
    assert_eq!(parsed.get_field(54).unwrap(), "1"); // Side::Buy
    assert_eq!(parsed.get_field(38).unwrap(), "50"); // order_qty
}

#[test]
fn test_execution_ack_typed_conversion() {
    let fix_msg = load_fixture("execution_ack", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let ack = ExecutionAcknowledgement::from_raw(&parsed)
        .expect("Failed to convert to ExecutionAcknowledgement");

    // Verify typed fields
    assert_eq!(ack.order_id, "ORD789");
    assert_eq!(ack.exec_id, "EXEC101");
    assert_eq!(ack.exec_ack_status, ExecAckStatus::Accepted);
    assert_eq!(ack.symbol, "GOOGL");
    assert_eq!(ack.side, Side::Buy);
    assert_eq!(ack.order_qty, 50.0);
    assert_eq!(ack.cl_ord_id, Some("CLORD456".to_string()));
    assert_eq!(ack.last_qty, Some(50.0));
    assert_eq!(ack.last_px, Some(2800.00));
    assert_eq!(ack.cum_qty, Some(50.0));
    assert_eq!(ack.avg_px, Some(2800.00));
}

#[test]
fn test_execution_ack_round_trip() {
    let fix_msg = load_fixture("execution_ack", "fix");

    // Parse to typed message
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");
    let ack = ExecutionAcknowledgement::from_raw(&parsed1)
        .expect("Failed to convert to ExecutionAcknowledgement");

    // Convert back to raw
    let raw = ack.to_raw();

    // Verify critical fields are preserved
    assert_eq!(raw.get_field(37), Some("ORD789"));
    assert_eq!(raw.get_field(17), Some("EXEC101"));
    assert_eq!(raw.get_field(1036), Some("1"));
    assert_eq!(raw.get_field(55), Some("GOOGL"));
    assert_eq!(raw.get_field(54), Some("1"));
    assert_eq!(raw.get_field(38), Some("50"));
}

#[test]
fn test_parse_order_cancel_request() {
    let fix_msg = load_fixture("order_cancel_request", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("order_cancel_request", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify message type
    assert_eq!(parsed.get_field(35), Some("F")); // OrderCancelRequest

    // Verify body fields match expected JSON
    assert_eq!(parsed.get_field(41).unwrap(), expected_json["orig_cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(11).unwrap(), expected_json["cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(55).unwrap(), expected_json["symbol"].as_str().unwrap());
    assert_eq!(parsed.get_field(54).unwrap(), "1"); // Side::Buy
}

#[test]
fn test_order_cancel_request_typed_conversion() {
    let fix_msg = load_fixture("order_cancel_request", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let cancel = OrderCancelRequest::from_raw(&parsed)
        .expect("Failed to convert to OrderCancelRequest");

    // Verify typed fields
    assert_eq!(cancel.orig_cl_ord_id, "ORD0001");
    assert_eq!(cancel.cl_ord_id, "CANCEL001");
    assert_eq!(cancel.symbol, "TSLA");
    assert_eq!(cancel.side, Side::Buy);
}

#[test]
fn test_order_cancel_request_round_trip() {
    let fix_msg = load_fixture("order_cancel_request", "fix");

    // Parse to typed message
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");
    let cancel = OrderCancelRequest::from_raw(&parsed1)
        .expect("Failed to convert to OrderCancelRequest");

    // Convert back to raw
    let raw = cancel.to_raw();

    // Verify critical fields are preserved
    assert_eq!(raw.get_field(41), Some("ORD0001"));
    assert_eq!(raw.get_field(11), Some("CANCEL001"));
    assert_eq!(raw.get_field(55), Some("TSLA"));
    assert_eq!(raw.get_field(54), Some("1"));
}

#[test]
fn test_parse_order_cancel_reject() {
    let fix_msg = load_fixture("order_cancel_reject", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("order_cancel_reject", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify message type
    assert_eq!(parsed.get_field(35), Some("9")); // OrderCancelReject

    // Verify body fields match expected JSON
    assert_eq!(parsed.get_field(37).unwrap(), expected_json["order_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(11).unwrap(), expected_json["cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(41).unwrap(), expected_json["orig_cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(39).unwrap(), "2"); // OrdStatus::Filled
    assert_eq!(parsed.get_field(102).unwrap(), "1"); // cxl_rej_reason
}

#[test]
fn test_order_cancel_reject_typed_conversion() {
    let fix_msg = load_fixture("order_cancel_reject", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let reject = OrderCancelReject::from_raw(&parsed)
        .expect("Failed to convert to OrderCancelReject");

    // Verify typed fields
    assert_eq!(reject.order_id, "EX12345");
    assert_eq!(reject.cl_ord_id, "CANCEL001");
    assert_eq!(reject.orig_cl_ord_id, "ORD0001");
    assert_eq!(reject.ord_status, OrdStatus::Filled);
    assert_eq!(reject.cxl_rej_reason, 1);
    assert_eq!(reject.text, Some("Order already filled".to_string()));
}

#[test]
fn test_order_cancel_reject_round_trip() {
    let fix_msg = load_fixture("order_cancel_reject", "fix");

    // Parse to typed message
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");
    let reject = OrderCancelReject::from_raw(&parsed1)
        .expect("Failed to convert to OrderCancelReject");

    // Convert back to raw
    let raw = reject.to_raw();

    // Verify critical fields are preserved
    assert_eq!(raw.get_field(37), Some("EX12345"));
    assert_eq!(raw.get_field(11), Some("CANCEL001"));
    assert_eq!(raw.get_field(41), Some("ORD0001"));
    assert_eq!(raw.get_field(39), Some("2"));
    assert_eq!(raw.get_field(102), Some("1"));
}

#[test]
fn test_parse_order_mass_cancel_request() {
    let fix_msg = load_fixture("order_mass_cancel_request", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("order_mass_cancel_request", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify message type
    assert_eq!(parsed.get_field(35), Some("q")); // OrderMassCancelRequest

    // Verify body fields match expected JSON
    assert_eq!(parsed.get_field(11).unwrap(), expected_json["cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(530).unwrap(), "7"); // MassCancelRequestType::CancelAllOrders
}

#[test]
fn test_order_mass_cancel_request_typed_conversion() {
    let fix_msg = load_fixture("order_mass_cancel_request", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let req = OrderMassCancelRequest::from_raw(&parsed)
        .expect("Failed to convert to OrderMassCancelRequest");

    // Verify typed fields
    assert_eq!(req.cl_ord_id, "MASSCANCEL001");
    assert_eq!(req.mass_cancel_request_type, MassCancelRequestType::CancelAllOrders);
    assert_eq!(req.text, Some("Cancel all open orders".to_string()));
}

#[test]
fn test_order_mass_cancel_request_round_trip() {
    let fix_msg = load_fixture("order_mass_cancel_request", "fix");

    // Parse to typed message
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");
    let req = OrderMassCancelRequest::from_raw(&parsed1)
        .expect("Failed to convert to OrderMassCancelRequest");

    // Convert back to raw
    let raw = req.to_raw();

    // Verify critical fields are preserved
    assert_eq!(raw.get_field(11), Some("MASSCANCEL001"));
    assert_eq!(raw.get_field(530), Some("7"));
}

#[test]
fn test_parse_order_mass_cancel_report() {
    let fix_msg = load_fixture("order_mass_cancel_report", "fix");
    let expected_json: Value = serde_json::from_str(&load_fixture("order_mass_cancel_report", "json"))
        .expect("Failed to parse expected JSON");

    // Parse the message
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Verify message type
    assert_eq!(parsed.get_field(35), Some("r")); // OrderMassCancelReport

    // Verify body fields match expected JSON
    assert_eq!(parsed.get_field(37).unwrap(), expected_json["order_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(11).unwrap(), expected_json["cl_ord_id"].as_str().unwrap());
    assert_eq!(parsed.get_field(530).unwrap(), "7"); // MassCancelRequestType::CancelAllOrders
    assert_eq!(parsed.get_field(531).unwrap(), "7"); // MassCancelResponse::CancelAllOrders
    assert_eq!(parsed.get_field(533).unwrap(), "15"); // total_affected_orders
}

#[test]
fn test_order_mass_cancel_report_typed_conversion() {
    let fix_msg = load_fixture("order_mass_cancel_report", "fix");
    let parsed = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");

    // Convert to typed message
    let report = OrderMassCancelReport::from_raw(&parsed)
        .expect("Failed to convert to OrderMassCancelReport");

    // Verify typed fields
    assert_eq!(report.order_id, "MASS123");
    assert_eq!(report.cl_ord_id, Some("MASSCANCEL001".to_string()));
    assert_eq!(report.mass_cancel_request_type, MassCancelRequestType::CancelAllOrders);
    assert_eq!(report.mass_cancel_response, MassCancelResponse::CancelAllOrders);
    assert_eq!(report.total_affected_orders, Some(15));
    assert_eq!(report.text, Some("Cancelled 15 orders".to_string()));
}

#[test]
fn test_order_mass_cancel_report_round_trip() {
    let fix_msg = load_fixture("order_mass_cancel_report", "fix");

    // Parse to typed message
    let parsed1 = RawFixMessage::parse(&fix_msg).expect("Failed to parse FIX message");
    let report = OrderMassCancelReport::from_raw(&parsed1)
        .expect("Failed to convert to OrderMassCancelReport");

    // Convert back to raw
    let raw = report.to_raw();

    // Verify critical fields are preserved
    assert_eq!(raw.get_field(37), Some("MASS123"));
    assert_eq!(raw.get_field(11), Some("MASSCANCEL001"));
    assert_eq!(raw.get_field(530), Some("7"));
    assert_eq!(raw.get_field(531), Some("7"));
    assert_eq!(raw.get_field(533), Some("15"));
}
// ============================================================================
// Program Trading / List Trading Tests
// ============================================================================

#[test]
fn test_parse_new_order_list() {
    // NewOrderList with basic fields
    let msg = "8=FIXT.1.1|9=100|35=E|66=LIST123|394=2|68=3|415=30|10=000|";
    let parsed = RawFixMessage::parse(msg).expect("Failed to parse NewOrderList");

    assert_eq!(parsed.get_field(35), Some("E")); // MsgType
    assert_eq!(parsed.get_field(66), Some("LIST123")); // ListID
    assert_eq!(parsed.get_field(394), Some("2")); // BidType = Disclosed
    assert_eq!(parsed.get_field(68), Some("3")); // TotNoOrders
    assert_eq!(parsed.get_field(415), Some("30")); // ProgPeriodInterval
}

#[test]
fn test_new_order_list_typed_conversion() {
    let msg = "8=FIXT.1.1|9=100|35=E|66=LIST456|390=BID789|394=1|414=1|68=5|893=Y|10=000|";
    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");

    let typed = fixie::NewOrderList::from_raw(&parsed).expect("Failed to convert to NewOrderList");

    assert_eq!(typed.list_id, "LIST456");
    assert_eq!(typed.bid_id, Some("BID789".to_string()));
    assert_eq!(typed.bid_type, fixie::BidType::NonDisclosed);
    assert_eq!(typed.prog_rpt_reqs, Some(fixie::ProgRptReqs::BuySideRequests));
    assert_eq!(typed.tot_no_orders, 5);
    assert_eq!(typed.last_fragment, Some(true));
}

#[test]
fn test_new_order_list_round_trip() {
    let msg = "8=FIXT.1.1|9=100|35=E|66=LISTRT|394=3|68=10|433=1|10=000|";
    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");
    let typed = fixie::NewOrderList::from_raw(&parsed).expect("Failed to convert");
    let raw = typed.to_raw();

    // Verify round-trip preserves key fields
    assert_eq!(raw.get_field(66), Some("LISTRT"));
    assert_eq!(raw.get_field(394), Some("3")); // BidType::NoBiddingProcess
    assert_eq!(raw.get_field(68), Some("10"));
    assert_eq!(raw.get_field(433), Some("1")); // ListExecInstType::Immediate
}

#[test]
fn test_parse_list_status() {
    let msg = "8=FIXT.1.1|9=100|35=N|66=LIST123|429=1|82=3|431=2|83=1|68=5|10=000|";
    let parsed = RawFixMessage::parse(msg).expect("Failed to parse ListStatus");

    assert_eq!(parsed.get_field(35), Some("N")); // MsgType
    assert_eq!(parsed.get_field(66), Some("LIST123")); // ListID
    assert_eq!(parsed.get_field(429), Some("1")); // ListStatusType = Ack
    assert_eq!(parsed.get_field(82), Some("3")); // NoRpts
    assert_eq!(parsed.get_field(431), Some("2")); // ListOrderStatus
    assert_eq!(parsed.get_field(83), Some("1")); // RptSeq
    assert_eq!(parsed.get_field(68), Some("5")); // TotNoOrders
}

#[test]
fn test_list_status_typed_conversion() {
    let msg = "8=FIXT.1.1|9=100|35=N|66=STAT999|429=4|82=1|431=3|83=2|68=10|444=Processing|60=20250108-12:00:00|10=000|";
    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");

    let typed = fixie::ListStatus::from_raw(&parsed).expect("Failed to convert to ListStatus");

    assert_eq!(typed.list_id, "STAT999");
    assert_eq!(typed.list_status_type, fixie::ListStatusType::ExecStarted);
    assert_eq!(typed.no_rpts, 1);
    assert_eq!(typed.list_order_status, fixie::ListOrderStatus::Executing);
    assert_eq!(typed.rpt_seq, 2);
    assert_eq!(typed.tot_no_orders, 10);
    assert_eq!(typed.list_status_text, Some("Processing".to_string()));
    assert_eq!(typed.transact_time, Some("20250108-12:00:00".to_string()));
}

#[test]
fn test_list_status_round_trip() {
    let msg = "8=FIXT.1.1|9=100|35=N|66=RTLIST|429=5|82=2|431=6|83=3|68=20|893=N|10=000|";
    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");
    let typed = fixie::ListStatus::from_raw(&parsed).expect("Failed to convert");
    let raw = typed.to_raw();

    // Verify round-trip preserves key fields
    assert_eq!(raw.get_field(66), Some("RTLIST"));
    assert_eq!(raw.get_field(429), Some("5")); // ListStatusType::AllDone
    assert_eq!(raw.get_field(82), Some("2"));
    assert_eq!(raw.get_field(431), Some("6")); // ListOrderStatus::AllDone
    assert_eq!(raw.get_field(83), Some("3"));
    assert_eq!(raw.get_field(68), Some("20"));
    assert_eq!(raw.get_field(893), Some("N")); // LastFragment = false
}

#[test]
fn test_new_order_list_with_nested_parties() {
    // NewOrderList with ListOrdGrp containing nested Parties group
    let msg = "8=FIXT.1.1|9=200|35=E|66=LIST001|394=2|68=1|\
73=1|11=ORDER1|67=1|55=AAPL|54=1|38=100|\
453=1|448=BROKER1|447=D|452=1|10=000|";

    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");

    // Verify list-level fields
    assert_eq!(parsed.get_field(35), Some("E"));
    assert_eq!(parsed.get_field(66), Some("LIST001"));
    assert_eq!(parsed.get_field(68), Some("1")); // TotNoOrders

    // Verify ListOrdGrp exists
    let list_ord_grp = parsed.groups.get(&73).expect("Should have ListOrdGrp");
    assert_eq!(list_ord_grp.len(), 1);

    let order = &parsed.group_arena[list_ord_grp[0]];
    assert_eq!(order.fields.get(&11), Some(&"ORDER1".to_string())); // ClOrdID
    assert_eq!(order.fields.get(&55), Some(&"AAPL".to_string())); // Symbol
    assert_eq!(order.fields.get(&38), Some(&"100".to_string())); // OrderQty

    // Verify nested Parties group within the order
    let parties = order.nested_groups.get(&453).expect("Should have Parties");
    assert_eq!(parties.len(), 1);

    let party = &parsed.group_arena[parties[0]];
    assert_eq!(party.fields.get(&448), Some(&"BROKER1".to_string())); // PartyID
    assert_eq!(party.fields.get(&447), Some(&"D".to_string())); // PartyIDSource
    assert_eq!(party.fields.get(&452), Some(&"1".to_string())); // PartyRole
}

#[test]
fn test_new_order_list_with_doubly_nested_parties() {
    // NewOrderList → ListOrdGrp → Parties → PartySubIDsGrp (2 levels of nesting)
    let msg = "8=FIXT.1.1|9=250|35=E|66=LIST002|394=2|68=1|\
73=1|11=ORDER2|67=1|55=MSFT|54=2|38=200|\
453=1|448=TRADER1|447=D|452=1|802=2|523=SUB1|803=1|523=SUB2|803=2|10=000|";

    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");

    let list_ord_grp = parsed.groups.get(&73).expect("Should have ListOrdGrp");
    let order = &parsed.group_arena[list_ord_grp[0]];

    // Get Parties group
    let parties = order.nested_groups.get(&453).expect("Should have Parties");
    let party = &parsed.group_arena[parties[0]];
    assert_eq!(party.fields.get(&448), Some(&"TRADER1".to_string()));

    // Verify PartySubIDsGrp (nested within Parties)
    let party_sub_ids = party.nested_groups.get(&802).expect("Should have PartySubIDsGrp");
    assert_eq!(party_sub_ids.len(), 2);

    let sub1 = &parsed.group_arena[party_sub_ids[0]];
    assert_eq!(sub1.fields.get(&523), Some(&"SUB1".to_string())); // PartySubID
    assert_eq!(sub1.fields.get(&803), Some(&"1".to_string())); // PartySubIDType

    let sub2 = &parsed.group_arena[party_sub_ids[1]];
    assert_eq!(sub2.fields.get(&523), Some(&"SUB2".to_string()));
    assert_eq!(sub2.fields.get(&803), Some(&"2".to_string()));
}

#[test]
fn test_new_order_list_with_triple_nesting() {
    // NewOrderList → ListOrdGrp → PreAllocGrp → NestedParties2 (3 levels!)
    let msg = "8=FIXT.1.1|9=300|35=E|66=LIST003|394=2|68=1|\
73=1|11=ORDER3|67=1|55=GOOGL|54=1|38=50|\
78=1|79=ACCOUNT1|80=50|\
756=1|757=PARTY1|758=D|759=1|10=000|";

    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");

    let list_ord_grp = parsed.groups.get(&73).expect("Should have ListOrdGrp");
    let order = &parsed.group_arena[list_ord_grp[0]];

    // Level 1: PreAllocGrp within order
    let pre_alloc = order.nested_groups.get(&78).expect("Should have PreAllocGrp");
    assert_eq!(pre_alloc.len(), 1);

    let alloc = &parsed.group_arena[pre_alloc[0]];
    assert_eq!(alloc.fields.get(&79), Some(&"ACCOUNT1".to_string())); // AllocAccount
    assert_eq!(alloc.fields.get(&80), Some(&"50".to_string())); // AllocQty

    // Level 2: NestedParties2 within PreAllocGrp
    let nested_parties = alloc.nested_groups.get(&756).expect("Should have NestedParties2");
    assert_eq!(nested_parties.len(), 1);

    let party = &parsed.group_arena[nested_parties[0]];
    assert_eq!(party.fields.get(&757), Some(&"PARTY1".to_string())); // Nested2PartyID
    assert_eq!(party.fields.get(&758), Some(&"D".to_string())); // Nested2PartyIDSource
    assert_eq!(party.fields.get(&759), Some(&"1".to_string())); // Nested2PartyRole
}

#[test]
fn test_new_order_list_round_trip_with_groups() {
    // Full round-trip with nested groups
    let msg = "8=FIXT.1.1|9=200|35=E|66=RTLIST|394=2|68=2|\
73=2|11=ORD1|67=1|55=AAPL|38=100|11=ORD2|67=2|55=MSFT|38=200|10=000|";

    let parsed = RawFixMessage::parse(msg).expect("Failed to parse");
    let encoded = parsed.encode();
    let reparsed = RawFixMessage::parse(&encoded).expect("Failed to reparse");

    // Verify top-level fields preserved
    assert_eq!(reparsed.get_field(66), Some("RTLIST"));
    assert_eq!(reparsed.get_field(68), Some("2"));

    // Verify group structure preserved
    let groups = reparsed.groups.get(&73).expect("Should have ListOrdGrp");
    assert_eq!(groups.len(), 2);

    let order1 = &reparsed.group_arena[groups[0]];
    assert_eq!(order1.fields.get(&11), Some(&"ORD1".to_string()));
    assert_eq!(order1.fields.get(&55), Some(&"AAPL".to_string()));

    let order2 = &reparsed.group_arena[groups[1]];
    assert_eq!(order2.fields.get(&11), Some(&"ORD2".to_string()));
    assert_eq!(order2.fields.get(&55), Some(&"MSFT".to_string()));
}

#[test]
fn test_program_trading_enum_values() {
    // Verify all Program Trading enums work correctly
    use fixie::*;

    // BidType
    assert_eq!(BidType::NonDisclosed.to_fix(), '1');
    assert_eq!(BidType::from_fix('2'), Some(BidType::Disclosed));

    // ProgRptReqs
    assert_eq!(ProgRptReqs::RealTimeExecutions.to_fix(), '3');
    assert_eq!(ProgRptReqs::from_fix('1'), Some(ProgRptReqs::BuySideRequests));

    // ListExecInstType
    assert_eq!(ListExecInstType::Immediate.to_fix(), '1');
    assert_eq!(ListExecInstType::from_fix('5'), Some(ListExecInstType::BuyDrivenCash));

    // ListStatusType
    assert_eq!(ListStatusType::Ack.to_fix(), '1');
    assert_eq!(ListStatusType::from_fix('6'), Some(ListStatusType::Alert));

    // ListOrderStatus
    assert_eq!(ListOrderStatus::InBiddingProcess.to_fix(), '1');
    assert_eq!(ListOrderStatus::from_fix('6'), Some(ListOrderStatus::AllDone));

    // PriceType (uses &str for multi-digit values)
    assert_eq!(PriceType::FixedCabinetPrice.to_fix(), "10");
    assert_eq!(PriceType::from_fix("11"), Some(PriceType::VariableCabinetPrice));
}

#[test]
fn test_msg_type_program_trading() {
    use fixie::MsgType;

    // Verify all Program Trading message types
    assert_eq!(MsgType::from_fix("E"), Some(MsgType::NewOrderList));
    assert_eq!(MsgType::from_fix("N"), Some(MsgType::ListStatus));
    assert_eq!(MsgType::from_fix("L"), Some(MsgType::ListExecute));
    assert_eq!(MsgType::from_fix("K"), Some(MsgType::ListCancelRequest));
    assert_eq!(MsgType::from_fix("M"), Some(MsgType::ListStatusRequest));
    assert_eq!(MsgType::from_fix("k"), Some(MsgType::BidRequest));
    assert_eq!(MsgType::from_fix("l"), Some(MsgType::BidResponse));
    assert_eq!(MsgType::from_fix("m"), Some(MsgType::ListStrikePrice));

    // Verify to_fix() round-trip
    assert_eq!(MsgType::NewOrderList.to_fix(), "E");
    assert_eq!(MsgType::ListStatus.to_fix(), "N");
    assert_eq!(MsgType::BidRequest.to_fix(), "k");
}
