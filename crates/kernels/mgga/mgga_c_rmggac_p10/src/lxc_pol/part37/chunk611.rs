//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 611/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk611<F: Float>(t15471: F, t15209: F, t15215: F, t515: F, t9523: F, t3352: F, t3351: F, t15218: F, t15221: F, t15228: F, t15232: F, t15236: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15472 = F::cast_from(0.2993560425465952141e-1_f64) * t15471;
    let t15474 = F::cast_from(0.19709219354514038085e-5_f64) * t15209;
    let t15476 = F::cast_from(0.2627895913935205078e-5_f64) * t15215;
    let t15477 = t515 * t9523;
    let t15478 = t3352 * t15477;
    let t15479 = t3351 * t15478;
    let t15480 = F::cast_from(0.12769379967989351819e-4_f64) * t15479;
    let t15481 = F::cast_from(0.85129199786595678799e-5_f64) * t15218;
    let t15482 = F::cast_from(0.85129199786595678799e-5_f64) * t15221;
    let t15485 = F::cast_from(0.15961724959986689775e-4_f64) * t15228;
    let t15486 = F::cast_from(0.1276937996798935182e-4_f64) * t15232;
    let t15487 = F::cast_from(0.2553875993597870364e-4_f64) * t15236;
    (t15472, t15474, t15476, t15478, t15480, t15481, t15482, t15485, t15486, t15487)
}
