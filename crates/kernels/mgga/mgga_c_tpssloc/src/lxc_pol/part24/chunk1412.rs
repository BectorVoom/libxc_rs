//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1412/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1412<F: Float>(t28: F, t9516: F, t1081: F, t2749: F, t23788: F, t46298: F, t25891: F, t9616: F, t2745: F, t25927: F, t46362: F, t46252: F) -> (F, F, F, F, F, F, F) {
    let t83613 = t28 * t9516;
    let t83617 = t1081 * t2749;
    let t83624 = t23788 * t46298;
    let t83627 = t25891 * t9616;
    let t83630 = t1081 * t2745;
    let t83645 = t25927 * t46362;
    let t83651 = t23788 * t46252;
    (t83613, t83617, t83624, t83627, t83630, t83645, t83651)
}
