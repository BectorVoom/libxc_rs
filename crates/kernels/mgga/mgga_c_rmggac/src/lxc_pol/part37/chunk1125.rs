//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1125/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1125<F: Float>(t15016: F, t15018: F, t15386: F, t15660: F, t15665: F, t15922: F, t15923: F, t15924: F, t15925: F, t15927: F, t15928: F, t15929: F, t70745: F, t70746: F, t73678: F) -> F {
    let t80552 = -t70745 + t15016 + t15018 - t15922 + t15923 - t15660 - t15924 + t15925 + t15386 - t15927 + t15928 + t15665 - t15929 + t73678 + t70746;
    t80552
}
