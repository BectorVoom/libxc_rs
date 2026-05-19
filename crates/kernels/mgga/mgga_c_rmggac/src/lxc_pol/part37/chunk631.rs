//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 631/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk631<F: Float>(t15354: F, t15357: F, t15364: F, t15368: F, t15377: F, t15380: F, t15389: F, t15392: F, t15400: F, t15412: F, t3285: F, t534: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15920 = F::cast_from(0.52557918278704101561e-6_f64) * t15354;
    let t15921 = F::cast_from(0.52557918278704101561e-6_f64) * t15357;
    let t15922 = F::cast_from(0.58171619854173713844e-5_f64) * t15364;
    let t15923 = F::cast_from(0.17451485956252114153e-4_f64) * t15368;
    let t15924 = F::cast_from(0.58171619854173713844e-5_f64) * t15377;
    let t15925 = F::cast_from(0.58171619854173713844e-5_f64) * t15380;
    let t15927 = F::cast_from(0.35038612185802734374e-6_f64) * t15389;
    let t15928 = F::cast_from(0.35038612185802734374e-6_f64) * t15392;
    let t15929 = F::cast_from(0.72714524817717142305e-5_f64) * t15400;
    let t15930 = F::cast_from(0.58171619854173713844e-5_f64) * t15412;
    let t15931 = t534 * t3285;
    (t15920, t15921, t15922, t15923, t15924, t15925, t15927, t15928, t15929, t15930, t15931)
}
