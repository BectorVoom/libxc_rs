//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 848/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk848<F: Float>(t13975: F, t8577: F, t1970: F, t1971: F, t209: F, t2367: F, t476: F, t515: F, t14225: F, t9152: F, t9188: F, t3352: F, t9158: F) -> (F, F, F, F) {
    let t75186 = t8577 * t13975;
    let t75192 = t1970 * t1971 * t515 * t2367 * t476 * t209;
    let t75195 = t14225 * t9188 * t9152;
    let t75198 = t14225 * t3352 * t9158;
    (t75186, t75192, t75195, t75198)
}
