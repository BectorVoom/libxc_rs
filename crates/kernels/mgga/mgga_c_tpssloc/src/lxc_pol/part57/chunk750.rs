//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 750/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk750<F: Float>(t29342: F, t29359: F, t1378: F, t2091: F, t3887: F, t6460: F, t1375: F, t20029: F, t20044: F, t20060: F, t2092: F, t24156: F, t24157: F, t26361: F, t26475: F, t28207: F, t28211: F, t28214: F, t28234: F, t5215: F, t5321: F, t6440: F, t6461: F, t7194: F, t7925: F, t7937: F) -> (F, F, F, F) {
    let t29360 = t29342 + t29359;
    let t29361 = t1378 * t29360;
    let t29372 = t3887 * t2091 * t6460;
    let t29375 = -2.0 * t20029 * t2092 + 4.0 * t5215 * t7925 - 0.16449340668482264365e-1 * t28207 + 2.0 * t7194 * t6440 - 0.3289868133696452873e-1 * t28211 - 0.6579736267392905746e-1 * t28214 - 0.76763589786250567036e-1 * t26361 - t20044 * t2092 - t1375 * t29361 - t20060 * t2092 - 0.16449340668482264365e-1 * t26475 - 2.0 * t5215 * t7937 - 2.0 * t5321 * t7937 + 0.3289868133696452873e-1 * t28234 - t7194 * t6461 + t24156 + t24157 + 2.0 * t1375 * t29372;
    (t29360, t29361, t29372, t29375)
}
