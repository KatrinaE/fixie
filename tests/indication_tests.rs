use fixie::*;

// ============================================================================
// IOI Tests
// ============================================================================

#[test]
fn test_ioi_new_buy_round_trip() {
    let ioi = IOI {
        ioi_id: "IOI-20250110-001".to_string(),
        ioi_trans_type: IOITransType::New,
        side: Side::Buy,
        ioi_qty: "L".to_string(), // Large
        symbol: Some("AAPL".to_string()),
        security_id: Some("037833100".to_string()),
        security_id_source: Some("1".to_string()), // CUSIP
        ioi_ref_id: None,
        currency: Some("USD".to_string()),
        price: Some(175.50),
        valid_until_time: Some("20250110-16:00:00".to_string()),
        ioi_qlty_ind: Some(IOIQltyInd::High),
        text: Some("High quality buy interest".to_string()),
        transact_time: Some("20250110-09:30:00".to_string()),
        url_link: Some("https://example.com/ioi/001".to_string()),
    };

    let raw = ioi.to_raw();
    assert_eq!(raw.get_field(35), Some("6"));
    assert_eq!(raw.get_field(23), Some("IOI-20250110-001"));
    assert_eq!(raw.get_field(28), Some("N"));
    assert_eq!(raw.get_field(54), Some("1"));
    assert_eq!(raw.get_field(27), Some("L"));
    assert_eq!(raw.get_field(55), Some("AAPL"));
    assert_eq!(raw.get_field(25), Some("H"));

    let parsed = IOI::from_raw(&raw).expect("Failed to parse IOI");
    assert_eq!(parsed.ioi_id, "IOI-20250110-001");
    assert_eq!(parsed.ioi_trans_type, IOITransType::New);
    assert_eq!(parsed.side, Side::Buy);
    assert_eq!(parsed.ioi_qty, "L");
    assert_eq!(parsed.symbol, Some("AAPL".to_string()));
    assert_eq!(parsed.price, Some(175.50));
    assert_eq!(parsed.ioi_qlty_ind, Some(IOIQltyInd::High));
}

#[test]
fn test_ioi_sell_medium_quality() {
    let ioi = IOI {
        ioi_id: "IOI-SELL-002".to_string(),
        ioi_trans_type: IOITransType::New,
        side: Side::Sell,
        ioi_qty: "M".to_string(), // Medium
        symbol: Some("MSFT".to_string()),
        security_id: None,
        security_id_source: None,
        ioi_ref_id: None,
        currency: Some("USD".to_string()),
        price: Some(420.00),
        valid_until_time: Some("20250110-17:00:00".to_string()),
        ioi_qlty_ind: Some(IOIQltyInd::Medium),
        text: Some("Medium size block available".to_string()),
        transact_time: Some("20250110-10:15:00".to_string()),
        url_link: None,
    };

    let raw = ioi.to_raw();
    assert_eq!(raw.get_field(54), Some("2")); // Sell
    assert_eq!(raw.get_field(25), Some("M")); // Medium quality

    let parsed = IOI::from_raw(&raw).expect("Failed to parse IOI");
    assert_eq!(parsed.side, Side::Sell);
    assert_eq!(parsed.ioi_qlty_ind, Some(IOIQltyInd::Medium));
}

#[test]
fn test_ioi_replace() {
    let ioi = IOI {
        ioi_id: "IOI-REPLACE-003".to_string(),
        ioi_trans_type: IOITransType::Replace,
        side: Side::Buy,
        ioi_qty: "1000".to_string(), // Specific quantity
        symbol: Some("GOOGL".to_string()),
        security_id: Some("02079K305".to_string()),
        security_id_source: Some("1".to_string()),
        ioi_ref_id: Some("IOI-ORIGINAL-003".to_string()),
        currency: Some("USD".to_string()),
        price: Some(140.25),
        valid_until_time: None,
        ioi_qlty_ind: Some(IOIQltyInd::Low),
        text: None,
        transact_time: Some("20250110-11:00:00".to_string()),
        url_link: None,
    };

    let raw = ioi.to_raw();
    assert_eq!(raw.get_field(28), Some("R")); // Replace
    assert_eq!(raw.get_field(26), Some("IOI-ORIGINAL-003")); // IOIRefID

    let parsed = IOI::from_raw(&raw).expect("Failed to parse IOI");
    assert_eq!(parsed.ioi_trans_type, IOITransType::Replace);
    assert_eq!(parsed.ioi_ref_id, Some("IOI-ORIGINAL-003".to_string()));
}

