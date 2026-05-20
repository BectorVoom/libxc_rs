//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1429/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1429<F: Float>(t20162: F, t7467: F, t55388: F, t7769: F, t28893: F, t105162: F, t105165: F, t105167: F, t105169: F, t105171: F, t105175: F, t105177: F, t105179: F, t105181: F, t105184: F, t105186: F, t105188: F, t105192: F, t105204: F, t105207: F, t1774: F, t1849: F, t20347: F, t20698: F, t2165: F, t2167: F, t27863: F, t29493: F, t29497: F, t5460: F, t652: F) -> (F, F, F, F) {
    let t107581 = F::new(0.405e2) * t20162 * t7467;
    let t107583 = F::new(81.0) * t55388 * t7769;
    let t107585 = F::new(81.0) * t28893 * t7467;
    let t108888 = -F::new(2.0) * t20347 * t2165 * t652 - F::new(6.0) * t1774 * t29493 + F::new(3.0) * t1849 * t29497 + t20698 * t2167 - F::new(12.0) * t27863 * t5460 + t105162 + t105165 - t105167 + t105169 + t105171 + t105175 + t105177 - t105179 - t105181 - t105184 - t105186 + t105188 + t105192 + t105204 - t105207;
    (t107581, t107583, t107585, t108888)
}
