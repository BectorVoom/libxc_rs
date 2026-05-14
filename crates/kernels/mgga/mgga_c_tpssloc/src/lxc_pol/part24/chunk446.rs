//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 446/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk446<F: Float>(t19: F, t2230: F, t2218: F, t2220: F, t2222: F, t2224: F, t2226: F, t2228: F, t601: F, t604: F) -> (F, F) {
    let t2232 = 0.9492e2 * t19 * t2230;
    let t2233 = t2218 - t2220 + t2222 - t2224 + t2226 - t2228 + t2232;
    let t2235 = t601 * t604;
    (t2233, t2235)
}
