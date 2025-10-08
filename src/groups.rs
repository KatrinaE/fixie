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
}
