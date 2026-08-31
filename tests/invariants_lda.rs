//! Evaluation invariant tests for LDA functionals.

use libxc_rs::{
    lookup_by_name, DerivativeOrder, EvaluationWorkspace, Functional, LdaInput, LdaOutput, Spin,
};

const LDA_CORPUS: &[&str] = &["lda_x", "lda_c_vwn", "lda_c_pw", "lda_c_pz", "lda_c_hl"];

#[test]
fn test_lda_evaluation_invariants() {
    let rho = [0.001, 0.01, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0];
    let np = rho.len();

    for &name in LDA_CORPUS {
        let id = lookup_by_name(name).unwrap_or_else(|_| panic!("lookup {name}"));
        let rust_func = Functional::new(id, Spin::Unpolarized)
            .unwrap_or_else(|e| panic!("construct {name}: {e}"));

        let mut zk_rust = vec![0.0; np];
        let mut vrho_rust = vec![0.0; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
        let mut output = LdaOutput::new(
            Some(&mut zk_rust),
            Some(&mut vrho_rust),
            None,
            None,
            None,
            np,
            Spin::Unpolarized,
        )
        .unwrap();

        rust_func
            .evaluate_lda(&input, DerivativeOrder::Vxc, &mut output, &mut ws)
            .unwrap_or_else(|e| panic!("evaluate {name}: {e}"));

        for i in 0..np {
            assert!(!zk_rust[i].is_nan(), "{name} zk[{i}] is NaN");
            assert!(!vrho_rust[i].is_nan(), "{name} vrho[{i}] is NaN");
            assert!(zk_rust[i].is_finite(), "{name} zk[{i}] is infinite");
            assert!(vrho_rust[i].is_finite(), "{name} vrho[{i}] is infinite");
        }
    }
}
