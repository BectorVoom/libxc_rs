//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2673/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2673<F: Float>(t39365: F, t19681: F, t2371: F, t54380: F, t54382: F, t39374: F, t39387: F, t20067: F, t3719: F, t3918: F, t39360: F, t39364: F, t39373: F, t39384: F) -> (F, F, F, F, F, F, F) {
    let t56167 = F::cast_from(0.11393789434848516922e-2_f64) * t39365;
    let t56168 = t19681 * t2371;
    let t56169 = F::cast_from(0.11696447245269292414e1_f64) * t56168;
    let t56170 = F::cast_from(0.32530743900905219526e-1_f64) * t54380;
    let t56171 = F::cast_from(0.96319466275353142155e0_f64) * t54382;
    let t56172 = F::cast_from(0.20508037716432813316e4_f64) * t39374;
    let t56173 = F::cast_from(0.5848223622634646207e0_f64) * t39387;
    let t56174 = F::new(3.0) * t20067 * t3719 * t3918 + t39360 + t39364 + t39373 - t39384 - t56167 + t56169 + t56170 + t56171 - t56172 - t56173;
    (t56167, t56169, t56170, t56171, t56172, t56173, t56174)
}
