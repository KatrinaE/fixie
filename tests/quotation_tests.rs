use fixie::*;

#[test]
fn test_quote_round_trip() {
    let quote = Quote {
        quote_id: "Q12345".to_string(),
        quote_req_id: Some("QR001".to_string()),
        quote_type: Some(QuoteType::Indicative),
        quote_response_level: Some(QuoteResponseLevel::AcknowledgeOnlyNegativeOrErroneous),
        symbol: "AAPL".to_string(),
        security_id: Some("037833100".to_string()),
        security_id_source: Some("1".to_string()),
        side: Some(Side::Buy),
        order_qty: Some(1000.0),
        bid_px: Some(150.25),
        offer_px: Some(150.30),
        bid_size: Some(1000.0),
        offer_size: Some(1000.0),
        valid_until_time: Some("20231231-17:00:00".to_string()),
        bid_spot_rate: None,
        offer_spot_rate: None,
        bid_forward_points: None,
        offer_forward_points: None,
        transact_time: None,
        settl_type: None,
        settl_date: None,
        ord_type: None,
        settl_date2: None,
        order_qty2: None,
        currency: None,
        quote_status: Some(QuoteStatus::Accepted),
        account: None,
        acct_id_source: None,
        account_type: None,
        bid_yield: None,
        offer_yield: None,
        text: None,
    };

    let raw = quote.to_raw();
    assert_eq!(raw.get_field(35), Some("S"));
    assert_eq!(raw.get_field(117), Some("Q12345"));
    assert_eq!(raw.get_field(131), Some("QR001"));
    assert_eq!(raw.get_field(55), Some("AAPL"));
    assert_eq!(raw.get_field(297), Some("0"));

    let parsed = Quote::from_raw(&raw).expect("Failed to parse Quote");
    assert_eq!(parsed.quote_id, "Q12345");
    assert_eq!(parsed.quote_req_id, Some("QR001".to_string()));
    assert_eq!(parsed.quote_type, Some(QuoteType::Indicative));
    assert_eq!(parsed.symbol, "AAPL");
    assert_eq!(parsed.quote_status, Some(QuoteStatus::Accepted));
}

#[test]
fn test_quote_request_round_trip() {
    let quote_request = QuoteRequest {
        quote_req_id: "QR12345".to_string(),
        rfq_req_id: Some("RFQ001".to_string()),
        cl_ord_id: None,
        order_qty: Some(5000.0),
        side: Some(Side::Buy),
        transact_time: Some("20231215-10:30:00".to_string()),
        symbol: Some("MSFT".to_string()),
        security_id: Some("594918104".to_string()),
        security_id_source: Some("1".to_string()),
        settl_type: None,
        settl_date: Some("20231220".to_string()),
        settl_date2: None,
        order_qty2: None,
        currency: None,
        account: None,
        acct_id_source: None,
        account_type: None,
        quote_type: Some(QuoteType::Tradeable),
        ord_type: None,
        valid_until_time: None,
        expire_time: None,
        prev_close_px: None,
        text: None,
    };

    let raw = quote_request.to_raw();
    assert_eq!(raw.get_field(35), Some("R"));
    assert_eq!(raw.get_field(131), Some("QR12345"));
    assert_eq!(raw.get_field(55), Some("MSFT"));
    assert_eq!(raw.get_field(54), Some("1"));

    let parsed = QuoteRequest::from_raw(&raw).expect("Failed to parse QuoteRequest");
    assert_eq!(parsed.quote_req_id, "QR12345");
    assert_eq!(parsed.symbol, Some("MSFT".to_string()));
    assert_eq!(parsed.side, Some(Side::Buy));
    assert_eq!(parsed.order_qty, Some(5000.0));
}

#[test]
fn test_quote_cancel_round_trip() {
    let quote_cancel = QuoteCancel {
        quote_req_id: Some("QR12345".to_string()),
        quote_id: "Q12345".to_string(),
        quote_cancel_type: QuoteCancelType::CancelAllQuotes,
        quote_response_level: Some(QuoteResponseLevel::AcknowledgeEachQuote),
        account: None,
        acct_id_source: None,
        account_type: None,
        trading_session_id: None,
        trading_session_sub_id: None,
        symbol: None,
        security_id: None,
        security_id_source: None,
        text: None,
    };

    let raw = quote_cancel.to_raw();
    assert_eq!(raw.get_field(35), Some("Z"));
    assert_eq!(raw.get_field(298), Some("4"));
    assert_eq!(raw.get_field(117), Some("Q12345"));

    let parsed = QuoteCancel::from_raw(&raw).expect("Failed to parse QuoteCancel");
    assert_eq!(parsed.quote_cancel_type, QuoteCancelType::CancelAllQuotes);
    assert_eq!(parsed.quote_id, "Q12345");
}

