use libxc_rs::{
    lookup_by_name, DerivativeOrder, EvaluationWorkspace, Functional, FunctionalBuilder, FunctionalId,
    LdaInput, LdaOutput, GgaInput, GgaOutput, Spin,
};

const PI: f64 = std::f64::consts::PI;

/// Closed-form Slater exchange energy per particle:
/// e_x(rho) = -3/4 * (3/pi)^(1/3) * rho^(4/3)
/// zk(rho) = e_x(rho) / rho = -3/4 * (3/pi)^(1/3) * rho^(1/3)
fn slater_zk_unpolarized(rho: f64) -> f64 {
    -0.75 * (3.0 / PI).cbrt() * rho.cbrt()
}

/// Closed-form Slater exchange potential:
/// vrho(rho) = d(e_x)/d(rho) = -(3/pi)^(1/3) * rho^(1/3)
fn slater_vrho_unpolarized(rho: f64) -> f64 {
    -(3.0 / PI).cbrt() * rho.cbrt()
}

#[test]
fn test_facade_lda_x_analytic_unpolarized() {
    let id = FunctionalId::from_raw(1).expect("lda_x is id 1");
    let func = Functional::new(id, Spin::Unpolarized).expect("construct lda_x unpolarized");

    let rho = [0.05, 0.1, 0.2, 0.5, 1.0, 2.0];
    let np = rho.len();
    let mut zk = vec![0.0; np];
    let mut vrho = vec![0.0; np];

    let input = LdaInput::new(&rho, np, Spin::Unpolarized).expect("valid input");
    let mut output = LdaOutput::new(
        Some(&mut zk),
        Some(&mut vrho),
        None,
        None,
        None,
        np,
        Spin::Unpolarized,
    )
    .expect("valid output");
    let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);

    func.evaluate_lda(&input, DerivativeOrder::Vxc, &mut output, &mut ws)
        .expect("evaluate lda_x");

    for i in 0..np {
        let expected_zk = slater_zk_unpolarized(rho[i]);
        let expected_vrho = slater_vrho_unpolarized(rho[i]);
        assert!(
            (zk[i] - expected_zk).abs() < 1e-15,
            "zk[{i}] = {}, expected {}",
            zk[i],
            expected_zk
        );
        assert!(
            (vrho[i] - expected_vrho).abs() < 1e-15,
            "vrho[{i}] = {}, expected {}",
            vrho[i],
            expected_vrho
        );
    }
}

#[test]
fn test_facade_lda_x_via_builder() {
    let id = lookup_by_name("lda_x").expect("lookup lda_x");
    let func = FunctionalBuilder::new(id)
        .spin(Spin::Unpolarized)
        .build()
        .expect("build lda_x");

    let rho = [0.1, 0.4];
    let np = rho.len();
    let mut zk = vec![0.0; np];

    let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
    let mut output = LdaOutput::new(Some(&mut zk), None, None, None, None, np, Spin::Unpolarized).unwrap();
    let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);

    func.evaluate_lda(&input, DerivativeOrder::Exc, &mut output, &mut ws)
        .expect("evaluate via builder");

    for (i, &r) in rho.iter().enumerate() {
        let expected_zk = slater_zk_unpolarized(r);
        assert!(
            (zk[i] - expected_zk).abs() < 1e-15,
            "zk[{i}] = {}, expected {}",
            zk[i],
            expected_zk
        );
    }
}

#[test]
fn test_facade_gga_pbe_evaluation() {
    let id = lookup_by_name("gga_x_pbe").expect("lookup gga_x_pbe");
    let pbe_x = FunctionalBuilder::new(id)
        .spin(Spin::Unpolarized)
        .build()
        .expect("build gga_x_pbe");

    let rho = [0.1, 0.2, 0.3];
    let sigma = [0.01, 0.04, 0.09];
    let np = rho.len();
    let mut zk = vec![0.0; np];
    let mut vrho = vec![0.0; np];
    let mut vsigma = vec![0.0; np];

    let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
    let mut output = GgaOutput::new(
        Some(&mut zk),
        Some(&mut vrho),
        Some(&mut vsigma),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        np,
        Spin::Unpolarized,
    )
    .unwrap();
    let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);

    pbe_x
        .evaluate_gga(&input, DerivativeOrder::Vxc, &mut output, &mut ws)
        .expect("evaluate gga_x_pbe");

    for i in 0..np {
        assert!(zk[i] < 0.0, "PBE exchange energy must be negative; got {}", zk[i]);
        assert!(vrho[i] < 0.0, "PBE vrho must be negative; got {}", vrho[i]);
    }
}

#[test]
fn test_facade_b3lyp_hybrid_propagation_and_eval() {
    let id = lookup_by_name("hyb_gga_xc_b3lyp").expect("lookup b3lyp");
    let b3lyp = FunctionalBuilder::new(id)
        .spin(Spin::Unpolarized)
        .build()
        .expect("build b3lyp");

    assert_eq!(b3lyp.auxiliary_functionals().len(), 4);
    assert!((b3lyp.exx_coefficient().unwrap() - 0.2).abs() < 1e-15);

    let rho = [0.1, 0.2];
    let sigma = [0.01, 0.04];
    let np = rho.len();
    let mut zk = vec![0.0; np];

    let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
    let mut output = GgaOutput::new(
        Some(&mut zk),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        np,
        Spin::Unpolarized,
    )
    .unwrap();
    let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);

    b3lyp
        .evaluate_gga(&input, DerivativeOrder::Exc, &mut output, &mut ws)
        .expect("evaluate b3lyp");

    for (i, &val) in zk.iter().enumerate() {
        assert!(val < 0.0, "B3LYP zk[{i}] must be negative; got {val}");
    }
}
