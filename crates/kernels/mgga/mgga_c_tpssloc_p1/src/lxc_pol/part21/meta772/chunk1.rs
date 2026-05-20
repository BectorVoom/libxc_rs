//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2674/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2674<F: Float>(t54387: F, t54389: F, t19575: F, t592: F, t15904: F, t16486: F, t16497: F, t1845: F, t193: F, t19603: F, t33159: F, t39393: F, t39397: F, t39400: F, t39408: F, t39411: F, t5126: F, t5160: F, t5161: F, t5308: F, t531: F, t55224: F) -> (F, F, F, F) {
    let t56178 = F::cast_from(0.11696447245269292414e1_f64) * t54387;
    let t56179 = F::cast_from(0.11696447245269292414e1_f64) * t54389;
    let t56185 = t592 * t19575;
    let t56186 = F::new(8.0) * t56185;
    let t56192 = -F::new(24.0) * t15904 * t1845 * t193 * t33159 * t531 - F::new(2.0) * t16486 * t5160 * t5161 + F::new(24.0) * t16497 * t5126 * t5308 + F::new(24.0) * t19603 * t55224 + t39393 - t39397 - t39400 + t39408 + t39411 - t56178 - t56179 - t56186;
    (t56178, t56179, t56186, t56192)
}
