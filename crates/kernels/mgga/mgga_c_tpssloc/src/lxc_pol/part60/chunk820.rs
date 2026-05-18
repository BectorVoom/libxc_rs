//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 820/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk820<F: Float>(t29286: F, t553: F, t24127: F, t6388: F, t1336: F, t1814: F, t2089: F, t24099: F, t26381: F, t26393: F, t26406: F, t28132: F, t28136: F, t28140: F, t28144: F, t28150: F, t544: F, t6378: F, t7934: F) -> F {
    let t29327 = t553 * t29286;
    let t29339 = t24127 * t6388;
    let t29342 = F::new(0.15352717957250113407e0) * t26381 - t24099 + t544 * t29327 + F::new(0.3289868133696452873e-1) * t26393 + F::new(2.0) * t1814 * t7934 + F::new(0.6579736267392905746e-1) * t28132 + t6378 * t2089 + F::new(0.3289868133696452873e-1) * t28136 + F::new(0.76763589786250567036e-1) * t26406 - F::new(0.3289868133696452873e-1) * t28140 + F::new(0.9869604401089358619e-1) * t28144 - F::new(0.6579736267392905746e-1) * t28150 + F::new(2.0) * t1336 * t29339;
    t29342
}
