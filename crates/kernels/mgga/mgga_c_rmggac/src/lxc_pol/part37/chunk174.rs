//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 174/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk174<F: Float>(t189: F, t191: F, t302: F, t68: F, t131: F, t290: F, t356: F, t274: F, t49: F, t288: F, t156: F, t5: F, t249: F, t433: F, t12: F, t13: F, t140: F) -> (F, F, F, F, F, F, F, F, F) {
    let t912 = 1.0 / t189;
    let t921 = 1.0 / t191;
    let t934 = t68 * t302;
    let t935 = t934 * t131;
    let t938 = t290 * t356;
    let t941 = t274 * t49;
    let t942 = t941 * t288;
    let t945 = t156 * t5;
    let t946 = t249 * t433;
    let t948 = 0.10843581300301739842e-1 * t945 * t946;
    let t951 = 1.0 / t13 / t12 * t140;
    (t912, t921, t934, t935, t938, t941, t942, t948, t951)
}
