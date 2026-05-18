//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 762/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk762<F: Float>(t28130: F, t6976: F, t22633: F, t19743: F, t3792: F, t22897: F, t1992: F, t6347: F, t6968: F, t6637: F, t6888: F, t6330: F) -> (F, F, F, F) {
    let t28131 = t6976 * t28130;
    let t28132 = t22633 * t28131;
    let t28134 = t19743 * t3792;
    let t28135 = t22897 * t28134;
    let t28136 = t1992 * t28135;
    let t28138 = t6968 * t6347;
    let t28139 = t6637 * t28138;
    let t28140 = t6888 * t28139;
    let t28142 = t6968 * t6330;
    (t28132, t28136, t28140, t28142)
}
