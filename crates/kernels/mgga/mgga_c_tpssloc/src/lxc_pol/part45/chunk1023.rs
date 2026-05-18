//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1023/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1023<F: Float>(t22674: F, t31607: F, t6897: F, t1985: F, t80707: F, t8621: F, t22633: F, t22635: F, t31549: F, t3719: F, t31550: F, t81228: F, t81326: F) -> (F, F, F, F) {
    let t115572 = t6897 * t22674 * t31607;
    let t115577 = t1985 * t80707 * t8621;
    let t115583 = t22633 * t22635 * t31549 * t3719;
    let t115586 = t81228 * t81326 * t31550;
    (t115572, t115577, t115583, t115586)
}
