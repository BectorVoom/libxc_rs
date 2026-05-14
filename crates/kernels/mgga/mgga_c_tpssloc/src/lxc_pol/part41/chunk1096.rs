//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1096/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1096<F: Float>(t11883: F, t1215: F, t6252: F, t1751: F, t5011: F, t1246: F, t6238: F, t19145: F, t3612: F, t1734: F, t5052: F, t1235: F, t6218: F, t19120: F, t493: F, t1243: F, t19045: F) -> (F, F, F, F, F, F, F, F) {
    let t19165 = t11883 * t1215;
    let t19166 = t6252 * t19165;
    let t19169 = t1751 * t5011;
    let t19170 = t19169 * t1246;
    let t19173 = t6238 * t1215;
    let t19174 = t19173 * t1246;
    let t19176 = t19145 * t3612;
    let t19179 = t5052 * t1734;
    let t19180 = t19179 * t1246;
    let t19189 = t1235 * t6218;
    let t19190 = t19189 * t1246;
    let t19197 = t493 * t19120;
    let t19201 = t19045 * t1243;
    (t19166, t19170, t19174, t19176, t19180, t19190, t19197, t19201)
}
