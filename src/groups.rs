use ahash::AHashMap as HashMap;
use std::sync::LazyLock;

/// Information about a nested repeating group within a parent group
#[derive(Debug, Clone)]
pub struct NestedGroupInfo {
    /// The NoXXX tag that indicates the number of nested group entries
    pub num_in_group_tag: u32,
    /// Optional: specific parent field tag that contains this nested group
    /// None means it can appear anywhere in the parent group
    pub parent_tag: Option<u32>,
}

/// Configuration for a repeating group
#[derive(Debug, Clone)]
pub struct GroupConfig {
    /// The NoXXX tag that indicates the number of group entries
    pub num_in_group_tag: u32,
    /// The first tag of the repeating group (acts as delimiter)
    pub delimiter_tag: u32,
    /// All tags that belong to this repeating group (in order)
    pub member_tags: Vec<u32>,
    /// Nested repeating groups that can appear within this group
    pub nested_groups: Vec<NestedGroupInfo>,
}

/// Key for context-specific group lookup
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupKey {
    pub num_in_group_tag: u32,
    pub msg_type: Option<String>, // None = default/generic, Some = message-specific
}

/// Registry of all known repeating groups in FIX 5.0 SP2
/// Maps (NoXXX tag, optional msg_type) -> GroupConfig
pub static GROUP_REGISTRY: LazyLock<HashMap<GroupKey, GroupConfig>> = LazyLock::new(|| {
    let mut registry = HashMap::new();

    // Parties component (used in many messages)
    registry.insert(
        GroupKey { num_in_group_tag: 453, msg_type: None },
        GroupConfig {
            num_in_group_tag: 453, // NoPartyIDs
            delimiter_tag: 448,    // PartyID
            member_tags: vec![
                448, // PartyID
                447, // PartyIDSource
                452, // PartyRole
                // Note: PartySubIDsGrp (802) is nested within this group
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 802, // NoPartySubIDs
                    parent_tag: None, // Can appear anywhere in Parties
                },
            ],
        },
    );

    // PartySubIDsGrp (nested within Parties)
    registry.insert(
        GroupKey { num_in_group_tag: 802, msg_type: None },
        GroupConfig {
            num_in_group_tag: 802, // NoPartySubIDs
            delimiter_tag: 523,    // PartySubID
            member_tags: vec![
                523, // PartySubID
                803, // PartySubIDType
            ],
            nested_groups: vec![], // No further nesting
        },
    );

    // SideCrossOrdModGrp (used in NewOrderCross 's', CrossOrderCancelReplaceRequest 't')
    // Based on FIX Trading Community Trade Appendix specification
    registry.insert(
        GroupKey { num_in_group_tag: 552, msg_type: Some("s".to_string()) },
        GroupConfig {
            num_in_group_tag: 552,
            delimiter_tag: 54, // Side (required)
            member_tags: vec![
                54,   // Side (required)
                2102, // ShortMarkingExemptIndicator (optional)
                41,   // OrigClOrdID (optional)
                11,   // ClOrdID (required)
                526,  // SecondaryClOrdID (optional)
                583,  // ClOrdLinkID (optional)
                586,  // OrigOrdModTime (optional)
                // Parties component (optional, not fully implemented)
                1690, // SideShortSaleExemptionReason (optional)
                229,  // TradeOriginationDate (optional)
                75,   // TradeDate (optional)
                1,    // Account (optional)
                660,  // AcctIDSource (optional)
                581,  // AccountType (optional)
                589,  // DayBookingInst (optional)
                590,  // BookingUnit (optional)
                591,  // PreallocMethod (optional)
                70,   // AllocID (optional)
                854,  // QtyType (optional)
                // OrderQtyData component (required)
                38,   // OrderQty
                152,  // CashOrderQty (optional)
                516,  // OrderPercent (optional)
                468,  // RoundingDirection (optional)
                469,  // RoundingModulus (optional)
                // CommissionData component (optional)
                12,   // Commission (optional)
                13,   // CommType (optional)
                528,  // OrderCapacity (optional)
                529,  // OrderRestrictions (optional)
                1724, // OrderOrigination (optional)
                1725, // OriginatingDeptID (optional)
                1726, // ReceivingDeptID (optional)
                1091, // PreTradeAnonymity (optional)
                582,  // CustOrderCapacity (optional)
                121,  // ForexReq (optional)
                120,  // SettlCurrency (optional)
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 453, // Parties (NoPartyIDs)
                    parent_tag: None,
                },
            ],
        },
    );

    // Same for CrossOrderCancelReplaceRequest 't'
    registry.insert(
        GroupKey { num_in_group_tag: 552, msg_type: Some("t".to_string()) },
        GroupConfig {
            num_in_group_tag: 552,
            delimiter_tag: 54,
            member_tags: vec![
                54, 2102, 41, 11, 526, 583, 586, 1690, 229, 75, 1, 660, 581, 589,
                590, 591, 70, 854, 38, 152, 516, 468, 469, 12, 13, 528, 529,
                1724, 1725, 1726, 1091, 582, 121, 120,
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 453, // Parties (NoPartyIDs)
                    parent_tag: None,
                },
            ],
        },
    );

    // SideCrossOrdCxlGrp (used in CrossOrderCancelRequest 'u')
    // Based on FIX Trading Community Trade Appendix specification
    registry.insert(
        GroupKey { num_in_group_tag: 552, msg_type: Some("u".to_string()) },
        GroupConfig {
            num_in_group_tag: 552,
            delimiter_tag: 54, // Side (required)
            member_tags: vec![
                54,   // Side (required)
                41,   // OrigClOrdID (optional, required for electronically submitted orders)
                11,   // ClOrdID (required)
                526,  // SecondaryClOrdID (optional)
                583,  // ClOrdLinkID (optional)
                586,  // OrigOrdModTime (optional)
                // Parties component (optional, not fully implemented)
                376,  // ComplianceID (optional)
                2404, // ComplianceText (optional)
                2351, // EncodedComplianceTextLen (optional)
                2352, // EncodedComplianceText (optional)
                229,  // TradeOriginationDate (optional)
                75,   // TradeDate (optional)
                58,   // Text (optional)
                354,  // EncodedTextLen (optional)
                355,  // EncodedText (optional)
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 453, // Parties (NoPartyIDs)
                    parent_tag: None,
                },
            ],
        },
    );

    // ========================================================================
    // Program Trading / List Trading Repeating Groups
    // ========================================================================

    // ListOrdGrp (Tag 73 = NoOrders)
    // Used in: NewOrderList (E), ListStatus (N)
    registry.insert(
        GroupKey { num_in_group_tag: 73, msg_type: Some("E".to_string()) },
        GroupConfig {
            num_in_group_tag: 73, // NoOrders
            delimiter_tag: 11,    // ClOrdID
            member_tags: vec![
                11,   // ClOrdID - Required
                67,   // ListSeqNo - Required
                160,  // SettlInstMode
                // Instrument component
                55,   // Symbol
                65,   // SymbolSfx
                48,   // SecurityID
                22,   // SecurityIDSource
                167,  // SecurityType
                200,  // MaturityMonthYear
                541,  // MaturityDate
                224,  // CouponPaymentDate
                225,  // IssueDate
                239,  // RepoCollateralSecurityType
                226,  // RepurchaseTerm
                227,  // RepurchaseRate
                228,  // Factor
                255,  // CreditRating
                543,  // InstrRegistry
                470,  // CountryOfIssue
                471,  // StateOrProvinceOfIssue
                472,  // LocaleOfIssue
                240,  // RedemptionDate
                202,  // StrikePrice
                947,  // StrikeCurrency
                206,  // OptAttribute
                231,  // ContractMultiplier
                223,  // CouponRate
                207,  // SecurityExchange
                106,  // Issuer
                107,  // SecurityDesc
                691,  // Pool
                // OrderQtyData component
                38,   // OrderQty
                152,  // CashOrderQty
                516,  // OrderPercent
                468,  // RoundingDirection
                469,  // RoundingModulus
                54,   // Side
                40,   // OrdType
                44,   // Price
                99,   // StopPx
                15,   // Currency
                376,  // ComplianceID
                377,  // SolicitedFlag
                23,   // IOIid
                117,  // QuoteID
                59,   // TimeInForce
                168,  // EffectiveTime
                432,  // ExpireDate
                126,  // ExpireTime
                427,  // GTBookingInst
                12,   // Commission
                13,   // CommType
                479,  // CommCurrency
                497,  // FundRenewWaiv
                528,  // OrderCapacity
                529,  // OrderRestrictions
                582,  // CustOrderCapacity
                121,  // ForexReq
                120,  // SettlCurrency
                775,  // BookingType
                58,   // Text
                354,  // EncodedTextLen
                355,  // EncodedText
                193,  // SettlDate2
                192,  // OrderQty2
                640,  // Price2
                77,   // PositionEffect
                203,  // CoveredOrUncovered
                210,  // MaxShow
                // Parties nested group (453)
                // PreAllocGrp nested group (78)
                114,  // LocateReqd
                60,   // TransactTime
                1,    // Account
                660,  // AcctIDSource
                581,  // AccountType
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 453, // Parties (NoPartyIDs)
                    parent_tag: None,
                },
                NestedGroupInfo {
                    num_in_group_tag: 78, // PreAllocGrp (NoAllocs)
                    parent_tag: None,
                },
            ],
        },
    );

    // Same ListOrdGrp for ListStatus (N)
    registry.insert(
        GroupKey { num_in_group_tag: 73, msg_type: Some("N".to_string()) },
        GroupConfig {
            num_in_group_tag: 73,
            delimiter_tag: 11,
            member_tags: vec![
                11, 67, 160, 55, 65, 48, 22, 167, 200, 541, 224, 225, 239, 226, 227,
                228, 255, 543, 470, 471, 472, 240, 202, 947, 206, 231, 223, 207, 106,
                107, 691, 38, 152, 516, 468, 469, 54, 40, 44, 99, 15, 376, 377, 23,
                117, 59, 168, 432, 126, 427, 12, 13, 479, 497, 528, 529, 582, 121,
                120, 775, 58, 354, 355, 193, 192, 640, 77, 203, 210, 114, 60, 1, 660,
                581,
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 453,
                    parent_tag: None,
                },
                NestedGroupInfo {
                    num_in_group_tag: 78,
                    parent_tag: None,
                },
            ],
        },
    );

    // PreAllocGrp (Tag 78 = NoAllocs)
    // Nested within ListOrdGrp
    registry.insert(
        GroupKey { num_in_group_tag: 78, msg_type: None },
        GroupConfig {
            num_in_group_tag: 78, // NoAllocs
            delimiter_tag: 79,    // AllocAccount
            member_tags: vec![
                79,   // AllocAccount - Required
                661,  // AllocAcctIDSource
                736,  // AllocSettlCurrency
                467,  // IndividualAllocID
                80,   // AllocQty
                // NestedParties2 (756) can be nested here
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 756, // NestedParties2 (NoNested2PartyIDs)
                    parent_tag: None,
                },
            ],
        },
    );

    // NestedParties2 (Tag 756 = NoNested2PartyIDs)
    // Doubly-nested within PreAllocGrp
    registry.insert(
        GroupKey { num_in_group_tag: 756, msg_type: None },
        GroupConfig {
            num_in_group_tag: 756, // NoNested2PartyIDs
            delimiter_tag: 757,    // Nested2PartyID
            member_tags: vec![
                757,  // Nested2PartyID
                758,  // Nested2PartyIDSource
                759,  // Nested2PartyRole
                // NestedPartySubIDsGrp can be nested here (806)
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 806, // NoNested2PartySubIDs
                    parent_tag: None,
                },
            ],
        },
    );

    // NstdPtys2SubGrp (Tag 806 = NoNested2PartySubIDs)
    // Triply-nested within NestedParties2
    registry.insert(
        GroupKey { num_in_group_tag: 806, msg_type: None },
        GroupConfig {
            num_in_group_tag: 806, // NoNested2PartySubIDs
            delimiter_tag: 760,    // Nested2PartySubID
            member_tags: vec![
                760,  // Nested2PartySubID
                807,  // Nested2PartySubIDType
            ],
            nested_groups: vec![], // No further nesting
        },
    );

    // BidDescrReqGrp (Tag 398 = NoBidDescriptors)
    // Used in: BidRequest (k) - Non-Disclosed convention
    registry.insert(
        GroupKey { num_in_group_tag: 398, msg_type: Some("k".to_string()) },
        GroupConfig {
            num_in_group_tag: 398, // NoBidDescriptors
            delimiter_tag: 399,    // BidDescriptorType
            member_tags: vec![
                399,  // BidDescriptorType - Required
                400,  // BidDescriptor
                401,  // SideValueInd
                404,  // LiquidityValue
                441,  // LiquidityNumSecurities
                402,  // LiquidityPctLow
                403,  // LiquidityPctHigh
                405,  // EFPTrackingError
                406,  // FairValue
                407,  // OutsideIndexPct
                408,  // ValueOfFutures
            ],
            nested_groups: vec![],
        },
    );

    // BidCompReqGrp (Tag 420 = NoBidComponents)
    // Used in: BidRequest (k) - Disclosed convention
    registry.insert(
        GroupKey { num_in_group_tag: 420, msg_type: Some("k".to_string()) },
        GroupConfig {
            num_in_group_tag: 420, // NoBidComponents
            delimiter_tag: 66,     // ListID
            member_tags: vec![
                66,   // ListID - Required
                54,   // Side
                336,  // TradingSessionID
                625,  // TradingSessionSubID
                430,  // NetGrossInd
                63,   // SettlType
                64,   // SettlDate
                1,    // Account
            ],
            nested_groups: vec![],
        },
    );

    // BidCompRspGrp (Tag 420 = NoBidComponents)
    // Used in: BidResponse (l)
    registry.insert(
        GroupKey { num_in_group_tag: 420, msg_type: Some("l".to_string()) },
        GroupConfig {
            num_in_group_tag: 420, // NoBidComponents
            delimiter_tag: 66,     // ListID
            member_tags: vec![
                12,   // Commission
                13,   // CommType
                479,  // CommCurrency
                497,  // FundRenewWaiv
                66,   // ListID - Required
                421,  // Country
                54,   // Side
                44,   // Price
                423,  // PriceType
                406,  // FairValue
                430,  // NetGrossInd
                63,   // SettlType
                64,   // SettlDate
                336,  // TradingSessionID
                625,  // TradingSessionSubID
                58,   // Text
            ],
            nested_groups: vec![],
        },
    );

    // OrdListStatGrp (Tag 73 = NoOrders)
    // Used in: ListStatus (N) - for order status reporting
    // Note: This overlaps with ListOrdGrp but has different fields for status reporting
    // We use the same configuration as ListOrdGrp for ListStatus above

    // StrikeRules (Tag 1201 = NoStrikeRules)
    // Used in: ListStrikePrice (m)
    registry.insert(
        GroupKey { num_in_group_tag: 1201, msg_type: Some("m".to_string()) },
        GroupConfig {
            num_in_group_tag: 1201, // NoStrikeRules
            delimiter_tag: 1223,    // StrikeRuleID
            member_tags: vec![
                1223, // StrikeRuleID - Required
                1202, // StartStrikePxRange
                1203, // EndStrikePxRange
                1204, // StrikeIncrement
                1304, // StrikeExerciseStyle
                1302, // MaturityMonthYearIncrementUnits
                1303, // MaturityMonthYearIncrement
            ],
            nested_groups: vec![],
        },
    );

    // ========================================================================
    // Mass Order Repeating Groups
    // ========================================================================

    // OrderEntryGrp (Tag 2430 = NoOrderEntries)
    // Used in: MassOrder (DJ)
    // Each entry is a full order specification (similar to NewOrderSingle)
    registry.insert(
        GroupKey { num_in_group_tag: 2430, msg_type: Some("DJ".to_string()) },
        GroupConfig {
            num_in_group_tag: 2430, // NoOrderEntries
            delimiter_tag: 11,      // ClOrdID
            member_tags: vec![
                11,   // ClOrdID - Required
                526,  // SecondaryClOrdID
                583,  // ClOrdLinkID
                // Instrument component
                55,   // Symbol
                65,   // SymbolSfx
                48,   // SecurityID
                22,   // SecurityIDSource
                167,  // SecurityType
                // OrderQtyData component
                38,   // OrderQty
                152,  // CashOrderQty
                516,  // OrderPercent
                54,   // Side - Required
                40,   // OrdType - Required
                44,   // Price
                99,   // StopPx
                15,   // Currency
                59,   // TimeInForce
                168,  // EffectiveTime
                432,  // ExpireDate
                126,  // ExpireTime
                60,   // TransactTime - Required
                1,    // Account
                660,  // AcctIDSource
                581,  // AccountType
                528,  // OrderCapacity
                529,  // OrderRestrictions
                582,  // CustOrderCapacity
                58,   // Text
                354,  // EncodedTextLen
                355,  // EncodedText
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 453, // Parties (NoPartyIDs)
                    parent_tag: None,
                },
                NestedGroupInfo {
                    num_in_group_tag: 78, // PreAllocGrp (NoAllocs)
                    parent_tag: None,
                },
            ],
        },
    );

    // OrderEntryAckGrp (Tag 2430 = NoOrderEntries)
    // Used in: MassOrderAck (DK)
    // Contains acknowledgement status for each order entry
    registry.insert(
        GroupKey { num_in_group_tag: 2430, msg_type: Some("DK".to_string()) },
        GroupConfig {
            num_in_group_tag: 2430, // NoOrderEntries
            delimiter_tag: 11,      // ClOrdID
            member_tags: vec![
                11,   // ClOrdID - Required
                37,   // OrderID
                39,   // OrdStatus
                103,  // OrdRejReason
                636,  // WorkingIndicator
                58,   // Text
                354,  // EncodedTextLen
                355,  // EncodedText
            ],
            nested_groups: vec![],
        },
    );

    // TargetMarketSegmentGrp (Tag 1793 = NoTargetMarketSegments)
    // Used in: OrderMassActionRequest (CA)
    // Specifies market segments to target for mass action
    registry.insert(
        GroupKey { num_in_group_tag: 1793, msg_type: Some("CA".to_string()) },
        GroupConfig {
            num_in_group_tag: 1793, // NoTargetMarketSegments
            delimiter_tag: 1300,    // MarketSegmentID
            member_tags: vec![
                1300, // MarketSegmentID - Required
                1301, // MarketID
            ],
            nested_groups: vec![],
        },
    );

    // AffectedMarketSegmentGrp (Tag 1793 = NoAffectedMarketSegments)
    // Used in: OrderMassActionReport (BZ)
    // Reports market segments affected by the mass action
    registry.insert(
        GroupKey { num_in_group_tag: 1793, msg_type: Some("BZ".to_string()) },
        GroupConfig {
            num_in_group_tag: 1793, // NoAffectedMarketSegments
            delimiter_tag: 1300,    // MarketSegmentID
            member_tags: vec![
                1300, // MarketSegmentID - Required
                1301, // MarketID
                533,  // TotalAffectedOrders
            ],
            nested_groups: vec![],
        },
    );

    // NotAffectedMarketSegmentGrp (Tag 1794 = NoNotAffMarketSegments)
    // Used in: OrderMassActionReport (BZ)
    // Reports market segments NOT affected by the mass action
    registry.insert(
        GroupKey { num_in_group_tag: 1794, msg_type: Some("BZ".to_string()) },
        GroupConfig {
            num_in_group_tag: 1794, // NoNotAffMarketSegments
            delimiter_tag: 1300,    // MarketSegmentID
            member_tags: vec![
                1300, // MarketSegmentID - Required
                1301, // MarketID
            ],
            nested_groups: vec![],
        },
    );

    // ========================================================================
    // Multileg Order Repeating Groups
    // ========================================================================

    // LegOrdGrp (Tag 555 = NoLegs)
    // Used in: NewOrderMultileg (AB), MultilegOrderCancelReplace (AC)
    // Each entry contains full leg specification with nested groups
    registry.insert(
        GroupKey { num_in_group_tag: 555, msg_type: Some("AB".to_string()) },
        GroupConfig {
            num_in_group_tag: 555, // NoLegs
            delimiter_tag: 600,    // LegSymbol
            member_tags: vec![
                600,  // LegSymbol
                601,  // LegSymbolSfx
                602,  // LegSecurityID
                603,  // LegSecurityIDSource
                604,  // LegProduct
                605,  // LegCFICode
                606,  // LegSecurityType
                607,  // LegMaturityMonthYear
                608,  // LegMaturityDate
                609,  // LegStrikePrice
                610,  // LegOptAttribute
                611,  // LegContractMultiplier
                612,  // LegCouponRate
                613,  // LegSecurityExchange
                614,  // LegIssuer
                615,  // LegSecurityDesc
                616,  // LegRatioQty
                623,  // LegRatioQty (alternative)
                624,  // LegSide
                566,  // LegPrice
                654,  // LegRefID
                687,  // LegQty
                690,  // LegSwapType
                564,  // LegPositionEffect
                565,  // LegCoveredOrUncovered
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 683, // LegStipulations (NoLegStipulations)
                    parent_tag: None,
                },
                NestedGroupInfo {
                    num_in_group_tag: 539, // NestedPartyIDs (NoNestedPartyIDs)
                    parent_tag: None,
                },
            ],
        },
    );

    // Same LegOrdGrp for MultilegOrderCancelReplace (AC)
    registry.insert(
        GroupKey { num_in_group_tag: 555, msg_type: Some("AC".to_string()) },
        GroupConfig {
            num_in_group_tag: 555,
            delimiter_tag: 600,
            member_tags: vec![
                600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613,
                614, 615, 616, 623, 624, 566, 654, 687, 690, 564, 565,
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 683,
                    parent_tag: None,
                },
                NestedGroupInfo {
                    num_in_group_tag: 539,
                    parent_tag: None,
                },
            ],
        },
    );

    // LegStipulations (Tag 683 = NoLegStipulations)
    // Nested within LegOrdGrp - leg-specific stipulations
    registry.insert(
        GroupKey { num_in_group_tag: 683, msg_type: None },
        GroupConfig {
            num_in_group_tag: 683, // NoLegStipulations
            delimiter_tag: 688,    // LegStipulationType
            member_tags: vec![
                688,  // LegStipulationType
                689,  // LegStipulationValue
            ],
            nested_groups: vec![],
        },
    );

    // NestedPartyIDs (Tag 539 = NoNestedPartyIDs)
    // Nested within LegOrdGrp - leg-level party information
    registry.insert(
        GroupKey { num_in_group_tag: 539, msg_type: None },
        GroupConfig {
            num_in_group_tag: 539, // NoNestedPartyIDs
            delimiter_tag: 524,    // NestedPartyID
            member_tags: vec![
                524,  // NestedPartyID
                525,  // NestedPartyIDSource
                538,  // NestedPartyRole
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 804, // NstdPtysSubGrp (NoNestedPartySubIDs)
                    parent_tag: None,
                },
            ],
        },
    );

    // NstdPtysSubGrp (Tag 804 = NoNestedPartySubIDs)
    // Nested within NestedPartyIDs
    registry.insert(
        GroupKey { num_in_group_tag: 804, msg_type: None },
        GroupConfig {
            num_in_group_tag: 804, // NoNestedPartySubIDs
            delimiter_tag: 545,    // NestedPartySubID
            member_tags: vec![
                545,  // NestedPartySubID
                805,  // NestedPartySubIDType
            ],
            nested_groups: vec![],
        },
    );

    // ========================================================================
    // Network Status Communication Groups
    // ========================================================================

    // CompIDReqGrp (Tag 936 = NoCompIDs) - for NetworkCounterpartySystemStatusRequest (BC)
    registry.insert(
        GroupKey { num_in_group_tag: 936, msg_type: Some("BC".to_string()) },
        GroupConfig {
            num_in_group_tag: 936, // NoCompIDs
            delimiter_tag: 930,    // RefCompID
            member_tags: vec![
                930,  // RefCompID
                931,  // RefSubID
                283,  // LocationID
                284,  // DeskID
            ],
            nested_groups: vec![],
        },
    );

    // CompIDStatGrp (Tag 936 = NoCompIDs) - for NetworkCounterpartySystemStatusResponse (BD)
    registry.insert(
        GroupKey { num_in_group_tag: 936, msg_type: Some("BD".to_string()) },
        GroupConfig {
            num_in_group_tag: 936, // NoCompIDs
            delimiter_tag: 930,    // RefCompID
            member_tags: vec![
                930,  // RefCompID
                931,  // RefSubID
                283,  // LocationID
                284,  // DeskID
                928,  // StatusValue (NetworkSystemStatus)
                929,  // StatusText
            ],
            nested_groups: vec![],
        },
    );

    // ========================================================================
    // Application Sequencing Groups
    // ========================================================================

    // ApplIDRequestGrp (Tag 1351 = NoApplIDs) - for ApplicationMessageRequest (BW)
    registry.insert(
        GroupKey { num_in_group_tag: 1351, msg_type: Some("BW".to_string()) },
        GroupConfig {
            num_in_group_tag: 1351, // NoApplIDs
            delimiter_tag: 1355,    // RefApplID
            member_tags: vec![
                1355,  // RefApplID
                1430,  // RefApplReqID
                1182,  // ApplBegSeqNum
                1183,  // ApplEndSeqNum
            ],
            nested_groups: vec![],
        },
    );

    // ========================================================================
    // Pre-Trade Messages - Section Markers for Repeating Groups
    // ========================================================================
    // This section is pre-allocated for parallel development of Pre-Trade messages.
    // Each category will add group configs in their designated section.

    // ========================================================================
    // [SECTION 100] Indication Messages Groups
    // Implementation: feature/pretrade-indication
    // ========================================================================

    // IOIQualGrp (Tag 199 = NoIOIQualifiers) - for IOI message
    registry.insert(
        GroupKey { num_in_group_tag: 199, msg_type: None },
        GroupConfig {
            num_in_group_tag: 199, // NoIOIQualifiers
            delimiter_tag: 104,    // IOIQualifier
            member_tags: vec![
                104,  // IOIQualifier
            ],
            nested_groups: vec![],
        },
    );

    // RoutingGrp (Tag 215 = NoRoutingIDs) - for IOI message
    registry.insert(
        GroupKey { num_in_group_tag: 215, msg_type: None },
        GroupConfig {
            num_in_group_tag: 215, // NoRoutingIDs
            delimiter_tag: 216,    // RoutingType
            member_tags: vec![
                216,  // RoutingType
                217,  // RoutingID
            ],
            nested_groups: vec![],
        },
    );

    // ========================================================================
    // [SECTION 200] Event Communication Messages Groups
    // Implementation: feature/pretrade-event-communication
    // ========================================================================

    // LinesOfTextGrp (Tag 33 = NoLinesOfText) - for Email and News messages
    registry.insert(
        GroupKey { num_in_group_tag: 33, msg_type: None },
        GroupConfig {
            num_in_group_tag: 33,  // NoLinesOfText
            delimiter_tag: 58,     // Text
            member_tags: vec![
                58,   // Text
                354,  // EncodedTextLen
                355,  // EncodedText
            ],
            nested_groups: vec![],
        },
    );

    // EmailRoutingGrp (Tag 215 = NoRoutingIDs) - for Email message
    // Note: RoutingGrp already defined in SECTION 100 as generic (msg_type: None)
    // Email can reuse the existing RoutingGrp definition

    // NewsRefGrp (Tag 1476 = NoNewsRefIDs) - for News message
    registry.insert(
        GroupKey { num_in_group_tag: 1476, msg_type: None },
        GroupConfig {
            num_in_group_tag: 1476, // NoNewsRefIDs
            delimiter_tag: 1477,    // NewsRefID
            member_tags: vec![
                1477,  // NewsRefID
                1478,  // NewsRefType (corresponds to Tag 1477 enum)
            ],
            nested_groups: vec![],
        },
    );

    // ========================================================================
    // [SECTION 300] Quotation/Negotiation Messages Groups
    // Implementation: feature/pretrade-quotation
    // ========================================================================

    // QuotQualGrp (Tag 735 = NoQuoteQualifiers)
    // Used in: Quote (S), MassQuote (i)
    // Qualifiers that provide additional information about the quote
    registry.insert(
        GroupKey { num_in_group_tag: 735, msg_type: None },
        GroupConfig {
            num_in_group_tag: 735, // NoQuoteQualifiers
            delimiter_tag: 695,    // QuoteQualifier
            member_tags: vec![
                695,  // QuoteQualifier - Required
            ],
            nested_groups: vec![],
        },
    );

    // QuotReqGrp (Tag 146 = NoRelatedSym)
    // Used in: QuoteRequest (R)
    // Group of securities for which quotes are requested
    registry.insert(
        GroupKey { num_in_group_tag: 146, msg_type: Some("R".to_string()) },
        GroupConfig {
            num_in_group_tag: 146, // NoRelatedSym
            delimiter_tag: 55,     // Symbol
            member_tags: vec![
                55,   // Symbol - Required
                65,   // SymbolSfx
                48,   // SecurityID
                22,   // SecurityIDSource
                167,  // SecurityType
                200,  // MaturityMonthYear
                541,  // MaturityDate
                201,  // PutOrCall
                202,  // StrikePrice
                206,  // OptAttribute
                231,  // ContractMultiplier
                223,  // CouponRate
                207,  // SecurityExchange
                106,  // Issuer
                107,  // SecurityDesc
                54,   // Side
                38,   // OrderQty
                40,   // OrdType
                44,   // Price
                15,   // Currency
                58,   // Text
                354,  // EncodedTextLen
                355,  // EncodedText
            ],
            nested_groups: vec![],
        },
    );

    // QuotSetGrp (Tag 296 = NoQuoteSets)
    // Used in: MassQuote (i), MassQuoteAcknowledgement (b)
    // Group of quote sets for mass quote operations
    registry.insert(
        GroupKey { num_in_group_tag: 296, msg_type: Some("i".to_string()) },
        GroupConfig {
            num_in_group_tag: 296, // NoQuoteSets
            delimiter_tag: 302,    // QuoteSetID
            member_tags: vec![
                302,  // QuoteSetID - Required
                311,  // UnderlyingSymbol
                312,  // UnderlyingSymbolSfx
                309,  // UnderlyingSecurityID
                305,  // UnderlyingSecurityIDSource
                313,  // UnderlyingSecurityType
                542,  // UnderlyingMaturityMonthYear
                // QuotEntryGrp nested here (295)
                304,  // QuoteSetValidUntilTime
                367,  // TotNoQuoteEntries
                304,  // LastFragment
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 295, // QuotEntryGrp (NoQuoteEntries)
                    parent_tag: None,
                },
            ],
        },
    );

    // Same QuotSetGrp for MassQuoteAcknowledgement (b)
    registry.insert(
        GroupKey { num_in_group_tag: 296, msg_type: Some("b".to_string()) },
        GroupConfig {
            num_in_group_tag: 296,
            delimiter_tag: 302,
            member_tags: vec![
                302, 311, 312, 309, 305, 313, 542, 304, 367, 304,
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 295,
                    parent_tag: None,
                },
            ],
        },
    );

    // QuotEntryGrp (Tag 295 = NoQuoteEntries)
    // Used in: nested within QuotSetGrp in MassQuote (i)
    // Individual quote entries within a quote set
    registry.insert(
        GroupKey { num_in_group_tag: 295, msg_type: Some("i".to_string()) },
        GroupConfig {
            num_in_group_tag: 295, // NoQuoteEntries
            delimiter_tag: 299,    // QuoteEntryID
            member_tags: vec![
                299,  // QuoteEntryID - Required
                55,   // Symbol
                65,   // SymbolSfx
                48,   // SecurityID
                22,   // SecurityIDSource
                167,  // SecurityType
                200,  // MaturityMonthYear
                541,  // MaturityDate
                132,  // BidPx
                133,  // OfferPx
                134,  // BidSize
                135,  // OfferSize
                62,   // ValidUntilTime
                188,  // BidSpotRate
                190,  // OfferSpotRate
                189,  // BidForwardPoints
                191,  // OfferForwardPoints
                631,  // MidPx
                632,  // BidYield
                633,  // MidYield
                634,  // OfferYield
                60,   // TransactTime
                336,  // TradingSessionID
                625,  // TradingSessionSubID
                64,   // SettlDate
                40,   // OrdType
                193,  // SettlDate2
                192,  // OrderQty2
                15,   // Currency
            ],
            nested_groups: vec![],
        },
    );

    // Same QuotEntryGrp for MassQuoteAcknowledgement (b)
    registry.insert(
        GroupKey { num_in_group_tag: 295, msg_type: Some("b".to_string()) },
        GroupConfig {
            num_in_group_tag: 295,
            delimiter_tag: 299,
            member_tags: vec![
                299, 55, 65, 48, 22, 167, 200, 541, 132, 133, 134, 135, 62, 188, 190, 189,
                191, 631, 632, 633, 634, 60, 336, 625, 64, 40, 193, 192, 15,
            ],
            nested_groups: vec![],
        },
    );

    // QuotCxlEntriesGrp (Tag 295 = NoQuoteEntries)
    // Used in: QuoteCancel (Z)
    // Quote entries to be cancelled
    registry.insert(
        GroupKey { num_in_group_tag: 295, msg_type: Some("Z".to_string()) },
        GroupConfig {
            num_in_group_tag: 295, // NoQuoteEntries
            delimiter_tag: 55,     // Symbol
            member_tags: vec![
                55,   // Symbol - Required
                65,   // SymbolSfx
                48,   // SecurityID
                22,   // SecurityIDSource
                167,  // SecurityType
            ],
            nested_groups: vec![],
        },
    );

    // QuotReqRjctGrp (Tag 146 = NoRelatedSym)
    // Used in: QuoteRequestReject (AG)
    // Group of symbols for which quote request was rejected
    registry.insert(
        GroupKey { num_in_group_tag: 146, msg_type: Some("AG".to_string()) },
        GroupConfig {
            num_in_group_tag: 146, // NoRelatedSym
            delimiter_tag: 55,     // Symbol
            member_tags: vec![
                55,   // Symbol - Required
                65,   // SymbolSfx
                48,   // SecurityID
                22,   // SecurityIDSource
                167,  // SecurityType
                200,  // MaturityMonthYear
                541,  // MaturityDate
                54,   // Side
                38,   // OrderQty
                40,   // OrdType
                58,   // Text
                354,  // EncodedTextLen
                355,  // EncodedText
            ],
            nested_groups: vec![],
        },
    );

    // LegQuotGrp (Tag 555 = NoLegs)
    // Used in: Quote (S), QuoteRequest (R), QuoteStatusReport (AI), RFQRequest (AH)
    // Leg-level pricing for multileg quotes
    registry.insert(
        GroupKey { num_in_group_tag: 555, msg_type: Some("S".to_string()) },
        GroupConfig {
            num_in_group_tag: 555, // NoLegs
            delimiter_tag: 600,    // LegSymbol
            member_tags: vec![
                600,  // LegSymbol - Required
                601,  // LegSymbolSfx
                602,  // LegSecurityID
                603,  // LegSecurityIDSource
                604,  // LegProduct
                605,  // LegCFICode
                606,  // LegSecurityType
                607,  // LegMaturityMonthYear
                608,  // LegMaturityDate
                609,  // LegStrikePrice
                610,  // LegOptAttribute
                611,  // LegContractMultiplier
                612,  // LegCouponRate
                613,  // LegSecurityExchange
                614,  // LegIssuer
                615,  // LegSecurityDesc
                687,  // LegQty
                690,  // LegSwapType
                587,  // LegSettlType
                588,  // LegSettlDate
                686,  // LegPriceType
                681,  // LegBidPx
                684,  // LegOfferPx
            ],
            nested_groups: vec![],
        },
    );

    // Same LegQuotGrp for QuoteRequest (R)
    registry.insert(
        GroupKey { num_in_group_tag: 555, msg_type: Some("R".to_string()) },
        GroupConfig {
            num_in_group_tag: 555,
            delimiter_tag: 600,
            member_tags: vec![
                600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613, 614,
                615, 687, 690, 587, 588, 686, 681, 684,
            ],
            nested_groups: vec![],
        },
    );

    // Same LegQuotGrp for QuoteStatusReport (AI)
    registry.insert(
        GroupKey { num_in_group_tag: 555, msg_type: Some("AI".to_string()) },
        GroupConfig {
            num_in_group_tag: 555,
            delimiter_tag: 600,
            member_tags: vec![
                600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613, 614,
                615, 687, 690, 587, 588, 686, 681, 684,
            ],
            nested_groups: vec![],
        },
    );

    // Same LegQuotGrp for RFQRequest (AH)
    registry.insert(
        GroupKey { num_in_group_tag: 555, msg_type: Some("AH".to_string()) },
        GroupConfig {
            num_in_group_tag: 555,
            delimiter_tag: 600,
            member_tags: vec![
                600, 601, 602, 603, 604, 605, 606, 607, 608, 609, 610, 611, 612, 613, 614,
                615, 687, 690, 587, 588, 686, 681, 684,
            ],
            nested_groups: vec![],
        },
    );

    // UndInstrmtGrp (Tag 711 = NoUnderlyings)
    // Used in: Quote (S), QuoteRequest (R), and other messages
    // Underlying security information for derivatives
    registry.insert(
        GroupKey { num_in_group_tag: 711, msg_type: None },
        GroupConfig {
            num_in_group_tag: 711, // NoUnderlyings
            delimiter_tag: 311,    // UnderlyingSymbol
            member_tags: vec![
                311,  // UnderlyingSymbol - Required
                312,  // UnderlyingSymbolSfx
                309,  // UnderlyingSecurityID
                305,  // UnderlyingSecurityIDSource
                313,  // UnderlyingSecurityType
                542,  // UnderlyingMaturityMonthYear
                241,  // UnderlyingMaturityDate
                317,  // UnderlyingCouponRate
                316,  // UnderlyingSecurityExchange
                318,  // UnderlyingIssuer
                879,  // UnderlyingSecurityDesc
                810,  // UnderlyingPx
                882,  // UnderlyingQty
            ],
            nested_groups: vec![],
        },
    );

    // RateSourceGrp (Tag 1445 = NoRateSources)
    // Used in: Quote (S)
    // Rate source information for quotes
    registry.insert(
        GroupKey { num_in_group_tag: 1445, msg_type: None },
        GroupConfig {
            num_in_group_tag: 1445, // NoRateSources
            delimiter_tag: 1446,    // RateSource
            member_tags: vec![
                1446,  // RateSource - Required
                1447,  // RateSourceType
                1448,  // ReferencePage
            ],
            nested_groups: vec![],
        },
    );

    // QuotReqLegsGrp (Tag 555 = NoLegs)
    // Used in: QuoteRequest (R)
    // Similar to LegQuotGrp but for quote requests
    // Note: Using message-specific key for QuoteRequest already handled above

    // StreamAsgnReqGrp (Tag 1498 = NoAsgnReqs)
    // Used in: StreamAssignmentRequest (CC)
    // Assignment requests for quote streams
    registry.insert(
        GroupKey { num_in_group_tag: 1498, msg_type: Some("CC".to_string()) },
        GroupConfig {
            num_in_group_tag: 1498, // NoAsgnReqs
            delimiter_tag: 453,     // NoPartyIDs (Parties component)
            member_tags: vec![
                // Parties component fields
                1499,  // MDStreamID
            ],
            nested_groups: vec![
                NestedGroupInfo {
                    num_in_group_tag: 453, // Parties
                    parent_tag: None,
                },
            ],
        },
    );

    // ========================================================================
    // [SECTION 400] Market Data Messages Groups
    // Implementation: feature/pretrade-market-data
    // ========================================================================

    // MDReqGrp (Tag 146 = NoRelatedSym) - for MarketDataRequest message
    registry.insert(
        GroupKey { num_in_group_tag: 146, msg_type: Some("V".to_string()) },
        GroupConfig {
            num_in_group_tag: 146, // NoRelatedSym
            delimiter_tag: 55,     // Symbol
            member_tags: vec![
                55,   // Symbol
                65,   // SymbolSfx
                48,   // SecurityID
                22,   // SecurityIDSource
                167,  // SecurityType
                200,  // MaturityMonthYear
                541,  // MaturityDate
                201,  // PutOrCall
                202,  // StrikePrice
                206,  // OptAttribute
                231,  // ContractMultiplier
                223,  // CouponRate
                207,  // SecurityExchange
                106,  // Issuer
                107,  // SecurityDesc
                1300, // MarketSegmentID
                1301, // MarketID
            ],
            nested_groups: vec![],
        },
    );

    // MDEntryTypesGrp (Tag 267 = NoMDEntryTypes) - for MarketDataRequest message
    registry.insert(
        GroupKey { num_in_group_tag: 267, msg_type: Some("V".to_string()) },
        GroupConfig {
            num_in_group_tag: 267, // NoMDEntryTypes
            delimiter_tag: 269,    // MDEntryType
            member_tags: vec![
                269,  // MDEntryType
            ],
            nested_groups: vec![],
        },
    );

    // MDFullGrp (Tag 268 = NoMDEntries) - for MarketDataSnapshotFullRefresh message
    registry.insert(
        GroupKey { num_in_group_tag: 268, msg_type: Some("W".to_string()) },
        GroupConfig {
            num_in_group_tag: 268, // NoMDEntries
            delimiter_tag: 269,    // MDEntryType
            member_tags: vec![
                269,  // MDEntryType - Required
                270,  // MDEntryPx
                271,  // MDEntrySize
                272,  // MDEntryDate
                273,  // MDEntryTime
                274,  // TickDirection
                275,  // MDMkt
                276,  // QuoteCondition
                277,  // TradeCondition
                282,  // MDEntryOriginator
                283,  // LocationID
                284,  // DeskID
                286,  // OpenCloseSettleFlag
                287,  // TimeInForce
                288,  // ExpireDate
                289,  // ExpireTime
                290,  // MinQty
                299,  // QuoteEntryID
                346,  // NumberOfOrders
                 387,  // TotalVolumeTraded
                1020, // TradeVolume
                1023, // MDPriceLevel
                336,  // TradingSessionID
                625,  // TradingSessionSubID
            ],
            nested_groups: vec![],
        },
    );

    // ========================================================================
    // [SECTION 500] Market Structure Reference Data Messages Groups
    // Implementation: feature/pretrade-market-structure
    // ========================================================================
    // Groups will be added here by the Market Structure PR:
    // - TradingSessionRulesGrp (Tag 1309 = NoTradingSessionRules)
    // - MarketSegmentGrp (Tag 1310 = NoMarketSegments)
    // - StrikeRuleGrp (Tag 1201 = NoStrikeRules) [may already exist]

    // ========================================================================
    // [SECTION 600] Securities Reference Data Messages Groups
    // Implementation: feature/pretrade-securities-reference
    // ========================================================================

    // SecListGrp (Tag 146 = NoRelatedSym) - for SecurityList (y)
    // Repeating group of securities in a security list
    registry.insert(
        GroupKey { num_in_group_tag: 146, msg_type: Some("y".to_string()) },
        GroupConfig {
            num_in_group_tag: 146, // NoRelatedSym
            delimiter_tag: 55,     // Symbol
            member_tags: vec![
                55,   // Symbol - Required
                48,   // SecurityID
                22,   // SecurityIDSource
                107,  // SecurityDesc
                167,  // SecurityType
                200,  // MaturityMonthYear
                541,  // MaturityDate
                202,  // StrikePrice
                15,   // Currency
                460,  // Product
                461,  // CFICode
                762,  // SecuritySubType
            ],
            nested_groups: vec![],
        },
    );

    // SecListUpdRelSymGrp (Tag 146 = NoRelatedSym) - for SecurityListUpdateReport (BK)
    // Repeating group of security updates in a security list update report
    registry.insert(
        GroupKey { num_in_group_tag: 146, msg_type: Some("BK".to_string()) },
        GroupConfig {
            num_in_group_tag: 146, // NoRelatedSym
            delimiter_tag: 55,     // Symbol
            member_tags: vec![
                55,   // Symbol - Required
                980,  // SecurityUpdateAction
                48,   // SecurityID
                22,   // SecurityIDSource
                107,  // SecurityDesc
                167,  // SecurityType
                200,  // MaturityMonthYear
                541,  // MaturityDate
                202,  // StrikePrice
                15,   // Currency
                460,  // Product
                461,  // CFICode
                762,  // SecuritySubType
            ],
            nested_groups: vec![],
        },
    );

    // SecTypesGrp (Tag 558 = NoSecurityTypes) - for SecurityTypes (w)
    // Repeating group of security types
    registry.insert(
        GroupKey { num_in_group_tag: 558, msg_type: Some("w".to_string()) },
        GroupConfig {
            num_in_group_tag: 558, // NoSecurityTypes
            delimiter_tag: 167,    // SecurityType
            member_tags: vec![
                167,  // SecurityType - Required
                762,  // SecuritySubType
                460,  // Product
                461,  // CFICode
                58,   // Text
            ],
            nested_groups: vec![],
        },
    );

    // DerivSecListGrp (Tag 146 = NoRelatedSym) - for DerivativeSecurityList (AA)
    // Repeating group of derivative securities in a derivative security list
    registry.insert(
        GroupKey { num_in_group_tag: 146, msg_type: Some("AA".to_string()) },
        GroupConfig {
            num_in_group_tag: 146, // NoRelatedSym
            delimiter_tag: 55,     // Symbol
            member_tags: vec![
                55,   // Symbol - Required
                48,   // SecurityID
                22,   // SecurityIDSource
                167,  // SecurityType
                200,  // MaturityMonthYear
                541,  // MaturityDate
                202,  // StrikePrice
                201,  // PutOrCall
                15,   // Currency
                460,  // Product
                461,  // CFICode
                762,  // SecuritySubType
            ],
            nested_groups: vec![],
        },
    );

    // SecMassStatGrp (Tag 146 = NoRelatedSym) - for SecurityMassStatus (CO)
    // Repeating group of security statuses in a security mass status report
    registry.insert(
        GroupKey { num_in_group_tag: 146, msg_type: Some("CO".to_string()) },
        GroupConfig {
            num_in_group_tag: 146, // NoRelatedSym
            delimiter_tag: 55,     // Symbol
            member_tags: vec![
                55,   // Symbol - Required
                48,   // SecurityID
                22,   // SecurityIDSource
                326,  // SecurityTradingStatus
                327,  // HaltReason
                75,   // TradeDate
                60,   // TransactTime
            ],
            nested_groups: vec![],
        },
    );

    registry
});

