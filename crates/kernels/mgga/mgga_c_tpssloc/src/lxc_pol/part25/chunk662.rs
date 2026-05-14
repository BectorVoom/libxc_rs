//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 662/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk662<F: Float>(t1352: F, t7208: F, t553: F, t7191: F, t1332: F, t1336: F, t2089: F, t544: F, t6971: F, t6980: F, t6984: F, t7202: F, t7204: F, t1378: F) -> (F, F, F, F) {
    let t7209 = t7208 * t1352;
    let t7211 = t553 * t7191;
    let t7213 = -t7202 - 0.3289868133696452873e-1 * t6971 - t7204 - 0.16449340668482264365e-1 * t6980 + 0.16449340668482264365e-1 * t6984 + t1332 * t2089 - t1336 * t7209 + t544 * t7211;
    let t7214 = t1378 * t7213;
    (t7209, t7211, t7213, t7214)
}
