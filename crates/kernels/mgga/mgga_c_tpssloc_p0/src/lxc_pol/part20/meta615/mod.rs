//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta615 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2216;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2217;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2218;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2219;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2220;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2221;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2222;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2223;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta615<F: Float>(t2250: F, t3966: F, t12606: F, t607: F, t12648: F, t12649: F, t12652: F, t12653: F, t12661: F, t12709: F, t1434: F, t2252: F, t31: F, t4018: F, t45872: F, t45993: F, t628: F, t642: F, t65: F, t80: F, t9263: F, t12623: F, t12656: F, t12662: F, t12665: F, t1411: F, t1426: F, t2251: F, t2304: F, t3962: F, t3968: F, t3971: F, t3997: F, t67: F, t9248: F, t9259: F, t9339: F, t12620: F, t12633: F, t12636: F, t12708: F, t1410: F, t2255: F, t2283: F, t3961: F, t3967: F, t3976: F, t608: F, t609: F, t7445: F, t9247: F, t9260: F, t9268: F, t9312: F, t16: F, t39031: F, t39033: F, t39035: F, t39037: F, t39039: F, t12566: F, t604: F, t2239: F, t3951: F, t12571: F, t12582: F, t12719: F, t1437: F, t2240: F, t2241: F, t39043: F, t39049: F, t39054: F, t3953: F, t3958: F, t4021: F, t45986: F, t605: F, t645: F, t86: F, t9239: F, t9243: F, t9342: F, t5: F, t12568: F, t12585: F, t12588: F, t2235: F, t2307: F, t39046: F, t39063: F, t45844: F, t9228: F, t9231: F, t9240: F, t112: F, t1268: F, t12725: F, t12734: F, t12739: F, t12813: F, t1458: F, t19456: F, t2314: F, t2363: F, t39235: F, t4028: F, t4072: F, t45590: F, t45602: F, t45632: F, t45637: F, t45782: F, t45814: F, t5113: F, t671: F, t9348: F, t9416: F, t40626: F, t4199: F, t9919: F, t12887: F, t758: F, t9892: F, t13123: F, t9882: F, t9888: F, t118: F, t2375: F, t4095: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t45997 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2216::<F>(t2250, t3966);
        let (t46006, t46022) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2217::<F>(t12606, t607, t12648, t12649, t12652, t12653, t12661, t12709, t1434, t2252, t31, t4018, t45872, t45993, t45997, t628, t642, t65, t80, t9263);
        let t46050 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2218::<F>(t12623, t12656, t12662, t12665, t1411, t1426, t2251, t2304, t3962, t3968, t3971, t3997, t607, t642, t67, t80, t9248, t9259, t9339);
        let t46080 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2219::<F>(t12620, t12633, t12636, t12708, t1410, t1434, t2250, t2255, t2283, t2304, t3961, t3967, t3976, t4018, t608, t609, t642, t7445, t80, t9247, t9260, t9268, t9312);
        let t46114 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2220::<F>(t16, t39031, t39033, t39035, t39037, t39039, t12566, t604, t2239, t3951, t12571, t12582, t12719, t1437, t2240, t2241, t39043, t39049, t39054, t3953, t3958, t4021, t45986, t46022, t46050, t46080, t605, t645, t86, t9239, t9243, t9342);
        let t46116 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2221::<F>(t5, t12568, t12585, t12588, t12719, t1437, t2235, t2240, t2307, t39046, t39063, t3958, t4021, t45844, t46114, t9228, t9231, t9239, t9240);
        let (t46117, t46118) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2222::<F>(t112, t46116, t1268, t12725, t12734, t12739, t12813, t1458, t19456, t2314, t2363, t39235, t4028, t4072, t45590, t45602, t45632, t45637, t45782, t45814, t5113, t671, t9348, t9416);
        let (t46120, t46126, t46129, t46131, t46133, t46135, t46137) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2223::<F>(t40626, t4199, t9919, t12887, t67, t758, t9892, t13123, t9882, t9888, t118, t2375, t4095);
    (t45997, t46006, t46117, t46118, t46120, t46126, t46129, t46131, t46133, t46135, t46137)
}