#[test]
fn test_quote_status_request_round_trip() {
    let quote_status_request = QuoteStatusRequest {
        quote_status_req_id: Some("QSR001".to_string()),
        quote_id: Some("Q12345".to_string()),
        symbol: Some("IBM".to_string()),
        security_id: Some("459200101".to_string()),
        security_id_source: Some("1".to_string()),
        account: None,
        acct_id_source: None,
        trading_session_id: None,
        trading_session_sub_id: None,
        subscription_request_type: None,
    };

    let raw = quote_status_request.to_raw();
    assert_eq!(raw.get_field(35), Some("a"));
    assert_eq!(raw.get_field(55), Some("IBM"));
    assert_eq!(raw.get_field(117), Some("Q12345"));

    let parsed = QuoteStatusRequest::from_raw(&raw).expect("Failed to parse QuoteStatusRequest");
    assert_eq!(parsed.symbol, Some("IBM".to_string()));
    assert_eq!(parsed.quote_id, Some("Q12345".to_string()));
}

#[test]
fn test_quote_response_round_trip() {
    let quote_response = QuoteResponse {
        quote_resp_id: "QRESP001".to_string(),
        quote_req_id: Some("QR12345".to_string()),
        quote_id: None,
        quote_resp_type: Some(1),
        cl_ord_id: None,
        order_qty: None,
        side: None,
        symbol: None,
        security_id: None,
        security_id_source: None,
        account: None,
        acct_id_source: None,
        account_type: None,
        trading_session_id: None,
        trading_session_sub_id: None,
        text: Some("Quote accepted".to_string()),
    };

    let raw = quote_response.to_raw();
    assert_eq!(raw.get_field(35), Some("AJ"));
    assert_eq!(raw.get_field(693), Some("QRESP001"));
    assert_eq!(raw.get_field(694), Some("1"));

    let parsed = QuoteResponse::from_raw(&raw).expect("Failed to parse QuoteResponse");
    assert_eq!(parsed.quote_resp_id, "QRESP001");
    assert_eq!(parsed.quote_resp_type, Some(1));
    assert_eq!(parsed.text, Some("Quote accepted".to_string()));
}

#[test]
fn test_quote_request_reject_round_trip() {
    let quote_request_reject = QuoteRequestReject {
        quote_req_id: "QR12345".to_string(),
        rfq_req_id: Some("RFQ001".to_string()),
        quote_request_reject_reason: Some(QuoteRequestRejectReason::UnknownSymbol),
        text: Some("Symbol not found".to_string()),
    };

    let raw = quote_request_reject.to_raw();
    assert_eq!(raw.get_field(35), Some("AG"));
    assert_eq!(raw.get_field(131), Some("QR12345"));
    assert_eq!(raw.get_field(658), Some("1"));

    let parsed = QuoteRequestReject::from_raw(&raw).expect("Failed to parse QuoteRequestReject");
    assert_eq!(parsed.quote_req_id, "QR12345");
    assert_eq!(parsed.quote_request_reject_reason, Some(QuoteRequestRejectReason::UnknownSymbol));
    assert_eq!(parsed.text, Some("Symbol not found".to_string()));
}

#[test]
fn test_rfq_request_round_trip() {
    let rfq_request = RFQRequest {
        rfq_req_id: "RFQ12345".to_string(),
        subscription_request_type: None,
    };

    let raw = rfq_request.to_raw();
    assert_eq!(raw.get_field(35), Some("AH"));
    assert_eq!(raw.get_field(644), Some("RFQ12345"));

    let parsed = RFQRequest::from_raw(&raw).expect("Failed to parse RFQRequest");
    assert_eq!(parsed.rfq_req_id, "RFQ12345");
}

