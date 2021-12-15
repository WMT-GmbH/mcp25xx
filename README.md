# mcp25xx
[<img alt="github" src="https://img.shields.io/badge/github-grey?style=for-the-badge&labelColor=555555&logo=github">](https://github.com/WMT-GmbH/mcp25xx)
[<img alt="Crates.io" src="https://img.shields.io/crates/v/mcp25xx?logo=rust&style=for-the-badge">](https://crates.io/crates/mcp25xx)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-mcp25xx-blue?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K">](https://docs.rs/mcp25xx)


`no_std` library for the MCP2510, MCP2515 and MCP25625 CAN controller chips.

API is implemented in terms of of the [embedded_hal](https://docs.rs/embedded-hal/) and [embedded_can](https://docs.rs/embedded-can/) traits.

Activating the `mcp2515` or `mcp25625` feature will enable
additional registers and instructions the MCP2510 does not support.

## Example

```rust
use embedded_can::blocking::Can as _;
use embedded_can::{Can, Frame, StandardId};
use mcp25xx::bitrates::clock_16mhz::CNF_500K_BPS;
use mcp25xx::registers::{OperationMode, RXB0CTRL, RXM};
use mcp25xx::{CanFrame, Config, MCP25xx};

// spi, cs and timer are structs implementing their respective embedded_hal traits.

let mut mcp25xx = MCP25xx { spi, cs };

let config = Config::default()
    .mode(OperationMode::NormalOperation)
    .bitrate(CNF_500K_BPS)
    .receive_buffer_0(RXB0CTRL::default().with_rxm(RXM::ReceiveAny));

mcp25xx.apply_config(&config).unwrap();

// Send a frame
let can_id = StandardId::new(123).unwrap();
let data = [1, 2, 3, 4, 5, 6, 7, 8];
let frame = CanFrame::new(can_id, &data).unwrap();
mcp25xx.try_write(&frame).unwrap();

// Receive a frame
if let Ok(frame) = mcp25xx.try_receive() {
    let _can_id = frame.id();
    let _data = frame.data();
}
```

#### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
