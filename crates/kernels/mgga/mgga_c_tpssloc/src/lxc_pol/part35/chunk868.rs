//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 868/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk868<F: Float>(t5465: F, t626: F, t5489: F, t6320: F, t67: F, t758: F, t750: F, t17: F, t588: F, t6328: F, t592: F, t3701: F, t6463: F, t1338: F, t6434: F, t562: F, t6414: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19471 = t626 * t5465;
    let t19480 = t626 * t5489;
    let t19541 = t6320 * t67;
    let t19542 = t19541 * t758;
    let t19575 = t6320 * t750;
    let t19576 = t17 * t19575;
    let t19591 = t588 * t6328;
    let t19593 = t592 * t6328;
    let t19596 = t6463 * t3701;
    let t19657 = t1338 * t6434;
    let t19660 = t562 * t6414;
    (t19471, t19480, t19542, t19576, t19591, t19593, t19596, t19657, t19660)
}
