//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 732/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk732<F: Float>(t182: F, t5151: F, t172: F, t1787: F, t763: F, t67: F, t758: F, t193: F, t533: F, t1845: F, t3701: F, t3692: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5153 = F::new(0.19751673498613801407e-1) * t5151 * t182;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    let t5156 = F::new(0.5848223622634646207e0) * t5155;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    let t5159 = F::new(0.18311447306006545054e-3) * t5158;
    let t5160 = t193 * t533;
    let t5161 = t1845 * t3701;
    let t5164 = F::new(0.5848223622634646207e0) * t3692;
    (t5153, t5154, t5155, t5156, t5157, t5158, t5159, t5160, t5161, t5164)
}
