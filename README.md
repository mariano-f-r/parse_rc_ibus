# IBUS Parser

A basic parser for the IBUS (Intelligent Bus) RC protocol, written in Rust. Ideal for decoding channel data in radio-controlled systems and other applications using the IBUS protocol.

## Features

- **Simple API**: Straightforward functions for parsing IBUS packets.
- **Error Handling**: Basic checksum verification to detect corrupted data.
- **Lightweight**: Minimal dependencies and a small footprint.

## Installation

Add `parse_rc_ibus` to your `Cargo.toml`:

```toml
[dependencies]
parse_rc_ibus = "0.1"
```

Then, in your Rust code:

```rust
use parse_rc_ibus::IbusPacket;
```

## Usage

### Parsing an IBUS Packet

To parse an IBUS packet, pass the byte array to `IbusPacket::try_from_bytes()`. This will return the parsed packet or an error if the data is invalid.

```rust
let buffer [u8; 32] = // ...
let packet = IbusPacket::try_from_bytes(&buffer)

```

## License

MIT License. See LISCENSE for more details.
