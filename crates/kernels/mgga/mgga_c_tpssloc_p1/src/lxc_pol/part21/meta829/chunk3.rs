//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2925/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2925<F: Float>(t17934: F, t2944: F, t10623: F, t5804: F, t59981: F, t60006: F, t60008: F, t60010: F, t60016: F, t60021: F, t60023: F, t60025: F, t60027: F, t60029: F, t60033: F, t60035: F) -> (F, F, F) {
    let t60906 = F::cast_from(0.11696447245269292414e1_f64) * t17934 * t2944;
    let t60908 = F::cast_from(0.11696447245269292414e1_f64) * t10623 * t5804;
    let t60909 = t60906 + t60908 + t59981 - t60006 + t60008 - t60010 - t60016 + t60021 + t60023 - t60025 - t60027 - t60029 + t60033 - t60035;
    (t60906, t60908, t60909)
}
