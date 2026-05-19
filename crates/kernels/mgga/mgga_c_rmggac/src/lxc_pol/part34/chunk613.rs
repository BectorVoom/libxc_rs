//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 613/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk613<F: Float>(t15470: F, t739: F, t15206: F, t15209: F, t15212: F, t15215: F, t515: F, t9523: F, t3352: F, t3351: F, t15218: F, t15221: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15471 = t739 * t15470;
    let t15472 = F::cast_from(0.2993560425465952141e-1_f64) * t15471;
    let t15473 = F::cast_from(0.87596530464506835935e-6_f64) * t15206;
    let t15474 = F::cast_from(0.19709219354514038085e-5_f64) * t15209;
    let t15475 = F::cast_from(0.87596530464506835935e-6_f64) * t15212;
    let t15476 = F::cast_from(0.2627895913935205078e-5_f64) * t15215;
    let t15477 = t515 * t9523;
    let t15478 = t3352 * t15477;
    let t15479 = t3351 * t15478;
    let t15480 = F::cast_from(0.12769379967989351819e-4_f64) * t15479;
    let t15481 = F::cast_from(0.85129199786595678799e-5_f64) * t15218;
    let t15482 = F::cast_from(0.85129199786595678799e-5_f64) * t15221;
    (t15472, t15473, t15474, t15475, t15476, t15478, t15480, t15481, t15482)
}
