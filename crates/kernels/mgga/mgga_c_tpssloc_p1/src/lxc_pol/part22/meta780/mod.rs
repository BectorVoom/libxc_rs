//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta780 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta780<F: Float>(t56390: F, t56394: F, t56398: F, t56400: F, t54432: F, t54434: F, t39570: F, t39585: F, t39590: F, t39593: F, t39595: F, t54429: F, t54430: F, t54431: F, t54436: F, t54437: F, t54438: F) -> (F, F, F, F, F, F, F) {
        let (t74481, t74482, t74483, t74484, t74485, t74486, t74487) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2670::<F>(t56390, t56394, t56398, t56400, t54432, t54434, t39570, t39585, t39590, t39593, t39595, t54429, t54430, t54431, t54436, t54437, t54438);
    (t74481, t74482, t74483, t74484, t74485, t74486, t74487)
}
