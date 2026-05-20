//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta395 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1686;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1687;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta395<F: Float>(t11282: F, t6068: F, t11285: F, t1155: F, t1164: F, t11292: F, t4883: F, t15218: F, t4882: F, t1190: F, t6238: F, t1743: F, t4965: F, t486: F, t6224: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18274, t18275, t18276, t18278, t18279, t18280, t18282, t18283, t18285, t18287, t18297) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1686::<F>(t11282, t6068, t11285, t1155, t1164, t11292, t4883, t15218, t4882, t1190, t6238, t1743, t4965);
        let t18300 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1687::<F>(t486, t6224);
    (t18274, t18275, t18276, t18278, t18279, t18280, t18282, t18283, t18285, t18287, t18297, t18300)
}
