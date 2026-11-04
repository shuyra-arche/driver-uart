#![no_std]

//! 串口驱动（prog-uart）的装配声明。
//! **U 态**：持有 `serial@10000000`（banner 里那张 PMP 是 S/U (R,W)），把"收到字节就拉线"打开。
//! 这一份是本 Unit 的唯一 `UnitFile` 所有者。

use unit::{Ending, ImageBuild, UnitFile};

pub static UNIT: UnitFile = UnitFile {
    name: "uart",
    loader: unit::LoaderConfig::user("uart"),
    operator: unit::OperatorConfig {
        namespace: &[unit::operator::Namespace {
            key: system_api::namespace::SERVICE,
            placement: unit::operator::Placement::Shared,
            path: "uart",
            scope: unit::operator::Scope::Subtree,
            ops: system_api::operator::Ops::PART
                .union(system_api::operator::Ops::UNPART)
                .union(system_api::operator::Ops::BIND)
                .union(system_api::operator::Ops::UNBIND)
                .union(system_api::operator::Ops::FIND),
            permit: Some(system_api::operator::Permit::Public),
            source_mark: None,
            bindings: &[],
        }],
    },
    identity: unit::Identity {
        aliases: &[],
        grants: &[],
    },
    control: unit::ControlConfig {
        supervision: None,
        after: Some(&[
            unit::Dependency::Bootstrap(unit::Bootstrap::Operator),
            unit::Dependency::Unit("hub"),
            unit::Dependency::Unit("router"),
        ]),
        ready: unit::Ready::Signal(unit::READY_MARK),
        ending: Some(Ending::Resident),

        construction: &[],
        retirement: &[],
    },
    supply: unit::SupplyConfig { bindings: &[] },
};

pub const BUILD: ImageBuild = ImageBuild {
    image: "uart",
    manifest: "server/Cargo.toml",
    package: "uart-server",
    bin: "uart",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
