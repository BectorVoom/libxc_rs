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
