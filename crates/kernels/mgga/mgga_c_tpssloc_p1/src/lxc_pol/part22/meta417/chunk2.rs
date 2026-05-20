//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1730/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1730<F: Float>(t1166: F, t18915: F, t4858: F, t4874: F, t1164: F, t3411: F, t6098: F, t4869: F, t4884: F, t1147: F, t1156: F, t18785: F) -> (F, F, F, F, F, F) {
    let t18917 = F::cast_from(0.5848223622634646207e0_f64) * t18915 * t1166;
    let t18918 = t4874 * t4858;
    let t18920 = F::cast_from(0.23392894490538584828e1_f64) * t1164 * t18918;
    let t18922 = F::cast_from(0.11696447245269292414e1_f64) * t3411 * t6098;
    let t18924 = F::cast_from(0.34631718211362927517e2_f64) * t4869 * t4884;
    let t18926 = t1147 * t18785 * t1156;
    (t18917, t18918, t18920, t18922, t18924, t18926)
}
