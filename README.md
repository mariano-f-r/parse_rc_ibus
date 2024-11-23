# IBUS Parser

A basic parser for the [IBUS RC protocol](http://blog.dsp.id.au/posts/2017/10/22/flysky-ibus-protocol/), written in Rust.
Ideal for decoding channel data in radio-controlled systems and other applications using the IBUS protocol.
MSRV is 1.81.

## Features

- **Simple API**: Straightforward functions for parsing IBUS packets.
- **Error Handling**: Basic checksum verification to detect corrupted data.
- **Lightweight**: Minimal dependencies and a small footprint.

## Installation

Add `parse_rc_ibus` to your `Cargo.toml`:

```toml
[dependencies]
parse_rc_ibus = "0.1.0"
```

Then, in your Rust code:

```rust
use parse_rc_ibus::IbusPacket;
```

## Usage

### Parsing an IBUS Packet

To parse an IBUS packet, pass the byte array to `IbusPacket::try_from_bytes()`. This will return the parsed packet or an error if the data is invalid.

```rust
let buffer [u8; 32] = [...];
let packet = IbusPacket::try_from_bytes(&buffer);
```

### Note
You will have to sync the UART manually. This can be done by iterating byte by byte until you see the IBUS header, then skipping the next packet.

## Contributing

Any and all contributions are welcome.

### Branching

If you choose to contribute, when forking the repository and naming your branch, follow the below table.
| Type | Branch Prefix |
| -------------- | --------------- |
| Bugfix | fix/ |
| Feature | feat/ |
| Chore | chore/ |

One final note on branches: the main branch should always compile, so changes will not be merged unless they also compile.

### Commits

In regards to commits, [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) should be used for merge/squash commits.
Otherwise, keep your commits concise and understandable.
No emojis.

### Issues

When writing an issue, follow these guidelines:

- **Bug Reports**: Prefix your issue with "BUG:". Please describe steps to reproduce, and include which features failed.
- **Feature Requests**: Prefix your issue with "FEATURE:". Please describe in detail how you want your proposed feature to work. Also include code snippets of how you want it to work if possible. If you are requesting that a feature be changed, include how you want the new version of the feature to work.

## License

MIT License. See LICENSE for more details.
