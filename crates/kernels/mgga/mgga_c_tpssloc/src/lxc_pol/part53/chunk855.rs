//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 855/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk855<F: Float>(t115305: F, t115330: F, t225: F, t32164: F, t115390: F, t115432: F, t115434: F, t113981: F, t114025: F, t114027: F, t114038: F, t1338: F, t32147: F, t32168: F, t115539: F, t115550: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t117128 = 0.25587863262083522346e0 * t115305;
    let t117133 = 0.3289868133696452873e-1 * t115330;
    let t117173 = t32164 * t225;
    let t117193 = 0.3289868133696452873e-1 * t115390;
    let t117209 = 0.10417915756705434098e0 * t115432;
    let t117210 = 0.25587863262083522346e0 * t115434;
    let t117217 = 0.26915170729426927235e-3 * t113981;
    let t117231 = 0.84334201618871038669e-2 * t114025;
    let t117232 = 0.18086994730174895102e0 * t114027;
    let t117235 = 119.0 / 1728.0 * t114038;
    let t117246 = t1338 * t32147;
    let t117275 = t32168 * t225;
    let t117284 = 0.10417915756705434098e0 * t115539;
    let t117287 = 0.3289868133696452873e-1 * t115550;
    (t117128, t117133, t117173, t117193, t117209, t117210, t117217, t117231, t117232, t117235, t117246, t117275, t117284, t117287)
}
