//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk630;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk631;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk632;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk633;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk634;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk635;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta90<F: Float>(t2244: F, t2274: F, t2250: F, t55: F, t240: F, t59: F, t2262: F, t2268: F, t2271: F, t39: F, t44: F, t51: F, t615: F, t618: F, t33: F, t40: F, t632: F, t73: F, t52: F, t636: F, t76: F, t634: F, t638: F, t72: F, t2245: F, t2252: F, t2255: F, t609: F, t629: F, t642: F, t66: F, t80: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2275, t2278, t2281) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk630::<F>(t2244, t2274, t2250, t55, t240, t59);
        let (t2282, t2283, t2284) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk631::<F>(t2281, t2262, t2268, t2271, t2275, t2278, t39, t44, t51, t615, t618, t33);
        let (t2289, t2291) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk632::<F>(t40, t632, t73);
        let (t2296, t2298) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk633::<F>(t52, t636, t76);
        let t2304 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk634::<F>(t2244, t2250, t2291, t2298, t634, t638, t72);
        let t2307 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk635::<F>(t2245, t2252, t2255, t2284, t2304, t609, t629, t642, t66, t80);
    (t2275, t2278, t2281, t2282, t2283, t2284, t2289, t2291, t2296, t2298, t2304, t2307)
}
