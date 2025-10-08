# Nested Groups Implementation: Box vs Index Approach

## Current Implementation: Box-Based

We use `Box<GroupEntry>` for recursive nested group structures.

```rust
pub struct GroupEntry {
    pub fields: HashMap<u32, String>,
    pub nested_groups: HashMap<u32, Vec<Box<GroupEntry>>>,
}
```

## Migration Difficulty Assessment

**Switching from Box to Index: EASY (1-2 days)**

The migration is straightforward because:
1. **Encapsulation** - GroupEntry is only used internally in parser/encoder
2. **No public API exposure** - External code doesn't directly manipulate GroupEntry
3. **Localized changes** - Only parser.rs and potentially encoder logic affected

### What Would Change

```rust
// Index-based approach
pub struct GroupEntry {
    pub fields: HashMap<u32, String>,
    pub nested_groups: HashMap<u32, Vec<GroupEntryId>>, // Just IDs
}

pub type GroupEntryId = usize;

pub struct RawFixMessage {
    pub fields: HashMap<u32, String>,
    pub groups: HashMap<u32, Vec<GroupEntryId>>,
    pub group_arena: Vec<GroupEntry>, // NEW: flat storage
}
```

### Files That Would Need Changes

1. **src/parser.rs** - Parser would write to arena, return indices
2. **src/parser.rs** - Encoder would traverse via indices
3. **Accessor methods** - Need arena parameter: `get_nested_group(&arena, id)`

### What Would NOT Change

- `groups.rs` - GroupConfig stays the same
- `messages.rs` - Message structs unchanged
- Public API - `RawFixMessage::parse()` signature unchanged
- Tests - Same behavior, just internal representation differs

## Approach Comparison

### Box-Based (Current Choice)

**Pros:**
- ✅ Natural tree structure mirrors FIX hierarchy
- ✅ Simple navigation: `entry.nested_groups[&453][0]`
- ✅ Self-contained ownership model
- ✅ Type-safe at compile time
- ✅ Easy serialization/deserialization
- ✅ Matches wire format conceptually
- ✅ Cleaner code, fewer bugs
- ✅ Standard Rust pattern for recursive types

**Cons:**
- ❌ Heap allocation per nested group entry (~16 bytes overhead per Box)
- ❌ Pointer indirection (minor CPU cost)
- ❌ Potential memory fragmentation
- ❌ Not cache-friendly for deep traversal

**Performance Characteristics:**
- Parse time: O(n) where n = total fields
- Memory: O(n) + Box overhead (~16 bytes × number of nested entries)
- Traversal: O(1) field access + pointer dereference

**Best For:**
- Typical workloads (messages < 100KB)
- Code maintainability priority
- Rapid development
- When parse time is not critical

### Index-Based Alternative

**Pros:**
- ✅ Cache-friendly linear memory layout
- ✅ No per-entry heap allocations
- ✅ Better CPU cache utilization
- ✅ Predictable memory layout
- ✅ Easier to implement custom allocators
- ✅ Potentially 10-30% faster for very large messages

**Cons:**
- ❌ More complex code (+100-200 lines)
- ❌ Arena must be passed around
- ❌ Easy to create index bugs
- ❌ Less intuitive API
- ❌ Harder to serialize
- ❌ Manual lifetime management
- ❌ Harder to debug (indices vs direct pointers)

**Performance Characteristics:**
- Parse time: O(n), potentially 10-30% faster for large messages
- Memory: O(n), more compact (no Box overhead)
- Traversal: O(1) array lookup, better cache locality

**Best For:**
- Very large messages (>1MB, 1000+ orders)
- Performance-critical parsing
- High-frequency trading scenarios
- When profiling shows parse time is bottleneck

## Example Code Comparison

### Box Approach (Current)

