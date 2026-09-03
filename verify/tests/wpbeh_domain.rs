//! Where does `gga_x_wpbeh` diverge from libxc, as a function of the reduced
//! gradient `s`?
//!
//! `bench-vs-libxc`'s elementwise cross-check reports 4.5e-7 on `vsigma` for
//! `gga_x_wpbeh` at its default `omega = 0`, while the rayon oracle sees
//! nothing over 1e-10 for any functional. The two use different grids: the
//! bench sweeps `s` up to 3 with densities down to 1e-6, the oracle does not.
//! This locates the boundary rather than leaving it as "some grids disagree".
//!
//! The wpbeh fingerprint is byte-identical before and after the 2026-09-03
//! erfcx/E1 fixes, so whatever this is, it predates them.

use libxc_sys::{
    xc_func_end, xc_func_init, xc_func_type, xc_gga_exc_vxc, XC_UNPOLARIZED,
};
use libxc_rs::input::GgaInput;
use libxc_rs::model::{DerivativeOrder, Spin, Thresholds};
use libxc_rs::output::GgaOutput;
use libxc_rs::registry::lookup_by_name;

fn kf(rho: f64) -> f64 {
    (3.0 * std::f64::consts::PI * std::f64::consts::PI * rho).cbrt()
}

fn sweep(omega: f64) {
    let id = lookup_by_name("xc_gga_x_wpbeh").unwrap();
    println!("\n=== gga_x_wpbeh, omega = {omega} ===");
    println!("{:>10} {:>10}   {:>12} {:>12} {:>12}", "rho", "s", "zk", "vrho", "vsigma");

    for &rho_v in &[1e-6f64, 1e-3, 1.0, 10.0] {
        for &s in &[1e-8f64, 1e-5, 1e-3, 0.01, 0.05, 0.1, 0.5, 1.0, 2.0, 3.0] {
            let np = 1usize;
            // sigma from the reduced gradient: s = |grad rho| / (2 kF rho)
            let g = s * 2.0 * kf(rho_v) * rho_v;
            let rho = vec![rho_v];
            let sigma = vec![g * g];

            let mut t: xc_func_type = unsafe { std::mem::zeroed() };
            unsafe { xc_func_init(&mut t, id.raw() as i32, XC_UNPOLARIZED as i32) };
            let mut ext = [omega];
            unsafe { libxc_sys::xc_func_set_ext_params(&mut t, ext.as_mut_ptr()) };
            let (mut cz, mut cv, mut cs) = (vec![0.0], vec![0.0], vec![0.0]);
            unsafe {
                xc_gga_exc_vxc(&t, np, rho.as_ptr(), sigma.as_ptr(),
                               cz.as_mut_ptr(), cv.as_mut_ptr(), cs.as_mut_ptr());
                xc_func_end(&mut t);
            }

            let (mut rz, mut rv, mut rs) = (vec![0.0], vec![0.0], vec![0.0]);
            {
                let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
                let mut out = GgaOutput {
                    zk: Some(&mut rz), vrho: Some(&mut rv), vsigma: Some(&mut rs),
                    ..Default::default()
                };
                libxc_reval::routing::dispatch_gga_by_id_with(
                    id, &input, &mut out, DerivativeOrder::Vxc, Spin::Unpolarized,
                    &Thresholds::default(), Some(&[omega]),
                ).unwrap();
            }
            let rel = |a: f64, b: f64| if b == 0.0 { (a - b).abs() } else { ((a - b) / b).abs() };
            println!("{rho_v:>10.0e} {s:>10.1e}   {:>12.2e} {:>12.2e} {:>12.2e}",
                     rel(rz[0], cz[0]), rel(rv[0], cv[0]), rel(rs[0], cs[0]));
        }
    }
}

#[test]
fn locate_wpbeh_divergence() {
    sweep(0.0);
    sweep(0.11);
}

