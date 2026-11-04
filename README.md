# UART Unit

This repository contains the public `uart-api` and `uart-client`, frozen at
baseline `b690431496b0beaeb5fb7cafb42e60b4c8b38c9d` (Source
`f488db5538dc8dd8a530346d76e343708a3c5df0`), plus the non-published
`uart-server` and `uart-declaration` extracted from Source at
`94e114614bec53424ac55e640fc80cf19a3e7107`. The API and client are shared by
the UART driver and the Terminal provider so that both sides name the same
type. Same repository, one set of full-SHA pins.

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

## `uart-server`

`uart-server` is the U-state runtime provider plus the thin `uart` binary.
`uart_server::serve` preserves the Source runtime: the same desk startup,
two-source wait, TX draining, RX pending batch, IRQ ownership until the batch
enters the RX ring, rack modes, waits, order and error strings. Provider
adaptation moves the shared Context/Hub/Device/Fail to the published
`driver-shared` crate and `Bytes`/names to `uart-api`; `E_UART = 9` remains
internal. The binary only calls `serve`.

The Source rack probes are integration harnesses and remain in Source. Source
contained no dedicated historical UART server host suite to copy.

## `uart-declaration`

`uart_declaration::UNIT` is the single `UnitFile` owner for the UART unit,
retaining the Source namespace operations, startup dependencies, readiness and
resident ending. `uart_declaration::BUILD` names the `uart-server` package and
the `uart` binary in `server/Cargo.toml`.

## Wire tests

`tests/wire` is a standalone host workspace (excluded from the package
workspace) that exercises the actual `uart-api` path:

```sh
cargo test --manifest-path tests/wire/Cargo.toml
```

It pins the cross-repository boundary: exact raw bytes with embedded zeros and
no header, empty and `MAX + 1` rejection, zero-filled valid data, `MAX`
round-trip, and rejection of a too-short output buffer.

## Declaration tests

`tests/declaration` is a standalone host workspace (also excluded from the
package workspace) that exercises the actual declaration crate:

```sh
cargo test --manifest-path tests/declaration/Cargo.toml
```

It checks loader identity, the UART namespace and exact operation set, empty
identity/supply, ordered startup dependencies, readiness signal, resident
ending, `UNIT.valid()`, and the `BUILD` package/binary contract including the
manifest path.

## Publication

All external dependencies use public GitHub full revisions. Root performs
native metadata/offline host and RV validation, independent review and
publication.
