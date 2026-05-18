//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 894/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk894<F: Float>(t15163: F, t34735: F, t14314: F, t551: F, t262: F, t7204: F, t1587: F, t3080: F, t2367: F, t7778: F, t739: F, t14174: F, t6355: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t76046 = t34735 * t15163;
    let t76048 = t14314 * t551;
    let t76049 = t262 * t76048;
    let t76050 = t7204 * t76049;
    let t76052 = t3080 * t1587;
    let t76053 = t262 * t76052;
    let t76054 = t7204 * t76053;
    let t76062 = t7778 * t2367;
    let t76063 = t739 * t76062;
    let t76064 = F::new(0.79828278012425390427e-1) * t76063;
    let t76066 = t6355 * t14174;
    (t76046, t76048, t76049, t76050, t76052, t76053, t76054, t76062, t76064, t76066)
}
