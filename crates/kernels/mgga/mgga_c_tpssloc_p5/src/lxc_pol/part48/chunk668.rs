//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 668/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk668<F: Float>(t1239: F, t68: F, t225: F, t3484: F, t3591: F, t3482: F, t3639: F, t500: F, t1376: F, t3753: F, t3880: F, t3850: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11604 = t1239 * t1239;
    let t11605 = F::new(1.0) / t11604;
    let t11606 = t68 * t11605;
    let t11613 = t3484 * t225;
    let t11925 = t3591 * t225;
    let t11928 = t3482 * t225;
    let t11947 = F::new(1.0) / t3639 / t500;
    let t12019 = t1376 * t1376;
    let t12020 = F::new(1.0) / t12019;
    let t12021 = t68 * t12020;
    let t12030 = t3753 * t225;
    let t12033 = t3880 * t225;
    let t12272 = t562 * t3850;
    (t11606, t11613, t11925, t11928, t11947, t12019, t12020, t12021, t12030, t12033, t12272)
}
