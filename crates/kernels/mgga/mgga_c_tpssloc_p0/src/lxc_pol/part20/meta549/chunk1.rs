//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2094/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2094<F: Float>(t2639: F, t9960: F, t2427: F, t9909: F, t39568: F, t761: F, t2535: F, t9716: F, t39382: F, t2531: F, t9713: F, t39302: F) -> (F, F, F, F, F, F, F) {
    let t41237 = t2639 * t9960;
    let t41251 = t2427 * t9909;
    let t41254 = F::cast_from(0.14035736694323150897e2_f64) * t761 * t39568;
    let t41255 = t9716 * t2535;
    let t41258 = F::cast_from(0.91082604192152556044e5_f64) * t761 * t39382;
    let t41259 = t2531 * t9713;
    let t41262 = F::cast_from(0.5848223622634646207e0_f64) * t761 * t39302;
    (t41237, t41251, t41254, t41255, t41258, t41259, t41262)
}
