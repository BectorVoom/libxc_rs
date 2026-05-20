//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta762 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2564;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2565;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2566;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2567;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2568;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta762<F: Float>(t1117: F, t11190: F, t21724: F, t3313: F, t4781: F, t5989: F, t11424: F, t21895: F, t1147: F, t21826: F, t1128: F, t21975: F, t11185: F, t11297: F, t11365: F, t1138: F, t11415: F, t1155: F, t1157: F, t15146: F, t1695: F, t18637: F, t18644: F, t18785: F, t21836: F, t21947: F, t21952: F, t3376: F, t3401: F, t4857: F, t4858: F, t51427: F, t51730: F, t6037: F, t6069: F, t6084: F, t51246: F, t1098: F, t21988: F, t1119: F, t50834: F, t51257: F, t63291: F, t63306: F, t63308: F, t63841: F, t63843: F, t63845: F, t71333: F, t71335: F, t71337: F, t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63893: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71391: F, t63911: F, t71144: F, t71400: F, t71403: F, t71406: F, t71408: F, t71411: F, t71414: F, t71417: F, t71420: F, t71423: F, t71426: F, t50846: F, t51271: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F, t71166: F, t71170: F, t71174: F, t71179: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t71850, t71853, t71855, t71860, t71863) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2564::<F>(t1117, t11190, t21724, t3313, t4781, t5989, t11424, t21895, t1147, t21826, t1128, t21975);
        let (t71867, t71868) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2565::<F>(t11185, t21724, t11297, t11365, t1138, t11415, t1155, t1157, t15146, t1695, t18637, t18644, t18785, t21836, t21947, t21952, t3376, t3401, t4857, t4858, t51427, t51730, t6037, t6069, t6084, t71850, t71853, t71855, t71860, t71863);
        let (t71876, t71879, t71902) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2566::<F>(t51246, t5989, t1098, t21988, t1119, t50834, t51257, t63291, t63306, t63308, t63841, t63843, t63845, t71333, t71335, t71337);
        let t71915 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2567::<F>(t63332, t63334, t63336, t63886, t63888, t63893, t71124, t71130, t71135, t71140, t71142, t71391);
        let t71929 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2568::<F>(t63911, t71144, t71400, t71403, t71406, t71408, t71411, t71414, t71417, t71420, t71423, t71426);
        let t71941 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2569::<F>(t50846, t51271, t71146, t71150, t71152, t71154, t71156, t71160, t71166, t71170, t71174, t71179);
    (t71850, t71853, t71855, t71867, t71868, t71876, t71879, t71902, t71915, t71929, t71941)
}
