# Performance Optimization Guide for fixie

This document covers performance measurement and optimization strategies for the FIX parser.

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench -p fixie

# Run specific benchmark
cargo bench -p fixie --bench fix_benchmarks -- parsing

# Generate HTML report (saved to target/criterion/reports/index.html)
cargo bench -p fixie
open target/criterion/reports/index.html
```

## Benchmark Categories

### 1. **Parsing** - Measures parse speed for different message sizes
- Simple (4 fields)
- Typical order (14 fields)
- Complex execution report (20+ fields)
- Large message (50+ tags)

**Key Metric**: Throughput (MB/s)

### 2. **Encoding** - Measures message encoding speed
- Tests building wire format from parsed messages
- Includes checksum calculation

**Key Metric**: Time per operation (µs)

### 3. **Round-trip** - Parse → Encode cycle time
- Most realistic workload scenario

**Key Metric**: Time per operation (µs)

### 4. **Field Access** - HashMap lookup performance
- Single field access
- Multiple field access patterns
- Typed conversion overhead

**Key Metric**: Time per operation (ns)

### 5. **Typed Conversion** - Struct serialization/deserialization
- `from_raw()` - Parse to typed struct
- `to_raw()` - Struct to raw message

**Key Metric**: Time per operation (µs)

### 6. **Delimiter Detection** - Auto-detection overhead
- Pipe vs SOH delimiter

**Key Metric**: Time difference (%)

### 7. **Throughput** - Batch processing performance
- Parse 100 messages sequentially

**Key Metric**: Messages per second

## Current Performance Baseline

Run `cargo bench -p fixie` to establish your baseline. Example output:

```
parsing/simple          time:   [500 ns 520 ns 540 ns]
                        thrpt:  [45 MB/s 47 MB/s 49 MB/s]

parsing/typical_order   time:   [2.5 µs 2.6 µs 2.7 µs]
                        thrpt:  [55 MB/s 58 MB/s 60 MB/s]

encoding/typical_order  time:   [1.8 µs 1.9 µs 2.0 µs]

round_trip/parse_encode time:   [4.3 µs 4.5 µs 4.7 µs]
```

## Optimization Opportunities

### 🔥 High Impact (Likely 2-10x improvement)

#### 1. **Replace HashMap with Static Field Array**
**Current**: `HashMap<u32, String>` - heap allocation per field
**Better**: Pre-allocated array indexed by tag

```rust
// Current (slow)
pub struct RawFixMessage {
    pub fields: HashMap<u32, String>,
}

// Optimized (faster)
pub struct RawFixMessage {
    // Common tags in array for O(1) access
    common_fields: [Option<String>; 256],
    // Rare/custom tags in HashMap
    custom_fields: Option<HashMap<u32, String>>,
}
```

**Expected Improvement**: 3-5x faster field access
**Complexity**: Medium
**Benchmark**: `field_access` group

#### 2. **Use `Cow<str>` or String Interning**
**Current**: Clone strings for every field
**Better**: Borrow when possible, intern common values

```rust
use std::borrow::Cow;

pub struct RawFixMessage {
    pub fields: HashMap<u32, Cow<'static, str>>,
}

// Intern common values
lazy_static! {
    static ref COMMON_VALUES: HashMap<&'static str, &'static str> = {
        // "1", "2", "D", "8", "A", "Buy", "Sell", etc.
    };
}
```

**Expected Improvement**: 2-3x less memory allocations
**Complexity**: High
**Benchmark**: `parsing` group

#### 3. **Avoid String Allocation in Parsing**
**Current**: `value.to_string()` for every field
**Better**: Parse into `&str`, only allocate when needed

```rust
// Current
let value = parts[1].to_string();
fields.insert(tag, value);

// Optimized
let value = parts[1]; // &str, no allocation
fields.insert(tag, value.to_string()); // Only allocate once
```

**Expected Improvement**: 30-50% faster parsing
**Complexity**: Low
**Benchmark**: `parsing` group

### 🚀 Medium Impact (Likely 20-50% improvement)

#### 4. **Pre-compile Delimiter Detection**
**Current**: Auto-detect delimiter on every parse
**Better**: Provide explicit parse method variants

```rust
impl RawFixMessage {
    pub fn parse_soh(input: &str) -> Result<Self> { ... }
    pub fn parse_pipe(input: &str) -> Result<Self> { ... }
    pub fn parse_auto(input: &str) -> Result<Self> { ... }
}
```

**Expected Improvement**: 10-20% faster parsing
**Complexity**: Low
**Benchmark**: `delimiter_detection` group

#### 5. **Optimize Encoding with StringBuilder**
**Current**: Multiple string concatenations
**Better**: Pre-calculate size, single allocation

```rust
// Calculate exact size needed
let total_size = self.fields.iter()
    .map(|(tag, val)| tag.to_string().len() + val.len() + 2)
    .sum();

