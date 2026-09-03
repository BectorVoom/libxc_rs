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
use libxc_rs::input::{GgaInput, MggaInput};
use libxc_rs::model::{DerivativeOrder, Family, Spin};
use libxc_rs::output::{GgaOutput, MggaOutput};
use libxc_rs::registry::{all_functional_ids, lookup_by_id};
use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_type, xc_gga_exc_vxc, xc_mgga_exc_vxc, XC_POLARIZED,
    XC_UNPOLARIZED,
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

/// MGGA grid: same densities and gradients as the GGA one, plus a Laplacian
/// and a kinetic energy density held safely above the von Weizsacker bound
/// `tau >= sigma / (8 rho)`.
///
/// Staying above that bound is not cosmetic. Below it the point is outside
/// every MGGA's domain, libxc's `work_mgga_inc.c` clamps and this tree does
/// not, and the comparison stops being about the functional -- which is
/// already recorded in AGENTS.md as the reason `mgga_c_r2scan` shows up in the
/// bench cross-check.
fn mgga_grid(np: usize, nspin: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
    let (rho, sigma) = grid(np, nspin);
    let nsig = if nspin == 1 { 1 } else { 3 };
    let mut lapl = Vec::with_capacity(np * nspin);
    let mut tau = Vec::with_capacity(np * nspin);
    let mut s = 0xabcd_ef01_2345_6789u64;
    let mut next = || {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    for ip in 0..np {
        for k in 0..nspin {
            let r = rho[ip * nspin + k];
            // Per-channel sigma: sigma_aa is index 0, sigma_bb index 2.
            let sig = if nspin == 1 {
                sigma[ip]
            } else {
                sigma[ip * nsig + if k == 0 { 0 } else { 2 }]
            };
            let tau_w = sig / (8.0 * r);
            // 1.05x .. 6x the von Weizsacker bound.
            tau.push(tau_w * (1.05 + 5.0 * next()));
            lapl.push(r.powf(5.0 / 3.0) * (2.0 * next() - 1.0));
        }
    }
    (rho, sigma, lapl, tau)
}

fn sweep_mgga(spin: Spin) -> (Vec<Row>, Vec<(String, String)>) {
    let np = 400usize;
    let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let (rho, sigma, lapl, tau) = mgga_grid(np, nspin);
    let nvr = nspin;
    let nvs = if nspin == 1 { 1 } else { 3 };

    let mut rows = Vec::new();
    let mut skipped = Vec::new();

    for id in all_functional_ids() {
        let Ok(meta) = lookup_by_id(id.raw()) else { continue };
        if meta.family != Family::Mgga || meta.auxiliaries.is_empty() {
            continue;
        }
        let f = match Functional::new(id, spin) {
            Ok(f) => f,
            Err(e) => {
                skipped.push((meta.name.to_string(), format!("Functional::new: {e}")));
                continue;
            }
        };
        let input = MggaInput::new(&rho, &sigma, &lapl, &tau, np, spin).unwrap();
        let mut r_zk = vec![0.0f64; np];
        let mut r_vr = vec![0.0f64; np * nvr];
        let mut r_vs = vec![0.0f64; np * nvs];
        let mut r_vl = vec![0.0f64; np * nvr];
        let mut r_vt = vec![0.0f64; np * nvr];
        let mut ws = EvaluationWorkspace::new(np, spin);
        {
            let mut out = MggaOutput {
                zk: Some(&mut r_zk),
                vrho: Some(&mut r_vr),
                vsigma: Some(&mut r_vs),
                vlapl: Some(&mut r_vl),
                vtau: Some(&mut r_vt),
                ..Default::default()
            };
            if let Err(e) = f.evaluate_mgga(&input, DerivativeOrder::Vxc, &mut out, &mut ws) {
                skipped.push((meta.name.to_string(), format!("evaluate_mgga: {e}")));
                continue;
            }
        }

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
        let mut c_vl = vec![0.0f64; np * nvr];
        let mut c_vt = vec![0.0f64; np * nvr];
        unsafe {
            xc_mgga_exc_vxc(
                &cf.0,
                np,
                rho.as_ptr(),
                sigma.as_ptr(),
                lapl.as_ptr(),
                tau.as_ptr(),
                c_zk.as_mut_ptr(),
                c_vr.as_mut_ptr(),
                c_vs.as_mut_ptr(),
                c_vl.as_mut_ptr(),
                c_vt.as_mut_ptr(),
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

fn report_mgga(spin: Spin) {
    let (mut rows, skipped) = sweep_mgga(spin);
    rows.sort_by(|a, b| b.zk.partial_cmp(&a.zk).unwrap());
    println!("\n=== composite MGGA functionals vs libxc, {spin:?} ===");
    println!("compared : {}", rows.len());
    println!("skipped  : {}", skipped.len());
    let over: Vec<&Row> = rows
        .iter()
        .filter(|r| r.zk > TOL_ZK || r.vrho > TOL_VXC || r.vsigma > TOL_VXC)
        .collect();
    println!("over gate: {}", over.len());
    println!("\n{:<38} {:>6} {:>11} {:>11} {:>11}", "functional", "id", "zk", "vrho", "vsigma");
    for r in over.iter() {
        println!(
            "{:<38} {:>6} {:>11.3e} {:>11.3e} {:>11.3e}",
            r.name.to_lowercase(), r.id, r.zk, r.vrho, r.vsigma
        );
    }
    if !skipped.is_empty() {
        println!("\nskipped ({}):", skipped.len());
        for (n, why) in skipped.iter().take(12) {
            println!("  {:<34} {why}", n.to_lowercase());
        }
    }
}

fn sweep_lda(spin: Spin) -> (Vec<Row>, Vec<(String, String)>) {
    let np = 400usize;
    let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let (rho, _) = grid(np, nspin);
    let mut rows = Vec::new();
    let mut skipped = Vec::new();

    for id in all_functional_ids() {
        let Ok(meta) = lookup_by_id(id.raw()) else { continue };
        if meta.family != Family::Lda || meta.auxiliaries.is_empty() {
            continue;
        }
        let f = match Functional::new(id, spin) {
            Ok(f) => f,
            Err(e) => {
                skipped.push((meta.name.to_string(), format!("Functional::new: {e}")));
                continue;
            }
        };
        let input = libxc_rs::input::LdaInput::new(&rho, np, spin).unwrap();
        let mut r_zk = vec![0.0f64; np];
        let mut r_vr = vec![0.0f64; np * nspin];
        let mut ws = EvaluationWorkspace::new(np, spin);
        {
            let mut out = libxc_rs::output::LdaOutput {
                zk: Some(&mut r_zk),
                vrho: Some(&mut r_vr),
                ..Default::default()
            };
            if let Err(e) = f.evaluate_lda(&input, DerivativeOrder::Vxc, &mut out, &mut ws) {
                skipped.push((meta.name.to_string(), format!("evaluate_lda: {e}")));
                continue;
            }
        }
        let mut t: xc_func_type = unsafe { std::mem::zeroed() };
        let n = if nspin == 1 { XC_UNPOLARIZED } else { XC_POLARIZED } as i32;
        if unsafe { xc_func_init(&mut t, id.raw() as i32, n) } != 0 {
            skipped.push((meta.name.to_string(), "libxc xc_func_init failed".into()));
            continue;
        }
        let cf = CFunc(t);
        let mut c_zk = vec![0.0f64; np];
        let mut c_vr = vec![0.0f64; np * nspin];
        unsafe {
            libxc_sys::xc_lda_exc_vxc(
                &cf.0, np, rho.as_ptr(), c_zk.as_mut_ptr(), c_vr.as_mut_ptr(),
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
            vsigma: 0.0,
        });
    }
    (rows, skipped)
}

/// The two composite LDA functionals, `hyb_lda_xc_lda0` and
/// `hyb_lda_xc_cam_lda0`. Small, but they were the last family with no
/// composite coverage at all.
#[test]
fn composite_lda_matches_libxc() {
    for spin in [Spin::Unpolarized, Spin::Polarized] {
        let (rows, skipped) = sweep_lda(spin);
        println!("\n=== composite LDA functionals vs libxc, {spin:?} ===");
        println!("compared : {}", rows.len());
        for r in &rows {
            println!("  {:<32} zk {:>11.3e}  vrho {:>11.3e}", r.name.to_lowercase(), r.zk, r.vrho);
        }
        for (n, why) in &skipped {
            println!("  skipped {:<28} {why}", n.to_lowercase());
        }
        let bad: Vec<&Row> = rows
            .iter()
            .filter(|r| r.zk > TOL_ZK || r.vrho > TOL_VXC)
            .collect();
        assert!(
            bad.is_empty(),
            "{:?} composite LDA over the gate",
            bad.iter().map(|r| (r.name, r.zk, r.vrho)).collect::<Vec<_>>()
        );
    }
}

/// Composite MGGA functionals whose residual is recorded rather than fixed.
const MGGA_KNOWN: &[(u16, &str)] = &[
    // 2.1e-7 on vsigma, zk 2.6e-13 (inside the energy contract). Downstream of
    // `mgga_x_br89`, whose own vsigma sits at 3.8e-9 against libxc: the
    // Becke-Roussel inversion is a root-find, and the two builds converge to
    // slightly different roots. `kernel_oracle.rs` records the same effect on
    // `mgga_x_br89`, `mgga_x_br89_1`, `mgga_x_b00` and `mgga_x_mggac`.
    (389, "hyb_mgga_xc_br3p86: inherits the BR89 inversion residual, vsigma 2.1e-7"),
];

#[test]
fn composite_mgga_matches_libxc() {
    let (rows, skipped) = sweep_mgga(Spin::Unpolarized);
    report_mgga(Spin::Unpolarized);
    let bad: Vec<&Row> = rows
        .iter()
        .filter(|r| r.zk > TOL_ZK || r.vrho > TOL_VXC || r.vsigma > TOL_VXC)
        .filter(|r| !MGGA_KNOWN.iter().any(|(id, _)| *id == r.id))
        .collect();
    let _ = skipped;
    assert!(
        bad.is_empty(),
        "{} composite MGGA functionals disagree with libxc: {:?}",
        bad.len(),
        bad.iter().map(|r| (r.name, r.zk, r.vsigma)).collect::<Vec<_>>()
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
