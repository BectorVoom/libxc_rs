//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 992/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk992<F: Float>(t77383: F, t3351: F, t3352: F, t44244: F, t515: F, t1971: F, t2144: F, t44293: F, t2447: F, t352: F, t875: F, t15523: F, t7720: F) -> (F, F, F, F, F) {
    let t77384 = F::new(0.42564599893297839398e-5) * t77383;
    let t77387 = t3351 * t3352 * t515 * t44244;
    let t77388 = F::new(0.12769379967989351819e-4) * t77387;
    let t77391 = t3351 * t1971 * t2144 * t44293;
    let t77392 = F::new(0.12769379967989351819e-4) * t77391;
    let t77393 = t2447 * t352;
    let t77396 = t3351 * t1971 * t875 * t77393;
    let t77397 = F::new(0.85129199786595678796e-5) * t77396;
    let t77398 = t7720 * t15523;
    (t77384, t77388, t77392, t77397, t77398)
}
