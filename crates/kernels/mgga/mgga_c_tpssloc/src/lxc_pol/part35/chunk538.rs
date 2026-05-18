//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 538/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk538<F: Float>(t1409: F, t2267: F, t2274: F, t2291: F, t2298: F, t111: F, t1441: F) -> (F, F, F, F, F) {
    let t3981 = t2267 * t1409;
    let t3990 = t2274 * t1409;
    let t4007 = t2291 * t1409;
    let t4012 = t2298 * t1409;
    let t4028 = t1441 * t111;
    (t3981, t3990, t4007, t4012, t4028)
}
