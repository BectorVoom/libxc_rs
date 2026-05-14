//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 912/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk912<F: Float>(t22949: F, t8607: F, t1983: F, t22584: F, t31758: F, t31035: F, t7217: F, t22597: F, t12734: F, t8533: F, t2314: F, t31772: F, t1874: F, t91857: F, t26977: F, t6525: F) -> (F, F, F, F, F, F, F, F) {
    let t115690 = t8607 * t22949;
    let t115695 = 3.0 * t1983 * t31758 * t22584;
    let t115698 = 2.0 * t1983 * t7217 * t31035;
    let t115700 = 6.0 * t8607 * t22597;
    let t115702 = 4.0 * t12734 * t8533;
    let t115704 = 4.0 * t2314 * t31772;
    let t115708 = 2.0 * t91857 * t1874;
    let t115712 = 4.0 * t26977 * t6525;
    (t115690, t115695, t115698, t115700, t115702, t115704, t115708, t115712)
}
