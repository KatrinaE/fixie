use fixie::market_data::*;
use fixie::types::*;

// ============================================================================
// MarketDataRequest Tests
// ============================================================================

#[test]
fn test_market_data_request_snapshot() {
    let req = MarketDataRequest {
        md_req_id: "SNAP-REQ-001".to_string(),
        subscription_request_type: SubscriptionRequestType::Snapshot,
        market_depth: 0, // Full book
        md_update_type: Some(MDUpdateType::FullRefresh),
        aggregated_book: Some(false),
        open_close_settle_flag: None,
        scope: Some("1".to_string()), // Local
        md_implicit_delete: None,
    };

    let raw = req.to_raw();
    assert_eq!(raw.get_field(35), Some("V"));
    assert_eq!(raw.get_field(262), Some("SNAP-REQ-001"));
    assert_eq!(raw.get_field(263), Some("0")); // Snapshot
    assert_eq!(raw.get_field(264), Some("0")); // Full book
    assert_eq!(raw.get_field(265), Some("0")); // FullRefresh
    assert_eq!(raw.get_field(266), Some("N")); // Not aggregated

    let parsed = MarketDataRequest::from_raw(&raw).expect("Failed to parse MarketDataRequest");
    assert_eq!(parsed.md_req_id, "SNAP-REQ-001");
    assert_eq!(parsed.subscription_request_type, SubscriptionRequestType::Snapshot);
    assert_eq!(parsed.market_depth, 0);
    assert_eq!(parsed.md_update_type, Some(MDUpdateType::FullRefresh));
    assert_eq!(parsed.aggregated_book, Some(false));
}

#[test]
fn test_market_data_request_subscribe() {
    let req = MarketDataRequest {
        md_req_id: "SUB-REQ-002".to_string(),
        subscription_request_type: SubscriptionRequestType::SnapshotPlusUpdates,
        market_depth: 1, // Top of book
        md_update_type: Some(MDUpdateType::IncrementalRefresh),
        aggregated_book: Some(true),
        open_close_settle_flag: Some("0".to_string()), // Daily open/close/settlement
        scope: Some("2".to_string()), // National
        md_implicit_delete: Some(true),
    };

    let raw = req.to_raw();
    assert_eq!(raw.get_field(263), Some("1")); // SnapshotPlusUpdates
    assert_eq!(raw.get_field(264), Some("1")); // Top of book
    assert_eq!(raw.get_field(265), Some("1")); // IncrementalRefresh
    assert_eq!(raw.get_field(266), Some("Y")); // Aggregated
    assert_eq!(raw.get_field(286), Some("0")); // Daily open/close/settlement
    assert_eq!(raw.get_field(547), Some("Y")); // Implicit delete

    let parsed = MarketDataRequest::from_raw(&raw).expect("Failed to parse MarketDataRequest");
    assert_eq!(parsed.subscription_request_type, SubscriptionRequestType::SnapshotPlusUpdates);
    assert_eq!(parsed.market_depth, 1);
    assert_eq!(parsed.md_update_type, Some(MDUpdateType::IncrementalRefresh));
    assert_eq!(parsed.aggregated_book, Some(true));
    assert_eq!(parsed.md_implicit_delete, Some(true));
}

#[test]
fn test_market_data_request_unsubscribe() {
    let req = MarketDataRequest {
        md_req_id: "UNSUB-REQ-003".to_string(),
        subscription_request_type: SubscriptionRequestType::DisablePreviousSnapshot,
        market_depth: 0,
        md_update_type: None,
        aggregated_book: None,
        open_close_settle_flag: None,
        scope: None,
        md_implicit_delete: None,
    };

    let raw = req.to_raw();
    assert_eq!(raw.get_field(263), Some("2")); // DisablePreviousSnapshot

    let parsed = MarketDataRequest::from_raw(&raw).expect("Failed to parse MarketDataRequest");
    assert_eq!(parsed.subscription_request_type, SubscriptionRequestType::DisablePreviousSnapshot);
}

#[test]
fn test_market_data_request_market_depth_levels() {
    // Test different market depth values
    let depths = vec![
        (0, "Full book"),
        (1, "Top of book"),
        (5, "5 levels deep"),
        (10, "10 levels deep"),
    ];

    for (depth, _desc) in depths {
        let req = MarketDataRequest {
            md_req_id: format!("DEPTH-{}", depth),
            subscription_request_type: SubscriptionRequestType::Snapshot,
            market_depth: depth,
            md_update_type: None,
            aggregated_book: None,
            open_close_settle_flag: None,
            scope: None,
            md_implicit_delete: None,
        };

        let raw = req.to_raw();
        let parsed = MarketDataRequest::from_raw(&raw).expect("Failed to parse MarketDataRequest");
        assert_eq!(parsed.market_depth, depth);
    }
}

