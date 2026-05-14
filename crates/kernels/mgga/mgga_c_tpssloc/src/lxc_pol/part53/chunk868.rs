//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 868/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk868<F: Float>(t22779: F, t32714: F, t5230: F, t8465: F, t8467: F, t1814: F, t31175: F, t26288: F, t5308: F, t6950: F, t3701: F, t5187: F, t26179: F, t8327: F, t31058: F, t7458: F) -> (F, F, F, F, F, F, F) {
    let t120410 = t22779 * t32714;
    let t120413 = t5230 * t8465 * t8467;
    let t120416 = t1814 * t31175 * t8467;
    let t120419 = t26288 * t6950 * t5308;
    let t120669 = t3701 * t5187;
    let t120719 = 2.0 * t26179 * t8327;
    let t120721 = 2.0 * t7458 * t31058;
    (t120410, t120413, t120416, t120419, t120669, t120719, t120721)
}
