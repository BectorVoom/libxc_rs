//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta478 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1432;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1433;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta478<F: Float>(t1164: F, t43689: F, t43692: F, t78287: F, t18622: F, t64451: F, t21833: F, t4869: F, t5989: F, t64257: F, t11292: F, t1156: F, t22237: F, t78242: F, t78247: F, t78250: F, t78254: F, t78281: F, t78283: F, t78286: F) -> (F, F, F, F, F, F, F) {
        let (t78291, t78294, t78296, t78298, t78302) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1432::<F>(t1164, t43689, t43692, t78287, t18622, t64451, t21833, t4869, t5989, t64257, t11292, t1156);
        let (t78304, t78305) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1433::<F>(t22237, t4869, t78242, t78247, t78250, t78254, t78281, t78283, t78286, t78291, t78294, t78296, t78298, t78302);
    (t78291, t78294, t78296, t78298, t78302, t78304, t78305)
}
