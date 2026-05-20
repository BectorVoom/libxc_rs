//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1497;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta489<F: Float>(t56390: F, t56392: F, t56394: F, t56398: F, t54432: F, t54434: F, t193: F, t20563: F, t39570: F, t39582: F, t39585: F, t39590: F, t39593: F, t39595: F, t39597: F, t5122: F, t5126: F, t6347: F, t75256: F) -> (F, F, F, F, F, F, F) {
        let (t79927, t79928, t79929, t79930, t79934, t79935, t79939) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1497::<F>(t56390, t56392, t56394, t56398, t54432, t54434, t193, t20563, t39570, t39582, t39585, t39590, t39593, t39595, t39597, t5122, t5126, t6347, t75256);
    (t79927, t79928, t79929, t79930, t79934, t79935, t79939)
}
