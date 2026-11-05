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

## Unit document validation — 2026-11-05

This change is based on UART
`298bc8809f297f46bc7592aebc96540b7b492efc` and adds the Unit-owned
`unit.toml` plus `tests/unit-document`. The existing declaration, server,
API/client, host suites, production dependency pins, toolchain file and all
pre-existing locks remain unchanged. The new suite adds its own committed
lock. No Source or Vanllia integration file is modified.

The document uses stable id/image `uart`, the actual `uart-server` Cargo
binary, and startup creation. Both `requires` and `after` contain `hub` and
`router` in that order: they are UART's necessary Unit providers and readiness
prerequisites. The host unconditionally includes the core System Unit; System
completes its internal Bootstrap services before creating ordinary Units.
That preserves the old `Bootstrap::Operator` prerequisite without introducing
an Operator Unit or repeating System in `requires`.

All old policy fields are compared directly against the actual retained
`uart_declaration::UNIT`: loader, namespace key/placement/path/scope/ops,
permit/source mark/bindings, identity aliases/grants, supervision, construction,
retirement, ready signal, resident ending and supply. The new groups list is
empty and guarded is false. The checks also compare the existing `BUILD` with
the actual `server/Cargo.toml` package, binary and entry path. Runtime RX/TX
publication remains in the unchanged server rather than becoming static
bindings or supply.

An independent native Rust executable linked the actual UART declaration and
public legacy Unit revision `90396f0fda376e0693fb1b9553aa21864d241057` and read:

| Native value | Result |
| --- | --- |
| `UNIT.control.ready` and `unit::READY_MARK` | `9887014109901452468` (`0x8935baf305f374b4`) |
| UART namespace key | `service` |
| UART namespace operation union | `279` (`0x117`) |
| UART identity grants | Empty |

These are values read from UART's Rust declaration, not constants copied from
another Unit. The ready mark is above `i64::MAX`. Its quoted decimal and quoted
hexadecimal forms both parse to the same native `u64` and survive record
encoding/decoding without truncation.

The new suite uses published Unit revision
`ad50b803c3bd5cd2ef90010d80badf23551e88d5` for document, record and runtime
conversion. The `legacy-unit` alias keeps public revision
`90396f0fda376e0693fb1b9553aa21864d241057` solely for the retained declaration's
enum comparisons. Its formal `tests/unit-document/Cargo.lock` contains both
exact public Git sources. No patch, local Unit path or staged manifest is used.
Build output and the process lock are dedicated
`/tmp/uart-unit-document-public-target` and
`/tmp/uart-unit-document-public-target.lock`. The exact reproduction command
is in `README.md`. All five tests passed with `--locked --offline`.

| Executed check | Result |
| --- | --- |
| Strict TOML parse/validation and every policy field versus the retained declaration | Passed |
| Complete binary record roundtrip, owned-document equality and eight borrowed strings within the record | Passed |
| Record-to-native runtime conversion versus every applicable legacy field | Passed |
| Quoted decimal/hex full-width Ready mark versus native Rust value | Passed |
| Real Cargo binary and retained build contract | Passed |
| Existing wire host suite, public dependencies, locked/offline | 6 passed |
| Existing declaration host suite, public dependencies, locked/offline | 6 passed |
| RISC-V workspace check, public dependencies, locked/offline | Passed |
| RISC-V `uart-server` / `uart` release build, public dependencies, locked/offline | Passed |
| Scoped rustfmt and `git diff --check` | Passed |
| New suite with its formal lock and public Unit revisions, without a Cargo patch | 5 passed |
| Locked/offline metadata: new document Unit and explicit legacy enum source | Both exact public revisions confirmed |

The worker used the installed `nightly-2026-10-09` selected by
`/workspace/.vanllia-tools/env.sh`, with separate build outputs under `/tmp/`.
The two existing host suites retain their pre-existing nonblocking unused
provider-dependency warning. The first offline production build could not
find the locked `driver-shared` checkout; `cargo fetch --locked` populated the
public dependency cache, after which the offline workspace check and release
build passed without any Cargo patch or lock change.

```sh
source /workspace/.vanllia-tools/env.sh
CARGO_TARGET_DIR=/tmp/uart-existing-wire-target cargo test --locked --offline \
  --manifest-path tests/wire/Cargo.toml --target x86_64-unknown-linux-gnu
CARGO_TARGET_DIR=/tmp/uart-existing-declaration-target cargo test --locked --offline \
  --manifest-path tests/declaration/Cargo.toml --target x86_64-unknown-linux-gnu
export CARGO_TARGET_RISCV64GC_UNKNOWN_NONE_ELF_RUSTFLAGS='-Crelocation-model=static -Cforce-frame-pointers=yes -Ccode-model=medium'
flock /tmp/uart-riscv-release-target.lock cargo check --locked --offline --workspace \
  --target riscv64gc-unknown-none-elf --target-dir /tmp/uart-riscv-release-target
flock /tmp/uart-riscv-release-target.lock cargo build --locked --offline --release \
  --target riscv64gc-unknown-none-elf --target-dir /tmp/uart-riscv-release-target \
  -p uart-server --bin uart
rustfmt --check --edition 2024 tests/unit-document/lib.rs
git diff --check
```

The independent UART reviewer first approved local checkpoint `71d4b33`, then
rechecked the public dependency handoff in the same review group. It ran the
actual five-test suite without a patch and separately changed its Rust harness
to keep the public legacy Unit and published document Unit as two explicit
Git sources. Three additional locked/offline tests passed: complete
TOML/record policy comparison with eight borrowed strings, native Ready plus
`u64::MAX` decimal/hex preservation, and record-to-runtime policy comparison.
The actual UART record is 89 bytes. The reviewer separately read the same
native UART values above, checked the real build contract, and confirmed the
old sources/manifests/locks have no diff. These three checks are additional
review evidence; the new suite itself contains five tests.

The initial local checkpoint used a CLI-only Unit patch; the legacy public
Unit alone correctly failed with E0432 because it lacks document/record/runtime
APIs. The final suite now resolves those APIs from published `ad50b803…` with
its committed lock and passes without that patch. Locked metadata confirms
that only the suite and retained declaration are local packages, and both
Unit versions come from public Git. No production Unit pin or existing lock
was upgraded during this handoff.

No old test scene was migrated. No QEMU, hardware execution, initrd composition,
push or PR creation was performed. These host comparisons and builds do not
establish MMIO/IRQ behavior or real startup acceptance.
