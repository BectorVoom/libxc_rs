//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2669/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2669<F: Float>(t39540: F, t2221: F, t5168: F, t39571: F, t39581: F, t2225: F, t5154: F, t9892: F, t39601: F, t39605: F, t39607: F, t39609: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t54427 = F::cast_from(0.17544670867903938621e1_f64) * t39540;
    let t54428 = t2221 * t5168;
    let t54429 = F::cast_from(36.0_f64) * t54428;
    let t54430 = F::cast_from(144.0_f64) * t39571;
    let t54431 = F::cast_from(8.0_f64) * t39581;
    let t54432 = t2225 * t5168;
    let t54433 = F::cast_from(60.0_f64) * t54432;
    let t54434 = t5154 * t9892;
    let t54435 = F::cast_from(0.51947577317044391277e2_f64) * t54434;
    let t54436 = F::cast_from(12.0_f64) * t39601;
    let t54437 = F::cast_from(960.0_f64) * t39605;
    let t54438 = F::cast_from(192.0_f64) * t39607;
    let t54439 = F::cast_from(240.0_f64) * t39609;
    (t54427, t54429, t54430, t54431, t54433, t54435, t54436, t54437, t54438, t54439)
}
