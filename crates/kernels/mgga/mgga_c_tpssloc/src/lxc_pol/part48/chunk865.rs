//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 865/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk865<F: Float>(t115305: F, t6897: F, t80645: F, t8621: F, t22633: F, t31550: F, t80650: F, t22635: F, t26331: F, t31549: F, t3734: F, t22704: F, t31559: F, t81326: F, t113931: F, t113934: F, t113941: F, t115292: F, t115294: F, t115299: F, t115303: F, t22656: F, t22670: F, t24095: F, t31642: F, t31655: F, t3758: F, t6993: F, t7214: F, t90665: F) -> (F,) {
    let t115306 = 0.63969658155208805863e-1 * t115305;
    let t115308 = t6897 * t80645 * t8621;
    let t115311 = t22633 * t80650 * t31550;
    let t115315 = t26331 * t22635 * t31549 * t3734;
    let t115318 = t22704 * t81326 * t31559;
    let t115322 = -2.0 * t22656 * t7214 - 12.0 * t90665 * t31655 - 2.0 * t3758 * t31642 - t113931 - 2.0 * t24095 * t6993 + t113934 + 0.38381794893125283518e-1 * t115292 + 0.38381794893125283518e-1 * t115294 + 0.3289868133696452873e-1 * t115299 + 0.16449340668482264365e-1 * t115303 - t115306 + 0.82246703342411321824e-2 * t115308 + 0.3289868133696452873e-1 * t115311 - 0.49348022005446793095e-1 * t115315 - t113941 - 0.16449340668482264365e-1 * t115318 - 2.0 * t22670 * t7214;
    (t115322,)
}
