//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta384 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1187;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1188;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta384<F: Float>(t11677: F, t15027: F, t3624: F, t52627: F, t1213: F, t1735: F, t248: F, t45017: F, t10477: F, t1742: F, t11713: F, t3503: F, t1210: F, t11647: F, t1731: F, t11718: F, t52835: F, t1744: F, t11716: F, t1174: F, t1725: F, t2402: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t52879, t52903, t53079, t53081, t53083) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1187::<F>(t11677, t15027, t3624, t52627, t1213, t1735, t248, t45017, t10477, t1742, t11713, t3503);
        let (t53087, t53099, t53238, t53274, t53336, t53440) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1188::<F>(t11713, t1210, t53081, t11647, t1731, t11718, t52835, t1744, t11716, t1174, t1725, t2402);
    (t52879, t52903, t53079, t53083, t53087, t53099, t53238, t53274, t53336, t53440)
}
