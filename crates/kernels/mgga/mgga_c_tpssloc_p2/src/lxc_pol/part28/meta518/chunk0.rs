//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1766/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1766<F: Float>(t22751: F, t22930: F, t22917: F, t22723: F, t22891: F, t22920: F, t117: F, t5247: F, t6559: F, t22674: F, t22686: F, t22663: F, t6883: F) -> (F, F, F, F, F, F, F) {
    let t80665 = t22751 * t22930;
    let t80667 = t22751 * t22917;
    let t80670 = t22723 * t22891;
    let t80671 = t80670 * t22920;
    let t80681 = t6559 * t5247 * t117;
    let t80683 = t80681 * t22674 * t22686;
    let t80689 = t6883 * t22663;
    (t80665, t80667, t80670, t80671, t80681, t80683, t80689)
}
