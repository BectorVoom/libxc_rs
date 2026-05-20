//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1022;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1023;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta162<F: Float>(t17: F, t3826: F, t1285: F, t592: F, t1287: F, t588: F, t2423: F, t3686: F, t3697: F, t3819: F, t3821: F, t3823: F, t3825: F, t225: F, t3817: F, t1365: F, t68: F, t3734: F, t1347: F, t3719: F, t1345: F, t1348: F, t546: F, t548: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3827, t3828, t3829, t3830, t3832, t3833, t3834, t3836, t3837) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1022::<F>(t17, t3826, t1285, t592, t1287, t588, t2423, t3686, t3697, t3819, t3821, t3823, t3825);
        let t3839 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1023::<F>(t225, t3817, t3837);
        let (t3844, t3847, t3850) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1024::<F>(t1365, t68, t3734, t1347, t3719, t1345, t1348, t3839, t546, t548);
    (t3827, t3828, t3829, t3830, t3832, t3833, t3834, t3836, t3839, t3844, t3847, t3850)
}
