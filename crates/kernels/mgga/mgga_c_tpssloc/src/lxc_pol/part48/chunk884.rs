//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 884/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk884<F: Float>(t8533: F, t9348: F, t23831: F, t7042: F, t23858: F, t8607: F, t26161: F, t31775: F, t92200: F, t1983: F, t23857: F, t8640: F, t22949: F, t22584: F, t31758: F, t31035: F, t7217: F) -> (F, F, F, F, F, F, F, F) {
    let t115674 = 2.0 * t9348 * t8533;
    let t115676 = 2.0 * t7042 * t23831;
    let t115678 = 2.0 * t8607 * t23858;
    let t115681 = 4.0 * t26161 * t92200 * t31775;
    let t115684 = 2.0 * t1983 * t8640 * t23857;
    let t115690 = t8607 * t22949;
    let t115695 = 3.0 * t1983 * t31758 * t22584;
    let t115698 = 2.0 * t1983 * t7217 * t31035;
    (t115674, t115676, t115678, t115681, t115684, t115690, t115695, t115698)
}
