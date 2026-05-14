//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 566/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk566<F: Float>(t15392: F, t15400: F, t15412: F, t3285: F, t534: F, t72: F, t3292: F, t623: F, t15862: F, t739: F, t3350: F, t7254: F) -> (F, F, F, F, F, F, F, F) {
    let t15928 = 0.35038612185802734374e-6 * t15392;
    let t15929 = 0.72714524817717142305e-5 * t15400;
    let t15930 = 0.58171619854173713844e-5 * t15412;
    let t15931 = t534 * t3285;
    let t15932 = t72 * t15931;
    let t15933 = t623 * t3292;
    let t15934 = 0.19957069503106347607e-1 * t15933;
    let t15935 = t739 * t15862;
    let t15936 = 0.59871208509319042821e-1 * t15935;
    let t16043 = t7254 * t3350;
    (t15928, t15929, t15930, t15931, t15932, t15934, t15936, t16043)
}
