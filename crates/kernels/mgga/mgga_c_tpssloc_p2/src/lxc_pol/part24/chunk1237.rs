//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1237/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1237<F: Float>(t828: F, t9632: F, t2553: F, t2379: F, t2631: F, t776: F, t1022: F, t2244: F, t1068: F, t3209: F, t1388: F, t3734: F) -> (F, F, F, F, F, F, F) {
    let t46519 = t9632 * t828;
    let t46606 = t2553 * t828;
    let t47072 = t2379 * t828;
    let t47320 = t2631 * t776;
    let t49975 = t2244 * t1022;
    let t50775 = t3209 * t1068;
    let t53789 = t1388 * t3734;
    (t46519, t46606, t47072, t47320, t49975, t50775, t53789)
}
