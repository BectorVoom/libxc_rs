//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 925/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk925<F: Float>(t32814: F, t81651: F, t82074: F, t23168: F, t32789: F, t23185: F, t32862: F, t32863: F, t6579: F, t32823: F, t1484: F, t1902: F) -> (F, F, F, F, F, F) {
    let t118632 = t81651 * t82074 * t32814;
    let t118649 = t23168 * t32789;
    let t118661 = t23185 * t82074 * t32862;
    let t118663 = t6579 * t32863;
    let t118678 = t6579 * t32823;
    let t118690 = t1902 * t1484;
    (t118632, t118649, t118661, t118663, t118678, t118690)
}
