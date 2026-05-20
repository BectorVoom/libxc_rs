//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1272/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1272<F: Float>(t9218: F, t2230: F, t594: F, t2229: F, t3: F, t19: F, t2239: F, t601: F, t83: F, t84: F, t85: F, t24: F) -> (F, F, F, F, F, F, F, F) {
    let t9219 = F::new(0.12804e4) * t9218;
    let t9220 = t594 * t2230;
    let t9221 = F::new(0.170856e4) * t9220;
    let t9222 = t2229 * t3;
    let t9223 = F::new(1.0) / t9222;
    let t9225 = F::new(0.75936e3) * t19 * t9223;
    let t9231 = t601 * t2239;
    let t9238 = F::new(1.0) / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    (t9219, t9220, t9221, t9223, t9225, t9231, t9238, t9239)
}
