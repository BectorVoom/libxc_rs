//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 477/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk477<F: Float>(t340: F, t974: F, t1604: F, t225: F, t1539: F, t248: F, t3051: F, t1041: F, t247: F, t375: F) -> (F, F, F, F, F) {
    let t4546 = t974 * t340;
    let t4557 = t1604 * t225;
    let t4571 = t248 * t3051 * t1539;
    let t4572 = t1041 * t4571;
    let t4582 = t247 * t375;
    (t4546, t4557, t4571, t4572, t4582)
}
