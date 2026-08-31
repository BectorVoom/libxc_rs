use libxc_rs::{lookup_by_id, Functional, FunctionalId, LibxcRsError, Spin};

#[test]
fn test_ext_params_defaults_and_roundtrip_all() {
    let mut tested_functionals = 0;
    let mut total_params_tested = 0;

    for raw in 1..=800 {
        let meta = match lookup_by_id(raw) {
            Ok(m) => m,
            Err(_) => continue,
        };

        if meta.ext_params.is_empty() {
            continue;
        }

        let id = meta.id;
        let mut func = Functional::new(id, Spin::Unpolarized)
            .unwrap_or_else(|e| panic!("Failed to construct functional {raw} ({}): {e}", meta.name));

        // 1. Verify default values match ExtParamSpec
        for (i, spec) in meta.ext_params.iter().enumerate() {
            let val = func.ext_param_by_index(i)
                .unwrap_or_else(|e| panic!("ext_param_by_index({i}) on {} failed: {e}", meta.name));
            let val_by_name = func.ext_param(&spec.name)
                .unwrap_or_else(|e| panic!("ext_param({}) on {} failed: {e}", spec.name, meta.name));

            assert_eq!(val, val_by_name);
            assert_eq!(
                val, spec.default_value,
                "Functional {} param {} default mismatch: got {val}, expected {}",
                meta.name, spec.name, spec.default_value
            );
            total_params_tested += 1;
        }

        // 2. Test set_ext_param single update
        let first_spec = &meta.ext_params[0];
        let new_val = first_spec.default_value + 1.2345;
        func.set_ext_param(&first_spec.name, new_val).expect("set_ext_param");
        assert_eq!(func.ext_param(&first_spec.name).unwrap(), new_val);

        // 3. Test set_ext_params slice update
        let new_values: Vec<f64> = meta.ext_params.iter().enumerate().map(|(i, s)| s.default_value + (i as f64 + 1.0) * 0.1).collect();
        func.set_ext_params(&new_values).expect("set_ext_params");
        for (i, &expected) in new_values.iter().enumerate() {
            assert_eq!(func.ext_param_by_index(i).unwrap(), expected);
        }

        // 4. Test error on invalid count
        let wrong_slice = vec![0.0; meta.ext_params.len() + 1];
        let err = func.set_ext_params(&wrong_slice).unwrap_err();
        match err {
            LibxcRsError::ExtParamCountMismatch { id: err_id, expected, actual } => {
                assert_eq!(err_id, id);
                assert_eq!(expected, meta.ext_params.len());
                assert_eq!(actual, wrong_slice.len());
            }
            other => panic!("Expected ExtParamCountMismatch, got {other:?}"),
        }

        // 5. Test error on unknown param name
        let err_name = func.set_ext_param("__non_existent_param_name__", 1.0).unwrap_err();
        assert!(matches!(err_name, LibxcRsError::UnknownExtParamName { .. }));

        tested_functionals += 1;
    }

    assert!(tested_functionals >= 40, "Expected at least 40 parameterized functionals, got {tested_functionals}");
    assert!(total_params_tested >= 100, "Expected at least 100 parameters tested, got {total_params_tested}");
}
