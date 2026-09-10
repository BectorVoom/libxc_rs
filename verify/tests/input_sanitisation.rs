//! libxc's input clamps, on points chosen to trigger them.
//!
//! `work_{lda,gga,mgga}_inc.c` sanitises before it evaluates:
//!
//! ```c
//! my_rho[0]   = m_max(p->dens_threshold, rho[0]);
//! my_sigma[0] = m_max(p->sigma_threshold * p->sigma_threshold, sigma[0]);
//! if(NEEDS_TAU){
//!   my_tau[0]   = m_max(p->tau_threshold, tau[0]);
//!   my_sigma[0] = m_min(my_sigma[0], 8.0*my_rho[0]*my_tau[0]);   /* Fermi hole */
//! }
//! /* polarized, after the same for spin 1: */
//! s_ave = 0.5*(my_sigma[0] + my_sigma[2]);
//! my_sigma[1] = clamp(my_sigma[1], -s_ave, +s_ave);
//! ```
//!
//! None of it was implemented here until 2026-09-10, and none of it is visible
//! on a grid built to stay inside a functional's domain -- `bench-vs-libxc`'s
//! `grid::mgga` constructs `tau` above the von Weizsaecker bound on purpose,
//! so the Fermi-hole clamp is the identity at every one of its points. The
//! only way to gate a clamp is to feed it something it has to clamp, which is
//! what this file does: each test builds a grid that violates exactly one
//! bound, checks against libxc, and asserts the violation was real so it
//! cannot pass vacuously.

use libxc_rs::input::{GgaInput, MggaInput};
use libxc_rs::model::{DerivativeOrder, Spin, Thresholds};
use libxc_rs::output::{GgaOutput, MggaOutput};
use libxc_rs::registry::lookup_by_name;
use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_type, xc_gga_exc_vxc, xc_mgga_exc_vxc, XC_POLARIZED,
    XC_UNPOLARIZED,
};

/// Relative agreement demanded. Well inside the 1e-12 energy contract; these
/// points are not near any conditioning cliff, so a missing clamp shows up as
/// a gross difference rather than a marginal one.
const TOL: f64 = 1e-12;

struct CFunc(xc_func_type);

impl Drop for CFunc {
    fn drop(&mut self) {
        unsafe { xc_func_end(&mut self.0) };
    }
}

fn c_init(id: i32, nspin: i32) -> CFunc {
    let mut t: xc_func_type = unsafe { std::mem::zeroed() };
    assert_eq!(unsafe { xc_func_init(&mut t, id, nspin) }, 0, "xc_func_init({id}) failed");
    CFunc(t)
}

/// Worst relative difference, skipping elements that are cancellation dust.
///
/// `scale` is the functional's own magnitude on this grid (its largest `|zk|`).
/// An element where *both* sides are below `scale * 1e-12` carries no
/// information: `gga_x_pbe`'s `vrho` for an empty spin channel is analytically
/// zero, and the two builds land on -4.4e-16 and -3.0e-16, which is a relative
/// difference of 0.8 and a statement about nothing. This is the same rule
/// `crates/kernels-rayon/oracle` adopted after it scored the same dust as
/// signal on `gga_k_tfvw`/`gga_k_absp4`; anything carrying magnitude still
/// faces the full relative tolerance.
fn worst(ours: &[f64], theirs: &[f64], scale: f64) -> f64 {
    let floor = scale * 1e-12;
    let mut w = 0.0f64;
    for (&a, &b) in ours.iter().zip(theirs.iter()) {
        if !a.is_finite() || !b.is_finite() {
            continue;
        }
        if a.abs() < floor && b.abs() < floor {
            continue;
        }
        let rel = if b == 0.0 { a.abs() } else { ((a - b) / b).abs() };
        if rel > w {
            w = rel;
        }
    }
    w
}

/// The functional's magnitude on this grid, from libxc's own `zk`.
fn scale_of(zk: &[f64]) -> f64 {
    zk.iter().filter(|v| v.is_finite()).fold(0.0f64, |m, v| m.max(v.abs()))
}

