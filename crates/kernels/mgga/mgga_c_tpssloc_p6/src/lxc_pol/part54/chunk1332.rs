//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1332/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1332<F: Float>(t32698: F, t6883: F, t113946: F, t1842: F, t1992: F, t22635: F, t32705: F, t81159: F, t6897: F, t8458: F, t90544: F, t114154: F) -> (F, F, F, F, F) {
    let t120269 = t6883 * t32698;
    let t120270 = F::cast_from(0.38381794893125283518e-1_f64) * t120269;
    let t120274 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t22635 * t113946 * t1842;
    let t120276 = t81159 * t32705;
    let t120277 = F::cast_from(0.76763589786250567037e-1_f64) * t120276;
    let t120296 = t6897 * t90544 * t8458;
    let t120297 = F::cast_from(0.82246703342411321825e-2_f64) * t120296;
    let t120304 = F::cast_from(0.82246703342411321825e-2_f64) * t114154;
    (t120270, t120274, t120277, t120297, t120304)
}
