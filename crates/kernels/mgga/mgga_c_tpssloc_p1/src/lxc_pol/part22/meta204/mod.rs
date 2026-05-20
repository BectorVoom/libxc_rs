//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta204 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1187;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1188;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1189;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta204<F: Float>(t1088: F, t5979: F, t123: F, t3237: F, t4721: F, t5973: F, t5977: F, t423: F, t1671: F, t4740: F, t1670: F, t1118: F, t3264: F, t1661: F, t3270: F, t3274: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5980, t5981, t5983, t5985, t5987, t5988, t5989) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1187::<F>(t1088, t5979, t123, t3237, t4721, t5973, t5977, t423, t1671, t4740, t1670, t1118);
        let (t5991, t5992) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1188::<F>(t3264, t5989, t1661);
        let (t5993, t5999) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1189::<F>(t3270, t5992, t3274, t4721, t5973, t5977, t5981);
    (t5980, t5981, t5983, t5985, t5987, t5988, t5989, t5991, t5992, t5993, t5999)
}
