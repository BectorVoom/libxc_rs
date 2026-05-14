//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 637/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk637<F: Float>(t22479: F, t510: F, t652: F, t1976: F, t2363: F, t2303: F, t71: F, t1863: F, t33: F, t9228: F, t43: F, t614: F, t2267: F, t38: F, t240: F, t2244: F, t2250: F, t2261: F, t44: F, t607: F, t6500: F) -> (F, F, F, F, F, F, F) {
    let t22480 = t510 * t22479;
    let t22482 = 2.0 * t652 * t22480;
    let t22483 = t1976 * t2363;
    let t22489 = t71 * t2303;
    let t22490 = t1863 * t22489;
    let t22493 = t9228 * t33;
    let t22502 = t614 * t43;
    let t22505 = t38 * t2267;
    let t22510 = 88.0 / 9.0 * t240;
    let t22511 = 88.0 / 9.0 * t2261 * t44 - 40.0 / 9.0 * t22502 * t607 + 5.0 / 18.0 * t22505 * t2244 + 5.0 / 6.0 * t6500 * t2250 - t22510;
    (t22480, t22482, t22483, t22489, t22490, t22493, t22511)
}
