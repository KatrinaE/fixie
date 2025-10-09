use clap::Parser;
use fixie::{RawFixMessage, MsgType, SOH};
use std::io::{self, Read};

/// A FIX protocol message parser and pretty-printer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// FIX message to parse (if not provided, reads from stdin)
    message: Option<String>,

    /// Display raw tag-value pairs
    #[arg(short, long)]
    raw: bool,
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

    if args.raw {
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
