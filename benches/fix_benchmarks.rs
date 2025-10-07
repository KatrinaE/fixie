use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use fixie::{RawFixMessage, NewOrderSingle, Side, OrdType};
use chrono::Utc;

// Sample FIX messages of varying complexity
fn simple_fix_message() -> String {
    "8=FIXT.1.1|35=D|55=AAPL|54=1|".to_string()
}

fn typical_new_order() -> String {
    "8=FIXT.1.1|9=150|35=D|49=BUYER|56=SELLER|34=1|52=20251006-15:30:00.000|1128=9|11=ORD0001|55=AAPL|54=1|38=100|40=2|44=150.25|60=20250106-15:30:00.123|10=123|".to_string()
}

fn complex_execution_report() -> String {
    "8=FIXT.1.1|9=200|35=8|49=EXCHANGE|56=CLIENT|34=5|52=20251006-15:30:00.000|1128=9|37=12345|11=ORD0001|17=EXEC001|150=2|39=2|55=MSFT|54=2|151=0|14=100|6=150.50|32=100|31=150.50|60=20250106-15:30:00.123|58=Order filled|10=123|".to_string()
}

fn large_message_with_many_tags() -> String {
    let mut msg = "8=FIXT.1.1|35=D".to_string();
    // Add 50 custom tags
    for i in 5000..5050 {
        msg.push_str(&format!("|{}=VALUE{}", i, i));
    }
    msg.push('|');
    msg
}

// Benchmark: Parsing different message sizes
fn bench_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");

    let messages = vec![
        ("simple", simple_fix_message()),
        ("typical_order", typical_new_order()),
        ("complex_exec_report", complex_execution_report()),
        ("large_50_tags", large_message_with_many_tags()),
    ];

    for (name, msg) in messages {
        group.throughput(Throughput::Bytes(msg.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), &msg, |b, msg| {
            b.iter(|| {
                RawFixMessage::parse(black_box(msg)).unwrap()
            });
        });
    }

    group.finish();
}

// Benchmark: Encoding messages
fn bench_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding");

    // Pre-parse messages for encoding
    let simple = RawFixMessage::parse(&simple_fix_message()).unwrap();
    let typical = RawFixMessage::parse(&typical_new_order()).unwrap();
    let complex = RawFixMessage::parse(&complex_execution_report()).unwrap();
    let large = RawFixMessage::parse(&large_message_with_many_tags()).unwrap();

    group.bench_function("simple", |b| {
        b.iter(|| black_box(&simple).encode());
    });

    group.bench_function("typical_order", |b| {
        b.iter(|| black_box(&typical).encode());
    });

    group.bench_function("complex_exec_report", |b| {
        b.iter(|| black_box(&complex).encode());
    });

    group.bench_function("large_50_tags", |b| {
        b.iter(|| black_box(&large).encode());
    });

    group.finish();
}

// Benchmark: Round-trip (parse + encode)
fn bench_round_trip(c: &mut Criterion) {
    let mut group = c.benchmark_group("round_trip");

    let msg = typical_new_order();
    group.throughput(Throughput::Bytes(msg.len() as u64));

    group.bench_function("parse_encode", |b| {
        b.iter(|| {
            let parsed = RawFixMessage::parse(black_box(&msg)).unwrap();
            black_box(parsed.encode())
        });
    });

    group.finish();
}

// Benchmark: Field access patterns
fn bench_field_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("field_access");

    let msg = RawFixMessage::parse(&typical_new_order()).unwrap();

    group.bench_function("single_field", |b| {
        b.iter(|| {
            black_box(msg.get_field(55)) // Symbol
        });
    });

    group.bench_function("multiple_fields", |b| {
        b.iter(|| {
            black_box(msg.get_field(55)); // Symbol
            black_box(msg.get_field(54)); // Side
            black_box(msg.get_field(38)); // OrderQty
            black_box(msg.get_field(44)); // Price
            black_box(msg.get_field(40)); // OrdType
        });
    });

    group.bench_function("typed_conversion", |b| {
        b.iter(|| {
            msg.get_field_as::<f64>(black_box(44)) // Price as f64
        });
    });

    group.finish();
}

// Benchmark: Typed message conversion
fn bench_typed_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("typed_conversion");

    let raw = RawFixMessage::parse(&typical_new_order()).unwrap();

    group.bench_function("from_raw", |b| {
        b.iter(|| {
            NewOrderSingle::from_raw(black_box(&raw)).unwrap()
        });
    });

    let order = NewOrderSingle {
        cl_ord_id: "ORD0001".to_string(),
        symbol: "AAPL".to_string(),
        side: Side::Buy,
        transact_time: Utc::now(),
        ord_type: OrdType::Limit,
        order_qty: 100.0,
        price: Some(150.25),
    };

    group.bench_function("to_raw", |b| {
        b.iter(|| {
            black_box(&order).to_raw()
        });
    });

    group.finish();
}

// Benchmark: Delimiter detection
fn bench_delimiter_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("delimiter_detection");

    let pipe_msg = typical_new_order();
    let soh_msg = pipe_msg.replace('|', "\x01");

    group.bench_function("pipe_delimited", |b| {
        b.iter(|| {
            RawFixMessage::parse(black_box(&pipe_msg)).unwrap()
        });
    });

    group.bench_function("soh_delimited", |b| {
        b.iter(|| {
            RawFixMessage::parse(black_box(&soh_msg)).unwrap()
        });
    });

    group.finish();
}

// Benchmark: High-throughput scenario (parse many messages)
fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("throughput");

    let messages: Vec<String> = (0..100)
        .map(|i| {
            format!("8=FIXT.1.1|35=D|11=ORD{}|55=AAPL|54=1|38=100|", i)
        })
        .collect();

    let total_bytes: usize = messages.iter().map(|m| m.len()).sum();
    group.throughput(Throughput::Bytes(total_bytes as u64));

    group.bench_function("parse_100_messages", |b| {
        b.iter(|| {
            for msg in &messages {
                black_box(RawFixMessage::parse(msg).unwrap());
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_parsing,
    bench_encoding,
    bench_round_trip,
    bench_field_access,
    bench_typed_conversion,
    bench_delimiter_detection,
    bench_throughput,
);

criterion_main!(benches);
