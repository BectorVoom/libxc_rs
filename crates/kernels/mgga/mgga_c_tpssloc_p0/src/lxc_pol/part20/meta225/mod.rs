//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1304;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta225<F: Float>(t112: F, t9346: F, t111: F, t2311: F, t2319: F, t649: F, t107: F, t2585: F, t2281: F, t667: F, t2333: F, t626: F, t2359: F, t655: F, t2332: F, t666: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9347, t9348) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1304::<F>(t112, t9346, t111, t2311);
        let (t9351, t9358, t9359, t9361, t9363, t9364, t9365, t9366) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1305::<F>(t2319, t649, t107, t2585, t2281, t667, t2333, t626, t2359, t655, t2332, t666);
    (t9347, t9348, t9351, t9358, t9359, t9361, t9363, t9364, t9365, t9366)
}
