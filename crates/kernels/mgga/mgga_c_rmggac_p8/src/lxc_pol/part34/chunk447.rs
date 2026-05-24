//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 447/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk447<F: Float>(t461: F, t9085: F, t674: F, t2185: F, t2411: F, t2144: F, t5267: F, t333: F, t618: F, t511: F, t352: F, t515: F) -> (F, F, F, F, F, F) {
    let t9086 = t9085 * t461;
    let t9087 = t9086 * t674;
    let t9090 = t2411 * t2185;
    let t9095 = t2144 * t5267;
    let t9104 = t618 * t333;
    let t9105 = t511 * t9104;
    let t9109 = t618 * t352;
    let t9110 = t515 * t9109;
    (t9086, t9087, t9090, t9095, t9105, t9110)
}
