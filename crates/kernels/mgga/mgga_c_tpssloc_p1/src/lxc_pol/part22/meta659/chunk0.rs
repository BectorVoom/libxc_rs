//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2201/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2201<F: Float>(t13261: F, t4166: F, t118: F, t2375: F, t5522: F, t16575: F, t706: F, t16710: F, t2663: F, t157: F, t46387: F, t12939: F, t5392: F, t607: F, t750: F) -> (F, F, F, F, F, F) {
    let t58904 = t4166 * t13261;
    let t58972 = t5522 * t118 * t2375;
    let t58976 = t706 * t16575;
    let t58984 = t16710 * t2663;
    let t58994 = t46387 * t157;
    let t59004 = t12939 * t750 * t5392 * t607;
    (t58904, t58972, t58976, t58984, t58994, t59004)
}
