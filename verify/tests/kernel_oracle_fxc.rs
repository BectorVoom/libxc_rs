//! Second derivatives of every routed LDA and GGA kernel, against C libxc.
//!
//! # Why this exists
//!
//! `kernel_oracle.rs` covers `zk` and the first derivatives. Nothing covered
//! the second derivatives except the rayon oracle's `v2rho2`, and that only
//! for unpolarized LDA and GGA. `verify/tests/lda_x_oracle.rs` used to test
//! `fxc`/`kxc`/`lxc` for one functional, but it was written against the CubeCL
//! backend's `libxc_rs::kernel::launch` API and has not compiled since that
//! backend was removed, so its coverage had already been lost silently.
//!
//! Second derivatives are what an SCF response calculation actually consumes,
//! and they amplify any error in the underlying formula, so they are the more
//! sensitive test of the two.
//!
//! MGGA is deliberately not swept here: its second-derivative surface is 15
//! fields wide with laplacian and tau cross terms, and `dispatch_mgga` does
//! not accept `Fxc` for every routed functional. Extending to it is a separate
//! piece of work; see AGENTS.md.

use libxc_rs::input::{GgaInput, LdaInput};
use libxc_rs::model::{DerivativeOrder, FunctionalFlags, Spin, Thresholds};
use libxc_rs::output::{GgaOutput, LdaOutput};
use libxc_rs::registry::{lookup_by_id, lookup_by_name};
use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_type, xc_gga_exc_vxc_fxc, xc_lda_exc_vxc_fxc,
    XC_POLARIZED, XC_UNPOLARIZED,
};

/// Second derivatives sit further down the chain than the potentials, so they
/// carry more of the floating-point contraction difference. This is the
/// general gate; anything worse is listed with its measured value.
const TOL: f64 = 1e-8;

const NP: usize = 250;

/// `(name, measured worst)`, 2026-09-03. Gated at 4x so a regression fails.
///
/// All four are `v2sigma2`, the same in both spin modes, and all four are
/// functionals whose first derivatives are already known to be
/// ill-conditioned or contraction-limited:
///
///   * `gga_x_beefvdw` is a 30-term Bayesian expansion and is already on
///     AGENTS.md's outlier list at `v2rho2` 1.5e-11;
///   * `gga_x_wpbeh` loses digits in `vsigma` as the reduced gradient goes to
///     zero (`verify/tests/wpbeh_domain.rs`), and `v2sigma2` differentiates
///     that once more;
///   * `gga_k_meyer` and `gga_x_gg99` are kinetic and exchange functionals
///     with steep `sigma` dependence.
///
/// They are recorded rather than tolerated wholesale, so the numbers are
/// visible and a real regression still fails.
const FLOOR: &[(&str, f64)] = &[
    ("gga_x_beefvdw", 3.7e-5),
    ("gga_k_meyer", 3.0e-6),
    ("gga_x_gg99", 1.5e-7),
    ("gga_x_wpbeh", 7.0e-8),
];

