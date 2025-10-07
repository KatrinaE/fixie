use ahash::AHashMap as HashMap;
use std::sync::LazyLock;

/// Configuration for a repeating group
#[derive(Debug, Clone)]
pub struct GroupConfig {
    /// The NoXXX tag that indicates the number of group entries
    pub num_in_group_tag: u32,
    /// The first tag of the repeating group (acts as delimiter)
    pub delimiter_tag: u32,
    /// All tags that belong to this repeating group (in order)
    pub member_tags: Vec<u32>,
}

/// Registry of all known repeating groups in FIX 5.0 SP2
/// Maps NoXXX tag -> GroupConfig
pub static GROUP_REGISTRY: LazyLock<HashMap<u32, GroupConfig>> = LazyLock::new(|| {
    let mut registry = HashMap::new();

    // Parties component (used in many messages)
    registry.insert(453, GroupConfig {
        num_in_group_tag: 453, // NoPartyIDs
        delimiter_tag: 448,    // PartyID
        member_tags: vec![
            448, // PartyID
            447, // PartyIDSource
            452, // PartyRole
            // Note: Nested PartySubIDsGrp (802) would go here in a full implementation
        ],
    });

    // SideCrossOrdModGrp (used in NewOrderCross, CrossOrderCancelReplaceRequest)
    registry.insert(552, GroupConfig {
        num_in_group_tag: 552, // NoSides
        delimiter_tag: 54,     // Side
        member_tags: vec![
            54,  // Side
            11,  // ClOrdID
            37,  // OrderID (optional)
            38,  // OrderQty
            528, // OrderCapacity
            // Many more optional fields would be here
        ],
    });

    // SideCrossOrdCxlGrp (used in CrossOrderCancelRequest)
    registry.insert(552, GroupConfig {
        num_in_group_tag: 552, // NoSides (reuses same tag)
        delimiter_tag: 54,     // Side
        member_tags: vec![
            54,  // Side
            11,  // ClOrdID
            37,  // OrderID (optional)
            41,  // OrigClOrdID
            // Additional optional fields
        ],
    });

    registry
});

/// Check if a tag is a NoXXX (number-in-group) tag
pub fn is_num_in_group_tag(tag: u32) -> bool {
    GROUP_REGISTRY.contains_key(&tag)
}

/// Get the delimiter tag for a given NoXXX tag
pub fn get_delimiter_tag(num_in_group_tag: u32) -> Option<u32> {
    GROUP_REGISTRY.get(&num_in_group_tag).map(|c| c.delimiter_tag)
}

/// Get all member tags for a given repeating group
pub fn get_member_tags(num_in_group_tag: u32) -> Option<&'static Vec<u32>> {
    GROUP_REGISTRY.get(&num_in_group_tag).map(|c| &c.member_tags)
}
