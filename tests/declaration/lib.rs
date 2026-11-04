//! `uart-declaration` 的宿主契约：实际 `UNIT` / `BUILD`，不是声明的一份副本。

#[cfg(test)]
mod tests {
    use std::path::Path;

    use env::ProgramKind;
    use system_api::namespace;
    use system_api::operator::{Ops, Permit};
    use uart_declaration::{BUILD, UNIT};
    use unit::{Bootstrap, Dependency, Ending, Ready};

    #[test]
    fn unit_declares_the_uart_domain_and_namespace() {
        assert_eq!(UNIT.name, "uart");
        assert_eq!(UNIT.loader.image, "uart");
        assert!(matches!(UNIT.loader.space, ProgramKind::User));
        assert!(UNIT.loader.args.is_empty());
        assert_eq!(UNIT.loader.stack, 0);

        let namespaces = UNIT.operator.namespace;
        assert_eq!(namespaces.len(), 1);
        let ns = &namespaces[0];
        assert_eq!(ns.key, namespace::SERVICE);
        assert!(matches!(&ns.placement, unit::operator::Placement::Shared));
        assert_eq!(ns.path, "uart");
        assert!(matches!(&ns.scope, unit::operator::Scope::Subtree));
        assert!(matches!(&ns.permit, Some(Permit::Public)));
        assert!(ns.source_mark.is_none());
        assert!(ns.bindings.is_empty());
        assert!(UNIT.valid());
    }

    #[test]
    fn namespace_ops_are_exactly_the_source_set() {
        let expected = Ops::PART
            .union(Ops::UNPART)
            .union(Ops::BIND)
            .union(Ops::UNBIND)
            .union(Ops::FIND);
        assert!(
            UNIT.operator.namespace[0].ops == expected,
            "uart namespace ops must match the Source UnitFile"
        );
    }

    #[test]
    fn identity_and_supply_are_empty() {
        assert!(UNIT.identity.aliases.is_empty());
        assert!(UNIT.identity.grants.is_empty());
        assert!(UNIT.supply.bindings.is_empty());
    }

    #[test]
    fn startup_dependencies_keep_their_order() {
        let after = UNIT.control.after.expect("uart has startup dependencies");
        assert_eq!(after.len(), 3);
        assert!(matches!(
            &after[0],
            Dependency::Bootstrap(Bootstrap::Operator)
        ));

        let Dependency::Unit(hub) = &after[1] else {
            panic!("uart second startup dependency must be hub");
        };
        assert_eq!(*hub, "hub");

        let Dependency::Unit(router) = &after[2] else {
            panic!("uart third startup dependency must be router");
        };
        assert_eq!(*router, "router");
    }

    #[test]
    fn readiness_and_resident_ending_are_declared() {
        assert!(UNIT.control.supervision.is_none());
        assert!(matches!(
            &UNIT.control.ready,
            Ready::Signal(unit::READY_MARK)
        ));
        assert!(matches!(&UNIT.control.ending, Some(Ending::Resident)));
        assert!(UNIT.control.construction.is_empty());
        assert!(UNIT.control.retirement.is_empty());
    }

    #[test]
    fn build_names_the_uart_server_package_and_binary() {
        assert_eq!(BUILD.image, "uart");
        assert_eq!(BUILD.image, UNIT.loader.image);
        assert_eq!(BUILD.manifest, "server/Cargo.toml");
        assert_eq!(BUILD.package, "uart-server");
        assert_eq!(BUILD.bin, "uart");
        assert!(BUILD.features.is_empty());
        assert_eq!(BUILD.target, "riscv64gc-unknown-none-elf");
        assert_eq!(BUILD.profile, "release");
        assert!(BUILD.default_features);

        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        assert!(
            root.join(BUILD.manifest).is_file(),
            "BUILD.manifest must name the actual server manifest"
        );
    }
}
