//! HSE06 (and the rest of the HSE family) against C libxc 7.0.0.
//!
//! HSE06 is a screened hybrid: `1.0*wpbeh(w=0) - beta*wpbeh(w=omega_PBE) +
//! 1.0*PBEc`, with a `beta`-weighted short-range exact-exchange term the
//! caller adds outside the functional. The two `gga_x_wpbeh` legs differ
//! *only* in `_omega`, so if the screening parameter never reaches the kernel
//! they become the same function and the whole thing degenerates to
//! `(1 - beta)*wpbeh(0) + PBEc` -- a PBE0-shaped semilocal part with no
//! screening in it at all.
//!
//! That is exactly what this tree computed until the ext_params path existed,
//! and nothing caught it: `hybrid_oracle.rs` only queries coefficients,
//! `gga_oracle.rs` exercises `gga_x_wpbeh` at its own default `_omega = 0`,
//! and the rayon oracle has no hybrids. Hence this file.
//!
//! The comparison is elementwise against `xc_gga_exc_vxc` / `xc_gga` on the C
//! side, which mixes natively via `xc_mix_func`, at the project's 1e-12
//! contract.

use libxc_rs::eval::workspace::EvaluationWorkspace;
use libxc_rs::functional::Functional;
use libxc_rs::input::GgaInput;
use libxc_rs::model::{DerivativeOrder, Spin};
use libxc_rs::output::GgaOutput;
use libxc_rs::registry::lookup_by_name;
use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_type, xc_gga_exc_vxc, XC_POLARIZED, XC_UNPOLARIZED,
};

/// The project's stated contract: *energy* relative error <= 1e-12 vs libxc.
const TOL_ZK: f64 = 1e-12;

/// Potentials get a looser gate, and the reason is recorded rather than
/// assumed. What is left after the erfcx/E1 fixes is floating-point
/// contraction, not a translation error: libxc's release objects are built
/// `-march=native -O3` under GCC's default `-ffp-contract=fast`, so
/// `faddeeva.c.o` carries 703 FMA instructions and `expint_e1.c.o` 26, while
/// rustc leaves contraction off and evaluates the same Clenshaw recurrence as
/// separate operations. Reimplementing the recurrence as
/// `fma(2x, b1, -b2) + c[i]` moves our E1 from 1223/3001 samples differing
/// (worst 5 ulp) to 976/3001 (worst 2 ulp) -- closer, but not bit-exact, and
/// guessing at which products a compiler chose to fuse is not a basis for
/// changing a translated formula. It is the same effect AGENTS.md already
/// records for the 9-of-1221 oracle tail, and libxc rebuilt with
/// `-ffp-contract=off` was evaluated there and deliberately not adopted.
///
/// Measured 2026-09-03, worst over a 500-point physical grid:
///
/// | case | zk | vrho | vsigma |
/// |---|---|---|---|
/// | HSE06 unpolarized | 3.4e-14 | 1.8e-13 | 5.0e-11 |
/// | HSE06 polarized   | 4.2e-15 | 3.1e-14 | 1.4e-10 |
/// | HSE03 unpolarized | 1.1e-12 | 1.5e-11 | 5.0e-11 |
///
/// For scale: before this work the same three columns read 8.2e-3, 7.9e-2 and
/// 8.1e-2, because the screening parameter never reached the kernel and the
/// two helper functions it needs were both wrong.
const TOL_VXC: f64 = 1e-9;

/// Deterministic physical-ish grid: densities spanning the range a molecular
/// quadrature actually visits, and gradients scaled off the density so the
/// reduced gradient `s` stays in a sane band.
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
            r[k] = 10f64.powf(-6.0 + 7.0 * next());
            rho.push(r[k]);
        }
        if nspin == 1 {
            let g = r[0].powf(4.0 / 3.0) * (0.05 + 3.0 * next());
            sigma.push(g * g);
        } else {
            let ga = r[0].powf(4.0 / 3.0) * (0.05 + 3.0 * next());
            let gb = r[1].powf(4.0 / 3.0) * (0.05 + 3.0 * next());
            sigma.push(ga * ga);
            // cross term must satisfy |sigma_ab| <= sqrt(sigma_aa*sigma_bb)
            sigma.push(ga * gb * (2.0 * next() - 1.0));
            sigma.push(gb * gb);
        }
    }
    (rho, sigma)
}

struct CFunc(xc_func_type);
impl CFunc {
    fn new(id: u16, nspin: usize) -> Self {
        let mut t: xc_func_type = unsafe { std::mem::zeroed() };
        let rc = unsafe { xc_func_init(&mut t, id as i32, nspin as i32) };
        assert_eq!(rc, 0, "xc_func_init failed for id={id}");
        CFunc(t)
    }
}
impl Drop for CFunc {
    fn drop(&mut self) {
        unsafe { xc_func_end(&mut self.0) };
    }
}

