//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 632/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk632<F: Float>(t15931: F, t72: F, t3292: F, t623: F, t15862: F, t739: F, t3350: F, t7254: F) -> (F, F, F, F) {
    let t15932 = t72 * t15931;
    let t15933 = t623 * t3292;
    let t15934 = F::new(0.19957069503106347607e-1) * t15933;
    let t15935 = t739 * t15862;
    let t15936 = F::new(0.59871208509319042821e-1) * t15935;
    let t16043 = t7254 * t3350;
    (t15932, t15934, t15936, t16043)
}
