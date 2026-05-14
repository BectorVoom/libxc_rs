//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 665/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk665<F: Float>(t70610: F, t13964: F, t14065: F, t14092: F, t14103: F, t14152: F, t14269: F, t15020: F, t14372: F, t15262: F, t16156: F, t15254: F, t14229: F, t8576: F, t14255: F, t3148: F, t3151: F, t38471: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t73624 = 0.65053455985619242964e-5 * t70610;
    let t73645 = 0.13010691197123848593e-4 * t13964;
    let t73658 = 0.58171619854173713844e-4 * t14065;
    let t73659 = 0.114000641766744825e-6 * t14092;
    let t73660 = 0.33133663046638785508e-1 * t14103;
    let t73666 = 0.13010691197123848593e-4 * t14152;
    let t73678 = 0.34695176525663596248e-4 * t14269;
    let t73679 = 2.0 * t15020;
    let t73680 = 0.8175676176687304687e-5 * t14372;
    let t73688 = t16156 * t15262;
    let t73690 = t16156 * t15254;
    let t73691 = 0.19863479950205658386e-4 * t73690;
    let t73692 = t8576 * t14229;
    let t73693 = t73692 * t14255;
    let t73696 = t38471 * t3148 * t3151;
    (t73624, t73645, t73658, t73659, t73660, t73666, t73678, t73679, t73680, t73688, t73691, t73692, t73693, t73696)
}