#[test]
fn test_market_data_request_minimal() {
    let req = MarketDataRequest {
        md_req_id: "MIN-REQ".to_string(),
        subscription_request_type: SubscriptionRequestType::Snapshot,
        market_depth: 1,
        md_update_type: None,
        aggregated_book: None,
        open_close_settle_flag: None,
        scope: None,
        md_implicit_delete: None,
    };

    let raw = req.to_raw();
    let parsed = MarketDataRequest::from_raw(&raw).expect("Failed to parse MarketDataRequest");
    assert_eq!(parsed.md_req_id, "MIN-REQ");
    assert_eq!(parsed.md_update_type, None);
    assert_eq!(parsed.aggregated_book, None);
}

// ============================================================================
// MarketDataSnapshotFullRefresh Tests
// ============================================================================

#[test]
fn test_market_data_snapshot_full() {
    let snapshot = MarketDataSnapshotFullRefresh {
        md_req_id: Some("SNAP-REQ-001".to_string()),
        symbol: Some("AAPL".to_string()),
        security_id: Some("037833100".to_string()),
        security_id_source: Some("1".to_string()), // CUSIP
        security_type: Some("CS".to_string()), // Common Stock
        financial_status: Some("1".to_string()), // Bankrupt
        corporate_action: Some("A".to_string()), // Ex-Dividend
        net_chg_prev_day: Some(2.75),
        md_stream_id: Some("STREAM-NASDAQ-001".to_string()),
        market_id: Some("XNAS".to_string()),
        market_segment_id: Some("MAIN".to_string()),
        trading_session_id: Some("CORE".to_string()),
        trading_session_sub_id: Some("PRE".to_string()),
        clearing_business_date: Some("20250110".to_string()),
    };

    let raw = snapshot.to_raw();
    assert_eq!(raw.get_field(35), Some("W"));
    assert_eq!(raw.get_field(262), Some("SNAP-REQ-001"));
    assert_eq!(raw.get_field(55), Some("AAPL"));
    assert_eq!(raw.get_field(48), Some("037833100"));
    assert_eq!(raw.get_field(22), Some("1"));
    assert_eq!(raw.get_field(167), Some("CS"));
    assert_eq!(raw.get_field(291), Some("1"));
    assert_eq!(raw.get_field(292), Some("A"));
    assert_eq!(raw.get_field(451), Some("2.75"));
    assert_eq!(raw.get_field(1301), Some("XNAS"));
    assert_eq!(raw.get_field(1300), Some("MAIN"));

    let parsed = MarketDataSnapshotFullRefresh::from_raw(&raw)
        .expect("Failed to parse MarketDataSnapshotFullRefresh");
    assert_eq!(parsed.symbol, Some("AAPL".to_string()));
    assert_eq!(parsed.security_id, Some("037833100".to_string()));
    assert_eq!(parsed.net_chg_prev_day, Some(2.75));
    assert_eq!(parsed.market_id, Some("XNAS".to_string()));
    assert_eq!(parsed.clearing_business_date, Some("20250110".to_string()));
}

#[test]
fn test_market_data_snapshot_symbol_only() {
    let snapshot = MarketDataSnapshotFullRefresh {
        md_req_id: Some("SNAP-002".to_string()),
        symbol: Some("MSFT".to_string()),
        security_id: None,
        security_id_source: None,
        security_type: None,
        financial_status: None,
        corporate_action: None,
        net_chg_prev_day: None,
        md_stream_id: None,
        market_id: Some("XNAS".to_string()),
        market_segment_id: None,
        trading_session_id: None,
        trading_session_sub_id: None,
        clearing_business_date: None,
    };

    let raw = snapshot.to_raw();
    assert_eq!(raw.get_field(55), Some("MSFT"));
    assert_eq!(raw.get_field(48), None); // No security ID

    let parsed = MarketDataSnapshotFullRefresh::from_raw(&raw)
        .expect("Failed to parse MarketDataSnapshotFullRefresh");
    assert_eq!(parsed.symbol, Some("MSFT".to_string()));
    assert_eq!(parsed.security_id, None);
}

