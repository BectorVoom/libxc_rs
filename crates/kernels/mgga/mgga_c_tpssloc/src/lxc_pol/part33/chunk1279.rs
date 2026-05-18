//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1279/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1279<F: Float>(t5527: F, t857: F, t23204: F, t28298: F, t81640: F, t225: F, t28442: F, t5544: F, t23164: F, t28276: F, t28342: F, t81979: F) -> (F, F, F, F, F, F) {
    let t98224 = t857 * t5527;
    let t98237 = t81640 * t23204 * t28298;
    let t98239 = t28442 * t225;
    let t98253 = t857 * t5544;
    let t98322 = t23164 * t23204 * t28276;
    let t98330 = t81979 * t28342;
    (t98224, t98237, t98239, t98253, t98322, t98330)
}
