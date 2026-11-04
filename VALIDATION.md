# UART provider validation

Validated on 2026-11-04 against UART base
`b690431496b0beaeb5fb7cafb42e60b4c8b38c9d`
(`codex/uart-console-contract`) and frozen Source
`94e114614bec53424ac55e640fc80cf19a3e7107`.

## Scope and source preservation

The change adds `uart-server` (library entry `serve`, binary `uart`),
`uart-declaration` (`UNIT`, `BUILD`), and declaration host checks. It reuses
public `driver-shared` at `e75b1e232f828666956fb7e02c61a7d84f13eb66`.
Source and Vanllia integration were not modified.

Root compared Rust tokens after removing comments/whitespace and normalizing
the single `programs::unit::READY_MARK` provider path:

- Six UART function bodies match Source: `main` -> `serve`, desk `start` and
  `plate`, MMIO `arm_rx`, `drain`, and `put`.
- Ten reused `driver-shared` function bodies match Source: Context
  `open`/`plate`/`line`, Hub `find`/`claim`/`face_of`, Device `open`/`view`,
  and Fail `at`/`report`.
- The `Desk` fields and their order, MMIO constants, and complete UART
  `UnitFile` initializer match Source (only `crate::unit` provider paths differ).
- All six API/client files and the three existing wire-test files, including
  their lock, remain byte-identical to the UART base. Source raw hashes and
  original attribution are retained in `PROVENANCE.md`.

Code/build/test snapshot SHA256:
`16a32b6eab7e5251efb1b84dece0db9b29aa8092d0a4d8093d851523a77446d4`.
This is SHA256 of the sorted records `relative_path + NUL + SHA256(file_bytes)
+ LF` for all 25 `.rs`, `.toml`, and `.lock` files, excluding `.git` and
`target`. Documentation is outside this digest.

## Executed checks

Root used installed `nightly-2026-10-09`, rustc
`1.101.0-nightly (a30aa9064 2026-10-08)` and cargo
`1.101.0-nightly (29c5daa1a 2026-10-07)`. The repository's pre-existing
`nightly-2026-09-04` toolchain file is unchanged; that toolchain was not tested.
Public dependencies were fetched before the locked/offline checks.

| Check | Result |
| --- | --- |
| Existing `tests/wire` host suite, locked/offline | 6 passed |
| New `tests/declaration` host suite, locked/offline | 6 passed |
| Workspace RISC-V `cargo check`, locked/offline | Passed |
| `uart-server` / `uart` RISC-V debug build, locked/offline | Passed |
| `uart-server` / `uart` RISC-V release build, locked/offline | Passed |
| ELF headers, load base, entry and symbols, both profiles | Passed |
| Locked/offline Cargo metadata and dependency audit | Passed |
| Scoped rustfmt on added Rust files and `git diff --check` | Passed |

Each host test workspace emits one nonblocking manifest warning for its
provider dependency being used only under `cfg(test)` (the wire warning also
exists at the base). No assertions were removed or relaxed.

Commands, with the installed toolchain selected and dependency cache populated:

```sh
export RUSTUP_TOOLCHAIN=nightly-2026-10-09
cargo test --locked --offline --manifest-path tests/wire/Cargo.toml --target x86_64-unknown-linux-gnu
cargo test --locked --offline --manifest-path tests/declaration/Cargo.toml --target x86_64-unknown-linux-gnu
cargo check --locked --offline --workspace --target riscv64gc-unknown-none-elf
export CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_RUSTFLAGS='-Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium'
cargo build --locked --offline --target riscv64gc-unknown-none-elf -p uart-server --bin uart
cargo build --locked --offline --release --target riscv64gc-unknown-none-elf -p uart-server --bin uart
cargo metadata --locked --offline --format-version 1 --filter-platform riscv64gc-unknown-none-elf
```

Both binaries are RISC-V ELF64 `ET_EXEC`, have no ELF interpreter, load from
`0x10000`, export `clean_ret`, and have an entry address equal to `_start`:
debug `0x20e14`, release `0x14e54`. The declaration selects U-state loading.
The normal server dependency closure contains 29 packages, with only
`uart-server` and `uart-api` local to this repository. It has one `uart-api`
identity, no sqware/programs/composition dependency, and every external Git
dependency is a public GitHub full revision. Existing dependency versions were
preserved in the root lock; only required packages were added.

## Independent review

A fresh group used genuine OpenCode Go `deepseek-v4.1-flash` requests in three
independent sessions; no substitute model was used:

- Namer: `uart-namer-276a2a47-8baf-4009-8012-9e258bda4b83`.
- Worker: `uart-worker-57062078-424a-48ae-89dd-f71f1fc194ac`.
- Reviewer: `uart-reviewer-c655b38f-afae-4dab-9761-450a8d343950`.

The reviewer inspected the actual final source snapshot and confirmed no
P0/P1/P2 blocker; no further patch was requested. The worker's initial
`READY_MARK` namespace-shadowing compile error was corrected in the same worker
session before final review. The reviewer correctly confirmed that the private
crate-root device alias is visible to descendant modules.

This was static review; the reviewer did not execute builds/tests or inspect
lock contents. Dependency resolution, lock auditing, ELF and execution of the
checks above are Root evidence.

## Runtime limits

No QEMU or hardware run was performed. Host codec/declaration tests, source
equivalence and RISC-V compilation do not prove IRQ delivery, MMIO timing, TX
THRE progress, RX-full wakeups or pending-batch/IRQ ownership at runtime.
There was no dedicated UART server host suite in the Source baseline; rack
integration probes remain in Source and were not run here. Vanllia intake,
initrd composition and real startup acceptance remain later work.
