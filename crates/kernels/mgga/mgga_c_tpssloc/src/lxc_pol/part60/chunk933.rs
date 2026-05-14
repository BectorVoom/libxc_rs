//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 933/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk933<F: Float>(t104990: F, t128418: F, t128420: F, t128422: F, t128429: F, t128438: F, t128441: F, t128443: F, t128444: F, t128449: F, t128452: F, t128454: F, t128457: F, t128460: F, t2040: F, t2165: F, t28951: F, t28952: F, t29252: F, t29855: F, t652: F, t7042: F, t7266: F, t8690: F) -> (F,) {
    let t130354 = -2.0 * t2165 * t28951 * t652 - 2.0 * t104990 * t2040 - 2.0 * t28952 * t7266 + 6.0 * t29252 * t8690 - 2.0 * t29855 * t7042 - t128418 - t128420 - t128422 + t128429 + t128438 - t128441 - t128443 - t128444 - t128449 - t128452 - t128454 - t128457 - t128460;
    (t130354,)
}
