//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 890/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk890<F: Float>(t3271: F, t3287: F, t1107: F, t3279: F, t281: F, t2820: F, t415: F, t1114: F, t699: F) -> (F, F, F, F, F) {
    let t3288 = t3287 * t3271;
    let t3290 = t1107 * t3279;
    let t3293 = t281 * t2820 * t415;
    let t3294 = F::cast_from(0.13692777777777777778e0_f64) * t3293;
    let t3295 = t699 * t1114;
    (t3288, t3290, t3293, t3294, t3295)
}
