//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2214/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2214<F: Float>(t23204: F, t28298: F, t81640: F, t225: F, t28442: F, t22986: F, t23270: F, t25191: F, t4300: F, t25192: F, t86873: F, t5544: F, t857: F) -> (F, F, F, F, F) {
    let t98237 = t81640 * t23204 * t28298;
    let t98239 = t28442 * t225;
    let t98248 = t22986 * t23270 * t25191 * t4300;
    let t98251 = t22986 * t86873 * t25192;
    let t98253 = t857 * t5544;
    (t98237, t98239, t98248, t98251, t98253)
}
