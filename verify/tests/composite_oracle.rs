//! Every composite (`xc_mix_init`) GGA functional against C libxc.
//!
//! # Why this exists
//!
//! HSE06 evaluated to the wrong function for as long as this tree existed,
//! because the screening parameter its auxiliary needs never reached the
//! kernel. Nothing caught it: the hybrid tests only query coefficients, and
//! the per-kernel oracles test each auxiliary at *its own* default parameters,
//! which is exactly the case a parent that overrides them does not exercise.
//!
//! libxc has 28 `xc_func_set_ext_params_name(p->func_aux[...], ...)` calls
//! across 10 source files. Fixing them one at a time by reading C is slow and
//! proves nothing about the ones not read. This sweeps every composite GGA the
//! registry knows and compares the whole evaluation against libxc, so a
//! parameter that fails to reach an auxiliary shows up as a number regardless
//! of which mechanism was supposed to deliver it.
//!
//! A functional this library cannot evaluate is reported as skipped with the
//! error, not silently passed.

use libxc_rs::eval::workspace::EvaluationWorkspace;
use libxc_rs::functional::Functional;
use libxc_rs::input::GgaInput;
use libxc_rs::model::{DerivativeOrder, Family, Spin};
use libxc_rs::output::GgaOutput;
use libxc_rs::registry::{all_functional_ids, lookup_by_id};
use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_type, xc_gga_exc_vxc, XC_POLARIZED, XC_UNPOLARIZED,
};

/// Functionals allowed to exceed the gate, each with the reason.
///
/// Both reasons are structural and predate the composite work; neither is a
/// mixing fault. A functional not on this list must meet the gate.
const KNOWN_GAPS: &[(u16, &str)] = &[
    // These four mix an internal libxc worker functional (id ~100001) that the
    // public registry does not expose, so `Functional::new` drops it and the
    // mix is missing a whole component. Nothing to do with parameters: they
    // disagree by the same amount with and without the override table.
    (591, "gga_k_gds08: aux list is missing libxc's internal worker functional"),
    (592, "gga_k_ghds10: aux list is missing libxc's internal worker functional"),
    (593, "gga_k_ghds10r: aux list is missing libxc's internal worker functional"),
    (594, "gga_k_tkvln: aux list is missing libxc's internal worker functional"),
    // Already on AGENTS.md's oracle-outlier list. zk 1.6e-10, the same
    // floating-point contraction floor described on TOL_VXC, amplified by a
    // 30-term Bayesian expansion.
    (286, "gga_xc_beefvdw: known FP-contraction outlier, zk 1.6e-10"),
];

/// Energy density: the project's stated contract.
const TOL_ZK: f64 = 1e-12;
/// Potentials: the floor set by GCC contracting `a*b + c` into FMA where rustc
/// does not. See `hse06_oracle.rs` for the measurement behind this.
const TOL_VXC: f64 = 1e-9;

