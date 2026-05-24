//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 971/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk971<F: Float>(t41218: F, t7603: F, t41221: F, t41224: F, t41227: F, t8761: F, t41276: F, t1635: F, t2084: F, t8746: F, t1624: F, t8764: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41285 = t7603 * t41218;
    let t41287 = t7603 * t41221;
    let t41289 = t7603 * t41224;
    let t41291 = t8761 * t41227;
    let t41294 = t8761 * t41276;
    let t41296 = t2084 * t1635;
    let t41297 = t8746 * t41296;
    let t41299 = t8761 * t41296;
    let t41301 = t2084 * t1624;
    let t41302 = t8764 * t41301;
    (t41285, t41287, t41289, t41291, t41294, t41297, t41299, t41301, t41302)
}
