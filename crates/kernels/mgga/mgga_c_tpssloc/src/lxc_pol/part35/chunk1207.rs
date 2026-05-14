//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1207/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1207<F: Float>(t27700: F, t95422: F, t2132: F, t2136: F, t5398: F, t19040: F, t7345: F, t18392: F, t27617: F, t4993: F, t28525: F, t461: F, t7324: F, t210: F, t29584: F, t27683: F, t27710: F) -> (F, F, F, F, F, F, F, F) {
    let t104364 = t95422 * t27700;
    let t104367 = t2132 * t5398 * t2136;
    let t104369 = t7345 * t19040;
    let t104371 = t7345 * t18392;
    let t104375 = t27617 * t4993;
    let t104387 = t7324 * t28525 * t461;
    let t104410 = t29584 * t210;
    let t104413 = t27710 * t27683;
    (t104364, t104367, t104369, t104371, t104375, t104387, t104410, t104413)
}