/// Check if a tag is a NoXXX (number-in-group) tag
pub fn is_num_in_group_tag(tag: u32, msg_type: Option<&str>) -> bool {
    // Try message-specific first, then fall back to generic
    let msg_key = GroupKey {
        num_in_group_tag: tag,
        msg_type: msg_type.map(|s| s.to_string()),
    };
    if GROUP_REGISTRY.contains_key(&msg_key) {
        return true;
    }

    let generic_key = GroupKey {
        num_in_group_tag: tag,
        msg_type: None,
    };
    GROUP_REGISTRY.contains_key(&generic_key)
}

/// Get the delimiter tag for a given NoXXX tag with message context
pub fn get_delimiter_tag(num_in_group_tag: u32, msg_type: Option<&str>) -> Option<u32> {
    // Try message-specific first
    if let Some(mt) = msg_type {
        let key = GroupKey {
            num_in_group_tag,
            msg_type: Some(mt.to_string()),
        };
        if let Some(config) = GROUP_REGISTRY.get(&key) {
            return Some(config.delimiter_tag);
        }
    }

    // Fall back to generic
    let generic_key = GroupKey {
        num_in_group_tag,
        msg_type: None,
    };
    GROUP_REGISTRY.get(&generic_key).map(|c| c.delimiter_tag)
}

