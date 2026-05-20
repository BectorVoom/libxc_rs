//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 683/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk683<F: Float>(t225: F, t3023: F, t1053: F, t68: F, t3021: F, t3167: F, t3215: F, t390: F, t1376: F, t3753: F, t3880: F, t3850: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10160 = t3023 * t225;
    let t10163 = t1053 * t1053;
    let t10164 = F::new(1.0) / t10163;
    let t10165 = t68 * t10164;
    let t10170 = t3021 * t225;
    let t11010 = t3167 * t225;
    let t11094 = F::new(1.0) / t3215 / t390;
    let t12019 = t1376 * t1376;
    let t12020 = F::new(1.0) / t12019;
    let t12021 = t68 * t12020;
    let t12030 = t3753 * t225;
    let t12033 = t3880 * t225;
    let t12272 = t562 * t3850;
    (t10160, t10165, t10170, t11010, t11094, t12019, t12020, t12021, t12030, t12033, t12272)
}