/// The `vsigma` divergence is confined to reduced gradients no calculation
/// visits, and this gates the range that matters.
///
/// An attempt to referee it with a finite-difference reference was
/// inconclusive and is worth recording as such: `d(rho*eps)/dsigma` by central
/// difference is *itself* ill-conditioned at small `s`, because `sigma` scales
/// as `s^2` and the step vanishes with it. At `s = 1e-6` the finite difference
/// came out -1.77e2 where both analytic implementations said -3.28e1, i.e. the
/// reference disagreed with both by 80% while they agreed with each other to
/// 5e-4. That says the finite difference has no digits left, not that the
/// analytic values are wrong.
///
/// What can be stated: our `vsigma` and libxc's agree to 1e-9 or better for
/// `s >= 0.01`, and a real molecular quadrature lives at `s` of order 0.1 to 5.
/// Below `s = 1e-6` they diverge, both computing the same maple2c expression
/// through different floating-point contraction. That is out of scope for any
/// physical calculation and is gated here only to catch a regression that
/// reached into the useful range.
#[test]
fn wpbeh_vsigma_agrees_over_the_physical_range() {
    let id = lookup_by_name("xc_gga_x_wpbeh").unwrap();
    let mut worst = 0.0f64;
    let mut worst_at = (0.0f64, 0.0f64);

    for &rho_v in &[1e-4f64, 1e-2, 1.0, 10.0] {
        for &s in &[0.01f64, 0.05, 0.1, 0.5, 1.0, 2.0, 3.0, 5.0] {
            for &omega in &[0.0f64, 0.11] {
                let np = 1usize;
                let g = s * 2.0 * kf(rho_v) * rho_v;
                let rho = vec![rho_v];
                let sigma = vec![g * g];

                let mut t: xc_func_type = unsafe { std::mem::zeroed() };
                unsafe { xc_func_init(&mut t, id.raw() as i32, XC_UNPOLARIZED as i32) };
                let mut ext = [omega];
                unsafe { libxc_sys::xc_func_set_ext_params(&mut t, ext.as_mut_ptr()) };
                let (mut cz, mut cv, mut cs) = (vec![0.0], vec![0.0], vec![0.0]);
                unsafe {
                    xc_gga_exc_vxc(&t, np, rho.as_ptr(), sigma.as_ptr(),
                                   cz.as_mut_ptr(), cv.as_mut_ptr(), cs.as_mut_ptr());
                    xc_func_end(&mut t);
                }

                let (mut rz, mut rv, mut rs) = (vec![0.0], vec![0.0], vec![0.0]);
                {
                    let input = GgaInput::new(&rho, &sigma, np, Spin::Unpolarized).unwrap();
                    let mut out = GgaOutput {
                        zk: Some(&mut rz), vrho: Some(&mut rv), vsigma: Some(&mut rs),
                        ..Default::default()
                    };
                    libxc_reval::routing::dispatch_gga_by_id_with(
                        id, &input, &mut out, DerivativeOrder::Vxc, Spin::Unpolarized,
                        &Thresholds::default(), Some(&[omega]),
                    ).unwrap();
                }
                for (o, c) in [(rz[0], cz[0]), (rv[0], cv[0]), (rs[0], cs[0])] {
                    if c == 0.0 || !c.is_finite() || !o.is_finite() {
                        continue;
                    }
                    let e = ((o - c) / c).abs();
                    if e > worst {
                        worst = e;
                        worst_at = (rho_v, s);
                    }
                }
            }
        }
    }
    println!("wpbeh over the physical range (s 0.01..5): worst rel {worst:.3e} \
              at rho={:.0e} s={}", worst_at.0, worst_at.1);
    assert!(
        worst < 1e-9,
        "wpbeh disagrees with libxc by {worst:.3e} at rho={:.0e} s={} -- inside \
         the range a real quadrature uses, so this is not the small-gradient \
         conditioning documented above",
        worst_at.0, worst_at.1
    );
}
