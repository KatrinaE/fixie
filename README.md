# Fixie

![Red fixie bicycle](img/fixie_bicycle.png "Fixie Bicycle")

A CLI tool and Rust library for parsing and working with
FIX ([Financial Information eXchange](https://www.fixtrading.org/online-specification/) protocol messages.

## Usage

### Command Line Tool

#### Installation
```bash
cargo build --release -p fixie
```
This will put the fixie binary in _./target/release_.

#### Usage
```bash
fixie <message>
```

##### Example Input

```bash
fixie '8=FIXT.1.1|9=178|35=D|49=TRADER1|56=MARKET1|34=1|52=20251006-15:00:00.000|11=ORD00123|21=1|55=MSFT|54=1|38=500|40=2|44=310.75|59=0|60=20251006-15:00:00.000|453=2|448=TRADER1|447=D|452=1|448=DESK22|447=D|452=24|9001=TRUE|9435=ALGOTYPE1|9436=VWAP|10=156|'
```
Note the quotes (') around the FIX message; they're necessary to prevent your terminal from
interpreting the pipes (|).

##### Example Output

```
╔════════════════════════════════════════════════════════╗
║  FIX Message: NewOrderSingle  
╚════════════════════════════════════════════════════════╝

Standard Header:
────────────────
     8 (BeginString         ): FIXT.1.1
     9 (BodyLength          ): 178
    34 (MsgSeqNum           ): 1
    35 (MsgType             ): D (NewOrderSingle)
    49 (SenderCompID        ): TRADER1
    52 (SendingTime         ): 20251006-15:00:00.000
    56 (TargetCompID        ): MARKET1

Message Body:
─────────────
    11 (ClOrdID             ): ORD00123
    21 (Unknown             ): 1
    38 (OrderQty            ): 500
    40 (OrdType             ): 2 (Limit)
    44 (Price               ): 310.75
    54 (Side                ): 1 (Buy)
    55 (Symbol              ): MSFT
    59 (Unknown             ): 0
    60 (TransactTime        ): 20251006-15:00:00.000
   447 (Unknown             ): D
   448 (Unknown             ): DESK22
   452 (Unknown             ): 24
   453 (Unknown             ): 2
  9001 (Unknown             ): TRUE
  9435 (Unknown             ): ALGOTYPE1
  9436 (Unknown             ): VWAP

Trailer:
────────
    10 (CheckSum            ): 156
```

#### --raw Flag Usage
Use the `--raw` flag to display raw tag-value pairs, one per line:

```bash
fixie --raw '8=FIXT.1.1|9=178|35=D|49=TRADER1|56=MARKET1|34=1|52=20251006-15:00:00.000|11=ORD00123|21=1|55=MSFT|54=1|38=500|40=2|44=310.75|59=0|60=20251006-15:00:00.000|453=2|448=TRADER1|447=D|452=1|448=DESK22|447=D|452=24|9001=TRUE|9435=ALGOTYPE1|9436=VWAP|10=156|'
```

#### --raw Flag Output

```bash
Raw FIX Message:
8=FIXT.1.1
9=178
10=156
11=ORD00123
21=1
34=1
35=D
38=500
40=2
44=310.75
49=TRADER1
52=20251006-15:00:00.000
54=1
55=MSFT
56=MARKET1
59=0
60=20251006-15:00:00.000
447=D
448=DESK22
452=24
453=2
9001=TRUE
9435=ALGOTYPE1
9436=VWAP
```

### Rust Library

#### Setup 
Add to your `Cargo.toml`:

```toml
[dependencies]
fixie = { path = "../fixie" }
```

#### Parsing

```rust
use fixie::{RawFixMessage, NewOrderSingle};

// Create an example message to parse
let fix_msg = "8=FIXT.1.1\x019=100\x0135=D\x01...";
let raw = RawFixMessage::parse(fix_msg)?;

// Convert to typed message
let order = NewOrderSingle::from_raw(&raw)?;
println!("Symbol: {}, Side: {:?}", order.symbol, order.side);
```

#### Encoding

```rust
use fixie::{RawFixMessage, NewOrderSingle, Side, OrdType};
use chrono::Utc;

let order = NewOrderSingle {
    cl_ord_id: "ORDER123".to_string(),
    symbol: "EURUSD".to_string(),
    side: Side::Buy,
    transact_time: Utc::now(),
    ord_type: OrdType::Limit,
    order_qty: 100.0,
    price: Some(1.25),
};

let raw = order.to_raw();
let wire_format = raw.encode();
```

## Supported FIX Versions

- FIXT 1.1 (session layer)
- FIX 5.0 SP2 (application layer)

## Supported Message Types

### Session Layer
- Logon (A)
- Logout (5)
- Heartbeat (0)

### Single General Order Handling
- NewOrderSingle (D)
- ExecutionReport (8)
- DontKnowTrade (Q)
- OrderCancelRequest (F)
- OrderCancelReject (9)
- OrderCancelReplaceRequest (G)
- OrderStatusRequest (H)
- ExecutionAcknowledgement (BN)

### Order Mass Handling
- OrderMassCancelRequest (q)
- OrderMassCancelReport (r)

### Cross Order Handling
- NewOrderCross (s)
- CrossOrderCancelRequest (u)
- CrossOrderCancelReplaceRequest (t)

### Market Data
- MarketDataRequest (V)
- MarketDataSnapshotFullRefresh (W)

## Bugs

Bug? [Submit an issue](https://github.com/KatrinaE/fixie/issues).
