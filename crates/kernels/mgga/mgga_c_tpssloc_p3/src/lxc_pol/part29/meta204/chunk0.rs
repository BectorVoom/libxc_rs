//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1021/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1021<F: Float>(t1539: F, t248: F, t3051: F, t1041: F, t1616: F, t884: F, t3071: F, t1023: F, t247: F, t375: F) -> (F, F, F, F, F, F, F) {
    let t4571 = t248 * t3051 * t1539;
    let t4572 = t1041 * t4571;
    let t4574 = t1616 * t884;
    let t4575 = t3071 * t4574;
    let t4578 = t1539 * t1023;
    let t4579 = t3071 * t4578;
    let t4582 = t247 * t375;
    (t4571, t4572, t4574, t4575, t4578, t4579, t4582)
}
