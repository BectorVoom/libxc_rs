//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 707/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk707<F: Float>(t14107: F, t3807: F, t13980: F, t2019: F, t2020: F, t13984: F, t14193: F, t16156: F, t13815: F, t2165: F, t7553: F, t217: F, t3119: F, t457: F, t7715: F) -> (F, F, F, F, F, F) {
    let t69710 = t3807 * t14107;
    let t69722 = t2019 * t2020 * t13980;
    let t69728 = t2019 * t2020 * t13984;
    let t69742 = t16156 * t14193;
    let t69745 = t7553 * t13815 * t2165;
    let t69755 = t217 * t457 * t7715 * t3119;
    (t69710, t69722, t69728, t69742, t69745, t69755)
}
