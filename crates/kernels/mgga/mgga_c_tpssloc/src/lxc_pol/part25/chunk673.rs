//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 673/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk673<F: Float>(t5: F, t2235: F, t2240: F, t2241: F, t2307: F, t605: F, t645: F, t86: F, t9226: F, t9228: F, t9231: F, t9239: F, t9240: F, t9243: F, t9342: F, t112: F, t111: F, t2311: F) -> (F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t9346 = piecewise3(t8, 0.0, -12.0 * t2235 * t2307 + 60.0 * t2240 * t9243 + 60.0 * t2241 * t9231 - 4.0 * t605 * t9342 - 12.0 * t645 * t9228 + t86 * t9226 - 120.0 * t9239 * t9240);
    let t9347 = t9346 * t112;
    let t9348 = t2311 * t111;
    (t9346, t9347, t9348)
}
