use libxc_rs::{lookup_by_id, lookup_by_name, DerivativeOrder, EvaluationWorkspace, Functional, LdaInput, LdaOutput, GgaInput, GgaOutput, MggaInput, MggaOutput, Spin};

const TIER1_CORPUS: &[&str] = &[
    "lda_x",
    "lda_c_pw",
    "lda_c_vwn",
    "lda_c_vwn_rpa",
    "gga_x_pbe",
    "gga_c_pbe",
    "gga_x_b88",
    "gga_c_lyp",
    "mgga_x_tpss",
];


// Ratchet: lower this as functionals are wired; never raise it.
// 2026-08-28: 24 (was 83 in v3, 80 in v2, 110 in v1).
// Remediation v4 resolved 59 functionals:
// - V4-01: Scraped 26 functionals with defaults in _init bodies
// - V4-02: Ported 21 custom setter transforms (N, T, pbe_lambda, s12h, lcgau, mpw91)
// - V4-03: Resolved 11 param name aliases, wb97/gas22/m06_sx arrays, and s12g
// - V4-05: Resolved gga_xc_hcth_93 info block
// The remaining 25 entries comprise:
// - 9 functionals with missing order/spin modules in kernels-rayon tree (5 potential-only + 4 missing)
// - 16 auxiliary mixed functionals and one-off setters
const MAX_UNSUPPORTED: usize = 25;

// Evaluability ratchet: must never fall below this count.
// Raised from 219 to 632 after V5-01 complete id-keyed dispatch was wired.
const MIN_EVALUABLE: usize = 632;

#[test]
fn test_unsupported_count_does_not_regress() {
    let n = libxc_reval::routing::UNSUPPORTED.len();
    assert!(
        n <= MAX_UNSUPPORTED,
        "UNSUPPORTED grew to {n}, ratchet is {MAX_UNSUPPORTED}"
    );
}

#[test]
fn test_registry_evaluability_does_not_regress() {
    let np = 4;
    let rho = [0.1, 0.2, 0.3, 0.4];
    let sigma = [0.01, 0.04, 0.09, 0.16];
    let lapl = [0.0; 4];
    let tau = [0.05, 0.06, 0.07, 0.08];

    let mut ok_count = 0;
    let mut err_count = 0;

    for raw in 1..=800 {
        let meta = match lookup_by_id(raw) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let id = meta.id;
        let func = match Functional::new(id, Spin::Unpolarized) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let mut zk = vec![0.0; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);

        let res = match meta.family {
            libxc_rs::Family::Lda => {
                let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
                let mut output = LdaOutput::new(Some(&mut zk), None, None, None, None, np, Spin::Unpolarized).unwrap();
                func.evaluate_lda(&input, DerivativeOrder::Exc, &mut output, &mut ws)
            }
            libxc_rs::Family::Gga => {
                let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
                let mut output = GgaOutput::new(
                    Some(&mut zk), None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                    np, Spin::Unpolarized,
                ).unwrap();
                func.evaluate_gga(&input, DerivativeOrder::Exc, &mut output, &mut ws)
            }
            libxc_rs::Family::Mgga => {
                let input = MggaInput::new(&rho, &sigma, &lapl, &tau, np, Spin::Unpolarized).unwrap();
                let mut output = MggaOutput {
                    zk: Some(&mut zk),
                    ..Default::default()
                };
                func.evaluate_mgga(&input, DerivativeOrder::Exc, &mut output, &mut ws)
            }
        };

        match res {
            Ok(()) => {
                for (i, &v) in zk.iter().enumerate() {
                    assert!(!v.is_nan(), "Functional {} ({}) produced NaN zk[{i}]", meta.name, raw);
                }
                ok_count += 1;
            }
            Err(e) => {
                println!("EVAL_ERR {raw:3} {:30} {e}", meta.name);
                err_count += 1;
            }
        }
    }

    println!("SWEEP total_ok={ok_count} eval_err={err_count}");
    assert!(
        ok_count >= MIN_EVALUABLE,
        "evaluable count regressed: got {ok_count}, ratchet is {MIN_EVALUABLE}"
    );
}

#[test]
fn test_unsupported_does_not_contain_tier1_corpus() {
    let unsupported: std::collections::HashSet<&str> =
        libxc_reval::routing::UNSUPPORTED.iter().map(|(f, _)| *f).collect();

    for &name in TIER1_CORPUS {
        assert!(
            !unsupported.contains(name),
            "REGRESSION: Tier-1 functional {name} was found in UNSUPPORTED list!"
        );
    }
}

#[test]
fn test_tier1_corpus_evaluates_without_error() {
    let np = 4;
    let rho = [0.1, 0.2, 0.3, 0.4];
    let sigma = [0.01, 0.04, 0.09, 0.16];
    let lapl = [0.0; 4];
    let tau = [0.05, 0.06, 0.07, 0.08];

    for &name in TIER1_CORPUS {
        let id = lookup_by_name(name).unwrap_or_else(|_| panic!("Failed to lookup {name}"));
        let meta = lookup_by_id(id.raw()).unwrap_or_else(|_| panic!("Failed to lookup meta for {name}"));
        let func = Functional::new(id, Spin::Unpolarized)
            .unwrap_or_else(|e| panic!("Failed to construct {name}: {e}"));

        let mut zk = vec![0.0; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);

        match meta.family {
            libxc_rs::Family::Lda => {
                let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();
                let mut output = LdaOutput::new(Some(&mut zk), None, None, None, None, np, Spin::Unpolarized).unwrap();
                func.evaluate_lda(&input, DerivativeOrder::Exc, &mut output, &mut ws)
                    .unwrap_or_else(|e| panic!("evaluate_lda failed on {name}: {e}"));
            }
            libxc_rs::Family::Gga => {
                let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
                let mut output = GgaOutput::new(
                    Some(&mut zk), None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                    np, Spin::Unpolarized,
                ).unwrap();
                func.evaluate_gga(&input, DerivativeOrder::Exc, &mut output, &mut ws)
                    .unwrap_or_else(|e| panic!("evaluate_gga failed on {name}: {e}"));
            }
            libxc_rs::Family::Mgga => {
                let input = MggaInput::new(&rho, &sigma, &lapl, &tau, np, Spin::Unpolarized).unwrap();
                let mut output = MggaOutput {
                    zk: Some(&mut zk),
                    ..Default::default()
                };
                func.evaluate_mgga(&input, DerivativeOrder::Exc, &mut output, &mut ws)
                    .unwrap_or_else(|e| panic!("evaluate_mgga failed on {name}: {e}"));
            }
        }

        for (i, &v) in zk.iter().enumerate() {
            assert!(
                !v.is_nan(),
                "Functional {name} produced NaN zk[{i}]"
            );
        }
    }
}
