# UART SDK

Public `uart-api` and `uart-client`, shared by the Source UART driver and the
Terminal provider so that both sides name the same type. Same repository, two
published packages, one full SHA.

## `uart-api`

`uart_api::frame` owns the single wire type and its names:

- `Bytes` — one nonempty run of raw bytes, `1..=MAX`, no header, no count, no
  terminator; empty is not a value (`Bytes::of(&[])` and an empty cell are
  `None`).
- `DRAIN_MAX = 64` — bytes one drain may carry; `MAX = 129` — bytes of one
  transfer (`const _: () = assert!(DRAIN_MAX <= MAX)`).
- `ME = "uart"`, `RX = "rx"`, `TX = "tx"` — the two tree tiles and the domain
  name.

## `uart-client`

`uart_client::{Console, road, find}` resolve the two tiles into the two rack
handles: `find` returns `Some(Console)` only when both directions resolve, map
and bind; any miss is `None`. No console abstraction, no retry policy of its
own.

The UART device driver, MMIO, scheduling, UnitFile and Source composition are
not part of this repository and stay in Source.

## Wire tests

`tests/wire` is a standalone host workspace (excluded from the package
workspace) that exercises the actual `uart-api` path:

```sh
cargo test --manifest-path tests/wire/Cargo.toml
```

It pins the cross-repository boundary: exact raw bytes with embedded zeros and
no header, empty and `MAX + 1` rejection, zero-filled valid data, `MAX`
round-trip, and rejection of a too-short output buffer.

## Publication

All external dependencies use public GitHub revisions. Root performs native
metadata/offline host and RV validation, independent review and publication.