#[test]
fn test_ioi_cancel() {
    let ioi = IOI {
        ioi_id: "IOI-CANCEL-004".to_string(),
        ioi_trans_type: IOITransType::Cancel,
        side: Side::Buy,
        ioi_qty: "L".to_string(),
        symbol: Some("TSLA".to_string()),
        security_id: None,
        security_id_source: None,
        ioi_ref_id: Some("IOI-ORIGINAL-004".to_string()),
        currency: None,
        price: None,
        valid_until_time: None,
        ioi_qlty_ind: None,
        text: Some("Cancelling previous IOI".to_string()),
        transact_time: Some("20250110-12:00:00".to_string()),
        url_link: None,
    };

    let raw = ioi.to_raw();
    assert_eq!(raw.get_field(28), Some("C")); // Cancel

    let parsed = IOI::from_raw(&raw).expect("Failed to parse IOI");
    assert_eq!(parsed.ioi_trans_type, IOITransType::Cancel);
}

#[test]
fn test_ioi_minimal() {
    let ioi = IOI {
        ioi_id: "IOI-MIN".to_string(),
        ioi_trans_type: IOITransType::New,
        side: Side::Buy,
        ioi_qty: "S".to_string(), // Small
        symbol: None,
        security_id: None,
        security_id_source: None,
        ioi_ref_id: None,
        currency: None,
        price: None,
        valid_until_time: None,
        ioi_qlty_ind: None,
        text: None,
        transact_time: None,
        url_link: None,
    };

    let raw = ioi.to_raw();
    let parsed = IOI::from_raw(&raw).expect("Failed to parse IOI");
    assert_eq!(parsed.ioi_id, "IOI-MIN");
    assert_eq!(parsed.symbol, None);
    assert_eq!(parsed.price, None);
}

// ============================================================================
// Advertisement Tests
// ============================================================================

#[test]
fn test_advertisement_new_sell_round_trip() {
    let adv = Advertisement {
        adv_id: "ADV-20250110-001".to_string(),
        adv_trans_type: AdvTransType::New,
        adv_side: AdvSide::Sell,
        quantity: 5000.0,
        symbol: Some("IBM".to_string()),
        security_id: Some("459200101".to_string()),
        security_id_source: Some("1".to_string()),
        adv_ref_id: None,
        qty_type: Some(QtyType::Units),
        price: Some(185.75),
        currency: Some("USD".to_string()),
        trade_date: Some("20250110".to_string()),
        transact_time: Some("20250110-14:30:00".to_string()),
        text: Some("Block trade completed on exchange".to_string()),
        url_link: Some("https://example.com/trades/001".to_string()),
        last_mkt: Some("NYSE".to_string()),
        trading_session_id: Some("CORE".to_string()),
        trading_session_sub_id: None,
    };

    let raw = adv.to_raw();
    assert_eq!(raw.get_field(35), Some("7"));
    assert_eq!(raw.get_field(2), Some("ADV-20250110-001"));
    assert_eq!(raw.get_field(5), Some("N"));
    assert_eq!(raw.get_field(4), Some("S"));
    assert_eq!(raw.get_field(53), Some("5000"));
    assert_eq!(raw.get_field(854), Some("0")); // Units

    let parsed = Advertisement::from_raw(&raw).expect("Failed to parse Advertisement");
    assert_eq!(parsed.adv_id, "ADV-20250110-001");
    assert_eq!(parsed.adv_trans_type, AdvTransType::New);
    assert_eq!(parsed.adv_side, AdvSide::Sell);
    assert_eq!(parsed.quantity, 5000.0);
    assert_eq!(parsed.qty_type, Some(QtyType::Units));
    assert_eq!(parsed.last_mkt, Some("NYSE".to_string()));
}