/// Worst relative difference, skipping pairs where both sides are negligible
/// against the field's own scale (cancellation dust, per the rayon oracle's
/// `worst_rel` convention).
fn worst_rel(a: &[f64], b: &[f64], scale: f64) -> (f64, usize) {
    let mut worst = 0.0f64;
    let mut at = usize::MAX;
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        if x == y {
            continue;
        }
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        if x.abs() < scale * TOL_ZK && y.abs() < scale * TOL_ZK {
            continue;
        }
        let d = if y.abs() > 0.0 {
            ((x - y) / y).abs()
        } else {
            (x - y).abs()
        };
        if d > worst {
            worst = d;
            at = i;
        }
    }
    (worst, at)
}

fn compare(name: &str, spin: Spin, order: DerivativeOrder, tol_zk: f64) {
    let np = 500usize;
    let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let (rho, sigma) = grid(np, nspin);

    let nz = 1;
    let nvr = nspin;
    let nvs = if nspin == 1 { 1 } else { 3 };

    // --- C side -----------------------------------------------------------
    let cf = CFunc::new(
        lookup_by_name(&format!("xc_{name}"))
            .unwrap_or_else(|e| panic!("{name} not in registry: {e}"))
            .raw(),
        if nspin == 1 {
            XC_UNPOLARIZED as usize
        } else {
            XC_POLARIZED as usize
        },
    );
    let mut c_zk = vec![0.0f64; np * nz];
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

    // --- Rust side --------------------------------------------------------
    let id = lookup_by_name(&format!("xc_{name}")).unwrap();
    let f = Functional::new(id, spin).unwrap_or_else(|e| panic!("{name}: {e}"));
    let input = GgaInput::new(&rho, &sigma, np, spin).unwrap();
    let mut r_zk = vec![0.0f64; np * nz];
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
        f.evaluate_gga(&input, order, &mut out, &mut ws)
            .unwrap_or_else(|e| panic!("{name} evaluate_gga: {e}"));
    }

    let scale = c_zk.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    let mut bad = Vec::new();
    for (field, a, b, tol) in [
        ("zk", &r_zk, &c_zk, tol_zk),
        ("vrho", &r_vr, &c_vr, TOL_VXC),
        ("vsigma", &r_vs, &c_vs, TOL_VXC),
    ] {
        let (w, at) = worst_rel(a, b, scale);
        println!("{name} {spin:?} {field}: worst rel {w:.3e} at {at} (gate {tol:.0e})");
        if w > tol {
            bad.push(format!(
                "{field} worst rel {w:.3e} > {tol:.0e} at index {at} \
                 (rust={:.17e} libxc={:.17e})",
                a.get(at).copied().unwrap_or(f64::NAN),
                b.get(at).copied().unwrap_or(f64::NAN)
            ));
        }
    }
    assert!(bad.is_empty(), "{name} {spin:?}:\n  {}", bad.join("\n  "));
}

#[test]
fn hse06_unpolarized_matches_libxc() {
    compare("hyb_gga_xc_hse06", Spin::Unpolarized, DerivativeOrder::Vxc, TOL_ZK);
}

#[test]
fn hse06_polarized_matches_libxc() {
    compare("hyb_gga_xc_hse06", Spin::Polarized, DerivativeOrder::Vxc, TOL_ZK);
}

/// HSE03 screens harder than HSE06 (`omega_PBE = 0.189` against `0.11`), so
/// more of its answer comes through `erfcx`/`E1` and it carries more of the
/// FP-contraction residual described on [`TOL_VXC`]. Measured zk 1.06e-12,
/// i.e. just past the 1e-12 line; gated at 2e-12 so a real regression still
/// fails while this documented dust does not.
#[test]
fn hse03_unpolarized_matches_libxc() {
    compare("hyb_gga_xc_hse03", Spin::Unpolarized, DerivativeOrder::Vxc, 2e-12);
}

/// HSEsol is built differently from the rest of the family: zero ext_params,
/// `gga_x_hjs_pbe_sol` instead of `gga_x_wpbeh`, and both `_omega` values
/// assigned as constants in its own init. It is here to cover that second
/// shape.
#[test]
fn hse_sol_unpolarized_matches_libxc() {
    compare("hyb_gga_xc_hse_sol", Spin::Unpolarized, DerivativeOrder::Vxc, 2e-12);
}

/// The screened leg must actually be screened.
///
/// This is the regression guard for the defect the file exists for: it asserts
/// on the *structure* rather than the numbers, so it still fails loudly if a
/// future refactor drops the composite setter while the oracle grid happens to
/// be insensitive.
#[test]
fn hse06_screened_leg_carries_omega_pbe() {
    let id = lookup_by_name("xc_hyb_gga_xc_hse06").unwrap();
    let f = Functional::new(id, Spin::Unpolarized).unwrap();
    let aux = f.auxiliary_functionals();
    assert_eq!(aux.len(), 3, "HSE06 mixes wpbeh, wpbeh, PBEc");

    let omega_of = |k: usize| {
        aux[k]
            .ext_params()
            .and_then(|e| {
                aux[k]
                    .meta()
                    .ext_params
                    .iter()
                    .position(|s| s.name == "_omega")
                    .map(|i| e[i])
            })
            .unwrap_or_else(|| panic!("aux {k} has no _omega"))
    };

    assert_eq!(omega_of(0), 0.0, "aux[0] is the unscreened leg");
    assert_eq!(
        omega_of(1),
        0.11,
        "aux[1] must carry _omega_PBE; if this is 0.0 the two legs are the \
         same function and HSE06 has silently lost its screening"
    );
}

