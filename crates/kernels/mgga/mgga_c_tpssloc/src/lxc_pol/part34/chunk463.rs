//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 463/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk463<F: Float>(t1291: F, t2663: F, t2225: F, t522: F, t2221: F, t2223: F, t2516: F, t521: F, t17: F, t1287: F, t592: F, t588: F, t1365: F, t68: F, t248: F, t2691: F, t557: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3813 = 0.24415263074675393405e-3 * t1291 * t2663;
    let t3819 = 20.0 * t2225 * t522;
    let t3821 = 12.0 * t2221 * t522;
    let t3823 = 32.0 * t2223 * t522;
    let t3824 = t521 * t2516;
    let t3825 = t17 * t3824;
    let t3832 = 8.0 * t592 * t1287;
    let t3836 = 8.0 * t588 * t1287;
    let t3843 = t68 * t1365;
    let t3862 = t2691 * t557 * t248;
    (t3813, t3819, t3821, t3823, t3824, t3825, t3832, t3836, t3843, t3862)
}
