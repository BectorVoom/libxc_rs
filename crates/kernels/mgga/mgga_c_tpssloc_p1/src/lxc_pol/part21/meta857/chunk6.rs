//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3114/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3114<F: Float>(t1164: F, t14854: F, t64451: F, t18926: F, t3411: F, t14858: F, t4884: F, t18915: F, t3419: F, t18283: F, t14855: F, t4869: F) -> (F, F, F, F, F, F) {
    let t64454 = F::cast_from(0.10254018858216406658e4_f64) * t1164 * t64451 * t14854;
    let t64456 = F::cast_from(0.11696447245269292414e1_f64) * t3411 * t18926;
    let t64458 = F::cast_from(0.69263436422725855034e2_f64) * t14858 * t4884;
    let t64460 = F::cast_from(0.5848223622634646207e0_f64) * t18915 * t3419;
    let t64462 = F::cast_from(0.69263436422725855036e2_f64) * t3411 * t18283;
    let t64464 = F::cast_from(0.20508037716432813315e4_f64) * t4869 * t14855;
    (t64454, t64456, t64458, t64460, t64462, t64464)
}
