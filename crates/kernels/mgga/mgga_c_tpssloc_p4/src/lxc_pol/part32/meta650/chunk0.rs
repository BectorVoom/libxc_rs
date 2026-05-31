//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2075/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2075<F: Float>(t91137: F, t26297: F, t80853: F, t80855: F, t26301: F, t1831: F, t80866: F, t131: F, t6931: F, t9537: F, t26322: F, t236: F, t26318: F, t91005: F) -> (F, F, F, F, F, F) {
    let t91138 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t91137;
    let t91140 = t80853 * t80855 * t26297;
    let t91141 = F::cast_from(0.40372756094140390854e-3_f64) * t91140;
    let t91143 = t80853 * t80855 * t26301;
    let t91144 = F::cast_from(0.40372756094140390854e-3_f64) * t91143;
    let t91149 = t80866 * t1831;
    let t91152 = t6931 * t131 * t9537;
    let t91154 = t91152 * t80855 * t26322;
    let t91155 = F::cast_from(0.6728792682356731809e-4_f64) * t91154;
    let t91158 = t91152 * t91005 * t236 * t26318;
    (t91138, t91141, t91144, t91149, t91155, t91158)
}
