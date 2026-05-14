//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1168/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1168<F: Float>(t29895: F, t30294: F, t29900: F, t30298: F, t2349: F, t50: F, t110143: F, t8269: F, t110532: F, t30311: F, t2281: F, t8266: F, t103: F, t1453: F, t112: F, t30349: F) -> (F, F, F, F, F, F, F, F) {
    let t111109 = 20.0 / 9.0 * t29895 * t30294;
    let t111111 = 50.0 / 27.0 * t29900 * t30298;
    let t111121 = t50 * t2349;
    let t111125 = t110143 * t8269;
    let t111127 = t110532 * t30311;
    let t111129 = t2281 * t8266;
    let t111134 = t103 * t1453;
    let t111226 = t30349 * t112;
    (t111109, t111111, t111121, t111125, t111127, t111129, t111134, t111226)
}
