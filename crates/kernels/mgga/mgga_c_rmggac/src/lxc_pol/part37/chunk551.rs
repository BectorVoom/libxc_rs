//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 551/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk551<F: Float>(t515: F, t9523: F, t3352: F, t3351: F, t15218: F, t15221: F, t15228: F, t15232: F, t15236: F, t875: F, t9551: F, t1971: F, t2338: F, t702: F, t638: F, t639: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15477 = t515 * t9523;
    let t15478 = t3352 * t15477;
    let t15479 = t3351 * t15478;
    let t15480 = 0.12769379967989351819e-4 * t15479;
    let t15481 = 0.85129199786595678799e-5 * t15218;
    let t15482 = 0.85129199786595678799e-5 * t15221;
    let t15485 = 0.15961724959986689775e-4 * t15228;
    let t15486 = 0.1276937996798935182e-4 * t15232;
    let t15487 = 0.2553875993597870364e-4 * t15236;
    let t15488 = t875 * t9551;
    let t15489 = t1971 * t15488;
    let t15490 = t3351 * t15489;
    let t15491 = 0.85129199786595678796e-5 * t15490;
    let t15492 = t2338 * t702;
    let t15494 = t638 * t639 * t15492;
    (t15478, t15480, t15481, t15482, t15485, t15486, t15487, t15489, t15491, t15492, t15494)
}