```rust
// Parsing nested group
let mut entry = GroupEntry {
    fields: HashMap::new(),
    nested_groups: HashMap::new(),
};

// Parse nested Parties group
if let Some(parties_config) = get_group_config(453) {
    let mut parties_entries = Vec::new();
    for party_fields in parse_party_fields() {
        let party_entry = Box::new(GroupEntry {
            fields: party_fields,
            nested_groups: HashMap::new(),
        });
        parties_entries.push(party_entry);
    }
    entry.nested_groups.insert(453, parties_entries);
}

// Accessing nested data
if let Some(parties) = entry.nested_groups.get(&453) {
    for party in parties {
        let party_id = party.fields.get(&448);
        // Simple, direct access
    }
}
```

### Index Approach (Alternative)

```rust
// Parsing nested group
let mut arena = Vec::new();
let entry_id = arena.len();
arena.push(GroupEntry {
    fields: HashMap::new(),
    nested_groups: HashMap::new(),
});

// Parse nested Parties group
if let Some(parties_config) = get_group_config(453) {
    let mut party_ids = Vec::new();
    for party_fields in parse_party_fields() {
        let party_id = arena.len();
        arena.push(GroupEntry {
            fields: party_fields,
            nested_groups: HashMap::new(),
        });
        party_ids.push(party_id);
    }
    arena[entry_id].nested_groups.insert(453, party_ids);
}

// Accessing nested data (more verbose)
if let Some(party_ids) = arena[entry_id].nested_groups.get(&453) {
    for &party_id in party_ids {
        let party = &arena[party_id];
        let party_id_field = party.fields.get(&448);
        // Need arena reference throughout
    }
}
```

## Migration Path

If benchmarking shows Box approach is too slow:

### Step 1: Add Arena Field (2 hours)
```rust
pub struct RawFixMessage {
    pub fields: HashMap<u32, String>,
    pub groups: HashMap<u32, Vec<Box<GroupEntry>>>, // Keep for now
    group_arena: Vec<GroupEntry>, // NEW: hidden field
}
```

### Step 2: Implement Arena Parser (4 hours)
Create parallel `parse_with_arena()` function that populates arena instead of Boxes.

### Step 3: Update Accessors (2 hours)
Add arena-based accessors while keeping Box accessors.

### Step 4: Benchmark (1 hour)
Compare both approaches on real workloads.

### Step 5: Switch Over (4 hours)
If arena is faster, replace Box fields with indices, remove old code.

### Step 6: Update Tests (2 hours)
Verify all tests pass with new implementation.

**Total Migration Time: 1-2 days**

## Performance Benchmarking Plan

When to consider switching to index approach:

1. **Measure parse time** for representative messages:
   - Small: 10 orders, 2 levels deep
   - Medium: 100 orders, 2 levels deep
   - Large: 1000 orders, 3 levels deep

2. **Profile memory allocation**:
   - Count Box allocations
   - Measure heap fragmentation
   - Check allocation hot spots

3. **Decision criteria**:
   - If parsing takes >10ms for typical messages → Consider index approach
   - If Box allocations are >30% of parse time → Consider index approach
   - If memory fragmentation is measurable issue → Consider index approach
   - Otherwise → Keep Box approach for maintainability

## Recommendation

**Start with Box approach** because:

1. **Premature optimization** - No evidence parsing is bottleneck
2. **Development speed** - Box approach is 2-3x faster to implement correctly
3. **Maintainability** - Simpler code means fewer bugs
4. **Easy migration** - Can switch later if profiling justifies it
5. **FIX message scale** - Even 1000-order lists are ~1-2MB, well within Box performance range

**Only migrate to index approach if:**
- Profiling shows parse time is >10% of total latency AND
- Large messages (>1000 orders) are common AND
- Optimization effort is justified by business requirements

## Related Files

- `src/parser.rs` - Contains GroupEntry and parsing logic
- `src/groups.rs` - GroupConfig (unchanged by this choice)
- `docs/NESTED_GROUPS.md` - User-facing documentation
- `docs/PROGRAM_TRADING_DESIGN.md` - Overall implementation plan

## References

- [Rust RFC: Recursive Types](https://rust-lang.github.io/rfcs/2000-const-generics.html)
- [Performance of Box vs Arena](https://without.boats/blog/two-memory-bugs-from-ringbahn/)
- FIX Protocol typical message sizes: 500 bytes - 50KB for program trading