#[test]
fn test_advertisement_buy_side() {
    let adv = Advertisement {
        adv_id: "ADV-BUY-002".to_string(),
        adv_trans_type: AdvTransType::New,
        adv_side: AdvSide::Buy,
        quantity: 2500.0,
        symbol: Some("AMZN".to_string()),
        security_id: None,
        security_id_source: None,
        adv_ref_id: None,
        qty_type: Some(QtyType::Contracts),
        price: Some(175.25),
        currency: Some("USD".to_string()),
        trade_date: Some("20250110".to_string()),
        transact_time: Some("20250110-15:00:00".to_string()),
        text: None,
        url_link: None,
        last_mkt: Some("NASDAQ".to_string()),
        trading_session_id: None,
        trading_session_sub_id: None,
    };

    let raw = adv.to_raw();
    assert_eq!(raw.get_field(4), Some("B")); // Buy
    assert_eq!(raw.get_field(854), Some("1")); // Contracts

    let parsed = Advertisement::from_raw(&raw).expect("Failed to parse Advertisement");
    assert_eq!(parsed.adv_side, AdvSide::Buy);
    assert_eq!(parsed.qty_type, Some(QtyType::Contracts));
}

#[test]
fn test_advertisement_cross_side() {
    let adv = Advertisement {
        adv_id: "ADV-CROSS-003".to_string(),
        adv_trans_type: AdvTransType::New,
        adv_side: AdvSide::Cross,
        quantity: 10000.0,
        symbol: Some("META".to_string()),
        security_id: None,
        security_id_source: None,
        adv_ref_id: None,
        qty_type: Some(QtyType::Units),
        price: Some(450.00),
        currency: Some("USD".to_string()),
        trade_date: Some("20250110".to_string()),
        transact_time: Some("20250110-16:00:00".to_string()),
        text: Some("Internal cross executed".to_string()),
        url_link: None,
        last_mkt: Some("NASDAQ".to_string()),
        trading_session_id: None,
        trading_session_sub_id: None,
    };

    let raw = adv.to_raw();
    assert_eq!(raw.get_field(4), Some("X")); // Cross

    let parsed = Advertisement::from_raw(&raw).expect("Failed to parse Advertisement");
    assert_eq!(parsed.adv_side, AdvSide::Cross);
}

#[test]
fn test_advertisement_trade_side() {
    let adv = Advertisement {
        adv_id: "ADV-TRADE-004".to_string(),
        adv_trans_type: AdvTransType::New,
        adv_side: AdvSide::Trade,
        quantity: 1000.0,
        symbol: Some("NFLX".to_string()),
        security_id: None,
        security_id_source: None,
        adv_ref_id: None,
        qty_type: None,
        price: Some(550.50),
        currency: Some("USD".to_string()),
        trade_date: Some("20250110".to_string()),
        transact_time: Some("20250110-13:00:00".to_string()),
        text: None,
        url_link: None,
        last_mkt: None,
        trading_session_id: None,
        trading_session_sub_id: None,
    };

    let raw = adv.to_raw();
    assert_eq!(raw.get_field(4), Some("T")); // Trade

    let parsed = Advertisement::from_raw(&raw).expect("Failed to parse Advertisement");
    assert_eq!(parsed.adv_side, AdvSide::Trade);
}

#[test]
fn test_advertisement_replace() {
    let adv = Advertisement {
        adv_id: "ADV-REPLACE-005".to_string(),
        adv_trans_type: AdvTransType::Replace,
        adv_side: AdvSide::Sell,
        quantity: 3000.0,
        symbol: Some("NVDA".to_string()),
        security_id: None,
        security_id_source: None,
        adv_ref_id: Some("ADV-ORIGINAL-005".to_string()),
        qty_type: Some(QtyType::Units),
        price: Some(880.00),
        currency: Some("USD".to_string()),
        trade_date: Some("20250110".to_string()),
        transact_time: Some("20250110-14:00:00".to_string()),
        text: Some("Correcting previous advertisement".to_string()),
        url_link: None,
        last_mkt: Some("NASDAQ".to_string()),
        trading_session_id: None,
        trading_session_sub_id: None,
    };

    let raw = adv.to_raw();
    assert_eq!(raw.get_field(5), Some("R")); // Replace
    assert_eq!(raw.get_field(3), Some("ADV-ORIGINAL-005"));

    let parsed = Advertisement::from_raw(&raw).expect("Failed to parse Advertisement");
    assert_eq!(parsed.adv_trans_type, AdvTransType::Replace);
    assert_eq!(parsed.adv_ref_id, Some("ADV-ORIGINAL-005".to_string()));
}

