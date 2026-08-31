use libxc_rs::{lookup_by_id, Functional, LibxcRsError, Spin};

#[test]
fn test_construct_all_registered_functionals() {
    let mut constructed = 0;
    let mut removed = 0;

    for raw in 1..=800 {
        let meta = match lookup_by_id(raw) {
            Ok(m) => m,
            Err(LibxcRsError::RemovedFunctionalId { .. }) => {
                removed += 1;
                continue;
            }
            Err(_) => continue,
        };

        let id = meta.id;

        // Test Unpolarized construction
        let func_unpol = Functional::new(id, Spin::Unpolarized)
            .unwrap_or_else(|e| panic!("Failed to construct unpolarized functional {raw} ({}): {e}", meta.name));
        assert_eq!(func_unpol.meta().id, id);
        assert_eq!(func_unpol.spin(), Spin::Unpolarized);
        assert_eq!(func_unpol.meta().name, meta.name);

        // Test Polarized construction
        let func_pol = Functional::new(id, Spin::Polarized)
            .unwrap_or_else(|e| panic!("Failed to construct polarized functional {raw} ({}): {e}", meta.name));
        assert_eq!(func_pol.meta().id, id);
        assert_eq!(func_pol.spin(), Spin::Polarized);

        constructed += 1;
    }

    assert!(constructed >= 600, "expected at least 600 constructible functionals, got {constructed}");
    assert!(removed >= 1, "expected at least 1 removed functional (id 104), got {removed}");
}
