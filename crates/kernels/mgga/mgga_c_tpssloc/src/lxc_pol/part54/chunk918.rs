//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 918/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk918<F: Float>(t25276: F, t25328: F, t858: F, t23237: F, t7479: F, t6552: F, t4119: F, t6554: F, t6553: F, t23204: F, t23164: F, t225: F, t7511: F, t13042: F, t1912: F, t23249: F, t23252: F, t23254: F, t23262: F, t25230: F, t25233: F, t2597: F, t2713: F, t7517: F, t855: F, t866: F) -> (F, F, F, F, F, F, F, F) {
    let t25329 = t25276 + t25328;
    let t25330 = t858 * t25329;
    let t25338 = t23237 * t7479;
    let t25339 = t6552 * t25338;
    let t25341 = t6554 * t4119;
    let t25342 = t6553 * t25341;
    let t25343 = t6552 * t25342;
    let t25345 = t23204 * t7479;
    let t25346 = t23164 * t25345;
    let t25348 = t7511 * t225;
    let t25351 = -0.16449340668482264365e-1 * t25230 + 2.0 * t855 * t25233 - t855 * t25330 - 0.19190897446562641759e-1 * t23249 + t23252 - 0.41123351671205660912e-2 * t23254 + t23262 + 2.0 * t2597 * t7517 + 2.0 * t2713 * t7517 - 0.16449340668482264365e-1 * t25339 - 0.16449340668482264365e-1 * t25343 + 0.82246703342411321825e-2 * t25346 - t25348 * t866 - t13042 * t1912;
    (t25329, t25330, t25339, t25341, t25343, t25346, t25348, t25351)
}
