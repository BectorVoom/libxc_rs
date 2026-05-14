//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 980/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk980<F: Float>(t125: F, t5366: F, t1233: F, t3273: F, t5371: F, t10121: F, t5380: F, t3275: F, t4415: F, t4416: F, t4460: F, t5407: F, t4417: F, t10117: F, t5389: F, t10089: F, t1232: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13675 = t125 * t5366;
    let t13677 = t3273 * t13675 * t1233;
    let t13680 = t125 * t5371;
    let t13682 = t10121 * t13680 * t1233;
    let t13685 = t125 * t5380;
    let t13687 = t3273 * t13685 * t3275;
    let t13691 = t4415 * t4416 * t4460;
    let t13695 = t4415 * t13685 * t1233;
    let t13698 = t125 * t5407;
    let t13700 = t4415 * t13698 * t4417;
    let t13703 = t10117 * t5389;
    let t13705 = t10089 * t1232;
    (t13677, t13682, t13685, t13687, t13691, t13695, t13698, t13700, t13703, t13705)
}
