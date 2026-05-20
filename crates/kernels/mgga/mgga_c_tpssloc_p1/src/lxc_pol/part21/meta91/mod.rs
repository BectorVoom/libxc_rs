//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta91 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk651;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk652;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk653;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk654;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk655;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta91<F: Float>(t40: F, t632: F, t73: F, t52: F, t636: F, t76: F, t2244: F, t2250: F, t634: F, t638: F, t72: F, t2245: F, t2252: F, t2255: F, t2284: F, t609: F, t629: F, t642: F, t66: F, t80: F, t5: F, t2233: F, t2235: F, t2240: F, t2241: F, t605: F, t645: F, t86: F, t112: F, t111: F, t649: F, t671: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2289, t2291) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk651::<F>(t40, t632, t73);
        let (t2296, t2298) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk652::<F>(t52, t636, t76);
        let t2304 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk653::<F>(t2244, t2250, t2291, t2298, t634, t638, t72);
        let t2307 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk654::<F>(t2245, t2252, t2255, t2284, t2304, t609, t629, t642, t66, t80);
        let (t2311, t2312, t2314) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk655::<F>(t5, t2233, t2235, t2240, t2241, t2307, t605, t645, t86, t112, t111, t649);
        let t2319 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk656::<F>(t671);
    (t2289, t2291, t2296, t2298, t2304, t2307, t2311, t2312, t2314, t2319)
}
