//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1107/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1107<F: Float>(t6553: F, t7479: F, t6552: F, t1519: F, t225: F, t258: F, t214: F, t1880: F, t1527: F, t6571: F) -> (F, F, F, F, F, F) {
    let t7480 = t6553 * t7479;
    let t7481 = t6552 * t7480;
    let t7484 = t1519 * t225 * t258;
    let t7485 = t214 * t7484;
    let t7486 = t1880 * t7485;
    let t7488 = t6571 * t1527;
    (t7480, t7481, t7484, t7485, t7486, t7488)
}