#[test]
fn test_quote_acknowledgment_round_trip() {
    let quote_ack = QuoteAcknowledgment {
        quote_ack_status: QuoteStatus::Accepted,
        quote_req_id: Some("QR12345".to_string()),
        quote_id: Some("Q12345".to_string()),
        quote_type: None,
        quote_cancel_type: None,
        account: None,
        acct_id_source: None,
        account_type: None,
        text: Some("Quote acknowledged".to_string()),
    };

    let raw = quote_ack.to_raw();
    assert_eq!(raw.get_field(35), Some("CW"));
    assert_eq!(raw.get_field(297), Some("0"));

    let parsed = QuoteAcknowledgment::from_raw(&raw).expect("Failed to parse QuoteAcknowledgment");
    assert_eq!(parsed.quote_ack_status, QuoteStatus::Accepted);
    assert_eq!(parsed.text, Some("Quote acknowledged".to_string()));
}

#[test]
fn test_quote_status_report_round_trip() {
    let quote_status_report = QuoteStatusReport {
        quote_status_req_id: Some("QSR001".to_string()),
        quote_req_id: Some("QR12345".to_string()),
        quote_id: Some("Q12345".to_string()),
        quote_type: Some(QuoteType::Tradeable),
        quote_status: Some(QuoteStatus::Pending),
        symbol: Some("TSLA".to_string()),
        security_id: Some("88160R101".to_string()),
        security_id_source: Some("1".to_string()),
        side: Some(Side::Buy),
        order_qty: Some(500.0),
        bid_px: Some(240.50),
        offer_px: Some(240.75),
        bid_size: None,
        offer_size: None,
        account: None,
        acct_id_source: None,
        account_type: None,
        trading_session_id: None,
        trading_session_sub_id: None,
        text: None,
    };

    let raw = quote_status_report.to_raw();
    assert_eq!(raw.get_field(35), Some("AI"));
    assert_eq!(raw.get_field(55), Some("TSLA"));
    assert_eq!(raw.get_field(297), Some("10"));

    let parsed = QuoteStatusReport::from_raw(&raw).expect("Failed to parse QuoteStatusReport");
    assert_eq!(parsed.symbol, Some("TSLA".to_string()));
    assert_eq!(parsed.quote_status, Some(QuoteStatus::Pending));
    assert_eq!(parsed.bid_px, Some(240.50));
}

#[test]
fn test_mass_quote_round_trip() {
    let mass_quote = MassQuote {
        quote_id: Some("MQ12345".to_string()),
        quote_req_id: Some("MQR001".to_string()),
        quote_type: Some(QuoteType::Tradeable),
        quote_response_level: Some(QuoteResponseLevel::AcknowledgeEachQuote),
        account: None,
        acct_id_source: None,
        account_type: None,
        def_bid_size: None,
        def_offer_size: None,
    };

    let raw = mass_quote.to_raw();
    assert_eq!(raw.get_field(35), Some("i"));
    assert_eq!(raw.get_field(117), Some("MQ12345"));
    assert_eq!(raw.get_field(537), Some("1"));

    let parsed = MassQuote::from_raw(&raw).expect("Failed to parse MassQuote");
    assert_eq!(parsed.quote_id, Some("MQ12345".to_string()));
    assert_eq!(parsed.quote_type, Some(QuoteType::Tradeable));
}

#[test]
fn test_mass_quote_acknowledgement_round_trip() {
    let mass_quote_ack = MassQuoteAcknowledgement {
        quote_id: Some("MQ12345".to_string()),
        quote_req_id: Some("MQR001".to_string()),
        quote_status: Some(QuoteStatus::Accepted),
        quote_reject_reason: None,
        quote_response_level: Some(QuoteResponseLevel::AcknowledgeOnlyNegativeOrErroneous),
        quote_type: Some(QuoteType::Tradeable),
        account: None,
        acct_id_source: None,
        account_type: None,
        text: None,
    };

    let raw = mass_quote_ack.to_raw();
    assert_eq!(raw.get_field(35), Some("b"));
    assert_eq!(raw.get_field(297), Some("0"));

    let parsed = MassQuoteAcknowledgement::from_raw(&raw).expect("Failed to parse MassQuoteAcknowledgement");
    assert_eq!(parsed.quote_status, Some(QuoteStatus::Accepted));
    assert_eq!(parsed.quote_type, Some(QuoteType::Tradeable));
}

