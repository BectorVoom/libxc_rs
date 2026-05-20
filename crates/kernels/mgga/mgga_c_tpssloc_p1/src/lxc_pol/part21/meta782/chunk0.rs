//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2716/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2716<F: Float>(t39603: F, t39609: F, t39611: F, t39620: F, t39628: F, t19575: F, t588: F, t39636: F, t19541: F, t2663: F, t39644: F, t54451: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t57203 = F::new(480.0) * t39603;
    let t57204 = F::new(160.0) * t39609;
    let t57205 = F::new(240.0) * t39611;
    let t57206 = F::new(2.0) * t39620;
    let t57207 = F::new(20.0) * t39628;
    let t57208 = t588 * t19575;
    let t57209 = F::new(8.0) * t57208;
    let t57210 = F::new(48.0) * t39636;
    let t57211 = t19541 * t2663;
    let t57212 = F::cast_from(0.24415263074675393405e-3_f64) * t57211;
    let t57213 = F::new(8.0) * t39644;
    let t57214 = F::cast_from(0.2077903092681775651e3_f64) * t54451;
    (t57203, t57204, t57205, t57206, t57207, t57209, t57210, t57212, t57213, t57214)
}
