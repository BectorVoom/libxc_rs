//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1151/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1151<F: Float>(t1164: F, t18926: F, t4869: F, t4875: F, t18711: F, t300: F, t3375: F, t6084: F, t1157: F, t3411: F, t6102: F, t18682: F, t18685: F, t18688: F, t18690: F, t18692: F, t18694: F, t18696: F, t18837: F, t18839: F, t18917: F, t18920: F, t18922: F, t18924: F) -> (F, F, F, F, F, F) {
    let t18928 = F::new(0.5848223622634646207e0) * t1164 * t18926;
    let t18930 = F::new(0.23392894490538584828e1) * t4869 * t4875;
    let t18932 = F::new(0.19751673498613801407e-1) * t300 * t18711;
    let t18933 = t3375 * t6084;
    let t18934 = t18933 * t1157;
    let t18936 = F::new(0.11696447245269292414e1) * t1164 * t18934;
    let t18938 = F::new(0.5848223622634646207e0) * t3411 * t6102;
    let t18939 = -t18682 - t18685 - t18917 + t18920 + t18922 - t18924 + t18688 + t18690 + t18692 - t18694 + t18696 - t18928 + t18930 + t18932 + t18837 + t18839 + t18936 - t18938;
    (t18928, t18930, t18932, t18936, t18938, t18939)
}
