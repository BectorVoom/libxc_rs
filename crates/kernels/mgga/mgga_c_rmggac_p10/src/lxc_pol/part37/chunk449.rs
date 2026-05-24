//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 449/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk449<F: Float>(t511: F, t9157: F, t209: F, t476: F, t570: F, t515: F, t618: F, t236: F, t498: F, t551: F, t107: F, t500: F) -> (F, F, F, F, F, F, F) {
    let t9158 = t511 * t9157;
    let t9163 = t570 * t476 * t209;
    let t9164 = t515 * t9163;
    let t9169 = t618 * t476 * t209;
    let t9170 = t236 * t9169;
    let t9182 = t551 * t498;
    let t9183 = t236 * t9182;
    let t9187 = t500 * t107;
    (t9158, t9163, t9164, t9169, t9170, t9183, t9187)
}