#[test]
fn test_market_data_snapshot_security_id_only() {
    let snapshot = MarketDataSnapshotFullRefresh {
        md_req_id: None,
        symbol: None,
        security_id: Some("594918104".to_string()), // MSFT CUSIP
        security_id_source: Some("1".to_string()),
        security_type: Some("CS".to_string()),
        financial_status: None,
        corporate_action: None,
        net_chg_prev_day: Some(-1.25),
        md_stream_id: Some("STREAM-002".to_string()),
        market_id: Some("XNYS".to_string()),
        market_segment_id: None,
        trading_session_id: None,
        trading_session_sub_id: None,
        clearing_business_date: Some("20250110".to_string()),
    };

    let raw = snapshot.to_raw();
    assert_eq!(raw.get_field(48), Some("594918104"));
    assert_eq!(raw.get_field(55), None); // No symbol

    let parsed = MarketDataSnapshotFullRefresh::from_raw(&raw)
        .expect("Failed to parse MarketDataSnapshotFullRefresh");
    assert_eq!(parsed.security_id, Some("594918104".to_string()));
    assert_eq!(parsed.symbol, None);
    assert_eq!(parsed.net_chg_prev_day, Some(-1.25));
}

#[test]
fn test_market_data_snapshot_unsolicited() {
    // Unsolicited snapshot (no MDReqID)
    let snapshot = MarketDataSnapshotFullRefresh {
        md_req_id: None,
        symbol: Some("GOOGL".to_string()),
        security_id: Some("02079K305".to_string()),
        security_id_source: Some("1".to_string()),
        security_type: Some("CS".to_string()),
        financial_status: None,
        corporate_action: None,
        net_chg_prev_day: Some(5.50),
        md_stream_id: Some("UNSOLICITED-STREAM".to_string()),
        market_id: Some("XNAS".to_string()),
        market_segment_id: Some("MAIN".to_string()),
        trading_session_id: Some("CORE".to_string()),
        trading_session_sub_id: None,
        clearing_business_date: Some("20250110".to_string()),
    };

    let raw = snapshot.to_raw();
    assert_eq!(raw.get_field(262), None); // No MDReqID for unsolicited

    let parsed = MarketDataSnapshotFullRefresh::from_raw(&raw)
        .expect("Failed to parse MarketDataSnapshotFullRefresh");
    assert_eq!(parsed.md_req_id, None);
    assert_eq!(parsed.symbol, Some("GOOGL".to_string()));
}

#[test]
fn test_market_data_snapshot_minimal() {
    let snapshot = MarketDataSnapshotFullRefresh {
        md_req_id: None,
        symbol: Some("TSLA".to_string()),
        security_id: None,
        security_id_source: None,
        security_type: None,
        financial_status: None,
        corporate_action: None,
        net_chg_prev_day: None,
        md_stream_id: None,
        market_id: None,
        market_segment_id: None,
        trading_session_id: None,
        trading_session_sub_id: None,
        clearing_business_date: None,
    };

    let raw = snapshot.to_raw();
    let parsed = MarketDataSnapshotFullRefresh::from_raw(&raw)
        .expect("Failed to parse MarketDataSnapshotFullRefresh");
    assert_eq!(parsed.symbol, Some("TSLA".to_string()));
    assert_eq!(parsed.net_chg_prev_day, None);
}

// ============================================================================
// MarketDataRequestReject Tests
// ============================================================================

#[test]
fn test_market_data_request_reject_unknown_symbol() {
    let reject = MarketDataRequestReject {
        md_req_id: "BAD-REQ-001".to_string(),
        md_req_rej_reason: Some(MDReqRejReason::UnknownSymbol),
        text: Some("Symbol XYZ123 not found in database".to_string()),
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    assert_eq!(raw.get_field(35), Some("Y"));
    assert_eq!(raw.get_field(262), Some("BAD-REQ-001"));
    assert_eq!(raw.get_field(281), Some("0")); // UnknownSymbol
    assert_eq!(raw.get_field(58), Some("Symbol XYZ123 not found in database"));

    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_id, "BAD-REQ-001");
    assert_eq!(parsed.md_req_rej_reason, Some(MDReqRejReason::UnknownSymbol));
    assert_eq!(parsed.text, Some("Symbol XYZ123 not found in database".to_string()));
}

#[test]
fn test_market_data_request_reject_duplicate_id() {
    let reject = MarketDataRequestReject {
        md_req_id: "DUP-REQ-002".to_string(),
        md_req_rej_reason: Some(MDReqRejReason::DuplicateMDReqID),
        text: Some("MDReqID already in use".to_string()),
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    assert_eq!(raw.get_field(281), Some("1")); // DuplicateMDReqID

    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_rej_reason, Some(MDReqRejReason::DuplicateMDReqID));
}

