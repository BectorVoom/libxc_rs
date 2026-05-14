//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 992/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk992<F: Float>(t1653: F, t6219: F, t3578: F, t1735: F, t5971: F, t11668: F, t5979: F, t1730: F, t6164: F, t2130: F, t47: F, t479: F, t471: F, t21762: F, t248: F, t3585: F) -> (F, F, F, F, F, F, F) {
    let t22153 = t6219 * t1653;
    let t22154 = t3578 * t22153;
    let t22157 = t1735 * t5971;
    let t22158 = t11668 * t22157;
    let t22161 = t1735 * t5979;
    let t22162 = t3578 * t22161;
    let t22169 = t1730 * t6164;
    let t22173 = 1.0 / t47 / t2130;
    let t22174 = t479 * t22173;
    let t22175 = t471 * t22174;
    let t22185 = t248 * t3585 * t21762;
    (t22154, t22158, t22162, t22169, t22173, t22175, t22185)
}
