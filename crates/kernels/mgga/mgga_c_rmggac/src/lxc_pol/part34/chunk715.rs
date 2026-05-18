//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 715/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk715<F: Float>(t14140: F, t2046: F, t7297: F, t1338: F, t2039: F, t638: F, t669: F, t2050: F, t2128: F, t31: F, t13823: F, t34796: F, t7756: F) -> (F, F, F, F) {
    let t70078 = t2046 * t7297 * t14140;
    let t70082 = t638 * t2039 * t669 * t1338;
    let t70086 = t2046 * t2050 * t2128 * t31;
    let t70100 = t13823 * t34796 * t7756;
    (t70078, t70082, t70086, t70100)
}
