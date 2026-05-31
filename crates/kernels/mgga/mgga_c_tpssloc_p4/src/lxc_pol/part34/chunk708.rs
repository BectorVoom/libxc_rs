//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 708/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk708<F: Float>(t19: F, t9223: F, t83: F, t84: F, t85: F, t24: F, t41: F, t42: F, t53: F, t54: F, t2585: F, t2769: F, t73: F) -> (F, F, F, F, F, F, F) {
    let t9225 = F::cast_from(0.75936e3_f64) * t19 * t9223;
    let t9238 = F::cast_from(1.0_f64) / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    let t9287 = F::cast_from(1.0_f64) / t42 / t41;
    let t9300 = F::cast_from(1.0_f64) / t54 / t53;
    let t9311 = F::cast_from(1232.0_f64) / F::cast_from(27.0_f64) * t2585;
    let t9321 = F::cast_from(1.0_f64) / t73 / t2769;
    (t9225, t9238, t9239, t9287, t9300, t9311, t9321)
}
