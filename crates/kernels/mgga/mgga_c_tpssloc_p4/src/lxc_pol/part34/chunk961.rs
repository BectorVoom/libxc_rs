//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 961/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk961<F: Float>(t1653: F, t6219: F, t3578: F, t1735: F, t5971: F, t11668: F, t5979: F, t1730: F, t6164: F, t2130: F, t47: F, t479: F) -> (F, F, F, F, F) {
    let t22153 = t6219 * t1653;
    let t22154 = t3578 * t22153;
    let t22157 = t1735 * t5971;
    let t22158 = t11668 * t22157;
    let t22161 = t1735 * t5979;
    let t22162 = t3578 * t22161;
    let t22169 = t1730 * t6164;
    let t22173 = F::cast_from(1.0_f64) / t47 / t2130;
    let t22174 = t479 * t22173;
    (t22154, t22158, t22162, t22169, t22174)
}
