//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 769/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk769<F: Float>(t262: F, t76052: F, t7204: F, t2367: F, t7778: F, t739: F, t14174: F, t6355: F, t15049: F, t2604: F, t15128: F, t352: F, t8620: F, t1971: F, t3351: F, t7190: F, t8950: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76053 = t262 * t76052;
    let t76054 = t7204 * t76053;
    let t76062 = t7778 * t2367;
    let t76063 = t739 * t76062;
    let t76064 = 0.79828278012425390427e-1 * t76063;
    let t76066 = t6355 * t14174;
    let t76075 = 0.2993560425465952141e-1 * t2604 * t15049;
    let t76077 = t15128 * t352;
    let t76078 = t262 * t76077;
    let t76079 = t8620 * t76078;
    let t76084 = 0.10215503974391481456e-3 * t3351 * t1971 * t7190 * t8950;
    (t76053, t76054, t76062, t76064, t76066, t76075, t76077, t76078, t76079, t76084)
}
