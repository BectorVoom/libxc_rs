//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta263 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1408;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1409;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1410;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1411;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1412;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta263<F: Float>(t10250: F, t4518: F, t2775: F, t343: F, t2244: F, t2988: F, t2987: F, t3014: F, t2990: F, t2262: F, t972: F, t10186: F, t10192: F, t10196: F, t10200: F, t10204: F, t10209: F, t10219: F, t10226: F, t10229: F, t10233: F, t10238: F, t10242: F, t10246: F, t2960: F, t2982: F, t2986: F, t2991: F, t973: F, t980: F, t2971: F, t2970: F, t2995: F, t2769: F, t40: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10251, t10254) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1408::<F>(t10250, t4518, t2775, t343);
        let t10255 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1409::<F>(t10254, t2244);
        let (t10256, t10259, t10260, t10263) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1410::<F>(t10255, t2988, t2987, t3014, t2990, t2262, t972);
        let t10266 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1411::<F>(t10186, t10192, t10196, t10200, t10204, t10209, t10219, t10226, t10229, t10233, t10238, t10242, t10246, t10251, t10256, t10260, t10263, t2960, t2982, t2986, t2991, t973, t980);
        let (t10267, t10273, t10274, t10276, t10277) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1412::<F>(t2960, t2971, t2970, t2995, t973, t2769, t40);
    (t10254, t10255, t10259, t10263, t10266, t10267, t10273, t10274, t10276, t10277)
}