#[test]
fn test_market_data_request_reject_insufficient_bandwidth() {
    let reject = MarketDataRequestReject {
        md_req_id: "BW-REQ-003".to_string(),
        md_req_rej_reason: Some(MDReqRejReason::InsufficientBandwidth),
        text: Some("Too many active subscriptions".to_string()),
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    assert_eq!(raw.get_field(281), Some("2")); // InsufficientBandwidth

    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_rej_reason, Some(MDReqRejReason::InsufficientBandwidth));
}

#[test]
fn test_market_data_request_reject_insufficient_permissions() {
    let reject = MarketDataRequestReject {
        md_req_id: "PERM-REQ-004".to_string(),
        md_req_rej_reason: Some(MDReqRejReason::InsufficientPermissions),
        text: Some("User does not have access to Level 2 data".to_string()),
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    assert_eq!(raw.get_field(281), Some("3")); // InsufficientPermissions

    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_rej_reason, Some(MDReqRejReason::InsufficientPermissions));
}

#[test]
fn test_market_data_request_reject_unsupported_subscription_type() {
    let reject = MarketDataRequestReject {
        md_req_id: "UNSUP-SUB-005".to_string(),
        md_req_rej_reason: Some(MDReqRejReason::UnsupportedSubscriptionRequestType),
        text: Some("Streaming subscriptions not supported".to_string()),
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    assert_eq!(raw.get_field(281), Some("4")); // UnsupportedSubscriptionRequestType

    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_rej_reason, Some(MDReqRejReason::UnsupportedSubscriptionRequestType));
}

#[test]
fn test_market_data_request_reject_unsupported_market_depth() {
    let reject = MarketDataRequestReject {
        md_req_id: "DEPTH-REQ-006".to_string(),
        md_req_rej_reason: Some(MDReqRejReason::UnsupportedMarketDepth),
        text: Some("Only top of book (depth=1) is supported".to_string()),
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    assert_eq!(raw.get_field(281), Some("5")); // UnsupportedMarketDepth

    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_rej_reason, Some(MDReqRejReason::UnsupportedMarketDepth));
}

#[test]
fn test_market_data_request_reject_unsupported_entry_type() {
    let reject = MarketDataRequestReject {
        md_req_id: "ENTRY-REQ-007".to_string(),
        md_req_rej_reason: Some(MDReqRejReason::UnsupportedMDEntryType),
        text: Some("Requested MD entry type not available for this instrument".to_string()),
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    assert_eq!(raw.get_field(281), Some("8")); // UnsupportedMDEntryType

    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_rej_reason, Some(MDReqRejReason::UnsupportedMDEntryType));
}

#[test]
fn test_market_data_request_reject_no_reason() {
    let reject = MarketDataRequestReject {
        md_req_id: "NOREASON-REQ-008".to_string(),
        md_req_rej_reason: None,
        text: Some("Request rejected for unspecified reason".to_string()),
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    assert_eq!(raw.get_field(281), None); // No reason code

    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_rej_reason, None);
    assert_eq!(parsed.text, Some("Request rejected for unspecified reason".to_string()));
}

#[test]
fn test_market_data_request_reject_minimal() {
    let reject = MarketDataRequestReject {
        md_req_id: "MIN-REJ".to_string(),
        md_req_rej_reason: None,
        text: None,
        encoded_text_len: None,
        encoded_text: None,
    };

    let raw = reject.to_raw();
    let parsed = MarketDataRequestReject::from_raw(&raw)
        .expect("Failed to parse MarketDataRequestReject");
    assert_eq!(parsed.md_req_id, "MIN-REJ");
    assert_eq!(parsed.md_req_rej_reason, None);
    assert_eq!(parsed.text, None);
}

// ============================================================================
// Enum Tests
// ============================================================================

#[test]
fn test_subscription_request_type_enum_values() {
    assert_eq!(SubscriptionRequestType::Snapshot.to_fix(), '0');
    assert_eq!(SubscriptionRequestType::SnapshotPlusUpdates.to_fix(), '1');
    assert_eq!(SubscriptionRequestType::DisablePreviousSnapshot.to_fix(), '2');

    assert_eq!(SubscriptionRequestType::from_fix('0'), Some(SubscriptionRequestType::Snapshot));
    assert_eq!(SubscriptionRequestType::from_fix('1'), Some(SubscriptionRequestType::SnapshotPlusUpdates));
    assert_eq!(SubscriptionRequestType::from_fix('2'), Some(SubscriptionRequestType::DisablePreviousSnapshot));
    assert_eq!(SubscriptionRequestType::from_fix('9'), None);
}