#[test]
fn test_advertisement_cancel() {
    let adv = Advertisement {
        adv_id: "ADV-CANCEL-006".to_string(),
        adv_trans_type: AdvTransType::Cancel,
        adv_side: AdvSide::Sell,
        quantity: 2000.0,
        symbol: Some("AMD".to_string()),
        security_id: None,
        security_id_source: None,
        adv_ref_id: Some("ADV-ORIGINAL-006".to_string()),
        qty_type: None,
        price: None,
        currency: None,
        trade_date: None,
        transact_time: Some("20250110-15:30:00".to_string()),
        text: Some("Cancelling erroneous advertisement".to_string()),
        url_link: None,
        last_mkt: None,
        trading_session_id: None,
        trading_session_sub_id: None,
    };

    let raw = adv.to_raw();
    assert_eq!(raw.get_field(5), Some("C")); // Cancel

    let parsed = Advertisement::from_raw(&raw).expect("Failed to parse Advertisement");
    assert_eq!(parsed.adv_trans_type, AdvTransType::Cancel);
}

// ============================================================================
// CrossRequest Tests
// ============================================================================

#[test]
fn test_cross_request_round_trip() {
    let cross_req = CrossRequest {
        cross_request_id: "CROSS-REQ-001".to_string(),
        symbol: Some("JPM".to_string()),
        security_id: Some("46625H100".to_string()),
        security_id_source: Some("1".to_string()),
        market_id: Some("XNYS".to_string()),
        market_segment_id: Some("EQUITY".to_string()),
        order_qty: Some(1500.0),
        compliance_id: Some("COMP-12345".to_string()),
        compliance_text: Some("Pre-cross notification for internal cross".to_string()),
    };

    let raw = cross_req.to_raw();
    assert_eq!(raw.get_field(35), Some("DS"));
    assert_eq!(raw.get_field(2672), Some("CROSS-REQ-001"));
    assert_eq!(raw.get_field(55), Some("JPM"));
    assert_eq!(raw.get_field(1301), Some("XNYS"));
    assert_eq!(raw.get_field(38), Some("1500"));
    assert_eq!(raw.get_field(376), Some("COMP-12345"));

    let parsed = CrossRequest::from_raw(&raw).expect("Failed to parse CrossRequest");
    assert_eq!(parsed.cross_request_id, "CROSS-REQ-001");
    assert_eq!(parsed.symbol, Some("JPM".to_string()));
    assert_eq!(parsed.market_id, Some("XNYS".to_string()));
    assert_eq!(parsed.order_qty, Some(1500.0));
    assert_eq!(parsed.compliance_id, Some("COMP-12345".to_string()));
}

#[test]
fn test_cross_request_minimal() {
    let cross_req = CrossRequest {
        cross_request_id: "CROSS-MIN-002".to_string(),
        symbol: None,
        security_id: None,
        security_id_source: None,
        market_id: None,
        market_segment_id: None,
        order_qty: None,
        compliance_id: None,
        compliance_text: None,
    };

    let raw = cross_req.to_raw();
    let parsed = CrossRequest::from_raw(&raw).expect("Failed to parse CrossRequest");
    assert_eq!(parsed.cross_request_id, "CROSS-MIN-002");
    assert_eq!(parsed.symbol, None);
    assert_eq!(parsed.order_qty, None);
}

#[test]
fn test_cross_request_with_market_segment() {
    let cross_req = CrossRequest {
        cross_request_id: "CROSS-SEG-003".to_string(),
        symbol: Some("BAC".to_string()),
        security_id: Some("060505104".to_string()),
        security_id_source: Some("1".to_string()),
        market_id: Some("XNYS".to_string()),
        market_segment_id: Some("MAIN".to_string()),
        order_qty: Some(5000.0),
        compliance_id: Some("COMP-67890".to_string()),
        compliance_text: Some("Large block cross notification".to_string()),
    };

    let raw = cross_req.to_raw();
    assert_eq!(raw.get_field(1300), Some("MAIN"));

    let parsed = CrossRequest::from_raw(&raw).expect("Failed to parse CrossRequest");
    assert_eq!(parsed.market_segment_id, Some("MAIN".to_string()));
}

// ============================================================================
// CrossRequestAck Tests
// ============================================================================

