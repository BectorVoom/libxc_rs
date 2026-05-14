//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1165/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1165<F: Float>(t31820: F, t576: F, t1395: F, t8660: F, t2029: F, t7222: F, t2105: F, t7002: F, t2098: F, t7020: F, t25: F, t25353: F, t606: F, t7540: F, t1408: F, t6665: F) -> (F, F, F, F, F, F, F, F) {
    let t116028 = t576 * t31820;
    let t116032 = t1395 * t8660;
    let t116036 = t7222 * t2029;
    let t116038 = t7002 * t2105;
    let t116044 = t2098 * t7020;
    let t118387 = t25 * t25353;
    let t118393 = t606 * t7540;
    let t118410 = t1408 * t6665;
    (t116028, t116032, t116036, t116038, t116044, t118387, t118393, t118410)
}
