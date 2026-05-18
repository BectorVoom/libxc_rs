//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1191/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1191<F: Float>(t300: F, t6063: F, t1166: F, t4858: F, t4874: F, t1164: F, t3411: F, t6098: F, t4869: F, t4884: F, t1147: F, t1156: F, t18785: F) -> (F, F, F, F, F) {
    let t18915 = t300 * t6063;
    let t18917 = F::new(0.5848223622634646207e0) * t18915 * t1166;
    let t18918 = t4874 * t4858;
    let t18920 = F::new(0.23392894490538584828e1) * t1164 * t18918;
    let t18922 = F::new(0.11696447245269292414e1) * t3411 * t6098;
    let t18924 = F::new(0.34631718211362927517e2) * t4869 * t4884;
    let t18926 = t1147 * t18785 * t1156;
    (t18917, t18920, t18922, t18924, t18926)
}