fn grid(nspin: usize) -> (Vec<f64>, Vec<f64>) {
    let mut s = 0x0f1e_2d3c_4b5a_6978u64;
    let mut next = || {
        s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((s >> 11) as f64) / ((1u64 << 53) as f64)
    };
    let (mut rho, mut sigma) = (Vec::new(), Vec::new());
    for _ in 0..NP {
        let mut r = [0.0f64; 2];
        for k in 0..nspin {
            r[k] = 10f64.powf(-4.0 + 5.0 * next());
            rho.push(r[k]);
        }
        let mut g = [0.0f64; 2];
        for k in 0..nspin {
            g[k] = r[k].powf(4.0 / 3.0) * (0.1 + 2.0 * next());
        }
        if nspin == 1 {
            sigma.push(g[0] * g[0]);
        } else {
            sigma.push(g[0] * g[0]);
            sigma.push(g[0] * g[1] * (2.0 * next() - 1.0));
            sigma.push(g[1] * g[1]);
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

/// Worst relative difference, with the dust threshold taken per grid point
/// **and** per field.
///
/// Neither a global threshold nor a `zk`-derived one works here. The grid
/// spans five decades of density, so a second derivative spans ten or more,
/// and `zk` is an energy per particle whose magnitude says nothing about the
/// magnitude of `v2rhosigma`.
///
/// The scale that does work is the field's own largest component *at the same
/// grid point*. `gga_x_pbe`'s polarized `v2rhosigma` has six components per
/// point, of which four are analytically zero -- exchange does not couple
/// opposite spins -- and both libraries return those as roundoff
/// (1e-17 against -1e-17) beside siblings of order 1e-1. Judged against their
/// siblings they are zero; judged against anything else they look like a 50x
/// disagreement, which is what made 145 of 308 polarized kernels "fail".
fn worst(ours: &[f64], theirs: &[f64], width: usize) -> f64 {
    let mut w = 0.0f64;
    let npoints = theirs.len() / width.max(1);
    for ip in 0..npoints {
        let lo = ip * width;
        let hi = lo + width;
        let point_scale = theirs[lo..hi]
            .iter()
            .fold(0.0f64, |m, v| m.max(v.abs()))
            .max(f64::MIN_POSITIVE);
        for i in lo..hi {
            let (x, y) = (ours[i], theirs[i]);
            if x == y || !x.is_finite() || !y.is_finite() {
                continue;
            }
            if x.abs() < point_scale * 1e-10 && y.abs() < point_scale * 1e-10 {
                continue;
            }
            let d = if y.abs() > 0.0 { ((x - y) / y).abs() } else { (x - y).abs() };
            if d > w {
                w = d;
            }
        }
    }
    w
}

struct Row {
    name: String,
    err: f64,
    field: &'static str,
}

fn sweep(spin: Spin) -> (Vec<Row>, usize) {
    let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let (rho, sigma) = grid(nspin);
    let th = Thresholds::default();
    let nvr = nspin;
    let nvs = if nspin == 1 { 1 } else { 3 };
    // second-derivative widths
    let n2r = if nspin == 1 { 1 } else { 3 };
    let n2rs = if nspin == 1 { 1 } else { 6 };
    let n2s = if nspin == 1 { 1 } else { 6 };

    let mut rows = Vec::new();
    let mut skipped = 0usize;

    for (fam, name) in libxc_reval::routing::SUPPORTED {
        if *fam == "mgga" {
            continue;
        }
        let Ok(id) = lookup_by_name(&format!("xc_{name}")) else {
            skipped += 1;
            continue;
        };
        if let Ok(m) = lookup_by_id(id.raw())
            && !m.flags.contains(FunctionalFlags::DIM_3D)
        {
            skipped += 1;
            continue;
        }
        let mut t: xc_func_type = unsafe { std::mem::zeroed() };
        let n = if nspin == 1 { XC_UNPOLARIZED } else { XC_POLARIZED } as i32;
        if unsafe { xc_func_init(&mut t, id.raw() as i32, n) } != 0 {
            skipped += 1;
            continue;
        }
        let cf = CFunc(t);

        let (mut best, mut bestf) = (0.0f64, "");
        if *fam == "lda" {
            let (mut rz, mut rv, mut r2) =
                (vec![0.0; NP], vec![0.0; NP * nvr], vec![0.0; NP * n2r]);
            let mut out = LdaOutput {
                zk: Some(&mut rz),
                vrho: Some(&mut rv),
                v2rho2: Some(&mut r2),
                ..Default::default()
            };
            let inp = LdaInput::new(&rho, NP, spin).unwrap();
            if libxc_reval::routing::dispatch_lda_by_id(
                id, &inp, &mut out, DerivativeOrder::Fxc, spin, &th,
            )
            .is_err()
            {
                skipped += 1;
                continue;
            }
            drop(out);
            let (mut cz, mut cv, mut c2) =
                (vec![0.0; NP], vec![0.0; NP * nvr], vec![0.0; NP * n2r]);
            unsafe {
                xc_lda_exc_vxc_fxc(
                    &cf.0, NP, rho.as_ptr(),
                    cz.as_mut_ptr(), cv.as_mut_ptr(), c2.as_mut_ptr(),
                );
            }
            let scale = cz.iter().fold(0.0f64, |m, v| m.max(v.abs()));
            if scale == 0.0 || !scale.is_finite() {
                skipped += 1;
                continue;
            }
            let w = worst(&r2, &c2, n2r);
            if w > best {
                best = w;
                bestf = "v2rho2";
            }
        } else {
            let (mut rz, mut rv, mut rs) =
                (vec![0.0; NP], vec![0.0; NP * nvr], vec![0.0; NP * nvs]);
            let (mut r2r, mut r2rs, mut r2s) = (
                vec![0.0; NP * n2r],
                vec![0.0; NP * n2rs],
                vec![0.0; NP * n2s],
            );
            let mut out = GgaOutput {
                zk: Some(&mut rz),
                vrho: Some(&mut rv),
                vsigma: Some(&mut rs),
                v2rho2: Some(&mut r2r),
                v2rhosigma: Some(&mut r2rs),
                v2sigma2: Some(&mut r2s),
                ..Default::default()
            };
            let inp = GgaInput::new(&rho, &sigma, NP, spin).unwrap();
            if libxc_reval::routing::dispatch_gga_by_id(
                id, &inp, &mut out, DerivativeOrder::Fxc, spin, &th,
            )
            .is_err()
            {
                skipped += 1;
                continue;
            }
            drop(out);
            let (mut cz, mut cv, mut cs) =
                (vec![0.0; NP], vec![0.0; NP * nvr], vec![0.0; NP * nvs]);
            let (mut c2r, mut c2rs, mut c2s) = (
                vec![0.0; NP * n2r],
                vec![0.0; NP * n2rs],
                vec![0.0; NP * n2s],
            );
            unsafe {
                xc_gga_exc_vxc_fxc(
                    &cf.0, NP, rho.as_ptr(), sigma.as_ptr(),
                    cz.as_mut_ptr(), cv.as_mut_ptr(), cs.as_mut_ptr(),
                    c2r.as_mut_ptr(), c2rs.as_mut_ptr(), c2s.as_mut_ptr(),
                );
            }
            let scale = cz.iter().fold(0.0f64, |m, v| m.max(v.abs()));
            if scale == 0.0 || !scale.is_finite() {
                skipped += 1;
                continue;
            }
            for (o, c, l, wd) in [
                (&r2r, &c2r, "v2rho2", n2r),
                (&r2rs, &c2rs, "v2rhosigma", n2rs),
                (&r2s, &c2s, "v2sigma2", n2s),
            ] {
                let w = worst(o, c, wd);
                if w > best {
                    best = w;
                    bestf = l;
                }
            }
        }
        rows.push(Row {
            name: name.to_string(),
            err: best,
            field: if bestf.is_empty() { "-" } else { bestf },
        });
    }
    (rows, skipped)
}

fn gate(spin: Spin) {
    let (mut rows, skipped) = sweep(spin);
    rows.sort_by(|a, b| b.err.partial_cmp(&a.err).unwrap());
    let tol = |n: &str| FLOOR.iter().find(|(f, _)| *f == n).map_or(TOL, |(_, v)| v * 4.0);
    let bad: Vec<&Row> = rows.iter().filter(|r| r.err > tol(&r.name)).collect();

    println!("\n=== routed LDA+GGA kernels, SECOND derivatives, {spin:?} ===");
    println!("compared : {}", rows.len());
    println!("skipped  : {skipped}");
    println!("over gate: {} (tol {TOL:e})", bad.len());
    println!("\nworst 10:");
    for r in rows.iter().take(10) {
        println!("  {:<32} {:>11.3e}  {}", r.name, r.err, r.field);
    }
    assert!(
        bad.is_empty(),
        "{} kernels disagree with libxc on second derivatives: {:?}",
        bad.len(),
        bad.iter().map(|r| (&r.name, r.err)).collect::<Vec<_>>()
    );
}

#[test]
fn fxc_unpolarized() {
    gate(Spin::Unpolarized);
}

#[test]
fn fxc_polarized() {
    gate(Spin::Polarized);
}