fn grid(np: usize, nspin: usize) -> (Vec<f64>, Vec<f64>) {
    let mut s = 0x5eed_1234_abcd_ef01u64;
    let mut next = || {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let nsig = if nspin == 1 { 1 } else { 3 };
    let mut rho = Vec::with_capacity(np * nspin);
    let mut sigma = Vec::with_capacity(np * nsig);
    for _ in 0..np {
        let mut r = [0.0f64; 2];
        for k in 0..nspin {
            // 1e-4 .. 1e1: the chemically active band. Deliberately not the
            // 1e-8 tail, where `gga_x_wpbeh`'s `vsigma` is ill-conditioned in
            // both libraries (see verify/tests/wpbeh_domain.rs) and a relative
            // comparison stops measuring correctness.
            r[k] = 10f64.powf(-4.0 + 5.0 * next());
            rho.push(r[k]);
        }
        if nspin == 1 {
            let g = r[0].powf(4.0 / 3.0) * (0.1 + 2.0 * next());
            sigma.push(g * g);
        } else {
            let ga = r[0].powf(4.0 / 3.0) * (0.1 + 2.0 * next());
            let gb = r[1].powf(4.0 / 3.0) * (0.1 + 2.0 * next());
            sigma.push(ga * ga);
            sigma.push(ga * gb * (2.0 * next() - 1.0));
            sigma.push(gb * gb);
        }
    }
    (rho, sigma)
}

struct CFunc(xc_func_type);
impl Drop for CFunc {
    fn drop(&mut self) {
        unsafe { xc_func_end(&mut self.0) };
    }
}

fn worst_rel(a: &[f64], b: &[f64], scale: f64) -> f64 {
    let mut worst = 0.0f64;
    for (x, y) in a.iter().zip(b.iter()) {
        if x == y || !x.is_finite() || !y.is_finite() {
            continue;
        }
        // Both sides negligible against the functional's own magnitude:
        // cancellation dust, not signal. Same rule the rayon oracle uses.
        if x.abs() < scale * 1e-12 && y.abs() < scale * 1e-12 {
            continue;
        }
        let d = if y.abs() > 0.0 {
            ((x - y) / y).abs()
        } else {
            (x - y).abs()
        };
        if d > worst {
            worst = d;
        }
    }
    worst
}

struct Row {
    name: &'static str,
    id: u16,
    zk: f64,
    vrho: f64,
    vsigma: f64,
}

fn sweep(spin: Spin) -> (Vec<Row>, Vec<(String, String)>) {
    let np = 400usize;
    let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let (rho, sigma) = grid(np, nspin);
    let nvr = nspin;
    let nvs = if nspin == 1 { 1 } else { 3 };

    let mut rows = Vec::new();
    let mut skipped = Vec::new();

    for id in all_functional_ids() {
        let meta = match lookup_by_id(id.raw()) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.family != Family::Gga || meta.auxiliaries.is_empty() {
            continue;
        }

        // --- Rust side. Skip, with the reason, anything we cannot evaluate.
        let f = match Functional::new(id, spin) {
            Ok(f) => f,
            Err(e) => {
                skipped.push((meta.name.to_string(), format!("Functional::new: {e}")));
                continue;
            }
        };
        let input = GgaInput::new(&rho, &sigma, np, spin).unwrap();
        let mut r_zk = vec![0.0f64; np];
        let mut r_vr = vec![0.0f64; np * nvr];
        let mut r_vs = vec![0.0f64; np * nvs];
        let mut ws = EvaluationWorkspace::new(np, spin);
        {
            let mut out = GgaOutput {
                zk: Some(&mut r_zk),
                vrho: Some(&mut r_vr),
                vsigma: Some(&mut r_vs),
                ..Default::default()
            };
            // A refusal here is the intended behaviour, not a failure. An
            // auxiliary whose libxc setter transforms its ext_params rather
            // than copying them (`gga_x_mpw91` writes seven struct fields from
            // three parameters) is on `extract_params.py`'s refusal list, so
            // its `dispatch_with` rejects runtime values instead of applying
            // the wrong ones. `gga_xc_opwlyp_d` is such a case: its parent
            // overrides the auxiliary's parameters, the auxiliary cannot
            // accept them, and an explicit error beats silently evaluating
            // with the default constants -- which is what it used to do.
            if let Err(e) = f.evaluate_gga(&input, DerivativeOrder::Vxc, &mut out, &mut ws) {
                skipped.push((meta.name.to_string(), format!("evaluate_gga: {e}")));
                continue;
            }
        }

        // --- C side.
        let mut t: xc_func_type = unsafe { std::mem::zeroed() };
        let rc = unsafe {
            xc_func_init(
                &mut t,
                id.raw() as i32,
                if nspin == 1 { XC_UNPOLARIZED } else { XC_POLARIZED } as i32,
            )
        };
        if rc != 0 {
            skipped.push((meta.name.to_string(), "libxc xc_func_init failed".into()));
            continue;
        }
        let cf = CFunc(t);
        let mut c_zk = vec![0.0f64; np];
        let mut c_vr = vec![0.0f64; np * nvr];
        let mut c_vs = vec![0.0f64; np * nvs];
        unsafe {
            xc_gga_exc_vxc(
                &cf.0,
                np,
                rho.as_ptr(),
                sigma.as_ptr(),
                c_zk.as_mut_ptr(),
                c_vr.as_mut_ptr(),
                c_vs.as_mut_ptr(),
            );
        }

        let scale = c_zk.iter().fold(0.0f64, |m, v| m.max(v.abs()));
        if scale == 0.0 || !scale.is_finite() {
            skipped.push((meta.name.to_string(), "libxc produced no finite zk".into()));
            continue;
        }
        rows.push(Row {
            name: meta.name,
            id: id.raw(),
            zk: worst_rel(&r_zk, &c_zk, scale),
            vrho: worst_rel(&r_vr, &c_vr, scale),
            vsigma: worst_rel(&r_vs, &c_vs, scale),
        });
    }
    (rows, skipped)
}

fn report(spin: Spin) {
    let (mut rows, skipped) = sweep(spin);
    rows.sort_by(|a, b| b.zk.partial_cmp(&a.zk).unwrap());

    println!("\n=== composite GGA functionals vs libxc, {spin:?} ===");
    println!("compared : {}", rows.len());
    println!("skipped  : {}", skipped.len());

    let over: Vec<&Row> = rows
        .iter()
        .filter(|r| r.zk > TOL_ZK || r.vrho > TOL_VXC || r.vsigma > TOL_VXC)
        .collect();
    let bad: Vec<&&Row> = over
        .iter()
        .filter(|r| !KNOWN_GAPS.iter().any(|(id, _)| *id == r.id))
        .collect();
    let known: Vec<&&Row> = over
        .iter()
        .filter(|r| KNOWN_GAPS.iter().any(|(id, _)| *id == r.id))
        .collect();

    println!(
        "over gate: {} unexpected + {} known (zk {TOL_ZK:e}, vrho/vsigma {TOL_VXC:e})",
        bad.len(),
        known.len()
    );
    for r in &known {
        let why = KNOWN_GAPS.iter().find(|(id, _)| *id == r.id).unwrap().1;
        println!("  known gap: {why}  (zk {:.3e})", r.zk);
    }
    if !bad.is_empty() {
        println!("\n{:<38} {:>6} {:>11} {:>11} {:>11}", "functional", "id", "zk", "vrho", "vsigma");
        for r in &bad {
            println!(
                "{:<38} {:>6} {:>11.3e} {:>11.3e} {:>11.3e}",
                r.name.to_lowercase(),
                r.id,
                r.zk,
                r.vrho,
                r.vsigma
            );
        }
    }
    println!("\nworst 8 by zk (whether or not they pass):");
    for r in rows.iter().take(8) {
        println!(
            "  {:<36} {:>11.3e} {:>11.3e} {:>11.3e}",
            r.name.to_lowercase(),
            r.zk,
            r.vrho,
            r.vsigma
        );
    }
    if !skipped.is_empty() {
        println!("\nskipped:");
        let mut by_reason: std::collections::BTreeMap<String, Vec<String>> = Default::default();
        for (n, why) in &skipped {
            let key = why.split(':').next().unwrap_or(why).to_string();
            by_reason.entry(key).or_default().push(n.to_lowercase());
        }
        for (why, names) in by_reason {
            println!("  {:3} {why}", names.len());
            println!("      e.g. {}", names.iter().take(4).cloned().collect::<Vec<_>>().join(", "));
        }
    }

    assert!(
        bad.is_empty(),
        "{} composite GGA functionals disagree with libxc beyond the gate; \
         see the table above",
        bad.len()
    );
}

#[test]
fn composite_gga_unpolarized_matches_libxc() {
    report(Spin::Unpolarized);
}

#[test]
fn composite_gga_polarized_matches_libxc() {
    report(Spin::Polarized);
}
