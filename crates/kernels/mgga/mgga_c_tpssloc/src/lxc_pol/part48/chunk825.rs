//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 825/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk825<F: Float>(t31193: F, t3719: F, t6637: F, t6888: F, t22685: F, t3734: F, t1992: F, t550: F, t6976: F, t81203: F, t31206: F, t6897: F, t794: F, t1985: F, t1998: F, t214: F, t22870: F) -> (F, F, F, F, F) {
    let t114077 = 0.3289868133696452873e-1 * t6888 * t6637 * t31193 * t3719;
    let t114081 = 0.9869604401089358619e-1 * t22685 * t6637 * t31193 * t3734;
    let t114085 = 0.16449340668482264365e-1 * t1992 * t6976 * t81203 * t550;
    let t114097 = t6897 * t794 * t31206;
    let t114098 = 0.16449340668482264365e-1 * t114097;
    let t114102 = 0.16449340668482264365e-1 * t1985 * t214 * t1998 * t22870;
    (t114077, t114081, t114085, t114098, t114102)
}
