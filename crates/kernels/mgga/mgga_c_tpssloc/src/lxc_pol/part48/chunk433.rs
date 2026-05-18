//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 433/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk433<F: Float>(t3046: F, t364: F, t354: F, t1043: F, t121: F, t248: F, t884: F, t1041: F, t1044: F, t2780: F, t283: F, t883: F) -> (F, F, F, F) {
    let t3047 = t364 * t3046;
    let t3048 = t354 * t3047;
    let t3051 = t121 * t1043;
    let t3053 = t248 * t3051 * t884;
    let t3054 = t1041 * t3053;
    let t3057 = t248 * t1044 * t2780;
    let t3061 = F::new(1.0) / t283 / t883;
    (t3048, t3054, t3057, t3061)
}
