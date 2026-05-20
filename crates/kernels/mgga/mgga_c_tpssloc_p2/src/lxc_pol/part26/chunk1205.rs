//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1205/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1205<F: Float>(t22666: F, t22685: F, t22686: F, t117: F, t5247: F, t6559: F, t22674: F, t1985: F, t22662: F, t22663: F, t6883: F, t225: F, t22624: F) -> (F, F, F, F, F, F) {
    let t80678 = t22685 * t22666 * t22686;
    let t80681 = t6559 * t5247 * t117;
    let t80683 = t80681 * t22674 * t22686;
    let t80687 = t1985 * t22666 * t22662;
    let t80689 = t6883 * t22663;
    let t80699 = t22624 * t225;
    (t80678, t80681, t80683, t80687, t80689, t80699)
}
