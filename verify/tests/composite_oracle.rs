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
    xc_func_end, xc_func_init, xc_func_type, xc_gga_exc_vxc, xc_mgga, xc_mgga_exc_vxc, XC_POLARIZED,
    XC_UNPOLARIZED,
};

/// Functionals allowed to exceed the gate, each with the reason.
///
/// Both reasons are structural and predate the composite work; neither is a
/// mixing fault. A functional not on this list must meet the gate.
const KNOWN_GAPS: &[(u16, &str)] = &[
    // gga_k_gds08 / ghds10 / ghds10r / tkvln used to be listed here: they
    // mix `lda_k_gds08_worker` (id 100001, libxc-internal), which this
    // library had no kernel for, and evaluated with that component missing.
    // Since 2026-09-11 the worker has a kernel, reached only as their
    // auxiliary (`libxc_core::meta::internal_auxiliary`), and all four are
    // under the gate like every other composite.
    // gga_xc_beefvdw (zk 1.6e-10) was listed here as a floating-point
    // contraction outlier. That was the oracle's `-march=native` build; against
    // a wheel-built libxc it passes the gate (2026-09-11).
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

// ---------------------------------------------------------------------------
// Composite MGGAs above the first derivative
// ---------------------------------------------------------------------------

/// `MggaOutput`'s fields in declaration order -- which is also the order of
/// libxc's `xc_mgga` output arguments -- with each field's derivative order.
const MGGA_FIELDS: [(&str, u8); 70] = [
    ("zk", 0), ("vrho", 1), ("vsigma", 1), ("vlapl", 1), ("vtau", 1), ("v2rho2", 2),
    ("v2rhosigma", 2), ("v2rholapl", 2), ("v2rhotau", 2), ("v2sigma2", 2),
    ("v2sigmalapl", 2), ("v2sigmatau", 2), ("v2lapl2", 2), ("v2lapltau", 2), ("v2tau2", 2),
    ("v3rho3", 3), ("v3rho2sigma", 3), ("v3rho2lapl", 3), ("v3rho2tau", 3),
    ("v3rhosigma2", 3), ("v3rhosigmalapl", 3), ("v3rhosigmatau", 3), ("v3rholapl2", 3),
    ("v3rholapltau", 3), ("v3rhotau2", 3), ("v3sigma3", 3), ("v3sigma2lapl", 3),
    ("v3sigma2tau", 3), ("v3sigmalapl2", 3), ("v3sigmalapltau", 3), ("v3sigmatau2", 3),
    ("v3lapl3", 3), ("v3lapl2tau", 3), ("v3lapltau2", 3), ("v3tau3", 3), ("v4rho4", 4),
    ("v4rho3sigma", 4), ("v4rho3lapl", 4), ("v4rho3tau", 4), ("v4rho2sigma2", 4),
    ("v4rho2sigmalapl", 4), ("v4rho2sigmatau", 4), ("v4rho2lapl2", 4),
    ("v4rho2lapltau", 4), ("v4rho2tau2", 4), ("v4rhosigma3", 4), ("v4rhosigma2lapl", 4),
    ("v4rhosigma2tau", 4), ("v4rhosigmalapl2", 4), ("v4rhosigmalapltau", 4),
    ("v4rhosigmatau2", 4), ("v4rholapl3", 4), ("v4rholapl2tau", 4), ("v4rholapltau2", 4),
    ("v4rhotau3", 4), ("v4sigma4", 4), ("v4sigma3lapl", 4), ("v4sigma3tau", 4),
    ("v4sigma2lapl2", 4), ("v4sigma2lapltau", 4), ("v4sigma2tau2", 4), ("v4sigmalapl3", 4),
    ("v4sigmalapl2tau", 4), ("v4sigmalapltau2", 4), ("v4sigmatau3", 4), ("v4lapl4", 4),
    ("v4lapl3tau", 4), ("v4lapl2tau2", 4), ("v4lapltau3", 4), ("v4tau4", 4),
];

fn mgga_widths(d: &libxc_rs::Dimensions) -> [usize; 70] {
    [
        d.zk as usize, d.vrho as usize, d.vsigma as usize, d.vlapl as usize,
        d.vtau as usize, d.v2rho2 as usize, d.v2rhosigma as usize, d.v2rholapl as usize,
        d.v2rhotau as usize, d.v2sigma2 as usize, d.v2sigmalapl as usize,
        d.v2sigmatau as usize, d.v2lapl2 as usize, d.v2lapltau as usize, d.v2tau2 as usize,
        d.v3rho3 as usize, d.v3rho2sigma as usize, d.v3rho2lapl as usize,
        d.v3rho2tau as usize, d.v3rhosigma2 as usize, d.v3rhosigmalapl as usize,
        d.v3rhosigmatau as usize, d.v3rholapl2 as usize, d.v3rholapltau as usize,
        d.v3rhotau2 as usize, d.v3sigma3 as usize, d.v3sigma2lapl as usize,
        d.v3sigma2tau as usize, d.v3sigmalapl2 as usize, d.v3sigmalapltau as usize,
        d.v3sigmatau2 as usize, d.v3lapl3 as usize, d.v3lapl2tau as usize,
        d.v3lapltau2 as usize, d.v3tau3 as usize, d.v4rho4 as usize,
        d.v4rho3sigma as usize, d.v4rho3lapl as usize, d.v4rho3tau as usize,
        d.v4rho2sigma2 as usize, d.v4rho2sigmalapl as usize, d.v4rho2sigmatau as usize,
        d.v4rho2lapl2 as usize, d.v4rho2lapltau as usize, d.v4rho2tau2 as usize,
        d.v4rhosigma3 as usize, d.v4rhosigma2lapl as usize, d.v4rhosigma2tau as usize,
        d.v4rhosigmalapl2 as usize, d.v4rhosigmalapltau as usize,
        d.v4rhosigmatau2 as usize, d.v4rholapl3 as usize, d.v4rholapl2tau as usize,
        d.v4rholapltau2 as usize, d.v4rhotau3 as usize, d.v4sigma4 as usize,
        d.v4sigma3lapl as usize, d.v4sigma3tau as usize, d.v4sigma2lapl2 as usize,
        d.v4sigma2lapltau as usize, d.v4sigma2tau2 as usize, d.v4sigmalapl3 as usize,
        d.v4sigmalapl2tau as usize, d.v4sigmalapltau2 as usize, d.v4sigmatau3 as usize,
        d.v4lapl4 as usize, d.v4lapl3tau as usize, d.v4lapl2tau2 as usize,
        d.v4lapltau3 as usize, d.v4tau4 as usize,
    ]
}

/// One buffer per field: `np * width` for the fields of `order` and below,
/// empty (a `NULL` / `None`) above it.
fn mgga_buffers(d: &libxc_rs::Dimensions, np: usize, order: DerivativeOrder) -> Vec<Vec<f64>> {
    mgga_widths(d)
        .iter()
        .zip(MGGA_FIELDS.iter())
        .map(|(w, (_, o))| if *o <= order as u8 { vec![0.0; w * np] } else { Vec::new() })
        .collect()
}

fn mgga_output(b: &mut [Vec<f64>]) -> MggaOutput<'_> {
    let mut it = b.iter_mut().map(|v| if v.is_empty() { None } else { Some(v.as_mut_slice()) });
    let mut nx = || it.next().unwrap();
    MggaOutput {
        zk: nx(), vrho: nx(), vsigma: nx(), vlapl: nx(), vtau: nx(), v2rho2: nx(),
        v2rhosigma: nx(), v2rholapl: nx(), v2rhotau: nx(), v2sigma2: nx(),
        v2sigmalapl: nx(), v2sigmatau: nx(), v2lapl2: nx(), v2lapltau: nx(), v2tau2: nx(),
        v3rho3: nx(), v3rho2sigma: nx(), v3rho2lapl: nx(), v3rho2tau: nx(),
        v3rhosigma2: nx(), v3rhosigmalapl: nx(), v3rhosigmatau: nx(), v3rholapl2: nx(),
        v3rholapltau: nx(), v3rhotau2: nx(), v3sigma3: nx(), v3sigma2lapl: nx(),
        v3sigma2tau: nx(), v3sigmalapl2: nx(), v3sigmalapltau: nx(), v3sigmatau2: nx(),
        v3lapl3: nx(), v3lapl2tau: nx(), v3lapltau2: nx(), v3tau3: nx(), v4rho4: nx(),
        v4rho3sigma: nx(), v4rho3lapl: nx(), v4rho3tau: nx(), v4rho2sigma2: nx(),
        v4rho2sigmalapl: nx(), v4rho2sigmatau: nx(), v4rho2lapl2: nx(),
        v4rho2lapltau: nx(), v4rho2tau2: nx(), v4rhosigma3: nx(), v4rhosigma2lapl: nx(),
        v4rhosigma2tau: nx(), v4rhosigmalapl2: nx(), v4rhosigmalapltau: nx(),
        v4rhosigmatau2: nx(), v4rholapl3: nx(), v4rholapl2tau: nx(), v4rholapltau2: nx(),
        v4rhotau3: nx(), v4sigma4: nx(), v4sigma3lapl: nx(), v4sigma3tau: nx(),
        v4sigma2lapl2: nx(), v4sigma2lapltau: nx(), v4sigma2tau2: nx(), v4sigmalapl3: nx(),
        v4sigmalapl2tau: nx(), v4sigmalapltau2: nx(), v4sigmatau3: nx(), v4lapl4: nx(),
        v4lapl3tau: nx(), v4lapl2tau2: nx(), v4lapltau3: nx(), v4tau4: nx(),
    }
}

