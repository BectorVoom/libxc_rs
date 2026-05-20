//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta121 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk614;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk615;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta121<F: Float>(t1193: F, t1706: F, t135: F, t1725: F, t1174: F, t1752: F, t225: F, t1243: F, t5000: F, t1390: F, t1845: F, t193: F, t531: F, t1799: F, t571: F, t1408: F, t3664: F, t1649: F, t3672: F, t172: F, t1787: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t5036, t5040, t5041, t5055, t5064) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk614::<F>(t1193, t1706, t135, t1725, t1174, t1752, t225, t1243, t5000);
        let t5122 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk615::<F>(t1390, t1845);
        let (t5126, t5127, t5134, t5142, t5154) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk616::<F>(t193, t531, t1799, t571, t1408, t3664, t1649, t3672, t172, t1787);
    (t5036, t5040, t5041, t5055, t5064, t5122, t5126, t5127, t5134, t5142, t5154)
}