#[test]
fn test_cross_request_ack_round_trip() {
    let ack = CrossRequestAck {
        cross_request_id: "CROSS-REQ-001".to_string(),
        symbol: Some("JPM".to_string()),
        security_id: Some("46625H100".to_string()),
        security_id_source: Some("1".to_string()),
        market_id: Some("XNYS".to_string()),
        market_segment_id: Some("EQUITY".to_string()),
    };

    let raw = ack.to_raw();
    assert_eq!(raw.get_field(35), Some("DT"));
    assert_eq!(raw.get_field(2672), Some("CROSS-REQ-001"));
    assert_eq!(raw.get_field(55), Some("JPM"));
    assert_eq!(raw.get_field(1301), Some("XNYS"));

    let parsed = CrossRequestAck::from_raw(&raw).expect("Failed to parse CrossRequestAck");
    assert_eq!(parsed.cross_request_id, "CROSS-REQ-001");
    assert_eq!(parsed.symbol, Some("JPM".to_string()));
    assert_eq!(parsed.market_id, Some("XNYS".to_string()));
}

#[test]
fn test_cross_request_ack_minimal() {
    let ack = CrossRequestAck {
        cross_request_id: "CROSS-ACK-MIN".to_string(),
        symbol: None,
        security_id: None,
        security_id_source: None,
        market_id: None,
        market_segment_id: None,
    };

    let raw = ack.to_raw();
    let parsed = CrossRequestAck::from_raw(&raw).expect("Failed to parse CrossRequestAck");
    assert_eq!(parsed.cross_request_id, "CROSS-ACK-MIN");
    assert_eq!(parsed.symbol, None);
}

// ============================================================================
// Enum Tests
// ============================================================================

#[test]
fn test_ioi_trans_type_enum_values() {
    assert_eq!(IOITransType::New.to_fix(), 'N');
    assert_eq!(IOITransType::Cancel.to_fix(), 'C');
    assert_eq!(IOITransType::Replace.to_fix(), 'R');

    assert_eq!(IOITransType::from_fix('N'), Some(IOITransType::New));
    assert_eq!(IOITransType::from_fix('C'), Some(IOITransType::Cancel));
    assert_eq!(IOITransType::from_fix('R'), Some(IOITransType::Replace));
    assert_eq!(IOITransType::from_fix('X'), None);
}

#[test]
fn test_ioi_qlty_ind_enum_values() {
    assert_eq!(IOIQltyInd::Low.to_fix(), 'L');
    assert_eq!(IOIQltyInd::Medium.to_fix(), 'M');
    assert_eq!(IOIQltyInd::High.to_fix(), 'H');

    assert_eq!(IOIQltyInd::from_fix('L'), Some(IOIQltyInd::Low));
    assert_eq!(IOIQltyInd::from_fix('M'), Some(IOIQltyInd::Medium));
    assert_eq!(IOIQltyInd::from_fix('H'), Some(IOIQltyInd::High));
    assert_eq!(IOIQltyInd::from_fix('X'), None);
}

#[test]
fn test_adv_side_enum_values() {
    assert_eq!(AdvSide::Buy.to_fix(), 'B');
    assert_eq!(AdvSide::Sell.to_fix(), 'S');
    assert_eq!(AdvSide::Cross.to_fix(), 'X');
    assert_eq!(AdvSide::Trade.to_fix(), 'T');

    assert_eq!(AdvSide::from_fix('B'), Some(AdvSide::Buy));
    assert_eq!(AdvSide::from_fix('S'), Some(AdvSide::Sell));
    assert_eq!(AdvSide::from_fix('X'), Some(AdvSide::Cross));
    assert_eq!(AdvSide::from_fix('T'), Some(AdvSide::Trade));
    assert_eq!(AdvSide::from_fix('Z'), None);
}

#[test]
fn test_adv_trans_type_enum_values() {
    assert_eq!(AdvTransType::New.to_fix(), 'N');
    assert_eq!(AdvTransType::Cancel.to_fix(), 'C');
    assert_eq!(AdvTransType::Replace.to_fix(), 'R');

    assert_eq!(AdvTransType::from_fix('N'), Some(AdvTransType::New));
    assert_eq!(AdvTransType::from_fix('C'), Some(AdvTransType::Cancel));
    assert_eq!(AdvTransType::from_fix('R'), Some(AdvTransType::Replace));
    assert_eq!(AdvTransType::from_fix('X'), None);
}

#[test]
fn test_qty_type_enum_values() {
    assert_eq!(QtyType::Units.to_fix(), '0');
    assert_eq!(QtyType::Contracts.to_fix(), '1');

    assert_eq!(QtyType::from_fix('0'), Some(QtyType::Units));
    assert_eq!(QtyType::from_fix('1'), Some(QtyType::Contracts));
    assert_eq!(QtyType::from_fix('9'), None);
}
