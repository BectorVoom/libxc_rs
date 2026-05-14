//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1180/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1180<F: Float>(t22751: F, t28149: F, t28139: F, t28159: F, t6897: F, t794: F, t28131: F, t81159: F, t552: F, t6434: F, t28164: F, t6914: F, t22704: F, t22705: F, t28181: F, t28182: F) -> (F, F, F, F, F, F, F, F) {
    let t97095 = t22751 * t28149;
    let t97108 = t22751 * t28139;
    let t97111 = t6897 * t794 * t28159;
    let t97124 = t81159 * t28131;
    let t97126 = t552 * t6434;
    let t97137 = t6914 * t28164;
    let t97142 = t22704 * t22705 * t28181;
    let t97148 = t6914 * t28182;
    (t97095, t97108, t97111, t97124, t97126, t97137, t97142, t97148)
}
