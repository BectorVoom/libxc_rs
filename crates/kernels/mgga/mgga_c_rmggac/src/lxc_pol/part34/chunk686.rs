//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 686/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk686<F: Float>(t14123: F, t3113: F, t68438: F, t14045: F, t14121: F, t1008: F, t464: F, t1966: F, t220: F, t14167: F, t14115: F, t68447: F) -> (F, F, F, F, F, F) {
    let t68876 = t3113 * t68438 * t14123;
    let t68884 = t14045 * t14121 * t14123;
    let t68889 = t464 * t1008;
    let t68891 = t1966 * t68889 * t220;
    let t68892 = t68891 * t14167;
    let t68906 = t68447 * t14115;
    (t68876, t68884, t68889, t68891, t68892, t68906)
}
