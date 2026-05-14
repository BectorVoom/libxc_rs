//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 983/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk983<F: Float>(t12725: F, t1874: F, t26135: F, t510: F, t652: F, t7000: F, t7685: F, t6876: F, t7688: F, t6999: F, t7753: F, t1983: F, t6880: F, t1266: F, t1976: F, t1980: F, t26002: F, t26005: F, t26006: F, t26098: F, t26138: F, t4026: F, t5361: F, t574: F, t7451: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26141 = 2.0 * t12725 * t1874;
    let t26142 = t510 * t26135;
    let t26144 = 2.0 * t652 * t26142;
    let t26145 = t7685 * t7000;
    let t26147 = 3.0 * t6876 * t7688;
    let t26149 = t7753 * t6999;
    let t26150 = t1983 * t26149;
    let t26153 = 3.0 * t7685 * t6880;
    let t26155 = -t1266 * t7451 - t1976 * t4026 + t1980 * t5361 - t26098 * t510 + t26138 * t574 - t26002 - t26005 - t26006 - t26141 - t26144 - t26145 + t26147 - t26150 + t26153;
    (t26141, t26142, t26144, t26145, t26147, t26149, t26150, t26153, t26155)
}