fn check_mgga(name: &str, spin: Spin, rho: &[f64], sigma: &[f64], lapl: &[f64], tau: &[f64]) {
    let id = lookup_by_name(name).unwrap();
    let nspin = if spin == Spin::Unpolarized { 1 } else { 2 };
    let np = rho.len() / nspin;
    let (nvr, nvs) = (nspin, if nspin == 1 { 1 } else { 3 });

    let cf = c_init(id.raw() as i32, if nspin == 1 { XC_UNPOLARIZED } else { XC_POLARIZED } as i32);
    let (mut cz, mut cvr, mut cvs, mut cvl, mut cvt) = (
        vec![0.0; np],
        vec![0.0; np * nvr],
        vec![0.0; np * nvs],
        vec![0.0; np * nvr],
        vec![0.0; np * nvr],
    );
    unsafe {
        xc_mgga_exc_vxc(
            &cf.0, np, rho.as_ptr(), sigma.as_ptr(), lapl.as_ptr(), tau.as_ptr(),
            cz.as_mut_ptr(), cvr.as_mut_ptr(), cvs.as_mut_ptr(), cvl.as_mut_ptr(),
            cvt.as_mut_ptr(),
        );
    }

    let (mut rz, mut rvr, mut rvs, mut rvl, mut rvt) = (
        vec![0.0; np],
        vec![0.0; np * nvr],
        vec![0.0; np * nvs],
        vec![0.0; np * nvr],
        vec![0.0; np * nvr],
    );
    {
        let input = MggaInput::new(rho, sigma, lapl, tau, np, spin).unwrap();
        let mut out = MggaOutput {
            zk: Some(&mut rz),
            vrho: Some(&mut rvr),
            vsigma: Some(&mut rvs),
            vlapl: Some(&mut rvl),
            vtau: Some(&mut rvt),
            ..Default::default()
        };
        libxc_reval::routing::dispatch_mgga_by_id(
            id, &input, &mut out, DerivativeOrder::Vxc, spin,
            &Thresholds::for_functional(id),
        )
        .unwrap();
    }

    let scale = scale_of(&cz);
    for (lbl, a, b) in [
        ("zk", &rz, &cz),
        ("vrho", &rvr, &cvr),
        ("vsigma", &rvs, &cvs),
        ("vtau", &rvt, &cvt),
    ] {
        let w = worst(a, b, scale);
        assert!(
            w < TOL,
            "{name} {spin:?}: {lbl} differs from libxc by {w:.3e} on inputs that need \
             clamping. libxc sanitises in `work_mgga_inc.c` before the maple2c body runs; \
             if that is not reproduced here the two libraries are evaluating the same \
             functional at different points. See crates/libxc-reval/src/screen.rs."
        );
    }
}

/// `sigma > 8 rho tau` -- outside the Fermi hole curvature bound, which libxc
/// clamps under `XC_ENFORCE_FERMI_HOLE_CURVATURE` (on by default in its CMake).
#[test]
fn fermi_hole_curvature_clamp_matches_libxc() {
    // tau chosen well below tau_W = sigma/(8 rho), so the clamp is active.
    let rho = [1.0, 0.5, 2.0, 0.25];
    let sigma = [2.0, 1.0, 4.0, 0.5];
    let lapl = [0.05, 0.02, 0.1, 0.01];
    let tau = [0.01, 0.02, 0.05, 0.001];

    let mut active = 0;
    for i in 0..rho.len() {
        if sigma[i] > 8.0 * rho[i] * tau[i] {
            active += 1;
        }
    }
    assert_eq!(active, rho.len(), "grid does not violate the bound; the test would be vacuous");

    for name in ["XC_MGGA_C_R2SCAN", "XC_MGGA_X_SCAN", "XC_MGGA_C_SCAN", "XC_MGGA_X_R2SCAN"] {
        check_mgga(name, Spin::Unpolarized, &rho, &sigma, &lapl, &tau);
    }
}

/// The same, polarized: each channel gets its own bound.
#[test]
fn fermi_hole_curvature_clamp_matches_libxc_polarized() {
    let rho = [1.0, 0.6, 0.5, 0.2];
    let sigma = [2.0, 0.3, 1.5, 1.0, 0.1, 0.8];
    let lapl = [0.05, 0.03, 0.02, 0.01];
    let tau = [0.01, 0.02, 0.03, 0.004];

    let mut active = 0;
    for ip in 0..2 {
        for c in 0..2 {
            if sigma[ip * 3 + 2 * c] > 8.0 * rho[ip * 2 + c] * tau[ip * 2 + c] {
                active += 1;
            }
        }
    }
    assert!(active >= 3, "only {active} of 4 channels violate the bound");

    for name in ["XC_MGGA_C_R2SCAN", "XC_MGGA_X_SCAN"] {
        check_mgga(name, Spin::Polarized, &rho, &sigma, &lapl, &tau);
    }
}

/// `tau < tau_threshold` (1e-20), which libxc raises before dividing by it.
#[test]
fn tau_threshold_clamp_matches_libxc() {
    let rho = [1.0, 0.5, 2.0];
    let sigma = [1e-30, 1e-32, 1e-28];
    let lapl = [0.05, 0.02, 0.1];
    let tau = [0.0, 1e-25, 1e-30];

    assert!(tau.iter().all(|&t| t < 1e-20), "grid does not trip the tau clamp");
    for name in ["XC_MGGA_C_R2SCAN", "XC_MGGA_X_SCAN"] {
        check_mgga(name, Spin::Unpolarized, &rho, &sigma, &lapl, &tau);
    }
}