#[test]
fn test_quote_with_minimal_fields() {
    let quote = Quote {
        quote_id: "Q99999".to_string(),
        quote_req_id: None,
        quote_type: None,
        quote_response_level: None,
        symbol: "TEST".to_string(),
        security_id: None,
        security_id_source: None,
        side: None,
        order_qty: None,
        bid_px: None,
        offer_px: None,
        bid_size: None,
        offer_size: None,
        valid_until_time: None,
        bid_spot_rate: None,
        offer_spot_rate: None,
        bid_forward_points: None,
        offer_forward_points: None,
        transact_time: None,
        settl_type: None,
        settl_date: None,
        ord_type: None,
        settl_date2: None,
        order_qty2: None,
        currency: None,
        quote_status: None,
        account: None,
        acct_id_source: None,
        account_type: None,
        bid_yield: None,
        offer_yield: None,
        text: None,
    };

    let raw = quote.to_raw();
    let parsed = Quote::from_raw(&raw).expect("Failed to parse minimal Quote");
    assert_eq!(parsed.quote_id, "Q99999");
    assert_eq!(parsed.symbol, "TEST");
    assert_eq!(parsed.quote_req_id, None);
    assert_eq!(parsed.bid_px, None);
}

#[test]
fn test_quote_status_multi_character_values() {
    // Test multi-character QuoteStatus values like "10", "11", "12", etc.
    let quote = Quote {
        quote_id: "Q001".to_string(),
        quote_req_id: None,
        quote_type: None,
        quote_response_level: None,
        symbol: "TEST".to_string(),
        security_id: None,
        security_id_source: None,
        side: None,
        order_qty: None,
        bid_px: None,
        offer_px: None,
        bid_size: None,
        offer_size: None,
        valid_until_time: None,
        bid_spot_rate: None,
        offer_spot_rate: None,
        bid_forward_points: None,
        offer_forward_points: None,
        transact_time: None,
        settl_type: None,
        settl_date: None,
        ord_type: None,
        settl_date2: None,
        order_qty2: None,
        currency: None,
        quote_status: Some(QuoteStatus::Pass),
        account: None,
        acct_id_source: None,
        account_type: None,
        bid_yield: None,
        offer_yield: None,
        text: None,
    };

    let raw = quote.to_raw();
    assert_eq!(raw.get_field(297), Some("11"));

    let parsed = Quote::from_raw(&raw).expect("Failed to parse Quote");
    assert_eq!(parsed.quote_status, Some(QuoteStatus::Pass));
}

#[test]
fn test_quote_condition_enum_values() {
    // QuoteCondition is stored in repeating groups, but we can test the enum conversions
    assert_eq!(QuoteCondition::OpenActive.to_fix(), 'A');
    assert_eq!(QuoteCondition::ClosedInactive.to_fix(), 'B');
    assert_eq!(QuoteCondition::ExchangeBest.to_fix(), 'C');
    assert_eq!(QuoteCondition::ConsolidatedBest.to_fix(), 'D');

    assert_eq!(QuoteCondition::from_fix('A'), Some(QuoteCondition::OpenActive));
    assert_eq!(QuoteCondition::from_fix('B'), Some(QuoteCondition::ClosedInactive));
}

#[test]
fn test_quote_cancel_type_enum_values() {
    // Test all QuoteCancelType variants
    assert_eq!(QuoteCancelType::CancelForOneOrMoreSecurities.to_fix(), '1');
    assert_eq!(QuoteCancelType::CancelForSecurityTypes.to_fix(), '2');
    assert_eq!(QuoteCancelType::CancelForUnderlyingSecurity.to_fix(), '3');
    assert_eq!(QuoteCancelType::CancelAllQuotes.to_fix(), '4');
    assert_eq!(QuoteCancelType::CancelQuoteSpecifiedInQuoteID.to_fix(), '5');
    assert_eq!(QuoteCancelType::CancelByQuoteType.to_fix(), '6');
    assert_eq!(QuoteCancelType::CancelForSecurityIssuer.to_fix(), '7');
    assert_eq!(QuoteCancelType::CancelForIssuerOfUnderlyingSecurity.to_fix(), '8');
}
