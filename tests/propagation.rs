use libxc_rs::{lookup_by_id, Functional, FunctionalId, Spin};

const PROPAGATION_PARENT_IDS: &[u16] = &[
    433, // hyb_gga_xc_cam_b3lyp
    470, // hyb_gga_xc_camy_b3lyp
    395, // hyb_gga_xc_wb97
    614, // hyb_gga_xc_wb97x_d4
    682, // hyb_gga_xc_apf
    490, // hyb_gga_xc_cam_qtp_00
    482, // hyb_gga_xc_cam_qtp_01
    491, // hyb_gga_xc_cam_qtp_02
    478, // hyb_gga_xc_lc_wpbe
];

#[test]
fn test_all_propagation_parents_construct() {
    for &parent_raw in PROPAGATION_PARENT_IDS {
        let id = FunctionalId::from_raw(parent_raw).expect("valid propagation parent id");
        let meta = lookup_by_id(parent_raw).expect("parent meta exists");

        let func = Functional::new(id, Spin::Unpolarized)
            .unwrap_or_else(|e| panic!("Failed to construct parent {parent_raw} ({}): {e}", meta.name));

        assert!(
            !func.auxiliary_functionals().is_empty(),
            "Parent {} must have auxiliary functionals",
            meta.name
        );
    }
}

#[test]
fn test_cam_b3lyp_parameter_propagation() {
    let id = FunctionalId::from_raw(433).expect("cam_b3lyp id");
    let mut func = Functional::new(id, Spin::Unpolarized).expect("construct cam_b3lyp");

    let auxs = func.auxiliary_functionals();
    assert_eq!(auxs.len(), 4, "CAM-B3LYP must have 4 auxiliaries");

    // Default CAM parameters: alpha=0.19, beta=0.46, omega=0.33
    // Aux 0 (lda_c_vwn) has no ext params
    // Aux 1 (gga_x_ityh) ext params: [omega=0.33] (propagated from parent's omega)
    let ityh_aux = &auxs[1];
    assert!(
        ityh_aux.meta().name.to_ascii_lowercase().ends_with("gga_x_ityh"),
        "Aux 1 must be ITYH; got {}",
        ityh_aux.meta().name
    );
    assert!((ityh_aux.ext_param_by_index(0).unwrap() - 0.33).abs() < 1e-12);

    // Now modify the parameters on parent and ensure propagation
    func.set_ext_param("_omega", 0.40).expect("set omega");

    let auxs_after = func.auxiliary_functionals();
    let ityh_after = &auxs_after[1];
    assert!((ityh_after.ext_param_by_index(0).unwrap() - 0.40).abs() < 1e-12);
}

#[test]
fn test_lc_wpbe_parameter_propagation() {
    let id = FunctionalId::from_raw(478).expect("lc_wpbe id");
    let mut func = Functional::new(id, Spin::Unpolarized).expect("construct lc_wpbe");

    let auxs = func.auxiliary_functionals();
    assert_eq!(auxs.len(), 2, "LC-wPBE has 2 auxiliaries");

    // Aux 0 is gga_x_wpbeh, receives omega (default 0.40)
    let wpbe_aux = &auxs[0];
    assert!(
        wpbe_aux.meta().name.to_ascii_lowercase().contains("wpbe"),
        "Aux 0 must be WPBE; got {}",
        wpbe_aux.meta().name
    );
    assert!((wpbe_aux.ext_param_by_index(0).unwrap() - 0.40).abs() < 1e-12);

    // Modify omega on parent
    func.set_ext_param("_omega", 0.30).expect("set omega");
    let auxs_after = func.auxiliary_functionals();
    assert!((auxs_after[0].ext_param_by_index(0).unwrap() - 0.30).abs() < 1e-12);
}
