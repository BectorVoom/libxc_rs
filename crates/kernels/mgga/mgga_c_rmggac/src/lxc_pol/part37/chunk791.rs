//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 791/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk791<F: Float>(t76707: F, t15626: F, t34884: F, t3352: F, t495: F, t515: F, t7230: F, t9523: F, t15502: F, t3351: F, t498: F, t9210: F, t321: F, t7248: F, t2144: F, t333: F, t7231: F) -> (F, F, F, F, F, F) {
    let t76708 = 0.1064114997332445985e-4 * t76707;
    let t76712 = t34884 * t15626;
    let t76713 = 0.12414674968878536491e-4 * t76712;
    let t76717 = t7230 * t3352 * t515 * t9523 * t495;
    let t76718 = 0.15961724959986689774e-4 * t76717;
    let t76722 = t3351 * t9210 * t515 * t15502 * t498;
    let t76723 = 0.85129199786595678796e-5 * t76722;
    let t76727 = t3351 * t7248 * t515 * t15502 * t321;
    let t76728 = 0.12769379967989351819e-4 * t76727;
    let t76732 = t3351 * t7231 * t2144 * t15502 * t333;
    (t76708, t76713, t76718, t76723, t76728, t76732)
}
