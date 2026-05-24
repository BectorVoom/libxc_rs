//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 399/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk399<F: Float>(t100: F, t1324: F, t55: F, t108: F, t105: F, t109: F, t97: F, tau1: F) -> (F, F, F, F, F) {
    let t1325 = t100 * t1324;
    let t1327 = tau1 * t55;
    let t1329 = -t1324;
    let t1330 = t108 * t1329;
    let t1333 = F::new(5.0) / F::new(3.0) * t105 * t1330 - F::new(5.0) / F::new(3.0) * t1327 * t109 + F::new(5.0) / F::new(3.0) * t97 * t1325;
    (t1325, t1327, t1329, t1330, t1333)
}