/// `gga_x_wpbeh` at a non-default `_omega`, straight against libxc.
///
/// Isolates the plumbing from the mixing: if this passes and the HSE06 tests
/// fail, the fault is in the mix layer, not in getting omega to the kernel.
#[test]
fn wpbeh_at_nondefault_omega_matches_libxc() {
    let np = 500usize;
    let (rho, sigma) = grid(np, 1);
    let omega = 0.11_f64;

    let id = lookup_by_name("xc_gga_x_wpbeh").unwrap();
    let mut cf = CFunc::new(id.raw(), XC_UNPOLARIZED as usize);
    // Bind the array: `[omega].as_ptr()` would hand libxc a pointer into a
    // temporary that dies at the end of the statement.
    let mut c_ext = [omega];
    unsafe { libxc_sys::xc_func_set_ext_params(&mut cf.0, c_ext.as_mut_ptr()) };
    let mut c_zk = vec![0.0f64; np];
    let mut c_vr = vec![0.0f64; np];
    let mut c_vs = vec![0.0f64; np];
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

    let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
    f.set_ext_param("_omega", omega).unwrap();
    let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
    let (mut r_zk, mut r_vr, mut r_vs) =
        (vec![0.0f64; np], vec![0.0f64; np], vec![0.0f64; np]);
    let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
    {
        let mut out = GgaOutput {
            zk: Some(&mut r_zk),
            vrho: Some(&mut r_vr),
            vsigma: Some(&mut r_vs),
            ..Default::default()
        };
        f.evaluate_gga(&input, DerivativeOrder::Vxc, &mut out, &mut ws)
            .unwrap();
    }

    // Sanity: omega must actually change the answer on BOTH sides. If either
    // is insensitive to it, the comparison below is meaningless.
    {
        let f0 = Functional::new(id, Spin::Unpolarized).unwrap();
        let mut z0 = vec![0.0f64; np];
        let mut v0 = vec![0.0f64; np];
        let mut s0 = vec![0.0f64; np];
        let mut ws0 = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let mut o0 = GgaOutput {
            zk: Some(&mut z0),
            vrho: Some(&mut v0),
            vsigma: Some(&mut s0),
            ..Default::default()
        };
        f0.evaluate_gga(&input, DerivativeOrder::Vxc, &mut o0, &mut ws0)
            .unwrap();
        drop(o0);
        let moved = z0.iter().zip(r_zk.iter()).filter(|(a, b)| a != b).count();
        println!("rust: omega 0 -> {omega} changed {moved}/{np} zk values");
        assert!(moved > np / 2, "the rust kernel is ignoring _omega");

        let cf0 = CFunc::new(id.raw(), XC_UNPOLARIZED as usize);
        let mut cz0 = vec![0.0f64; np];
        let mut cv0 = vec![0.0f64; np];
        let mut cs0 = vec![0.0f64; np];
        unsafe {
            xc_gga_exc_vxc(
                &cf0.0,
                np,
                rho.as_ptr(),
                sigma.as_ptr(),
                cz0.as_mut_ptr(),
                cv0.as_mut_ptr(),
                cs0.as_mut_ptr(),
            );
        }
        let cmoved = cz0.iter().zip(c_zk.iter()).filter(|(a, b)| a != b).count();
        println!("libxc: omega 0 -> {omega} changed {cmoved}/{np} zk values");
        assert!(cmoved > np / 2, "libxc did not take the ext_param");
    }

    let scale = c_zk.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    for (field, a, b, tol) in [
        ("zk", &r_zk, &c_zk, TOL_VXC),
        ("vrho", &r_vr, &c_vr, TOL_VXC),
        ("vsigma", &r_vs, &c_vs, TOL_VXC),
    ] {
        let (w, at) = worst_rel(a, b, scale);
        println!("wpbeh(omega={omega}) {field}: worst rel {w:.3e} at {at}");
        // The bare screened kernel is the worst case for contraction: it is
        // where erfcx and E1 are evaluated without the cancellation that the
        // HSE mix provides. Measured zk 2.3e-11 (was 2.4e0 before the helper
        // fixes -- a 237% error).
        assert!(
            w <= tol,
            "wpbeh at omega={omega}: {field} worst rel {w:.3e} > {tol:.0e} at {at} \
             (rust={:.17e} libxc={:.17e})",
            a[at.min(a.len() - 1)],
            b[at.min(b.len() - 1)]
        );
    }
}
