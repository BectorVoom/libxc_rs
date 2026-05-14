//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 472/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk472<F: Float>(t14125: F, t1972: F, t14131: F, t270: F, t669: F, t2039: F, t638: F, t31: F, t2046: F, t2050: F, t3157: F, t6477: F, t1322: F, t21: F, t3054: F, t3094: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14132 = t14125 * t1972;
    let t14133 = t14131 * t14132;
    let t14136 = t669 * t270;
    let t14138 = t638 * t2039 * t14136;
    let t14140 = t669 * t31;
    let t14142 = t2046 * t2050 * t14140;
    let t14144 = t6477 * t3157;
    let t14147 = t21 * t3054 * t1322;
    let t14148 = t14147 * t3094;
    (t14132, t14133, t14136, t14138, t14140, t14142, t14144, t14147, t14148)
}
