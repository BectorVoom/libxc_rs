//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 540/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk540<F: Float>(t13839: F, t15272: F, t234: F, t551: F, t3157: F, t3167: F, t8368: F, t2367: F, t649: F, t27: F, t2145: F, t262: F, t3068: F, t7282: F, t2411: F, t3140: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15273 = t13839 * t15272;
    let t15280 = t234 * t551;
    let t15281 = t15280 * t3157;
    let t15284 = t8368 * t3167;
    let t15286 = t649 * t2367;
    let t15287 = t27 * t15286;
    let t15288 = t2145 * t15287;
    let t15290 = t262 * t551;
    let t15291 = t3068 * t15290;
    let t15292 = t7282 * t15291;
    let t15296 = t2411 * t3140;
    (t15273, t15280, t15281, t15284, t15287, t15288, t15290, t15291, t15292, t15296)
}
