//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1205/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1205<F: Float>(t109: F, t1845: F, t6347: F, t1851: F, t5456: F, t106944: F, t106946: F, t106948: F, t84036: F, t86586: F, t96713: F, t96721: F, t107007: F, t107015: F, t107031: F, t1807: F, t20612: F, t26224: F, t26989: F, t29286: F, t568: F, t84400: F, t90551: F, t90582: F, t90642: F, t97503: F, t97509: F) -> (F, F, F, F) {
    let t110 = F::new(1.0) < t109;
    let t107504 = t6347 * t1845;
    let t107571 = t1851 * t5456;
    let t107634 = piecewise3::<F>(t110, F::new(0.0), -t84036 - F::new(22.0) / F::new(3.0) * t86586 - F::new(4.0) * t96713 + F::new(2.0) * t96721 - F::new(3.0) / F::new(2.0) * t106944 + F::new(3.0) / F::new(2.0) * t106946 - t106948 / F::new(4.0));
    let t107694 = -F::cast_from(0.31253747270116302294e0_f64) * t90551 - F::new(18.0) * t26224 * t26989 * t20612 + F::cast_from(0.9869604401089358619e-1_f64) * t107007 + F::cast_from(0.15626873635058151147e0_f64) * t90582 + F::cast_from(0.9869604401089358619e-1_f64) * t107015 - F::cast_from(0.9869604401089358619e-1_f64) * t97503 - t84400 + F::new(3.0) * t1807 * t29286 * t568 + F::cast_from(0.49348022005446793095e-1_f64) * t97509 - F::cast_from(0.19739208802178717238e0_f64) * t107031 + F::cast_from(0.49348022005446793095e-1_f64) * t90642;
    (t107504, t107571, t107634, t107694)
}
