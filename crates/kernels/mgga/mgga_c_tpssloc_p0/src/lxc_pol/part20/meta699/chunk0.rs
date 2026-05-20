//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2666/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2666<F: Float>(t5154: F, t9905: F, t15968: F, t67: F, t758: F, t17: F, t750: F, t2225: F, t5166: F, t15921: F, t592: F, t39478: F) -> (F, F, F, F, F, F) {
    let t54392 = t5154 * t9905;
    let t54393 = F::cast_from(0.35089341735807877242e1_f64) * t54392;
    let t54395 = t15968 * t67 * t758;
    let t54396 = F::cast_from(0.54934341918019635162e-3_f64) * t54395;
    let t54398 = t17 * t15968 * t750;
    let t54399 = F::new(3.0) * t54398;
    let t54400 = t2225 * t5166;
    let t54401 = F::new(60.0) * t54400;
    let t54402 = t592 * t15921;
    let t54403 = F::new(24.0) * t54402;
    let t54404 = F::cast_from(0.5848223622634646207e0_f64) * t39478;
    (t54393, t54396, t54399, t54401, t54403, t54404)
}
