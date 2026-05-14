//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 501/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk501<F: Float>(t14374: F, t3144: F, t495: F, t515: F, t698: F, t1971: F, t7230: F, t3225: F, t7508: F, t2144: F, t8231: F, t3351: F, t352: F, t875: F, t13799: F, t13803: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14375 = t14374 * t3144;
    let t14384 = t515 * t698 * t495;
    let t14385 = t1971 * t14384;
    let t14386 = t7230 * t14385;
    let t14387 = 0.53205749866622299248e-5 * t14386;
    let t14388 = t7508 * t3225;
    let t14389 = 0.34093327067806677161e-2 * t14388;
    let t14390 = t2144 * t8231;
    let t14391 = t1971 * t14390;
    let t14392 = t3351 * t14391;
    let t14393 = 0.12769379967989351819e-4 * t14392;
    let t14394 = t698 * t352;
    let t14395 = t875 * t14394;
    let t14396 = t1971 * t14395;
    let t14397 = t3351 * t14396;
    let t14398 = 0.85129199786595678796e-5 * t14397;
    let t14399 = 0.20455996240684006296e-1 * t13799;
    let t14400 = 0.40911992481368012592e-1 * t13803;
    (t14375, t14385, t14387, t14389, t14391, t14393, t14396, t14398, t14399, t14400)
}
