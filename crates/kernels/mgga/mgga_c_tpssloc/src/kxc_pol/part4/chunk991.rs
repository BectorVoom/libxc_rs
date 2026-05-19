//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 991/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk991<F: Float>(t16562: F, t16574: F, t145: F, t185: F, t5520: F, t751: F, t157: F, t182: F, t12861: F, t4119: F, t4315: F, t5392: F) -> (F, F, F, F, F, F) {
    let t16575 = t16562 + t16574;
    let t16576 = t145 * t16575;
    let t16577 = t16576 * t185;
    let t16578 = t5520 * t751;
    let t16579 = t16575 * t157;
    let t16581 = F::cast_from(0.19751673498613801407e-1_f64) * t16579 * t182;
    let t16582 = F::new(2.0) * t12861;
    let t16583 = t4315 * t4119;
    let t16586 = t751 * t5392;
    (t16577, t16578, t16581, t16582, t16583, t16586)
}
