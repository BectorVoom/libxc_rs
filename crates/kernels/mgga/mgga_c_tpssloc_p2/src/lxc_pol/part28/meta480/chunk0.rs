//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1694/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1694<F: Float>(t4072: F, t88: F, t1453: F, t22470: F, t666: F, t22473: F, t4067: F, t6530: F, t1982: F, t8944: F, t1388: F, t1845: F) -> (F, F, F, F, F, F, F) {
    let t26117 = t88 * t4072;
    let t26127 = t22470 * t1453;
    let t26129 = t1453 * t666;
    let t26130 = t22473 * t26129;
    let t26132 = t6530 * t4067;
    let t26161 = t1982 * t8944;
    let t26163 = t1845 * t1388;
    (t26117, t26127, t26129, t26130, t26132, t26161, t26163)
}
