//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 895/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk895<F: Float>(t225: F, t33940: F, t114762: F, t116514: F, t121382: F, t121391: F, t121399: F, t121403: F, t121409: F, t121451: F, t2053: F, t25168: F, t2597: F, t26581: F, t26679: F, t26700: F, t26728: F, t26729: F, t2713: F, t2718: F, t32006: F, t33982: F, t4147: F, t7092: F, t855: F, t866: F) -> (F,) {
    let t123487 = t33940 * t225;
    let t123503 = 2.0 * t4147 * t32006 + 4.0 * t26700 * t7092 - 12.0 * t25168 * t26728 * t26581 + t116514 - 0.15352717957250113407e0 * t114762 + 0.6579736267392905746e-1 * t121382 - t123487 * t866 + 2.0 * t2597 * t33982 + 0.3289868133696452873e-1 * t121391 - 12.0 * t121451 * t26729 + 4.0 * t855 * t2718 * t2053 * t26679 + 0.16449340668482264365e-1 * t121399 - 0.6579736267392905746e-1 * t121403 + 2.0 * t2713 * t33982 - 0.3289868133696452873e-1 * t121409;
    (t123503,)
}
