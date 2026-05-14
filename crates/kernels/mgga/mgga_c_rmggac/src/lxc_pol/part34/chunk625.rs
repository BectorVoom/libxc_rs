//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 625/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk625<F: Float>(t132: F, t14090: F, t240: F, t31: F, t4738: F, t69695: F, t71: F, t14107: F, t3807: F, t13980: F, t2019: F, t2020: F, t13984: F, t14193: F, t16156: F, t13815: F, t2165: F, t7553: F) -> (F, F, F, F, F, F) {
    let t69701 = t69695 * t14090 * t71 * t132 * t240 * t4738 * t31;
    let t69710 = t3807 * t14107;
    let t69722 = t2019 * t2020 * t13980;
    let t69728 = t2019 * t2020 * t13984;
    let t69742 = t16156 * t14193;
    let t69745 = t7553 * t13815 * t2165;
    (t69701, t69710, t69722, t69728, t69742, t69745)
}
