
use libxc_rs::{lookup_by_id, DerivativeOrder, EvaluationWorkspace, Functional, LdaInput, LdaOutput, GgaInput, GgaOutput, MggaInput, MggaOutput, Spin};

fn main() {
    let np = 4;
    let rho = [0.1, 0.2, 0.3, 0.4];
    let sigma = [0.01, 0.04, 0.09, 0.16];
    let lapl = [0.0; 4];
    let tau = [0.05, 0.06, 0.07, 0.08];

    for raw in 1..=800 {
        let meta = match lookup_by_id(raw) {
            Ok(m) => m,
            Err(_) => continue,
        };
        let id = meta.id;
        let func = match Functional::new(id, Spin::Unpolarized) {
            Ok(f) => f,
            Err(e) => {
                println!("CTOR_ERR {raw} {}: {e}", meta.name);
                continue;
            }
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
        if let Err(e) = res {
            println!("EVAL_ERR {raw:3} {:30} {e}", meta.name);
        }
    }
}
