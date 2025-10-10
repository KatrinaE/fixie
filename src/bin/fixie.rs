use clap::Parser;
use fixie::{RawFixMessage, MsgType, SOH, GroupEntry};
use std::io::{self, Read};
use serde_json::json;

/// A FIX protocol message parser and pretty-printer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// FIX message to parse (if not provided, reads from stdin)
    message: Option<String>,

    /// Display raw tag-value pairs
    #[arg(short, long)]
    raw: bool,

    /// Display as JSON
    #[arg(short, long)]
    json: bool,
}

// FIX tag names for common tags
fn tag_name(tag: u32) -> &'static str {
    match tag {
        6 => "AvgPx",
        8 => "BeginString",
        9 => "BodyLength",
        10 => "CheckSum",
        11 => "ClOrdID",
        14 => "CumQty",
        17 => "ExecID",
        31 => "LastPx",
        32 => "LastQty",
        34 => "MsgSeqNum",
        35 => "MsgType",
        37 => "OrderID",
        38 => "OrderQty",
        39 => "OrdStatus",
        40 => "OrdType",
        41 => "OrigClOrdID",
        44 => "Price",
        49 => "SenderCompID",
        52 => "SendingTime",
        54 => "Side",
        55 => "Symbol",
        56 => "TargetCompID",
        58 => "Text",
        60 => "TransactTime",
        98 => "EncryptMethod",
        102 => "CxlRejReason",
        108 => "HeartBtInt",
        112 => "TestReqID",
        150 => "ExecType",
        151 => "LeavesQty",
        262 => "MDReqID",
        263 => "SubscriptionRequestType",
        264 => "MarketDepth",
        269 => "MDEntryType",
        270 => "MDEntryPx",
        271 => "MDEntrySize",
        273 => "MDEntryTime",
        1128 => "ApplVerID",
        // Program Trading tags
        66 => "ListID",
        67 => "ListSeqNo",
        68 => "TotNoOrders",
        73 => "NoOrders",
        78 => "NoAllocs",
        79 => "AllocAccount",
        80 => "AllocQty",
        82 => "NoRpts",
        83 => "RptSeq",
        390 => "BidID",
        391 => "ClientBidID",
        394 => "BidType",
        398 => "NoBidDescriptors",
        399 => "BidDescriptorType",
        400 => "BidDescriptor",
        401 => "SideValueInd",
        414 => "ProgRptReqs",
        415 => "ProgPeriodInterval",
        420 => "NoBidComponents",
        423 => "PriceType",
        429 => "ListStatusType",
        430 => "NetGrossInd",
        431 => "ListOrderStatus",
        433 => "ListExecInstType",
        444 => "ListStatusText",
        453 => "NoPartyIDs",
        448 => "PartyID",
        447 => "PartyIDSource",
        452 => "PartyRole",
        523 => "PartySubID",
        756 => "NoNested2PartyIDs",
        757 => "Nested2PartyID",
        760 => "Nested2PartySubID",
        802 => "NoPartySubIDs",
        803 => "PartySubIDType",
        806 => "NoNested2PartySubIDs",
        807 => "Nested2PartySubIDType",
        893 => "LastFragment",
        1201 => "NoStrikeRules",
        1223 => "StrikeRuleID",
        // Mass Order tags
        533 => "TotalAffectedOrders",
        584 => "MassStatusReqID",
        585 => "MassStatusReqType",
        1300 => "MarketSegmentID",
        1301 => "MarketID",
        1369 => "MassActionReportID",
        1373 => "MassActionType",
        1374 => "MassActionScope",
        1375 => "MassActionResponse",
        1376 => "MassActionRejectReason",
        1793 => "NoTargetMarketSegments",
        1794 => "NoNotAffMarketSegments",
        2427 => "OrderResponseLevel",
        2429 => "OrderEntryAction",
        2430 => "NoOrderEntries",
        2436 => "MassOrderID",
        // Multileg Order tags
        442 => "MultilegReportingType",
        539 => "NoNestedPartyIDs",
        524 => "NestedPartyID",
        525 => "NestedPartyIDSource",
        538 => "NestedPartyRole",
        545 => "NestedPartySubID",
        555 => "NoLegs",
        564 => "LegPositionEffect",
        566 => "LegPrice",
        587 => "LegSettlType",
        588 => "LegSettlDate",
        600 => "LegSymbol",
        602 => "LegSecurityID",
        603 => "LegSecurityIDSource",
        604 => "NoLegSecurityAltID",
        605 => "LegSecurityAltID",
        606 => "LegSecurityAltIDSource",
        607 => "LegProduct",
        608 => "LegCFICode",
        609 => "LegSecurityType",
        610 => "LegMaturityMonthYear",
        611 => "LegMaturityDate",
        612 => "LegStrikePrice",
        613 => "LegOptAttribute",
        614 => "LegContractMultiplier",
        615 => "LegCouponRate",
        616 => "LegSecurityExchange",
        617 => "LegIssuer",
        618 => "EncodedLegIssuerLen",
        619 => "EncodedLegIssuer",
        620 => "LegSecurityDesc",
        621 => "EncodedLegSecurityDescLen",
        622 => "EncodedLegSecurityDesc",
        623 => "LegRatioQty",
        624 => "LegSide",
        625 => "TradingSessionSubID",
        654 => "LegRefID",
        683 => "NoLegStipulations",
        688 => "LegStipulationType",
        689 => "LegStipulationValue",
        804 => "NoNestdPtysSubGrp",
        805 => "NestdPtysSubGrpID",
        1377 => "MultilegModel",
        1378 => "MultilegPriceMethod",
        // Infrastructure Message tags
        45 => "RefSeqNum",
        283 => "LocationID",
        284 => "DeskID",
        354 => "EncodedTextLen",
        355 => "EncodedText",
        372 => "RefMsgType",
        379 => "BusinessRejectRefID",
        380 => "BusinessRejectReason",
        928 => "StatusValue",
        929 => "StatusText",
        930 => "RefCompID",
        931 => "RefSubID",
        932 => "NetworkRespID",
        933 => "NetworkReqID",
        934 => "LastNetworkRespID",
        935 => "NetworkRequestType",
        936 => "NoCompIDs",
        937 => "NetworkStatusResponseType",
        1180 => "ApplID",
        1182 => "ApplBegSeqNum",
        1183 => "ApplEndSeqNum",
        1346 => "ApplReqID",
        1347 => "ApplReqType",
        1348 => "ApplResponseType",
        1353 => "ApplResponseID",
        1354 => "ApplResponseError",
        1355 => "RefApplID",
        1356 => "ApplReportID",
        1426 => "ApplReportType",
        1430 => "RefApplReqID",
        1351 => "NoApplIDs",
        553 => "Username",
        554 => "Password",
        923 => "UserRequestID",
        924 => "UserRequestType",
        925 => "NewPassword",
        926 => "UserStatus",
        927 => "UserStatusText",
        // Pre-Trade Indication tags
        2 => "AdvId",
        3 => "AdvRefID",
        4 => "AdvSide",
        5 => "AdvTransType",
        15 => "Currency",
        22 => "SecurityIDSource",
        23 => "IOIID",
        25 => "IOIQltyInd",
        26 => "IOIRefID",
        27 => "IOIQty",
        28 => "IOITransType",
        30 => "LastMkt",
        48 => "SecurityID",
        53 => "Quantity",
        62 => "ValidUntilTime",
        75 => "TradeDate",
        149 => "URLLink",
        336 => "TradingSessionID",
        376 => "ComplianceID",
        854 => "QtyType",
        2404 => "ComplianceText",
        2672 => "CrossRequestID",
        // Pre-Trade Event Communication tags
        42 => "OrigTime",
        61 => "Urgency",
        94 => "EmailType",
        95 => "RawDataLength",
        96 => "RawData",
        147 => "Subject",
        148 => "Headline",
        164 => "EmailThreadID",
        356 => "EmailMsgID",
        1472 => "NewsID",
        1473 => "NewsCategory",
        1474 => "LanguageCode",
        // Pre-Trade Quotation tags
        63 => "SettlType",
        64 => "SettlDate",
        117 => "QuoteID",
        126 => "ExpireTime",
        131 => "QuoteReqID",
        132 => "BidPx",
        133 => "OfferPx",
        134 => "BidSize",
        135 => "OfferSize",
        140 => "PrevClosePx",
        188 => "BidSpotRate",
        189 => "BidForwardPoints",
        190 => "OfferSpotRate",
        191 => "OfferForwardPoints",
        192 => "OrderQty2",
        193 => "SettlDate2",
        293 => "DefBidSize",
        294 => "DefOfferSize",
        297 => "QuoteStatus",
        298 => "QuoteCancelType",
        300 => "QuoteRejectReason",
        301 => "QuoteResponseLevel",
        537 => "QuoteType",
        581 => "AccountType",
        632 => "BidYield",
        633 => "OfferYield",
        644 => "RFQReqID",
        649 => "QuoteStatusReqID",
        658 => "QuoteRequestRejectReason",
        660 => "AcctIDSource",
        693 => "QuoteRespID",
        694 => "QuoteRespType",
        // Pre-Trade Market Structure tags
        207 => "SecurityExchange",
        325 => "UnsolicitedIndicator",
        335 => "TradSesReqID",
        338 => "TradSesMethod",
        339 => "TradSesMode",
        340 => "TradSesStatus",
        341 => "TradSesStartTime",
        342 => "TradSesOpenTime",
        343 => "TradSesPreCloseTime",
        344 => "TradSesCloseTime",
        345 => "TradSesEndTime",
        387 => "TotalVolumeTraded",
        567 => "TradSesStatusRejReason",
        1325 => "ParentMktSegmID",
        1327 => "TradSesUpdateAction",
        1368 => "TradSesEvent",
        1393 => "MarketReqID",
        1394 => "MarketReportID",
        1395 => "MarketUpdateAction",
        1396 => "MarketSegmentDesc",
        1397 => "EncodedMktSegmDescLen",
        1398 => "EncodedMktSegmDesc",
        // Pre-Trade Securities Reference tags
        107 => "SecurityDesc",
        167 => "SecurityType",
        200 => "MaturityMonthYear",
        201 => "PutOrCall",
        202 => "StrikePrice",
        305 => "UnderlyingSecurityIDSource",
        309 => "UnderlyingSecurityID",
        311 => "UnderlyingSymbol",
        320 => "SecurityReqID",
        321 => "SecurityRequestType",
        322 => "SecurityResponseID",
        324 => "SecurityStatusReqID",
        326 => "SecurityTradingStatus",
        327 => "HaltReason",
        393 => "TotalNumSecurities",
        460 => "Product",
        461 => "CFICode",
        541 => "MaturityDate",
        557 => "TotalNumSecurityTypes",
        559 => "SecurityListRequestType",
        560 => "SecurityRequestResult",
        715 => "ClearingBusinessDate",
        762 => "SecuritySubType",
        964 => "SecurityReportID",
        980 => "SecurityUpdateAction",
        1654 => "SecurityMassStatusReqID",
        _ => "Unknown",
    }
}

