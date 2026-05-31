//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 898/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk898<F: Float>(t12823: F, t8327: F, t31058: F, t4034: F, t9348: F, t22947: F, t3701: F, t31054: F, t31056: F, t31059: F, t214: F, t6624: F) -> (F, F, F, F, F, F, F, F) {
    let t112535 = F::cast_from(2.0_f64) * t12823 * t8327;
    let t112537 = F::cast_from(4.0_f64) * t4034 * t31058;
    let t112542 = F::cast_from(2.0_f64) * t9348 * t8327;
    let t112611 = t3701 * t22947;
    let t112620 = F::cast_from(4.0_f64) * t31054;
    let t112621 = F::cast_from(4.0_f64) * t31056;
    let t112622 = F::cast_from(4.0_f64) * t31059;
    let t112660 = t214 * t6624;
    (t112535, t112537, t112542, t112611, t112620, t112621, t112622, t112660)
}
