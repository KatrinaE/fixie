# Fixie

![Red fixie bicycle](img/fixie_bicycle.png "Fixie Bicycle")

Fixie is a Rust library and CLI tool for working with
FIX ([Financial Information eXchange](https://www.fixtrading.org/online-specification/)) protocol messages.

It is in **pre-release** stage. It is not production ready.

Currently, Fixie supports FIX 5.0 [infrastructure](https://www.fixtrading.org/online-specification/business-area-infrastructure/),
[pre-trade](https://www.fixtrading.org/online-specification/business-area-pretrade/),
and [trade](https://www.fixtrading.org/online-specification/business-area-trade/)
messages. It does not yet support [post-trade](https://www.fixtrading.org/online-specification/business-area-posttrade/)  messages.

## Usage

### Command Line Tool

#### Installation
You will need Rust and cargo installed on your machine.

1. Get the code
```bash
git clone https://github.com/KatrinaE/fixie.git
```

2. Build the binary
```bash
cargo build --release -p fixie
```
This will put the fixie binary in _./target/release_.

#### Usage
Fixie can display FIX messages in three different formats: JSON (default), pretty-printed, and raw.

```bash
fixie '<message>'         # Display FIX message as JSON
fixie --pp '<message>'    # Display FIX message pretty-printed
fixie --raw '<message>'   # Display FIX message as raw values
```

In all three formats, you will need quotes (') around the FIX message 
(they're necessary to prevent your terminal from interpreting the pipes (|)).

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
1. Get the code
```bash
git clone https://github.com/KatrinaE/fixie.git
```

2. Add to your `Cargo.toml`:

```toml
[dependencies]
fixie = { path = "../fixie" }
```

#### Data Types
Each FIX message type has a corresponding data type in _src/messages.rs_ (for example, NewOrderSingle, ExecutionReport, Logon, etc.).
The field names for each message are defined on that message's struct in _src/message_types_. This directory is organized according to
the categories in the FIX spec. For example, NewOrderSingle is in _src/message_types/single_general_order_handling.rs_ because
NewOrderSingle is in the Single General Order Handling category in the [FIX spec](https://www.fixtrading.org/online-specification/business-area-trade/).

#### Parsing Example

```rust
use fixie::{NewOrderSingle, RawFixMessage};

pub fn main() {
    // Create an example message to parse
    // This message uses \x01 (SOH) as a delimiter, but you can also use |
    let fix_msg = "8=FIXT.1.1\x019=178\x0135=D\x0149=TRADER1\x0156=MARKET1\x0134=1\x0152=20251006-15:00:00.000\x0111=ORD00123\x0121=1\x0155=MSFT\x0154=1\x0138=500\x0140=2\x0144=310.75\x0159=0\x0160=20251006-15:00:00.000\x01453=2\x01448=TRADER1\x01447=D\x01452=1\x01448=DESK22\x01447=D\x01452=24\x019001=TRUE\x019435=ALGOTYPE1\x019436=VWAP\x0110=156\x01";

    // Parse into raw data format
    let raw = RawFixMessage::parse(fix_msg).expect("Failed to parse raw message");

    // Convert raw data to typed data
	// Find a full list of available types in src/messages.rs.
    let order = NewOrderSingle::from_raw(&raw).expect("Failed to construct order");

    // Now you can use the data in your code
    println!("Symbol: {}, Side: {:?}", order.symbol, order.side);
}
```

#### Encoding Example

```rust
use chrono::Utc;
use fixie::{NewOrderSingle, OrdType, RawFixMessage, Side, SOH};

pub fn main() {
    let order = NewOrderSingle {
        cl_ord_id: "ORDER123".to_string(),
        symbol: "EURUSD".to_string(),
        side: Side::Buy,
        transact_time: Utc::now(),
        ord_type: OrdType::Limit,
        order_qty: 100.0,
        price: Some(1.25),
    };

    // Convert typed data to raw data
    let raw = order.to_raw();

    // Encode in \x01-delimited format for transmission
    let wire_format = raw.encode();
    println!("Wire format: {:?}", wire_format);

    // Encode in pipe-delimited format for debugging
    let pipe_format = wire_format.replace(SOH, "|");
    println!("Debug format: {}", pipe_format);
}
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
