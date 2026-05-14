//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 892/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk892<F: Float>(t113931: F, t113934: F, t113941: F, t115292: F, t115294: F, t115299: F, t115303: F, t115306: F, t115308: F, t115311: F, t115315: F, t115318: F, t22656: F, t22670: F, t24095: F, t31642: F, t31655: F, t3758: F, t6993: F, t7214: F, t90665: F) -> (F,) {
    let t115322 = -2.0 * t22656 * t7214 - 12.0 * t90665 * t31655 - 2.0 * t3758 * t31642 - t113931 - 2.0 * t24095 * t6993 + t113934 + 0.38381794893125283518e-1 * t115292 + 0.38381794893125283518e-1 * t115294 + 0.3289868133696452873e-1 * t115299 + 0.16449340668482264365e-1 * t115303 - t115306 + 0.82246703342411321824e-2 * t115308 + 0.3289868133696452873e-1 * t115311 - 0.49348022005446793095e-1 * t115315 - t113941 - 0.16449340668482264365e-1 * t115318 - 2.0 * t22670 * t7214;
    (t115322,)
}