/// libxc's generic `xc_mgga`, every field of `b` that is non-empty requested.
fn c_mgga(cf: &CFunc, np: usize, rho: &[f64], sigma: &[f64], lapl: &[f64], tau: &[f64], b: &mut [Vec<f64>]) {
    let p: Vec<*mut f64> = b
        .iter_mut()
        .map(|v| if v.is_empty() { std::ptr::null_mut() } else { v.as_mut_ptr() })
        .collect();
    unsafe {
        xc_mgga(
            &cf.0, np, rho.as_ptr(), sigma.as_ptr(), lapl.as_ptr(), tau.as_ptr(),
            p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11],
            p[12], p[13], p[14], p[15], p[16], p[17], p[18], p[19], p[20], p[21], p[22],
            p[23], p[24], p[25], p[26], p[27], p[28], p[29], p[30], p[31], p[32], p[33],
            p[34], p[35], p[36], p[37], p[38], p[39], p[40], p[41], p[42], p[43], p[44],
            p[45], p[46], p[47], p[48], p[49], p[50], p[51], p[52], p[53], p[54], p[55],
            p[56], p[57], p[58], p[59], p[60], p[61], p[62], p[63], p[64], p[65], p[66],
            p[67], p[68], p[69],
        );
    }
}

/// Worst relative difference over one field, each point measured against the
/// largest libxc component at that point (the rule `kernel_oracle_fxc.rs`
/// uses: a derivative component that is small next to its siblings is not a
/// meaningful denominator).
fn worst_pointwise(ours: &[f64], theirs: &[f64], width: usize) -> f64 {
    let mut w = 0.0f64;
    for (a, b) in ours.chunks(width.max(1)).zip(theirs.chunks(width.max(1))) {
        let ps = b.iter().fold(0.0f64, |m, v| m.max(v.abs())).max(f64::MIN_POSITIVE);
        for (x, y) in a.iter().zip(b.iter()) {
            if x == y || !x.is_finite() || !y.is_finite() {
                continue;
            }
            w = w.max((x - y).abs() / ps);
        }
    }
    w
}

