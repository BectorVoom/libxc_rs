//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 920/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk920<F: Float>(t12823: F, t8327: F, t31058: F, t4034: F, t9348: F, t191: F, t192: F, t23855: F, t22947: F, t3701: F, t31054: F, t31056: F) -> (F, F, F, F, F, F, F) {
    let t112535 = F::new(2.0) * t12823 * t8327;
    let t112537 = F::new(4.0) * t4034 * t31058;
    let t112542 = F::new(2.0) * t9348 * t8327;
    let t112547 = t23855 * t191 * t192;
    let t112611 = t3701 * t22947;
    let t112620 = F::new(4.0) * t31054;
    let t112621 = F::new(4.0) * t31056;
    (t112535, t112537, t112542, t112547, t112611, t112620, t112621)
}