fn msg_type_name(msg_type: &str) -> &'static str {
    MsgType::from_fix(msg_type)
        .map(|mt| match mt {
            MsgType::Logon => "Logon",
            MsgType::Logout => "Logout",
            MsgType::Heartbeat => "Heartbeat",
            MsgType::TestRequest => "TestRequest",
            MsgType::ResendRequest => "ResendRequest",
            MsgType::Reject => "Reject",
            MsgType::NewOrderSingle => "NewOrderSingle",
            MsgType::OrderCancelRequest => "OrderCancelRequest",
            MsgType::OrderCancelReplaceRequest => "OrderCancelReplaceRequest",
            MsgType::OrderStatusRequest => "OrderStatusRequest",
            MsgType::DontKnowTrade => "DontKnowTrade",
            MsgType::OrderCancelReject => "OrderCancelReject",
            MsgType::ExecutionReport => "ExecutionReport",
            MsgType::ExecutionAcknowledgement => "ExecutionAcknowledgement",
            MsgType::OrderMassCancelRequest => "OrderMassCancelRequest",
            MsgType::OrderMassCancelReport => "OrderMassCancelReport",
            MsgType::NewOrderCross => "NewOrderCross",
            MsgType::CrossOrderCancelRequest => "CrossOrderCancelRequest",
            MsgType::CrossOrderCancelReplaceRequest => "CrossOrderCancelReplaceRequest",
            MsgType::MarketDataRequest => "MarketDataRequest",
            MsgType::MarketDataSnapshotFullRefresh => "MarketDataSnapshotFullRefresh",
            MsgType::MarketDataRequestReject => "MarketDataRequestReject",
            MsgType::NewOrderList => "NewOrderList",
            MsgType::ListStatus => "ListStatus",
            MsgType::ListExecute => "ListExecute",
            MsgType::ListCancelRequest => "ListCancelRequest",
            MsgType::ListStatusRequest => "ListStatusRequest",
            MsgType::BidRequest => "BidRequest",
            MsgType::BidResponse => "BidResponse",
            MsgType::ListStrikePrice => "ListStrikePrice",
            MsgType::MassOrder => "MassOrder",
            MsgType::MassOrderAck => "MassOrderAck",
            MsgType::OrderMassActionRequest => "OrderMassActionRequest",
            MsgType::OrderMassActionReport => "OrderMassActionReport",
            MsgType::OrderMassStatusRequest => "OrderMassStatusRequest",
            MsgType::NewOrderMultileg => "NewOrderMultileg",
            MsgType::MultilegOrderCancelReplace => "MultilegOrderCancelReplace",
            MsgType::BusinessMessageReject => "BusinessMessageReject",
            MsgType::NetworkCounterpartySystemStatusRequest => "NetworkCounterpartySystemStatusRequest",
            MsgType::NetworkCounterpartySystemStatusResponse => "NetworkCounterpartySystemStatusResponse",
            MsgType::ApplicationMessageRequest => "ApplicationMessageRequest",
            MsgType::ApplicationMessageRequestAck => "ApplicationMessageRequestAck",
            MsgType::ApplicationMessageReport => "ApplicationMessageReport",
            MsgType::UserRequest => "UserRequest",
            MsgType::UserResponse => "UserResponse",
            MsgType::UserNotification => "UserNotification",
            MsgType::Advertisement => "Advertisement",
            MsgType::CrossRequest => "CrossRequest",
            MsgType::CrossRequestAck => "CrossRequestAck",
            MsgType::IOI => "IOI",
            MsgType::Email => "Email",
            MsgType::News => "News",
            MsgType::MassQuote => "MassQuote",
            MsgType::MassQuoteAcknowledgement => "MassQuoteAcknowledgement",
            MsgType::Quote => "Quote",
            MsgType::QuoteAcknowledgment => "QuoteAcknowledgment",
            MsgType::QuoteCancel => "QuoteCancel",
            MsgType::QuoteRequest => "QuoteRequest",
            MsgType::QuoteRequestReject => "QuoteRequestReject",
            MsgType::QuoteResponse => "QuoteResponse",
            MsgType::QuoteStatusReport => "QuoteStatusReport",
            MsgType::QuoteStatusRequest => "QuoteStatusRequest",
            MsgType::RFQRequest => "RFQRequest",
            MsgType::MarketDefinition => "MarketDefinition",
            MsgType::MarketDefinitionRequest => "MarketDefinitionRequest",
            MsgType::MarketDefinitionUpdateReport => "MarketDefinitionUpdateReport",
            MsgType::TradingSessionList => "TradingSessionList",
            MsgType::TradingSessionListRequest => "TradingSessionListRequest",
            MsgType::TradingSessionListUpdateReport => "TradingSessionListUpdateReport",
            MsgType::TradingSessionStatus => "TradingSessionStatus",
            MsgType::TradingSessionStatusRequest => "TradingSessionStatusRequest",
            MsgType::DerivativeSecurityList => "DerivativeSecurityList",
            MsgType::DerivativeSecurityListRequest => "DerivativeSecurityListRequest",
            MsgType::SecurityDefinition => "SecurityDefinition",
            MsgType::SecurityDefinitionRequest => "SecurityDefinitionRequest",
            MsgType::SecurityDefinitionUpdateReport => "SecurityDefinitionUpdateReport",
            MsgType::SecurityList => "SecurityList",
            MsgType::SecurityListRequest => "SecurityListRequest",
            MsgType::SecurityListUpdateReport => "SecurityListUpdateReport",
            MsgType::SecurityMassStatus => "SecurityMassStatus",
            MsgType::SecurityMassStatusRequest => "SecurityMassStatusRequest",
            MsgType::SecurityStatus => "SecurityStatus",
            MsgType::SecurityStatusRequest => "SecurityStatusRequest",
            MsgType::SecurityTypeRequest => "SecurityTypeRequest",
            MsgType::SecurityTypes => "SecurityTypes",
        })
        .unwrap_or("Unknown")
}

