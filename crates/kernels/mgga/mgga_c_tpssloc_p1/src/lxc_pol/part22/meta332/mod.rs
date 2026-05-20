//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1521;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1522;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta332<F: Float>(t12283: F, t5259: F, t5293: F, t120: F, t5286: F, t5303: F, t1340: F, t16060: F, t3789: F, t5234: F, t3798: F, t1354: F, t12211: F, t5223: F, t3804: F, t820: F, t1351: F, t1824: F, t3792: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16239, t16241, t16242, t16269, t16278, t16285, t16288) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1521::<F>(t12283, t5259, t5293, t120, t5286, t5303, t1340, t16060, t3789, t5234, t3798);
        let (t16290, t16294, t16305) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1522::<F>(t1354, t16288, t12211, t5223, t3804, t820);
        let (t16306, t16311) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1523::<F>(t1351, t1824, t3792);
    (t16239, t16241, t16242, t16269, t16278, t16285, t16288, t16290, t16294, t16305, t16306, t16311)
}
