//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1036/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1036<F: Float>(t1888: F, t23270: F, t25044: F, t5657: F, t1880: F, t25224: F, t28263: F, t28276: F, t6552: F, t1484: F, t25038: F, t98169: F, t20800: F, t6553: F, t6554: F, t28294: F) -> (F, F, F, F, F, F) {
    let t105437 = t1888 * t23270 * t25044 * t5657;
    let t105441 = t1880 * t25224 * t28263;
    let t105445 = t6552 * t25224 * t28276;
    let t105449 = t25038 * t23270 * t98169 * t1484;
    let t105453 = t6552 * t6553 * t6554 * t20800;
    let t105462 = t1880 * t25224 * t28294;
    (t105437, t105441, t105445, t105449, t105453, t105462)
}
