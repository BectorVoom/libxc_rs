//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk736;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk737;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk738;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk739;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta142<F: Float>(t225: F, t3817: F, t3837: F, t1365: F, t68: F, t3734: F, t1347: F, t3719: F, t1345: F, t1348: F, t546: F, t548: F, t550: F, t1343: F, t820: F, t3791: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3839, t3843, t3844, t3847, t3850) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk736::<F>(t225, t3817, t3837, t1365, t68, t3734, t1347, t3719, t1345, t1348, t546, t548);
        let t3851 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk737::<F>(t3850, t550);
        let t3853 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk738::<F>(t1343, t3851, t820);
        let (t3856, t3858) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk739::<F>(t3791, t550, t1343, t820);
    (t3839, t3843, t3844, t3847, t3850, t3851, t3853, t3856, t3858)
}
