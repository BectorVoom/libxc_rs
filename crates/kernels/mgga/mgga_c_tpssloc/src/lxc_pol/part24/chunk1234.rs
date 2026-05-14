//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1234/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1234<F: Float>(t23384: F, t23693: F, t23698: F, t3166: F, t362: F, t23383: F, t6712: F) -> (F, F, F, F) {
    let t82562 = t23384 * t23693;
    let t82564 = t23384 * t23698;
    let t82566 = t362 * t3166;
    let t82573 = t6712 * t23383;
    (t82562, t82564, t82566, t82573)
}
