//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 906/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk906<F: Float>(t3333: F, t3359: F, t3236: F, t3238: F, t3245: F, t3250: F, t3254: F) -> (F, F, F) {
    let t3360 = t3333 * t3359;
    let t3363 = F::cast_from(0.12361111111111111111e-1_f64) * t3236;
    let t3368 = t3363 - F::cast_from(0.61805555555555555556e-2_f64) * t3238 - F::cast_from(0.61805555555555555555e-2_f64) * t3245 + F::cast_from(0.18541666666666666667e-1_f64) * t3250 + F::cast_from(0.92708333333333333333e-2_f64) * t3254;
    (t3360, t3363, t3368)
}
