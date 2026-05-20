//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1379/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1379<F: Float>(t11282: F, t1164: F, t3403: F, t43679: F, t11294: F, t3411: F, t11131: F, t3399: F, t3402: F, t11176: F, t300: F, t1166: F) -> (F, F, F, F, F, F, F) {
    let t43683 = F::cast_from(0.6233709278045326953e3_f64) * t1164 * t11282 * t43679 * t3403;
    let t43685 = F::cast_from(0.4155806185363551302e3_f64) * t3411 * t11294;
    let t43687 = F::cast_from(0.14035736694323150897e2_f64) * t3411 * t11131;
    let t43688 = t3399 * t3399;
    let t43689 = F::new(1.0) / t43688;
    let t43691 = t3402 * t3402;
    let t43692 = F::new(1.0) / t43691;
    let t43695 = F::cast_from(0.91082604192152556044e5_f64) * t1164 * t43689 * t43679 * t43692;
    let t43700 = t300 * t11176;
    let t43702 = F::cast_from(0.23392894490538584828e1_f64) * t43700 * t1166;
    (t43683, t43685, t43687, t43689, t43692, t43695, t43702)
}
