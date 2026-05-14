//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1042/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1042<F: Float>(t22704: F, t31091: F, t81326: F, t2006: F, t213: F, t225: F, t31138: F, t6883: F, t31120: F, t31108: F, t6897: F, t794: F, t114172: F, t22892: F, t6891: F, t22573: F, t8449: F) -> (F, F, F, F, F, F, F) {
    let t114278 = t22704 * t81326 * t31091;
    let t114285 = t213 * t2006 * t225;
    let t114291 = t6883 * t31138;
    let t114296 = t6883 * t31120;
    let t114299 = t6897 * t794 * t31108;
    let t114316 = t22892 * t114172 * t6891;
    let t114360 = t8449 * t22573;
    (t114278, t114285, t114291, t114296, t114299, t114316, t114360)
}
