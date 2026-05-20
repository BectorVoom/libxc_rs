//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3002/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3002<F: Float>(t1581: F, t49541: F, t60887: F, t14473: F, t4498: F, t60332: F, t942: F, t951: F, t959: F, t10623: F, t5808: F, t17954: F, t2907: F) -> (F, F, F, F, F) {
    let t62742 = F::cast_from(0.14035736694323150897e2_f64) * t49541 * t1581 * t60887;
    let t62744 = F::cast_from(0.69263436422725855034e2_f64) * t14473 * t4498;
    let t62748 = F::cast_from(0.5848223622634646207e0_f64) * t959 * t942 * t60332 * t951;
    let t62750 = F::cast_from(0.5848223622634646207e0_f64) * t10623 * t5808;
    let t62753 = F::cast_from(0.35089341735807877242e1_f64) * t959 * t17954 * t2907;
    (t62742, t62744, t62748, t62750, t62753)
}
