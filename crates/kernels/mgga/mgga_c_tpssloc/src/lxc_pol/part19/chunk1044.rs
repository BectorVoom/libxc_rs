//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1044/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1044<F: Float>(t2511: F, t39377: F, t39378: F, t1294: F, t1307: F, t3918: F, t39335: F, t39338: F, t39340: F, t39342: F, t39346: F, t39349: F, t39350: F, t39356: F, t39360: F, t39364: F, t39366: F, t39367: F, t39373: F, t39375: F, t6999: F) -> (F, F, F, F) {
    let t39380 = t2511 * t2511;
    let t39381 = 1.0 / t39380;
    let t39382 = t39377 * t39378 * t39381;
    let t39384 = 0.91082604192152556044e5 * t1294 * t39382;
    let t39385 = 24.0 * t1307 * t3918 * t39350 - 36.0 * t3918 * t39367 * t6999 - t39335 - t39338 + t39340 - t39342 + t39346 + t39349 + t39356 + t39360 + t39364 - t39366 + t39373 - t39375 - t39384;
    (t39381, t39382, t39384, t39385)
}
