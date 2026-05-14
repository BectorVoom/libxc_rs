//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 850/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk850<F: Float>(t1053: F, t3215: F, t390: F, t1239: F, t3639: F, t500: F, t1376: F, t68: F, t1995: F, t67: F, t246: F, t3700: F, t570: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10163 = t1053 * t1053;
    let t10164 = 1.0 / t10163;
    let t11094 = 1.0 / t3215 / t390;
    let t11604 = t1239 * t1239;
    let t11605 = 1.0 / t11604;
    let t11947 = 1.0 / t3639 / t500;
    let t12019 = t1376 * t1376;
    let t12020 = 1.0 / t12019;
    let t12021 = t68 * t12020;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12461 = 1.0 / t3700 / t570;
    (t10164, t11094, t11605, t11947, t12019, t12020, t12021, t12419, t12461)
}
