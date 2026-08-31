//! Evaluation invariant tests for GGA functionals.

use libxc_rs::{
    lookup_by_name, DerivativeOrder, EvaluationWorkspace, Functional, GgaInput, GgaOutput, Spin,
};

const GGA_CORPUS: &[&str] = &[
    "gga_x_pbe",
    "gga_c_pbe",
    "gga_x_b88",
    "gga_c_lyp",
    "gga_x_rpbe",
    "gga_k_lgap",
    "gga_k_dk",
];

#[test]
fn test_gga_evaluation_invariants() {
    let rho = [0.01, 0.05, 0.1, 0.2, 0.5, 1.0];
    let sigma = [0.001, 0.005, 0.01, 0.04, 0.1, 0.5];
    let np = rho.len();

    for &name in GGA_CORPUS {
        let id = lookup_by_name(name).unwrap_or_else(|_| panic!("lookup {name}"));
        let rust_func = Functional::new(id, Spin::Unpolarized)
            .unwrap_or_else(|e| panic!("construct {name}: {e}"));

        let mut zk_rust = vec![0.0; np];
        let mut vrho_rust = vec![0.0; np];
        let mut vsigma_rust = vec![0.0; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
        let mut output = GgaOutput::new(
            Some(&mut zk_rust),
            Some(&mut vrho_rust),
            Some(&mut vsigma_rust),
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

        rust_func
            .evaluate_gga(&input, DerivativeOrder::Vxc, &mut output, &mut ws)
            .unwrap_or_else(|e| panic!("evaluate {name}: {e}"));

        for i in 0..np {
            assert!(!zk_rust[i].is_nan(), "{name} zk[{i}] is NaN");
            assert!(!vrho_rust[i].is_nan(), "{name} vrho[{i}] is NaN");
            assert!(!vsigma_rust[i].is_nan(), "{name} vsigma[{i}] is NaN");
            assert!(zk_rust[i].is_finite(), "{name} zk[{i}] is infinite");
            assert!(vrho_rust[i].is_finite(), "{name} vrho[{i}] is infinite");
            assert!(vsigma_rust[i].is_finite(), "{name} vsigma[{i}] is infinite");
        }
    }
}