fn format_value(tag: u32, value: &str) -> String {
    match tag {
        35 => format!("{} ({})", value, msg_type_name(value)),
        54 => match value {
            "1" => "1 (Buy)".to_string(),
            "2" => "2 (Sell)".to_string(),
            _ => value.to_string(),
        },
        40 => match value {
            "1" => "1 (Market)".to_string(),
            "2" => "2 (Limit)".to_string(),
            _ => value.to_string(),
        },
        39 => match value {
            "0" => "0 (New)".to_string(),
            "1" => "1 (PartiallyFilled)".to_string(),
            "2" => "2 (Filled)".to_string(),
            "4" => "4 (Cancelled)".to_string(),
            "8" => "8 (Rejected)".to_string(),
            _ => value.to_string(),
        },
        150 => match value {
            "0" => "0 (New)".to_string(),
            "1" => "1 (PartialFill)".to_string(),
            "2" => "2 (Fill)".to_string(),
            "4" => "4 (Cancelled)".to_string(),
            "8" => "8 (Rejected)".to_string(),
            _ => value.to_string(),
        },
        98 => match value {
            "0" => "0 (None)".to_string(),
            _ => value.to_string(),
        },
        1128 => match value {
            "9" => "9 (FIX.5.0SP2)".to_string(),
            _ => value.to_string(),
        },
        _ => value.to_string(),
    }
}

