//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1188/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1188<F: Float>(t114226: F, t1799: F, t22633: F, t22635: F, t1992: F, t31091: F, t90566: F, t32698: F, t6883: F, t113946: F, t1842: F, t32705: F, t81159: F, t6897: F, t8458: F, t90544: F) -> (F, F, F, F, F, F) {
    let t120253 = 0.3289868133696452873e-1 * t22633 * t22635 * t114226 * t1799;
    let t120258 = 0.3289868133696452873e-1 * t1992 * t90566 * t31091;
    let t120269 = t6883 * t32698;
    let t120270 = 0.38381794893125283518e-1 * t120269;
    let t120274 = 0.3289868133696452873e-1 * t1992 * t22635 * t113946 * t1842;
    let t120276 = t81159 * t32705;
    let t120277 = 0.76763589786250567037e-1 * t120276;
    let t120296 = t6897 * t90544 * t8458;
    (t120253, t120258, t120270, t120274, t120277, t120296)
}
