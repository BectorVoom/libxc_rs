//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1083/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1083<F: Float>(t300: F, t6063: F, t1166: F, t4858: F, t4874: F, t1164: F, t3411: F, t6098: F, t4869: F, t4884: F, t1147: F, t1156: F, t18785: F, t4875: F, t18711: F, t3375: F, t6084: F) -> (F, F, F, F, F, F, F, F) {
    let t18915 = t300 * t6063;
    let t18917 = 0.5848223622634646207e0 * t18915 * t1166;
    let t18918 = t4874 * t4858;
    let t18920 = 0.23392894490538584828e1 * t1164 * t18918;
    let t18922 = 0.11696447245269292414e1 * t3411 * t6098;
    let t18924 = 0.34631718211362927517e2 * t4869 * t4884;
    let t18926 = t1147 * t18785 * t1156;
    let t18928 = 0.5848223622634646207e0 * t1164 * t18926;
    let t18930 = 0.23392894490538584828e1 * t4869 * t4875;
    let t18932 = 0.19751673498613801407e-1 * t300 * t18711;
    let t18933 = t3375 * t6084;
    (t18917, t18920, t18922, t18924, t18928, t18930, t18932, t18933)
}
