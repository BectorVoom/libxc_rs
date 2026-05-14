//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 764/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk764<F: Float>(t26429: F, t1338: F, t7918: F, t1352: F, t5287: F, t7208: F, t27051: F, t553: F, t1332: F, t1336: F, t1814: F, t2089: F, t22728: F, t22731: F, t22746: F, t22753: F, t22896: F, t24108: F, t24110: F, t26434: F, t26437: F, t26449: F, t26463: F, t26468: F, t5230: F, t544: F, t7211: F, t7934: F) -> (F,) {
    let t27096 = 0.38381794893125283518e-1 * t26429;
    let t27097 = t1338 * t7918;
    let t27098 = t27097 * t1352;
    let t27103 = t7208 * t5287;
    let t27105 = t553 * t27051;
    let t27113 = -t27096 - t1336 * t27098 + 0.16449340668482264365e-1 * t26434 - 0.82246703342411321825e-2 * t26437 + t24108 + t24110 - 0.82246703342411321825e-2 * t22728 - t22731 - t1336 * t27103 + t544 * t27105 + 0.9869604401089358619e-1 * t26449 + t22746 + t22753 + t1332 * t7934 - 0.16449340668482264365e-1 * t26463 + t1814 * t7211 + t22896 + t5230 * t2089 - 0.16449340668482264365e-1 * t26468;
    (t27113,)
}
