//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 730/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk730<F: Float>(t1970: F, t1971: F, t209: F, t2367: F, t476: F, t515: F, t14225: F, t9152: F, t9188: F, t3352: F, t9158: F, t14286: F, t558: F, t262: F, t7192: F, t638: F, t639: F, t668: F, t8849: F) -> (F, F, F, F, F, F, F) {
    let t75192 = t1970 * t1971 * t515 * t2367 * t476 * t209;
    let t75195 = t14225 * t9188 * t9152;
    let t75198 = t14225 * t3352 * t9158;
    let t75200 = t14286 * t558;
    let t75201 = t262 * t75200;
    let t75202 = t7192 * t75201;
    let t75206 = t638 * t639 * t8849 * t668;
    (t75192, t75195, t75198, t75200, t75201, t75202, t75206)
}