fn pretty_print(msg: &RawFixMessage) {
    // Print header
    if let Some(msg_type) = msg.get_field(35) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║  FIX Message: {}  ", msg_type_name(msg_type));
        println!("╚════════════════════════════════════════════════════════╝\n");
    }

    // Sort tags for consistent output
    let mut tags: Vec<&u32> = msg.fields.keys().collect();
    tags.sort();

    // Group tags by category
    let header_tags = [8, 9, 35, 49, 56, 34, 52, 1128];
    let trailer_tags = [10];

    println!("Standard Header:");
    println!("────────────────");
    for tag in &tags {
        if header_tags.contains(tag) {
            if let Some(value) = msg.get_field(**tag) {
                println!("  {:4} ({:20}): {}", tag, tag_name(**tag), format_value(**tag, value));
            }
        }
    }

    println!("\nMessage Body:");
    println!("─────────────");
    for tag in &tags {
        if !header_tags.contains(tag) && !trailer_tags.contains(tag) {
            if let Some(value) = msg.get_field(**tag) {
                println!("  {:4} ({:20}): {}", tag, tag_name(**tag), format_value(**tag, value));
            }
        }
    }

    println!("\nTrailer:");
    println!("────────");
    for tag in &tags {
        if trailer_tags.contains(tag) {
            if let Some(value) = msg.get_field(**tag) {
                println!("  {:4} ({:20}): {}", tag, tag_name(**tag), format_value(**tag, value));
            }
        }
    }
    println!();
}