#[test]
fn test_md_update_type_enum_values() {
    assert_eq!(MDUpdateType::FullRefresh.to_fix(), '0');
    assert_eq!(MDUpdateType::IncrementalRefresh.to_fix(), '1');

    assert_eq!(MDUpdateType::from_fix('0'), Some(MDUpdateType::FullRefresh));
    assert_eq!(MDUpdateType::from_fix('1'), Some(MDUpdateType::IncrementalRefresh));
    assert_eq!(MDUpdateType::from_fix('9'), None);
}

#[test]
fn test_md_req_rej_reason_enum_values() {
    assert_eq!(MDReqRejReason::UnknownSymbol.to_fix(), "0");
    assert_eq!(MDReqRejReason::DuplicateMDReqID.to_fix(), "1");
    assert_eq!(MDReqRejReason::InsufficientBandwidth.to_fix(), "2");
    assert_eq!(MDReqRejReason::InsufficientPermissions.to_fix(), "3");
    assert_eq!(MDReqRejReason::UnsupportedSubscriptionRequestType.to_fix(), "4");
    assert_eq!(MDReqRejReason::UnsupportedMarketDepth.to_fix(), "5");
    assert_eq!(MDReqRejReason::UnsupportedMDUpdateType.to_fix(), "6");
    assert_eq!(MDReqRejReason::UnsupportedAggregatedBook.to_fix(), "7");
    assert_eq!(MDReqRejReason::UnsupportedMDEntryType.to_fix(), "8");
    assert_eq!(MDReqRejReason::UnsupportedTradingSessionID.to_fix(), "9");
    assert_eq!(MDReqRejReason::UnsupportedScope.to_fix(), "A");
    assert_eq!(MDReqRejReason::UnsupportedOpenCloseSettleFlag.to_fix(), "B");
    assert_eq!(MDReqRejReason::UnsupportedMDImplicitDelete.to_fix(), "C");

    assert_eq!(MDReqRejReason::from_fix("0"), Some(MDReqRejReason::UnknownSymbol));
    assert_eq!(MDReqRejReason::from_fix("5"), Some(MDReqRejReason::UnsupportedMarketDepth));
    assert_eq!(MDReqRejReason::from_fix("A"), Some(MDReqRejReason::UnsupportedScope));
    assert_eq!(MDReqRejReason::from_fix("C"), Some(MDReqRejReason::UnsupportedMDImplicitDelete));
    assert_eq!(MDReqRejReason::from_fix("Z"), None);
}

#[test]
fn test_md_entry_type_enum_values() {
    assert_eq!(MDEntryType::Bid.to_fix(), "0");
    assert_eq!(MDEntryType::Offer.to_fix(), "1");
    assert_eq!(MDEntryType::Trade.to_fix(), "2");
    assert_eq!(MDEntryType::IndexValue.to_fix(), "3");
    assert_eq!(MDEntryType::OpeningPrice.to_fix(), "4");
    assert_eq!(MDEntryType::ClosingPrice.to_fix(), "5");
    assert_eq!(MDEntryType::SettlementPrice.to_fix(), "6");
    assert_eq!(MDEntryType::TradingSessionHighPrice.to_fix(), "7");
    assert_eq!(MDEntryType::TradingSessionLowPrice.to_fix(), "8");
    assert_eq!(MDEntryType::TradingSessionVWAPPrice.to_fix(), "9");
    assert_eq!(MDEntryType::Imbalance.to_fix(), "A");
    assert_eq!(MDEntryType::TradeVolume.to_fix(), "B");
    assert_eq!(MDEntryType::OpenInterest.to_fix(), "C");

    assert_eq!(MDEntryType::from_fix("0"), Some(MDEntryType::Bid));
    assert_eq!(MDEntryType::from_fix("1"), Some(MDEntryType::Offer));
    assert_eq!(MDEntryType::from_fix("2"), Some(MDEntryType::Trade));
    assert_eq!(MDEntryType::from_fix("A"), Some(MDEntryType::Imbalance));
    assert_eq!(MDEntryType::from_fix("C"), Some(MDEntryType::OpenInterest));
    assert_eq!(MDEntryType::from_fix("Z"), None);
}