/// Get all member tags for a given repeating group with message context
pub fn get_member_tags(num_in_group_tag: u32, msg_type: Option<&str>) -> Option<&'static Vec<u32>> {
    // Try message-specific first
    if let Some(mt) = msg_type {
        let key = GroupKey {
            num_in_group_tag,
            msg_type: Some(mt.to_string()),
        };
        if let Some(config) = GROUP_REGISTRY.get(&key) {
            return Some(&config.member_tags);
        }
    }

    // Fall back to generic
    let generic_key = GroupKey {
        num_in_group_tag,
        msg_type: None,
    };
    GROUP_REGISTRY.get(&generic_key).map(|c| &c.member_tags)
}

/// Get nested group information for a given repeating group
pub fn get_nested_groups(num_in_group_tag: u32, msg_type: Option<&str>) -> Option<&'static Vec<NestedGroupInfo>> {
    // Try message-specific first
    if let Some(mt) = msg_type {
        let key = GroupKey {
            num_in_group_tag,
            msg_type: Some(mt.to_string()),
        };
        if let Some(config) = GROUP_REGISTRY.get(&key) {
            return Some(&config.nested_groups);
        }
    }

    // Fall back to generic
    let generic_key = GroupKey {
        num_in_group_tag,
        msg_type: None,
    };
    GROUP_REGISTRY.get(&generic_key).map(|c| &c.nested_groups)
}

