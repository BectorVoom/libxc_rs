//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2107;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2108;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2109;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2110;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2111;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2112;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2113;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2114;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2115;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2116;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2117;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2118;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta676<F: Float>(t12571: F, t1410: F, t26012: F, t7441: F, t27971: F, t645: F, t72: F, t1437: F, t7445: F, t1863: F, t27975: F, t1864: F, t5445: F, t2240: F, t5399: F, t22544: F, t22549: F, t22551: F, t26009: F, t26013: F, t26016: F, t90114: F, t90192: F, t90248: F, t90251: F, t90330: F, t3953: F, t3961: F, t3967: F, t4017: F, t1433: F, t4021: F, t1865: F, t22523: F, t22554: F, t26063: F, t26067: F, t26084: F, t27966: F, t27972: F, t6490: F, t6506: F, t6510: F, t7432: F, t90308: F, t90312: F, t641: F, t19445: F, t79: F, t27948: F, t33: F, t55921: F, t6489: F, t19299: F, t608: F, t26083: F, t26051: F, t26055: F, t26090: F, t27976: F, t6492: F, t7442: F, t7446: F, t1862: F, t5392: F, t1409: F, t605: F, t3966: F, t2235: F, t17635: F, t19334: F, t26045: F, t26048: F, t26070: F, t26073: F, t26076: F, t27982: F, t7435: F, t26021: F, t26025: F, t26028: F, t27979: F, t7428: F, t90182: F, t90185: F, t16558: F, t17686: F, t17691: F, t1860: F, t22502: F, t22505: F, t26024: F, t26044: F, t27949: F, t27950: F, t27953: F, t27957: F, t5398: F, t6486: F, t6500: F, t6509: F, t67: F, t83791: F, t83796: F, t83803: F, t5: F, t96409: F, t96441: F, t112: F, t5456: F, t6514: F, t19534: F, t88: F, t1873: F, t28007: F, t6534: F, t26114: F, t7467: F, t26117: F, t26135: F, t7676: F, t2314: F, t28017: F, t5113: F, t96356: F, t28002: F, t12725: F, t75560: F, t19451: F) -> (F, F, F) {
        let (t96443, t96454, t96458, t96462, t96466, t96469) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2107::<F>(t12571, t1410, t26012, t7441, t27971, t645, t72, t1437, t7445, t1863, t27975, t1864, t5445);
        let t96478 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2108::<F>(t1863, t96469, t2240, t5399, t22544, t22549, t22551, t26009, t26013, t26016, t90114, t90192, t90248, t90251, t90330, t96443, t96454, t96458, t96462, t96466);
        let t96509 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2109::<F>(t3953, t3961, t3967, t1437, t4017, t72, t1433, t4021, t1865, t22523, t22554, t26063, t26067, t26084, t27966, t27972, t6490, t6506, t6510, t7432, t90308, t90312);
        let (t96517, t96521, t96529, t96532, t96535) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2110::<F>(t5445, t641, t72, t19445, t79, t2240, t27948, t33, t55921, t6489, t19299, t608);
        let t96545 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2111::<F>(t12571, t26083, t1865, t22523, t22554, t26051, t26055, t26067, t26090, t27976, t6490, t6492, t7442, t7446, t96517, t96521, t96529, t96532, t96535);
        let (t96547, t96551, t96553, t96556, t96559, t96562) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2112::<F>(t1862, t2240, t5392, t1409, t605, t3966, t72, t79, t2235, t5399, t17635, t19334);
        let t96579 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2113::<F>(t1865, t26045, t26048, t26070, t26073, t26076, t27982, t6492, t6506, t6510, t7435, t7442, t96547, t96551, t96553, t96556, t96559, t96562);
        let t96605 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2114::<F>(t26021, t26025, t26028, t26045, t26051, t26063, t26070, t26073, t26076, t27979, t6506, t6510, t7428, t7432, t7435, t7442, t7446, t90182, t90185);
        let t96649 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2115::<F>(t2235, t5392, t16558, t17635, t17686, t17691, t1860, t1864, t1865, t22502, t22505, t26021, t26024, t26025, t26028, t26044, t26048, t27949, t27950, t27953, t27957, t5398, t6486, t6500, t6509, t67, t7428, t7441, t7445, t7446, t83791, t83796, t83803);
        let (t96654, t96655, t96657) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2116::<F>(t5, t96409, t96441, t96478, t96509, t96545, t96579, t96605, t96649, t112, t5456, t6514, t19534, t88);
        let (t96659, t96661, t96663, t96665, t96667, t96669, t96671) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2117::<F>(t1873, t96657, t28007, t6534, t26114, t7467, t26117, t26135, t7676, t2314, t28017, t5113);
        let t96682 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2118::<F>(t1873, t96356, t28002, t6534, t12725, t7467, t75560, t19451, t96654, t96655, t96659, t96661, t96663, t96665, t96667, t96669, t96671);
    (t96654, t96655, t96682)
}
