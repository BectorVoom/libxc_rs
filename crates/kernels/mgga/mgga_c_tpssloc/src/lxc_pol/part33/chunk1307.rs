//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1307/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1307<F: Float>(t105201: F, t22574: F, t26162: F, t1873: F, t22425: F, t652: F, t105162: F, t105165: F, t105167: F, t105169: F, t105171: F, t105175: F, t105177: F, t105179: F, t105181: F, t105184: F, t105186: F, t105188: F, t105192: F, t1459: F, t1980: F, t20698: F, t20717: F, t20720: F, t28855: F, t4028: F, t6517: F, t96686: F) -> F {
    let t105204 = F::new(18.0) * t22574 * t26162 * t105201;
    let t105207 = F::new(2.0) * t652 * t22425 * t1873;
    let t105208 = -F::new(6.0) * t1459 * t96686 + t1980 * t20698 - F::new(6.0) * t20717 * t6517 - F::new(2.0) * t20720 * t6517 - F::new(12.0) * t28855 * t4028 + t105162 + t105165 - t105167 + t105169 + t105171 + t105175 + t105177 - t105179 - t105181 - t105184 - t105186 + t105188 + t105192 + t105204 - t105207;
    t105208
}