fn json_output(msg: &RawFixMessage) {
    // Build JSON object with simple key-value pairs
    let mut output = serde_json::Map::new();

    // Add all regular fields
    for (tag, value) in &msg.fields {
        let field_name = tag_name(*tag);
        output.insert(field_name.to_string(), json!(value));
    }

    // Add repeating groups with their actual data
    for (group_tag, entry_ids) in &msg.groups {
        let group_name = tag_name(*group_tag);
        let mut group_entries = Vec::new();

        for entry_id in entry_ids {
            if let Some(entry) = msg.get_group_entry(*entry_id) {
                let entry_obj = build_group_entry_json(entry, msg);
                group_entries.push(entry_obj);
            }
        }

        output.insert(group_name.to_string(), json!(group_entries));
    }

    println!("{}", serde_json::to_string_pretty(&json!(output)).unwrap());
}

fn build_group_entry_json(entry: &GroupEntry, msg: &RawFixMessage) -> serde_json::Value {
    let mut entry_obj = serde_json::Map::new();

    // Add all fields from this group entry
    for (tag, value) in &entry.fields {
        let field_name = tag_name(*tag);
        entry_obj.insert(field_name.to_string(), json!(value));
    }

    // Add nested groups recursively
    for (nested_group_tag, nested_entry_ids) in &entry.nested_groups {
        let nested_group_name = tag_name(*nested_group_tag);
        let mut nested_entries = Vec::new();

        for nested_entry_id in nested_entry_ids {
            if let Some(nested_entry) = msg.get_group_entry(*nested_entry_id) {
                let nested_obj = build_group_entry_json(nested_entry, msg);
                nested_entries.push(nested_obj);
            }
        }

        entry_obj.insert(nested_group_name.to_string(), json!(nested_entries));
    }

    json!(entry_obj)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Get the FIX message from args or stdin
    let message = if let Some(msg) = args.message {
        msg
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string()
    };

    // Parse the message
    let parsed = RawFixMessage::parse(&message)?;

    if args.json {
        // Output as JSON
        json_output(&parsed);
    } else if args.raw {
        // Print raw tag-value pairs
        println!("Raw FIX Message:");
        let mut tags: Vec<&u32> = parsed.fields.keys().collect();
        tags.sort();
        for tag in tags {
            if let Some(value) = parsed.get_field(*tag) {
                println!("{}={}{}", tag, value, SOH);
            }
        }
    } else {
        // Pretty print
        pretty_print(&parsed);
    }

    Ok(())
}
