//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1325/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1325<F: Float>(t30349: F, t580: F, t1404: F, t8283: F, t1858: F, t8199: F, t110280: F, t110282: F, t110484: F, t110919: F, t111243: F, t111284: F, t1396: F, t1398: F, t16507: F, t16546: F, t2206: F, t2212: F, t30095: F, t30350: F, t30395: F) -> F {
    let t111289 = F::new(2.0) * t30349 * t580;
    let t111291 = F::new(2.0) * t8283 * t1404;
    let t111293 = F::new(2.0) * t8199 * t1858;
    let t111296 = t16507 * t2212 + t30095 * t1858 + F::new(2.0) * t30350 * t1404 + F::new(2.0) * t1396 * t30395 + t110919 + t1398 * (t111243 + t111284) + F::new(2.0) * t110484 + t111289 + t111291 + t110280 + t111293 + t2206 * t16546 + F::new(2.0) * t110282;
    t111296
}
