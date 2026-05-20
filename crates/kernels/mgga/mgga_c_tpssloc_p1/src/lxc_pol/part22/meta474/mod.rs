//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1869;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1870;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta474<F: Float>(t20811: F, t20812: F, t20821: F, t20832: F, t225: F, t20756: F, t9946: F, t4226: F, t5544: F, t20800: F, t824: F, t1504: F, t1506: F, t228: F, t230: F, t4225: F, t5601: F, t5605: F, t5608: F, t232: F, t860: F, t1509: F, t5584: F) -> (F, F, F, F, F, F, F, F) {
        let (t20835, t20843, t20846, t20849, t20852) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1869::<F>(t20811, t20812, t20821, t20832, t225, t20756, t9946, t4226, t5544, t20800, t824, t1504, t1506, t228, t230, t4225, t5601, t5605, t5608);
        let (t20853, t20854, t20856) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1870::<F>(t20852, t232, t860, t1509, t5584);
    (t20835, t20843, t20846, t20849, t20852, t20853, t20854, t20856)
}
