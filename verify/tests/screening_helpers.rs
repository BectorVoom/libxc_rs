//! `xc_erfcx` and `xc_E1_scaled` against libxc's own C implementations.
//!
//! These two helpers are reached *only* on the screened path: at `omega = 0`
//! every screening term in `gga_x_wpbeh` drops out, so `gga_oracle.rs` -- which
//! tests wpbeh at its default `omega = 0` -- never calls them. They were
//! therefore unverified against C until HSE06 needed a non-zero omega.

use libxc_rkernel_math::expint_e1::xc_e1_scaled;
// Force libxc-sys to be linked so the extern C symbols below resolve.
use libxc_sys as _;

unsafe extern "C" {
    fn xc_erfcx(x: f64) -> f64;
    fn xc_expint_e1_impl(x: f64, scale: i32) -> f64;
}

fn ulps(a: f64, b: f64) -> i64 {
    if a == b {
        return 0;
    }
    (a.to_bits() as i64 - b.to_bits() as i64).abs()
}

#[test]
fn erfcx_matches_libxc() {
    // wpbeh calls erfcx(sqrt(aux5)) with aux5 >= 0, so the positive axis is
    // what matters, but cover both sides.
    let mut worst = (0i64, 0.0f64, 0.0f64, 0.0f64);
    let mut n_diff = 0usize;
    let mut n = 0usize;
    for i in -2000..=20000i32 {
        let x = i as f64 * 0.01;
        let ours = libxc_rkernel_math::erf::xc_erfcx(x);
        let theirs = unsafe { xc_erfcx(x) };
        n += 1;
        let u = ulps(ours, theirs);
        if u != 0 {
            n_diff += 1;
        }
        if u > worst.0 {
            worst = (u, x, ours, theirs);
        }
    }
    println!("erfcx: {n_diff}/{n} differ, worst {} ulp at x={} ours={:e} libxc={:e}",
             worst.0, worst.1, worst.2, worst.3);
    // 1 ulp. The residual is GCC contracting `a*b + c` into an FMA inside the
    // Chebyshev sum, which rustc does not do -- the same effect that accounts
    // for the oracle's remaining sub-1e-11 tail (see AGENTS.md). Before the
    // Faddeeva table replaced the Abramowitz & Stegun fit this was 5463/22001
    // differing, worst ~1.4e13 ulp (0.3% relative).
    assert!(worst.0 <= 2, "erfcx differs from libxc by {} ulp at x={}", worst.0, worst.1);
}

#[test]
fn e1_scaled_matches_libxc() {
    let mut worst = (0i64, 0.0f64, 0.0f64, 0.0f64);
    let mut n_diff = 0usize;
    let mut n = 0usize;
    // wpbeh's aux5 spans a wide positive range; sample logarithmically plus a
    // linear sweep through every Chebyshev region boundary (0, 1, 4, 10).
    let mut xs: Vec<f64> = Vec::new();
    for i in -3000..=3000i32 {
        xs.push(i as f64 * 0.005);
    }
    for i in 0..=600i32 {
        xs.push(10f64.powf(-6.0 + i as f64 * 0.02));
    }
    for &x in &xs {
        if x == 0.0 {
            continue;
        }
        let ours = xc_e1_scaled(x);
        let theirs = unsafe { xc_expint_e1_impl(x, 1) };
        if !ours.is_finite() || !theirs.is_finite() {
            continue;
        }
        n += 1;
        let u = ulps(ours, theirs);
        if u != 0 {
            n_diff += 1;
        }
        if u > worst.0 {
            worst = (u, x, ours, theirs);
        }
    }
    println!("E1_scaled: {n_diff}/{n} differ, worst {} ulp at x={} ours={:e} libxc={:e}",
             worst.0, worst.1, worst.2, worst.3);
    // Same story as erfcx: a few ulp of GCC FMA contraction in the Clenshaw
    // recurrence, against 705/6601 differing by up to ~2.2e13 ulp before the
    // six corrupted E11 coefficients were regenerated from libxc's C.
    assert!(worst.0 <= 4, "E1_scaled differs from libxc by {} ulp at x={}", worst.0, worst.1);
    // And say the same thing in relative terms, which is what the 1e-12
    // contract actually cares about.
    let rel = ((worst.2 - worst.3) / worst.3).abs();
    assert!(rel < 1e-14, "E1_scaled worst relative error {rel:e}");
}
