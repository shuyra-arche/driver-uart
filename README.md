# UART Unit

UART provides Shuyra's serial RX/TX service. The API and Console client are shared
with the Terminal provider.

## Layout and contract

- `api/`: raw-byte frame and RX/TX names.
- `client/`: discovery and paired Console endpoints.
- `server/`: UART device adapter, service loop and the `uart` binary.
- `unit.toml`: startup policy requiring and following Hub and Router.
- `declaration/`: compatibility Rust Unit/build declarations.
- `tests/wire/`, `tests/declaration/`, `tests/unit-document/`: host suites.

`Bytes` contains 1 through `MAX = 129` raw bytes, without a header, length prefix
or terminator; an empty cell is not a value. `DRAIN_MAX = 64` bounds one drain.
Console discovery succeeds only when both directions resolve, map and bind.
RX retains IRQ ownership until its pending batch enters the RX ring.

## Build and test

Install Rust through rustup; the checkout pins nightly-2026-09-04 and RV64.

```sh
for suite in wire declaration unit-document; do
  cargo test --locked --manifest-path "tests/$suite/Cargo.toml" --target x86_64-unknown-linux-gnu || exit
done
CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_RUSTFLAGS='-Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium' \
  cargo build --locked --release --target riscv64gc-unknown-none-elf -p uart-server --bin uart
```

Uncached dependencies require network access. These host suites check wire and
declaration contracts, not IRQ delivery, MMIO timing or RX/TX progress.
Use [Vanllia](https://github.com/shuyra-arche/vanllia) for composed runtime checks.

See [source attribution](PROVENANCE.md) and the
[organization contribution guide](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
