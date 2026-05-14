//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 781/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk781<F: Float>(t19: F, t9223: F, t83: F, t84: F, t85: F, t24: F, t41: F, t42: F, t53: F, t54: F, t2585: F, t2769: F, t73: F, t3241: F, t76: F, t107: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9225 = 0.75936e3 * t19 * t9223;
    let t9238 = 1.0 / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    let t9287 = 1.0 / t42 / t41;
    let t9300 = 1.0 / t54 / t53;
    let t9311 = 1232.0 / 27.0 * t2585;
    let t9321 = 1.0 / t73 / t2769;
    let t9330 = 1.0 / t76 / t3241;
    let t9358 = 154.0 / 27.0 * t2585 * t107;
    (t9225, t9238, t9239, t9287, t9300, t9311, t9321, t9330, t9358)
}
