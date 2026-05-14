//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 869/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk869<F: Float>(t11023: F, t3201: F, t1003: F, t10359: F, t1058: F, t1061: F, t1063: F, t11024: F, t11028: F, t11031: F, t11034: F, t11037: F, t11040: F, t11043: F, t11046: F, t11049: F, t11051: F, t11055: F, t11059: F, t11061: F, t11065: F, t11067: F, t11078: F, t3076: F, t3180: F, t3186: F, t3189: F, t3193: F, t3197: F, t3200: F, t3202: F, t3204: F, t353: F, t384: F) -> (F,) {
    let t11081 = t11023 * t3201;
    let t11084 = 3.0 * t3180 * t3197 + 6.0 * t3186 * t11024 + t1058 * t11028 + 3.0 * t1058 * t11031 + 6.0 * t11034 * t3189 - 3.0 * t11037 * t3202 - 3.0 * t3200 * t11040 + t353 * t11043 + t11046 * t11049 + 3.0 * t11051 * t1061 + 6.0 * t3186 * t11055 + 6.0 * t11059 * t11061 - 6.0 * t11065 * t11067 + 3.0 * t1003 * t3204 + 3.0 * t3076 * t1063 + t10359 * t384 + 6.0 * t3180 * t3193 + 3.0 * t1058 * t11078 - 3.0 * t3200 * t11081;
    (t11084,)
}
