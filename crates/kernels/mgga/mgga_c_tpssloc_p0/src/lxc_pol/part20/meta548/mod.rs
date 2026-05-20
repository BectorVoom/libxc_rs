//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta548 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2091;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta548<F: Float>(t118: F, t2576: F, t794: F, t9516: F, t207: F, t40394: F, t40399: F, t2582: F, t9541: F, t786: F, t9580: F, t2578: F, t9546: F, t9555: F, t2573: F, t41008: F, t2566: F, t2570: F, t9551: F, t2588: F, t40341: F, t12998: F, t2553: F, t686: F, t9524: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41181, t41185, t41187, t41189, t41190) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2091::<F>(t118, t2576, t794, t9516, t207, t40394, t40399, t2582, t9541, t786, t9580, t2578);
        let (t41192, t41194, t41197, t41200, t41203) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2092::<F>(t9546, t9555, t2573, t41008, t2566, t2570, t9551, t2588, t40341, t12998, t2553, t686, t9524);
    (t41181, t41185, t41187, t41189, t41190, t41192, t41194, t41197, t41200, t41203)
}