/// Check if a tag represents a nested group within a parent group context
pub fn is_nested_group(parent_num_in_group_tag: u32, child_num_in_group_tag: u32, msg_type: Option<&str>) -> bool {
    if let Some(nested_groups) = get_nested_groups(parent_num_in_group_tag, msg_type) {
        nested_groups.iter().any(|ng| ng.num_in_group_tag == child_num_in_group_tag)
    } else {
        false
    }
}

/// Get the full GroupConfig for a given repeating group (for testing purposes)
#[cfg(test)]
pub fn get_group_config(num_in_group_tag: u32, msg_type: Option<&str>) -> Option<&'static GroupConfig> {
    // Try message-specific first
    if let Some(mt) = msg_type {
        let key = GroupKey {
            num_in_group_tag,
            msg_type: Some(mt.to_string()),
        };
        if let Some(config) = GROUP_REGISTRY.get(&key) {
            return Some(config);
        }
    }

    // Fall back to generic
    let generic_key = GroupKey {
        num_in_group_tag,
        msg_type: None,
    };
    GROUP_REGISTRY.get(&generic_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_nested_groups_parties() {
        // Parties component (453) should have PartySubIDsGrp (802) as nested group
        let nested = get_nested_groups(453, None).expect("Parties should have nested groups");
        assert_eq!(nested.len(), 1);
        assert_eq!(nested[0].num_in_group_tag, 802);
        assert_eq!(nested[0].parent_tag, None);
    }

    #[test]
    fn test_get_nested_groups_party_sub_ids() {
        // PartySubIDsGrp (802) should have no nested groups
        let nested = get_nested_groups(802, None).expect("PartySubIDsGrp should exist");
        assert_eq!(nested.len(), 0);
    }

    #[test]
    fn test_get_nested_groups_message_specific() {
        // SideCrossOrdModGrp for NewOrderCross ('s') should have Parties as nested
        let nested = get_nested_groups(552, Some("s")).expect("SideCrossOrdModGrp should have nested groups");
        assert_eq!(nested.len(), 1);
        assert_eq!(nested[0].num_in_group_tag, 453);
    }

    #[test]
    fn test_is_nested_group_positive() {
        // 802 (PartySubIDsGrp) is nested within 453 (Parties)
        assert!(is_nested_group(453, 802, None));
    }

    #[test]
    fn test_is_nested_group_negative() {
        // 802 is NOT nested within 552
        assert!(!is_nested_group(552, 802, None));
    }

    #[test]
    fn test_is_nested_group_message_specific() {
        // 453 (Parties) is nested within 552 (SideCrossOrdModGrp) for message 's'
        assert!(is_nested_group(552, 453, Some("s")));
    }

    #[test]
    fn test_nested_group_two_levels() {
        // Verify we can query both levels:
        // Level 1: 552 contains 453
        assert!(is_nested_group(552, 453, Some("s")));
        // Level 2: 453 contains 802
        assert!(is_nested_group(453, 802, None));
    }

    #[test]
    fn test_get_nested_groups_nonexistent() {
        // Non-existent group should return None
        assert!(get_nested_groups(9999, None).is_none());
    }

    #[test]
    fn test_is_nested_group_nonexistent_parent() {
        // Non-existent parent should return false
        assert!(!is_nested_group(9999, 802, None));
    }

    // ========================================================================
    // Program Trading Group Tests
    // ========================================================================

    #[test]
    fn test_list_ord_grp_new_order_list() {
        // ListOrdGrp (73) for NewOrderList (E)
        let config = get_group_config(73, Some("E")).expect("ListOrdGrp should exist for NewOrderList");
        assert_eq!(config.num_in_group_tag, 73);
        assert_eq!(config.delimiter_tag, 11); // ClOrdID
        assert!(config.member_tags.contains(&55)); // Symbol
        assert!(config.member_tags.contains(&38)); // OrderQty
        assert_eq!(config.nested_groups.len(), 2); // Parties + PreAllocGrp
    }

    #[test]
    fn test_list_ord_grp_list_status() {
        // ListOrdGrp (73) for ListStatus (N)
        let config = get_group_config(73, Some("N")).expect("ListOrdGrp should exist for ListStatus");
        assert_eq!(config.num_in_group_tag, 73);
        assert_eq!(config.delimiter_tag, 11); // ClOrdID
    }

    #[test]
    fn test_pre_alloc_grp() {
        // PreAllocGrp (78) - generic, not message-specific
        let config = get_group_config(78, None).expect("PreAllocGrp should exist");
        assert_eq!(config.num_in_group_tag, 78);
        assert_eq!(config.delimiter_tag, 79); // AllocAccount
        assert!(config.member_tags.contains(&80)); // AllocQty
        assert_eq!(config.nested_groups.len(), 1); // NestedParties2
        assert_eq!(config.nested_groups[0].num_in_group_tag, 756);
    }

    #[test]
    fn test_nested_parties2() {
        // NestedParties2 (756)
        let config = get_group_config(756, None).expect("NestedParties2 should exist");
        assert_eq!(config.num_in_group_tag, 756);
        assert_eq!(config.delimiter_tag, 757); // Nested2PartyID
        assert_eq!(config.nested_groups.len(), 1); // NstdPtys2SubGrp
        assert_eq!(config.nested_groups[0].num_in_group_tag, 806);
    }

    #[test]
    fn test_nstd_ptys2_sub_grp() {
        // NstdPtys2SubGrp (806)
        let config = get_group_config(806, None).expect("NstdPtys2SubGrp should exist");
        assert_eq!(config.num_in_group_tag, 806);
        assert_eq!(config.delimiter_tag, 760); // Nested2PartySubID
        assert_eq!(config.nested_groups.len(), 0); // No further nesting
    }

    #[test]
    fn test_triple_nesting_pre_alloc() {
        // Verify 3-level nesting: ListOrdGrp -> PreAllocGrp -> NestedParties2 -> NstdPtys2SubGrp
        // Level 1: ListOrdGrp contains PreAllocGrp
        assert!(is_nested_group(73, 78, Some("E")));
        // Level 2: PreAllocGrp contains NestedParties2
        assert!(is_nested_group(78, 756, None));
        // Level 3: NestedParties2 contains NstdPtys2SubGrp
        assert!(is_nested_group(756, 806, None));
    }

    #[test]
    fn test_bid_descr_req_grp() {
        // BidDescrReqGrp (398) for BidRequest (k)
        let config = get_group_config(398, Some("k")).expect("BidDescrReqGrp should exist");
        assert_eq!(config.num_in_group_tag, 398);
        assert_eq!(config.delimiter_tag, 399); // BidDescriptorType
        assert!(config.member_tags.contains(&400)); // BidDescriptor
        assert!(config.member_tags.contains(&406)); // FairValue
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_bid_comp_req_grp() {
        // BidCompReqGrp (420) for BidRequest (k)
        let config = get_group_config(420, Some("k")).expect("BidCompReqGrp should exist");
        assert_eq!(config.num_in_group_tag, 420);
        assert_eq!(config.delimiter_tag, 66); // ListID
        assert!(config.member_tags.contains(&54)); // Side
        assert!(config.member_tags.contains(&430)); // NetGrossInd
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_bid_comp_rsp_grp() {
        // BidCompRspGrp (420) for BidResponse (l)
        let config = get_group_config(420, Some("l")).expect("BidCompRspGrp should exist");
        assert_eq!(config.num_in_group_tag, 420);
        assert_eq!(config.delimiter_tag, 66); // ListID
        assert!(config.member_tags.contains(&12)); // Commission
        assert!(config.member_tags.contains(&44)); // Price
        assert!(config.member_tags.contains(&423)); // PriceType
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_bid_comp_message_specific() {
        // Tag 420 has different meanings for BidRequest (k) vs BidResponse (l)
        let req_config = get_group_config(420, Some("k")).expect("BidCompReqGrp should exist");
        let rsp_config = get_group_config(420, Some("l")).expect("BidCompRspGrp should exist");

        // Both have same tag and delimiter
        assert_eq!(req_config.num_in_group_tag, rsp_config.num_in_group_tag);
        assert_eq!(req_config.delimiter_tag, rsp_config.delimiter_tag);

        // But different member tags (request doesn't have commission, response does)
        assert!(!req_config.member_tags.contains(&12)); // Commission
        assert!(rsp_config.member_tags.contains(&12)); // Commission
    }

    #[test]
    fn test_strike_rules() {
        // StrikeRules (1201) for ListStrikePrice (m)
        let config = get_group_config(1201, Some("m")).expect("StrikeRules should exist");
        assert_eq!(config.num_in_group_tag, 1201);
        assert_eq!(config.delimiter_tag, 1223); // StrikeRuleID
        assert!(config.member_tags.contains(&1202)); // StartStrikePxRange
        assert!(config.member_tags.contains(&1204)); // StrikeIncrement
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_list_ord_grp_has_parties_nested() {
        // ListOrdGrp should have Parties (453) as nested group
        let nested = get_nested_groups(73, Some("E")).expect("ListOrdGrp should have nested groups");
        assert!(nested.iter().any(|n| n.num_in_group_tag == 453));
    }

    #[test]
    fn test_list_ord_grp_has_pre_alloc_nested() {
        // ListOrdGrp should have PreAllocGrp (78) as nested group
        let nested = get_nested_groups(73, Some("E")).expect("ListOrdGrp should have nested groups");
        assert!(nested.iter().any(|n| n.num_in_group_tag == 78));
    }

    // ========================================================================
    // Mass Order Group Tests
    // ========================================================================

    #[test]
    fn test_order_entry_grp_mass_order() {
        // OrderEntryGrp (2430) for MassOrder (DJ)
        let config = get_group_config(2430, Some("DJ")).expect("OrderEntryGrp should exist for MassOrder");
        assert_eq!(config.num_in_group_tag, 2430);
        assert_eq!(config.delimiter_tag, 11); // ClOrdID
        assert!(config.member_tags.contains(&55)); // Symbol
        assert!(config.member_tags.contains(&38)); // OrderQty
        assert!(config.member_tags.contains(&54)); // Side
        assert!(config.member_tags.contains(&40)); // OrdType
        assert_eq!(config.nested_groups.len(), 2); // Parties + PreAllocGrp
    }

    #[test]
    fn test_order_entry_ack_grp_mass_order_ack() {
        // OrderEntryAckGrp (2430) for MassOrderAck (DK)
        let config = get_group_config(2430, Some("DK")).expect("OrderEntryAckGrp should exist for MassOrderAck");
        assert_eq!(config.num_in_group_tag, 2430);
        assert_eq!(config.delimiter_tag, 11); // ClOrdID
        assert!(config.member_tags.contains(&37)); // OrderID
        assert!(config.member_tags.contains(&39)); // OrdStatus
        assert!(config.member_tags.contains(&103)); // OrdRejReason
        assert_eq!(config.nested_groups.len(), 0); // No nested groups
    }

    #[test]
    fn test_target_market_segment_grp() {
        // TargetMarketSegmentGrp (1793) for OrderMassActionRequest (CA)
        let config = get_group_config(1793, Some("CA")).expect("TargetMarketSegmentGrp should exist");
        assert_eq!(config.num_in_group_tag, 1793);
        assert_eq!(config.delimiter_tag, 1300); // MarketSegmentID
        assert!(config.member_tags.contains(&1300)); // MarketSegmentID
        assert!(config.member_tags.contains(&1301)); // MarketID
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_affected_market_segment_grp() {
        // AffectedMarketSegmentGrp (1793) for OrderMassActionReport (BZ)
        let config = get_group_config(1793, Some("BZ")).expect("AffectedMarketSegmentGrp should exist");
        assert_eq!(config.num_in_group_tag, 1793);
        assert_eq!(config.delimiter_tag, 1300); // MarketSegmentID
        assert!(config.member_tags.contains(&1300)); // MarketSegmentID
        assert!(config.member_tags.contains(&1301)); // MarketID
        assert!(config.member_tags.contains(&533)); // TotalAffectedOrders
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_not_affected_market_segment_grp() {
        // NotAffectedMarketSegmentGrp (1794) for OrderMassActionReport (BZ)
        let config = get_group_config(1794, Some("BZ")).expect("NotAffectedMarketSegmentGrp should exist");
        assert_eq!(config.num_in_group_tag, 1794);
        assert_eq!(config.delimiter_tag, 1300); // MarketSegmentID
        assert!(config.member_tags.contains(&1300)); // MarketSegmentID
        assert!(config.member_tags.contains(&1301)); // MarketID
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_order_entry_grp_has_parties_nested() {
        // OrderEntryGrp should have Parties (453) as nested group
        let nested = get_nested_groups(2430, Some("DJ")).expect("OrderEntryGrp should have nested groups");
        assert!(nested.iter().any(|n| n.num_in_group_tag == 453));
    }

    #[test]
    fn test_order_entry_grp_has_pre_alloc_nested() {
        // OrderEntryGrp should have PreAllocGrp (78) as nested group
        let nested = get_nested_groups(2430, Some("DJ")).expect("OrderEntryGrp should have nested groups");
        assert!(nested.iter().any(|n| n.num_in_group_tag == 78));
    }

    #[test]
    fn test_message_specific_tag_2430() {
        // Tag 2430 has different meanings for MassOrder (DJ) vs MassOrderAck (DK)
        let order_config = get_group_config(2430, Some("DJ")).expect("OrderEntryGrp should exist");
        let ack_config = get_group_config(2430, Some("DK")).expect("OrderEntryAckGrp should exist");

        // Both have same tag and delimiter
        assert_eq!(order_config.num_in_group_tag, ack_config.num_in_group_tag);
        assert_eq!(order_config.delimiter_tag, ack_config.delimiter_tag);

        // But different member tags (order has Symbol, ack has OrdStatus)
        assert!(order_config.member_tags.contains(&55)); // Symbol
        assert!(!ack_config.member_tags.contains(&55)); // Symbol
        assert!(!order_config.member_tags.contains(&39)); // OrdStatus
        assert!(ack_config.member_tags.contains(&39)); // OrdStatus

        // Different nested groups
        assert_eq!(order_config.nested_groups.len(), 2); // Parties + PreAllocGrp
        assert_eq!(ack_config.nested_groups.len(), 0); // No nested groups
    }

    // ========================================================================
    // Multileg Order Group Tests
    // ========================================================================

    #[test]
    fn test_leg_ord_grp_new_order_multileg() {
        // LegOrdGrp (555) for NewOrderMultileg (AB)
        let config = get_group_config(555, Some("AB")).expect("LegOrdGrp should exist");
        assert_eq!(config.num_in_group_tag, 555);
        assert_eq!(config.delimiter_tag, 600); // LegSymbol
        assert!(config.member_tags.contains(&600)); // LegSymbol
        assert!(config.member_tags.contains(&606)); // LegSecurityType
        assert!(config.member_tags.contains(&609)); // LegStrikePrice
        assert!(config.member_tags.contains(&624)); // LegSide
        assert!(config.member_tags.contains(&566)); // LegPrice
        assert_eq!(config.nested_groups.len(), 2); // LegStipulations + NestedPartyIDs
    }

    #[test]
    fn test_leg_ord_grp_multileg_cancel_replace() {
        // LegOrdGrp (555) for MultilegOrderCancelReplace (AC)
        let config = get_group_config(555, Some("AC")).expect("LegOrdGrp should exist");
        assert_eq!(config.num_in_group_tag, 555);
        assert_eq!(config.delimiter_tag, 600); // LegSymbol
    }

    #[test]
    fn test_leg_stipulations() {
        // LegStipulations (683) - generic, not message-specific
        let config = get_group_config(683, None).expect("LegStipulations should exist");
        assert_eq!(config.num_in_group_tag, 683);
        assert_eq!(config.delimiter_tag, 688); // LegStipulationType
        assert!(config.member_tags.contains(&688)); // LegStipulationType
        assert!(config.member_tags.contains(&689)); // LegStipulationValue
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_nested_party_ids() {
        // NestedPartyIDs (539)
        let config = get_group_config(539, None).expect("NestedPartyIDs should exist");
        assert_eq!(config.num_in_group_tag, 539);
        assert_eq!(config.delimiter_tag, 524); // NestedPartyID
        assert!(config.member_tags.contains(&524)); // NestedPartyID
        assert!(config.member_tags.contains(&525)); // NestedPartyIDSource
        assert!(config.member_tags.contains(&538)); // NestedPartyRole
        assert_eq!(config.nested_groups.len(), 1); // NstdPtysSubGrp
        assert_eq!(config.nested_groups[0].num_in_group_tag, 804);
    }

    #[test]
    fn test_nstd_ptys_sub_grp() {
        // NstdPtysSubGrp (804)
        let config = get_group_config(804, None).expect("NstdPtysSubGrp should exist");
        assert_eq!(config.num_in_group_tag, 804);
        assert_eq!(config.delimiter_tag, 545); // NestedPartySubID
        assert!(config.member_tags.contains(&545)); // NestedPartySubID
        assert!(config.member_tags.contains(&805)); // NestedPartySubIDType
        assert_eq!(config.nested_groups.len(), 0); // No further nesting
    }

    #[test]
    fn test_leg_ord_grp_has_leg_stipulations_nested() {
        // LegOrdGrp should have LegStipulations (683) as nested group
        let nested = get_nested_groups(555, Some("AB")).expect("LegOrdGrp should have nested groups");
        assert!(nested.iter().any(|n| n.num_in_group_tag == 683));
    }

    #[test]
    fn test_leg_ord_grp_has_nested_party_ids_nested() {
        // LegOrdGrp should have NestedPartyIDs (539) as nested group
        let nested = get_nested_groups(555, Some("AB")).expect("LegOrdGrp should have nested groups");
        assert!(nested.iter().any(|n| n.num_in_group_tag == 539));
    }

    #[test]
    fn test_triple_nesting_multileg() {
        // Verify 3-level nesting: LegOrdGrp -> NestedPartyIDs -> NstdPtysSubGrp
        // Level 1: LegOrdGrp contains NestedPartyIDs
        assert!(is_nested_group(555, 539, Some("AB")));
        // Level 2: NestedPartyIDs contains NstdPtysSubGrp
        assert!(is_nested_group(539, 804, None));
    }

    #[test]
    fn test_comp_id_req_grp_network_status_request() {
        // CompIDReqGrp for NetworkCounterpartySystemStatusRequest (BC)
        let config = get_group_config(936, Some("BC")).expect("CompIDReqGrp for BC should exist");
        assert_eq!(config.delimiter_tag, 930); // RefCompID
        assert_eq!(config.member_tags.len(), 4);
        assert!(config.member_tags.contains(&930)); // RefCompID
        assert!(config.member_tags.contains(&931)); // RefSubID
        assert!(config.member_tags.contains(&283)); // LocationID
        assert!(config.member_tags.contains(&284)); // DeskID
        assert_eq!(config.nested_groups.len(), 0);
    }

    #[test]
    fn test_comp_id_stat_grp_network_status_response() {
        // CompIDStatGrp for NetworkCounterpartySystemStatusResponse (BD)
        let config = get_group_config(936, Some("BD")).expect("CompIDStatGrp for BD should exist");
        assert_eq!(config.delimiter_tag, 930); // RefCompID
        assert_eq!(config.member_tags.len(), 6);
        assert!(config.member_tags.contains(&930)); // RefCompID
        assert!(config.member_tags.contains(&931)); // RefSubID
        assert!(config.member_tags.contains(&283)); // LocationID
        assert!(config.member_tags.contains(&284)); // DeskID
        assert!(config.member_tags.contains(&928)); // StatusValue
        assert!(config.member_tags.contains(&929)); // StatusText
        assert_eq!(config.nested_groups.len(), 0);
    }
}
