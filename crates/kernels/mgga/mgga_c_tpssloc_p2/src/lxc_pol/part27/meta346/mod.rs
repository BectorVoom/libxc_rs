//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1433;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1434;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1435;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1436;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1437;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1438;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta346<F: Float>(t12680: F, t607: F, t2250: F, t3981: F, t12606: F, t43: F, t1409: F, t2244: F, t9300: F, t2274: F, t3966: F, t3990: F, t55: F, t12677: F, t1414: F, t1420: F, t2262: F, t2275: F, t2278: F, t39: F, t3982: F, t3985: F, t51: F, t615: F, t9311: F, t33: F, t12649: F, t12653: F, t12656: F, t12662: F, t12665: F, t1427: F, t1434: F, t2255: F, t2304: F, t3962: F, t3968: F, t3998: F, t4018: F, t609: F, t642: F, t80: F, t12645: F, t12566: F, t12568: F, t12571: F, t12582: F, t12585: F, t12588: F, t1437: F, t2235: F, t2240: F, t2241: F, t2307: F, t3953: F, t3958: F, t4021: F, t605: F, t645: F, t86: F, t9228: F, t9231: F, t9239: F, t5: F, t112: F, t111: F, t4025: F, t1441: F, t2319: F, t649: F, t671: F, t2363: F, t88: F, t1454: F, t2281: F, t4044: F, t626: F, t4068: F, t1453: F, t2332: F, t9365: F, t2331: F, t4067: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12681, t12684, t12687, t12695, t12699, t12702) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1433::<F>(t12680, t607, t2250, t3981, t12606, t43, t1409, t2244, t9300, t2274, t3966, t3990);
        let t12708 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1434::<F>(t12606, t55, t12677, t12681, t12684, t12687, t12695, t12699, t12702, t1414, t1420, t2262, t2275, t2278, t39, t3982, t3985, t51, t615, t9311);
        let t12718 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1435::<F>(t12708, t33, t12649, t12653, t12656, t12662, t12665, t1427, t1434, t2255, t2304, t3962, t3968, t3998, t4018, t609, t642, t80);
        let (t12719, t12722) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1436::<F>(t12645, t12718, t12566, t12568, t12571, t12582, t12585, t12588, t1437, t2235, t2240, t2241, t2307, t3953, t3958, t4021, t605, t645, t86, t9228, t9231, t9239);
        let (t12723, t12724, t12725) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1437::<F>(t5, t12722, t112, t111, t4025);
        let (t12728, t12734) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1438::<F>(t1441, t2319, t649, t671);
        let (t12739, t12747, t12750, t12752, t12754, t12757) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1439::<F>(t2363, t88, t1454, t2281, t4044, t626, t4068, t1453, t2332, t9365, t2331, t4067);
    (t12719, t12723, t12724, t12725, t12728, t12734, t12739, t12747, t12750, t12752, t12754, t12757)
}
