//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 892/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk892<F: Float>(t1983: F, t33136: F, t7940: F, t28817: F, t8607: F, t28823: F, t114360: F, t127122: F, t127124: F, t127125: F, t128438: F, t128441: F, t128443: F, t128444: F, t128449: F, t128452: F, t128454: F, t128457: F, t28969: F, t29201: F, t29247: F, t29380: F, t8450: F) -> (F,) {
    let t128460 = 2.0 * t1983 * t7940 * t33136;
    let t128464 = 6.0 * t8607 * t28817;
    let t128466 = 2.0 * t8607 * t28823;
    let t128469 = -6.0 * t114360 * t29247 + 3.0 * t28969 * t8450 - 2.0 * t29201 * t8450 + 6.0 * t29380 * t8450 - t127122 - t127124 - t127125 + t128438 - t128441 - t128443 - t128444 - t128449 - t128452 - t128454 - t128457 - t128460 + t128464 + t128466;
    (t128469,)
}
