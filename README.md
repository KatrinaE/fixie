# Fixie

![Red fixie bicycle](img/fixie_bicycle.png "Fixie Bicycle")

Fixie is a Rust library and CLI tool for working with
FIX ([Financial Information eXchange](https://www.fixtrading.org/online-specification/) protocol messages.

It is in **pre-release** stage. It is not production ready.

Currently, Fixie supports [infrastructure](https://www.fixtrading.org/online-specification/business-area-infrastructure/),
[pre-trade](https://www.fixtrading.org/online-specification/business-area-pretrade/),
and [trade](https://www.fixtrading.org/online-specification/business-area-trade/)
messages. It does not yet support [post-trade](https://www.fixtrading.org/online-specification/business-area-posttrade/)  messages.

## Usage

### Command Line Tool

#### Installation
You will need Rust and cargo installed on your machine.

```bash
cargo build --release -p fixie
```
This will put the fixie binary in _./target/release_.

#### Usage
Fixie can display FIX messages in three different formats: JSON (default), pretty-printed, and raw.

```bash
fixie <message>         # Display FIX message as JSON
fixie --pp <message>    # Display FIX message pretty-printed
fixie --raw <message>   # Display FIX message as raw values
```

#### Example Input: JSON

```bash
fixie '8=FIXT.1.1|9=178|35=D|49=TRADER1|56=MARKET1|34=1|52=20251006-15:00:00.000|11=ORD00123|21=1|55=MSFT|54=1|38=500|40=2|44=310.75|59=0|60=20251006-15:00:00.000|453=2|448=TRADER1|447=D|452=1|448=DESK22|447=D|452=24|9001=TRUE|9435=ALGOTYPE1|9436=VWAP|10=156|'
```

#### Example Output: JSON

```bash
{
  "BeginString": "FIXT.1.1",
  "BodyLength": "178",
  "CheckSum": "156",
  "ClOrdID": "ORD00123",
  "MsgSeqNum": "1",
  "MsgType": "D",
  "NoPartyIDs": [
    {
      "PartyID": "TRADER1",
      "PartyIDSource": "D",
      "PartyRole": "1"
    },
    {
      "PartyID": "DESK22",
      "PartyIDSource": "D",
      "PartyRole": "24"
    }
  ],
  "OrdType": "2",
  "OrderQty": "500",
  "Price": "310.75",
  "SenderCompID": "TRADER1",
  "SendingTime": "20251006-15:00:00.000",
  "Side": "1",
  "Symbol": "MSFT",
  "TargetCompID": "MARKET1",
  "TransactTime": "20251006-15:00:00.000",
  "Unknown": "0"
}
```

##### Example Input: --pp (Pretty Print)

```bash
fixie --pp '8=FIXT.1.1|9=178|35=D|49=TRADER1|56=MARKET1|34=1|52=20251006-15:00:00.000|11=ORD00123|21=1|55=MSFT|54=1|38=500|40=2|44=310.75|59=0|60=20251006-15:00:00.000|453=2|448=TRADER1|447=D|452=1|448=DESK22|447=D|452=24|9001=TRUE|9435=ALGOTYPE1|9436=VWAP|10=156|'
```
Note the quotes (') around the FIX message; they're necessary to prevent your terminal from
interpreting the pipes (|).

##### Example Output: --pp (Pretty Print)

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

#### Example Input: --raw

```bash
fixie --raw '8=FIXT.1.1|9=178|35=D|49=TRADER1|56=MARKET1|34=1|52=20251006-15:00:00.000|11=ORD00123|21=1|55=MSFT|54=1|38=500|40=2|44=310.75|59=0|60=20251006-15:00:00.000|453=2|448=TRADER1|447=D|452=1|448=DESK22|447=D|452=24|9001=TRUE|9435=ALGOTYPE1|9436=VWAP|10=156|'
```

#### Example Output: --raw

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

### Infrastructure & Session Layer
- Logon (A)
- Logout (5)
- Heartbeat (0)

### Business Message Rejects
- BusinessMessageReject (j)

### Confirmations
- DontKnowTrade (Q)

### Single General Order Handling
- NewOrderSingle (D)
- ExecutionReport (8)
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
- MarketDataRequestReject (Y)

### Program Trading
- NewOrderList (E)
- ListStatus (N)

### Mass Order Messages
- MassOrder (DJ)
- MassOrderAck (DK)
- OrderMassActionRequest (CA)
- OrderMassActionReport (BZ)
- OrderMassStatusRequest (AF)

### Multileg Order Messages
- NewOrderMultileg (AB)
- MultilegOrderCancelReplace (AC)

### Application Sequencing
- ApplicationMessageRequest (BW)
- ApplicationMessageRequestAck (BX)
- ApplicationMessageReport (BY)

### Network Status
- NetworkCounterpartySystemStatusRequest (BC)
- NetworkCounterpartySystemStatusResponse (BD)

### User Management
- UserRequest (BE)
- UserResponse (BF)
- UserNotification (CB)

## Features

### Nested Repeating Groups
Fixie supports multi-level nested repeating groups with up to 4 levels of nesting:
- **2-level**: NewOrderList → ListOrdGrp → Parties
- **3-level**: ListOrdGrp → Parties → PartySubIDsGrp
- **4-level**: ListOrdGrp → PreAllocGrp → NestedParties2 → NstdPtys2SubGrp

Nested groups use an index-based arena structure for efficient memory layout and cache performance.

## Bugs

Bug? [Submit an issue](https://github.com/KatrinaE/fixie/issues).
