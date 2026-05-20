//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1721;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1722;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1723;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta338<F: Float>(t12645: F, t12718: F, t12566: F, t12568: F, t12571: F, t12582: F, t12585: F, t12588: F, t1437: F, t2235: F, t2240: F, t2241: F, t2307: F, t3953: F, t3958: F, t4021: F, t605: F, t645: F, t86: F, t9228: F, t9231: F, t9239: F, t5: F, t112: F, t111: F, t4025: F, t1441: F, t2319: F, t649: F, t671: F, t2363: F, t88: F, t1454: F, t2281: F, t4044: F, t626: F, t4068: F, t1453: F, t2332: F, t9365: F, t2331: F, t4067: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12719, t12722) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1721::<F>(t12645, t12718, t12566, t12568, t12571, t12582, t12585, t12588, t1437, t2235, t2240, t2241, t2307, t3953, t3958, t4021, t605, t645, t86, t9228, t9231, t9239);
        let (t12723, t12724, t12725) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1722::<F>(t5, t12722, t112, t111, t4025);
        let (t12728, t12734) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1723::<F>(t1441, t2319, t649, t671);
        let (t12739, t12747, t12750, t12752, t12754, t12757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1724::<F>(t2363, t88, t1454, t2281, t4044, t626, t4068, t1453, t2332, t9365, t2331, t4067);
    (t12719, t12723, t12724, t12725, t12728, t12734, t12739, t12747, t12750, t12752, t12754, t12757)
}
