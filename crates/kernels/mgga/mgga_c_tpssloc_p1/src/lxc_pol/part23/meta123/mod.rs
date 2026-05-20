//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk618;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk619;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta123<F: Float>(t17: F, t5168: F, t1408: F, t3704: F, t1649: F, t3711: F, t1804: F, t3726: F, t131: F, t3732: F, t205: F, t1799: F, t213: F, t118: F, t794: F, t3739: F, t1808: F, t225: F, t1811: F, t1814: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5169, t5170, t5178, t5192, t5194, t5195, t5196) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk618::<F>(t17, t5168, t1408, t3704, t1649, t3711, t1804, t3726, t131, t3732, t205, t1799, t213);
        let (t5202, t5203, t5215, t5220, t5234) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk619::<F>(t118, t1799, t794, t3739, t1808, t225, t1811, t3726, t1814, t68);
    (t5169, t5170, t5178, t5192, t5194, t5195, t5196, t5202, t5203, t5215, t5220, t5234)
}