/// The cross term outside `+-(sigma_aa + sigma_bb)/2`, which is where a
/// polarized grid that draws `sigma` component-wise puts it.
#[test]
fn cross_sigma_clamp_matches_libxc() {
    let rho = [1.0, 0.8, 0.5, 0.4];
    // sigma_ab far outside +-s_ave in both directions.
    let sigma = [1.0, 50.0, 1.2, 0.7, -30.0, 0.9];
    let np = 2;

    let mut active = 0;
    for ip in 0..np {
        let s_ave = 0.5 * (sigma[ip * 3] + sigma[ip * 3 + 2]);
        if sigma[ip * 3 + 1] < -s_ave || sigma[ip * 3 + 1] > s_ave {
            active += 1;
        }
    }
    assert_eq!(active, np, "cross term is inside the bound; the test would be vacuous");

    for name in ["XC_GGA_C_PBE", "XC_GGA_C_LYP", "XC_GGA_X_B88"] {
        let id = lookup_by_name(name).unwrap();
        let cf = c_init(id.raw() as i32, XC_POLARIZED as i32);
        let (mut cz, mut cvr, mut cvs) = (vec![0.0; np], vec![0.0; np * 2], vec![0.0; np * 3]);
        unsafe {
            xc_gga_exc_vxc(
                &cf.0, np, rho.as_ptr(), sigma.as_ptr(),
                cz.as_mut_ptr(), cvr.as_mut_ptr(), cvs.as_mut_ptr(),
            );
        }
        let (mut rz, mut rvr, mut rvs) = (vec![0.0; np], vec![0.0; np * 2], vec![0.0; np * 3]);
        {
            let input = GgaInput::new(&rho, &sigma, np, Spin::Polarized).unwrap();
            let mut out = GgaOutput {
                zk: Some(&mut rz),
                vrho: Some(&mut rvr),
                vsigma: Some(&mut rvs),
                ..Default::default()
            };
            libxc_reval::routing::dispatch_gga_by_id(
                id, &input, &mut out, DerivativeOrder::Vxc, Spin::Polarized,
                &Thresholds::for_functional(id),
            )
            .unwrap();
        }
        let scale = scale_of(&cz);
        for (lbl, a, b) in [("zk", &rz, &cz), ("vrho", &rvr, &cvr), ("vsigma", &rvs, &cvs)] {
            let w = worst(a, b, scale);
            assert!(
                w < TOL,
                "{name} polarized: {lbl} differs from libxc by {w:.3e} with the cross term \
                 outside +-s_ave, which `work_gga_inc.c` clamps."
            );
        }
    }
}

/// One spin channel below `dens_threshold`, which libxc raises to it while
/// still evaluating the point (the *total* density is what it screens on).
#[test]
fn spin_density_clamp_matches_libxc() {
    let rho = [1.0, 0.0, 0.5, 1e-40];
    let sigma = [0.4, 0.0, 0.0, 0.2, 0.0, 0.0];
    let np = 2;

    for name in ["XC_GGA_C_PBE", "XC_GGA_X_PBE", "XC_GGA_C_LYP"] {
        let id = lookup_by_name(name).unwrap();
        let th = Thresholds::for_functional(id);
        assert!(rho[1] < th.density && rho[3] < th.density, "grid does not trip the rho clamp");

        let cf = c_init(id.raw() as i32, XC_POLARIZED as i32);
        let (mut cz, mut cvr, mut cvs) = (vec![0.0; np], vec![0.0; np * 2], vec![0.0; np * 3]);
        unsafe {
            xc_gga_exc_vxc(
                &cf.0, np, rho.as_ptr(), sigma.as_ptr(),
                cz.as_mut_ptr(), cvr.as_mut_ptr(), cvs.as_mut_ptr(),
            );
        }
        let (mut rz, mut rvr, mut rvs) = (vec![0.0; np], vec![0.0; np * 2], vec![0.0; np * 3]);
        {
            let input = GgaInput::new(&rho, &sigma, np, Spin::Polarized).unwrap();
            let mut out = GgaOutput {
                zk: Some(&mut rz),
                vrho: Some(&mut rvr),
                vsigma: Some(&mut rvs),
                ..Default::default()
            };
            libxc_reval::routing::dispatch_gga_by_id(
                id, &input, &mut out, DerivativeOrder::Vxc, Spin::Polarized, &th,
            )
            .unwrap();
        }
        let scale = scale_of(&cz);
        for (lbl, a, b) in [("zk", &rz, &cz), ("vrho", &rvr, &cvr), ("vsigma", &rvs, &cvs)] {
            let w = worst(a, b, scale);
            assert!(
                w < TOL,
                "{name} polarized: {lbl} differs from libxc by {w:.3e} with one spin channel \
                 below dens_threshold, which `work_gga_inc.c` raises to it."
            );
        }
    }
}