/// `(name, id, worst field, worst difference, nonzero libxc values compared,
/// of which bit-identical)` for every composite MGGA that claims `order`,
/// comparing every field of exactly that order.
#[allow(clippy::type_complexity)]
fn sweep_mgga_order(
    spin: Spin,
    order: DerivativeOrder,
) -> (Vec<(&'static str, u16, &'static str, f64, usize, usize)>, Vec<(String, String)>) {
    use libxc_rs::model::FunctionalFlags;
    let np = 200usize;
    let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let (rho, sigma, lapl, tau) = mgga_grid(np, nspin);
    let d = libxc_rs::Dimensions::mgga(spin);
    let widths = mgga_widths(&d);
    let claim = [
        FunctionalFlags::HAVE_EXC,
        FunctionalFlags::HAVE_VXC,
        FunctionalFlags::HAVE_FXC,
        FunctionalFlags::HAVE_KXC,
        FunctionalFlags::HAVE_LXC,
    ][order as usize];

    let mut rows = Vec::new();
    let mut skipped = Vec::new();
    for id in all_functional_ids() {
        let Ok(meta) = lookup_by_id(id.raw()) else { continue };
        if meta.family != Family::Mgga || meta.auxiliaries.is_empty() || !meta.flags.contains(claim) {
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
        let mut ours = mgga_buffers(&d, np, order);
        let mut ws = EvaluationWorkspace::new(np, spin);
        if let Err(e) = f.evaluate_mgga(&input, order, &mut mgga_output(&mut ours), &mut ws) {
            skipped.push((meta.name.to_string(), format!("evaluate_mgga: {e}")));
            continue;
        }
        let mut t: xc_func_type = unsafe { std::mem::zeroed() };
        let n = if nspin == 1 { XC_UNPOLARIZED } else { XC_POLARIZED } as i32;
        if unsafe { xc_func_init(&mut t, id.raw() as i32, n) } != 0 {
            skipped.push((meta.name.to_string(), "libxc xc_func_init failed".into()));
            continue;
        }
        let cf = CFunc(t);
        let mut theirs = mgga_buffers(&d, np, order);
        c_mgga(&cf, np, &rho, &sigma, &lapl, &tau, &mut theirs);

        let (mut worst, mut field, mut nonzero, mut same) = (0.0f64, "-", 0usize, 0usize);
        // `mgga_c_scanl_vv10` and `_rvv10` mix a deorbitalized functional.
        let over_deorbitalized = order == DerivativeOrder::Lxc
            && meta.auxiliaries.iter().any(|(a, _)| libxc_rs::meta::deorbitalized(*a).is_some());
        for (k, (name, o)) in MGGA_FIELDS.iter().enumerate() {
            if *o != order as u8 || widths[k] == 0 {
                continue;
            }
            if over_deorbitalized && LIBXC_UNZEROED.contains(name) {
                assert!(ours[k].iter().all(|v| v.is_finite()), "{} {name}: not finite", meta.name);
                continue;
            }
            for (x, y) in ours[k].iter().zip(theirs[k].iter()) {
                if *y != 0.0 {
                    nonzero += 1;
                    same += usize::from(x.to_bits() == y.to_bits());
                }
            }
            let w = worst_pointwise(&ours[k], &theirs[k], widths[k]);
            if w > worst {
                worst = w;
                field = name;
            }
        }
        rows.push((meta.name, id.raw(), field, worst, nonzero, same));
    }
    (rows, skipped)
}

/// Fourth-derivative fields libxc's own deorbitalization computes from
/// uninitialized memory, so no comparison against it means anything.
///
/// `xc_mgga_vars_allocate_all` (`deorbitalize_func.c:188-189`) mallocs the base
/// meta-GGA's `v4sigmalapltau2` buffer and then memsets `v4sigmalapl2tau` a
/// second time instead of it. `xc_mgga` does not repair that: it zeroes a
/// lapl-tau cross field only for a functional with both `NEEDS_LAPLACIAN` and
/// `NEEDS_TAU` (`mgga.c:262`), and none of the SCAN-L bases needs the
/// Laplacian -- so the kernel, which accumulates with `+=`, adds nothing to
/// whatever the heap held, and `maple2c/deorbitalize_4.c` reads
/// `mgga_v4sigmalapltau2` into exactly these six outputs. On a fresh heap it
/// happens to be zero and the two libraries agree bit for bit; after a sweep
/// has churned the allocator, libxc returns noise (`v4sigma3lapl` 1.04
/// relative, `mgga_c_scanl_vv10`, polarized, 2026-09-11). This tree zeroes the
/// buffer, which is what libxc's allocator plainly means to do.
const LIBXC_UNZEROED: [&str; 6] = [
    "v4rho2sigmalapl", "v4rhosigma2lapl", "v4rhosigmalapl2",
    "v4sigma3lapl", "v4sigma2lapl2", "v4sigmalapl3",
];

/// Gate for composite MGGAs above the first derivative. Pointwise-scaled, as
/// `kernel_oracle_fxc.rs` gates second derivatives.
const TOL_HIGHER: f64 = 1e-8;

/// Every composite MGGA at every order from vxc to lxc that its flags claim,
/// both spins, every field of that order, against libxc's own `xc_mgga`.
///
/// Until 2026-09-11 the mix handed its auxiliaries buffers only through second
/// order, so all 36 composite MGGAs refused kxc and lxc, and
/// `hyb_mgga_xc_b0kcis` refused everything above vxc. This is the first
/// third- and fourth-derivative coverage the tree has had.
#[test]
fn composite_mgga_higher_orders_match_libxc() {
    let mut bad = Vec::new();
    for spin in [Spin::Unpolarized, Spin::Polarized] {
        for order in [DerivativeOrder::Vxc, DerivativeOrder::Fxc, DerivativeOrder::Kxc, DerivativeOrder::Lxc] {
            let (mut rows, skipped) = sweep_mgga_order(spin, order);
            rows.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap());
            println!("\n=== composite MGGA vs libxc, {order:?} {spin:?} ===");
            let (nz, eq): (usize, usize) = rows.iter().fold((0, 0), |a, r| (a.0 + r.4, a.1 + r.5));
            println!("compared : {} functionals, {nz} nonzero libxc values, {eq} bit-identical", rows.len());
            for (n, why) in &skipped {
                println!("  skipped {:<30} {why}", n.to_lowercase());
            }
            for (n, _, fld, w, _, _) in rows.iter().take(5) {
                println!("  {:<32} {:>14} {:>10.3e}", n.to_lowercase(), fld, w);
            }
            for r in rows.iter().filter(|r| r.3 > TOL_HIGHER) {
                bad.push(format!("{} {order:?} {spin:?}: {} {:.3e}", r.0.to_lowercase(), r.2, r.3));
            }
            // A comparison of two all-zero buffers passes any gate. Every
            // composite MGGA has a nonzero rho derivative at every order.
            for r in rows.iter().filter(|r| r.4 == 0) {
                bad.push(format!("{} {order:?} {spin:?}: compared no nonzero values", r.0.to_lowercase()));
            }
        }
    }
    assert!(bad.is_empty(), "{} over the gate:\n{}", bad.len(), bad.join("\n"));
}