let mut result = String::with_capacity(total_size);
// Build in one pass
```

**Expected Improvement**: 30-40% faster encoding
**Complexity**: Low
**Benchmark**: `encoding` group

#### 6. **Cache Sorted Tags for Encoding**
**Current**: Sort tags on every encode
**Better**: Maintain sorted order or cache

```rust
pub struct RawFixMessage {
    fields: HashMap<u32, String>,
    sorted_tags: Vec<u32>, // Cached, invalidate on modification
}
```

**Expected Improvement**: 20-30% faster encoding
**Complexity**: Medium
**Benchmark**: `encoding` group

### ⚡ Low Impact (Likely 5-15% improvement)

#### 7. **Use `ahash` Instead of Default HashMap**
**Current**: `std::collections::HashMap` (SipHash)
**Better**: `ahash::AHashMap` (faster, non-cryptographic)

```toml
[dependencies]
ahash = "0.8"
```

```rust
use ahash::AHashMap;

pub struct RawFixMessage {
    pub fields: AHashMap<u32, String>,
}
```

**Expected Improvement**: 10-15% faster
**Complexity**: Very Low (change one line)
**Benchmark**: All groups

#### 8. **Avoid Redundant Allocations in encode()**
**Current**: Multiple Vec allocations
**Better**: Reuse allocations

```rust
// Reuse tag vector allocation
impl RawFixMessage {
    fn encode_with_buffer(&self, tag_buffer: &mut Vec<u32>) -> String {
        tag_buffer.clear();
        tag_buffer.extend(self.fields.keys());
        tag_buffer.sort_unstable();
        // ... use tag_buffer
    }
}
```

**Expected Improvement**: 5-10% faster encoding
**Complexity**: Medium
**Benchmark**: `encoding` group

#### 9. **Use Fast Integer Parsing**
**Current**: `str::parse::<u32>()`
**Better**: `atoi` or `fast_parse` crate

```rust
fn fast_parse_tag(s: &str) -> Option<u32> {
    // Hand-optimized for small integers (most tags are < 1000)
    let bytes = s.as_bytes();
    if bytes.is_empty() || bytes.len() > 5 {
        return None;
    }

    let mut result = 0u32;
    for &b in bytes {
        if b < b'0' || b > b'9' {
            return None;
        }
        result = result * 10 + (b - b'0') as u32;
    }
    Some(result)
}
```

**Expected Improvement**: 5-10% faster parsing
**Complexity**: Low
**Benchmark**: `parsing` group

## Zero-Copy Parsing (Advanced)

For ultra-high performance, consider a zero-copy design:

```rust
pub struct RawFixMessageRef<'a> {
    input: &'a str,
    fields: HashMap<u32, &'a str>, // No String allocation!
}

impl<'a> RawFixMessageRef<'a> {
    pub fn parse(input: &'a str) -> Result<Self> {
        // Parse into references, no allocations
    }
}
```

**Expected Improvement**: 5-10x faster for read-only use cases
**Complexity**: Very High (lifetime management)
**Trade-off**: Can't modify or outlive input string

## Profiling Real Workloads

Use `perf` or `cargo flamegraph` for CPU profiling:

```bash
# Install flamegraph
cargo install flamegraph

# Profile benchmarks
cargo flamegraph --bench fix_benchmarks -- --bench

# View flamegraph.svg
open flamegraph.svg
```

## Comparison Benchmarks

To compare before/after optimizations:

```bash
# Save baseline
cargo bench -p fixie -- --save-baseline before

# Make changes...

# Compare against baseline
cargo bench -p fixie -- --baseline before
```

## Real-World Performance Targets

### High-Frequency Trading (Latency-Critical)
- **Target**: < 1 µs parse time for typical messages
- **Focus**: Zero-copy, minimize allocations
- **Trade-off**: Complexity vs speed

### Market Data Feed (Throughput-Critical)
- **Target**: > 100K messages/second
- **Focus**: Batch processing, amortize overhead
- **Trade-off**: Latency vs throughput

### General Trading Application (Balanced)
- **Target**: < 10 µs round-trip for typical messages
- **Focus**: Reasonable defaults, predictable performance
- **Trade-off**: Simplicity vs optimization

## Memory Profiling

Use `heaptrack` or `valgrind` for memory profiling:

```bash
# Install heaptrack
sudo apt install heaptrack

# Profile
heaptrack cargo bench -p fixie

# Analyze
heaptrack_gui heaptrack.*.gz
```

## Recommended Optimization Order

1. ✅ **Benchmark first** - Establish baseline
2. 🔥 **Profile hotspots** - Find actual bottlenecks
3. 🎯 **Pick one optimization** - Don't optimize blindly
4. ⚡ **Implement** - Make the change
5. 📊 **Measure** - Re-run benchmarks
6. ✔️ **Verify** - Ensure correctness still holds
7. 🔁 **Repeat** - Move to next optimization

**Golden Rule**: Measure, don't guess!

## Example: Measuring a Specific Optimization

```bash
# 1. Baseline
cargo bench -p fixie -- --save-baseline before

# 2. Make change (e.g., switch to AHashMap)
# Edit Cargo.toml and src/parser.rs

# 3. Measure improvement
cargo bench -p fixie -- --baseline before

# Output shows change:
# parsing/typical_order
#   time:   [2.1 µs 2.2 µs 2.3 µs]
#   change: [-15.2% -12.8% -10.5%] (improvement)
```

## Further Reading

- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion.rs Documentation](https://bheisler.github.io/criterion.rs/book/)
- [FIX Engine Performance Patterns](https://www.fixtrading.org/standards/)
