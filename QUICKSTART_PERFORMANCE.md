# Performance Quick Start

## TL;DR - How to Measure Performance

```bash
# Run all benchmarks (takes ~5-10 minutes)
cargo bench -p fixie

# View HTML report with graphs
open target/criterion/reports/index.html

# Run just one benchmark group
cargo bench -p fixie -- parsing

# Quick sanity check (faster, less accurate)
cargo bench -p fixie --bench fix_benchmarks -- --quick
```

## What Gets Measured

1. **Parsing Speed** - How fast can we parse FIX messages?
2. **Encoding Speed** - How fast can we encode to wire format?
3. **Field Access** - How fast are HashMap lookups?
4. **Throughput** - How many messages per second?

## Top 3 Optimization Opportunities (Easy Wins)

### 1. Switch to `ahash` (5 minutes, ~10-15% faster)

**Add to Cargo.toml:**
```toml
[dependencies]
ahash = "0.8"
```

**Change in src/parser.rs:**
```rust
// Before
use std::collections::HashMap;

// After
use ahash::AHashMap as HashMap;
```

**Expected**: 10-15% faster across the board

### 2. Pre-allocate String capacity in encode() (10 minutes, ~30% faster)

**In src/parser.rs, `encode()` method:**
```rust
pub fn encode(&self) -> String {
    // Calculate approximate size
    let approx_size = self.fields.iter()
        .map(|(tag, val)| 6 + val.len()) // tag + "=" + SOH + value
        .sum::<usize>() + 50; // header/trailer overhead

    let mut message = String::with_capacity(approx_size);
    // ... rest of encode logic
}
```

**Expected**: 30% faster encoding

### 3. Cache delimiter detection (5 minutes, ~10-20% faster)

**Add parse variants:**
```rust
impl RawFixMessage {
    // Fast path when you know the delimiter
    pub fn parse_pipe(input: &str) -> Result<Self, FixParseError> {
        Self::parse_with_delimiter(input, '|')
    }

    pub fn parse_soh(input: &str) -> Result<Self, FixParseError> {
        Self::parse_with_delimiter(input, SOH)
    }

    fn parse_with_delimiter(input: &str, delimiter: char) -> Result<Self, FixParseError> {
        // ... parsing logic with known delimiter
    }
}
```

**Expected**: 10-20% faster when delimiter is known

## Comparing Before/After

```bash
# Save baseline
cargo bench -p fixie -- --save-baseline before

# Make your optimization changes...

# Compare
cargo bench -p fixie -- --baseline before
```

Output shows improvement:
```
parsing/typical_order
  time:   [2.1 µs 2.2 µs 2.3 µs]
  change: [-15.2% -12.8% -10.5%] (improvement)  <-- This means faster!
```

## Performance Targets by Use Case

| Use Case | Target | Optimization Level |
|----------|--------|-------------------|
| General trading app | < 10 µs/msg | Default is fine |
| Market data feed | > 10K msgs/sec | Apply easy wins (ahash, etc) |
| HFT / Low latency | < 1 µs/msg | Advanced (zero-copy, etc) |

## When to Optimize

**Optimize when**:
- Processing > 10K messages/second
- Latency is critical (< 1ms requirement)
- Profiling shows parser is bottleneck

**Don't optimize when**:
- Handling < 100 messages/second
- Network latency dominates
- Code clarity matters more

## Quick Profiling

```bash
# CPU profiling with flamegraph
cargo install flamegraph
cargo flamegraph --bench fix_benchmarks -- --bench

# Memory profiling
cargo install cargo-bloat
cargo bloat --release -p fixie
```

## See Full Guide

For detailed optimization strategies, see [PERFORMANCE.md](PERFORMANCE.md)
