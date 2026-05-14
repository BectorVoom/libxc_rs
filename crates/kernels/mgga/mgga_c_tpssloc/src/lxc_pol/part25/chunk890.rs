//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 890/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk890<F: Float>(t1864: F, t645: F, t192: F, t532: F, t1982: F, t3701: F, t3914: F, t1390: F, t3719: F, t3734: F, t191: F, t3660: F, t1887: F, t6916: F) -> (F, F, F, F, F, F, F, F) {
    let t22550 = t1864 * t645;
    let t22573 = t192 * t532;
    let t22574 = t1982 * t22573;
    let t22578 = t3701 * t3914;
    let t22584 = t1390 * t3719;
    let t22596 = t1390 * t3734;
    let t22607 = t3660 * t191 * t192;
    let t22633 = t6916 * t1887;
    (t22550, t22573, t22574, t22578, t22584, t22596, t22607, t22633)
}