/// `lda_k_gds08_worker`'s metadata is written by hand
/// (`libxc_core::meta::internal_auxiliary`): the worker is not in libxc's
/// public header, so nothing generates it. This holds it to libxc's own info
/// block, which `xc_func_init` does resolve for id 100001.
#[test]
fn gds08_worker_metadata_matches_libxc() {
    use libxc_rs::meta::{internal_auxiliary, LDA_K_GDS08_WORKER};
    let m = internal_auxiliary(LDA_K_GDS08_WORKER).expect("worker metadata");
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    assert_eq!(unsafe { xc_func_init(&mut t, 100001, XC_UNPOLARIZED as i32) }, 0);
    let cf = CFunc(t);
    let info = unsafe { &*cf.0.info };
    assert_eq!(info.number, 100001);
    assert_eq!(info.flags as u32, m.flags.bits(), "flags");
    assert_eq!(info.kind, m.kind as i32, "kind");
    assert_eq!(info.family, m.family as i32, "family");
    assert_eq!(info.dens_threshold.to_bits(), m.default_density_threshold.to_bits());
    let ep = &info.ext_params;
    assert_eq!(ep.n as usize, m.ext_params.len());
    for (k, spec) in m.ext_params.iter().enumerate() {
        let name = unsafe { std::ffi::CStr::from_ptr(*ep.names.add(k)) };
        assert_eq!(name.to_str().unwrap(), spec.name);
        assert_eq!(unsafe { *ep.values.add(k) }.to_bits(), spec.default_value.to_bits(), "{}", spec.name);
    }
    // And it is not reachable from the public registry.
    assert!(lookup_by_id(LDA_K_GDS08_WORKER.raw()).is_err());
}
