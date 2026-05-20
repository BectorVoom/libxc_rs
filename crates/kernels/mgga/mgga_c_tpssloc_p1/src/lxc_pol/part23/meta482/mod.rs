//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta482 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1451;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1452;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1453;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1454;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1455;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1456;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1457;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1458;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1459;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1460;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1461;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1462;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta482<F: Float>(t6218: F, t11668: F, t11779: F, t1214: F, t1227: F, t15615: F, t1735: F, t1748: F, t19033: F, t21745: F, t21749: F, t22197: F, t22208: F, t248: F, t3506: F, t3508: F, t3577: F, t3578: F, t4582: F, t47: F, t471: F, t479: F, t488: F, t5005: F, t6207: F, t65600: F, t65605: F, t72255: F, t72352: F, t72366: F, t77606: F, t77957: F, t8025: F, t78118: F, t78120: F, t78122: F, t78125: F, t78128: F, t78132: F, t78196: F, t78199: F, t78227: F, t78229: F, t78232: F, t78236: F, t78239: F, t78242: F, t78247: F, t78250: F, t78254: F, t78281: F, t78283: F, t78286: F, t78291: F, t78294: F, t78296: F, t78298: F, t78302: F, t78304: F, t78310: F, t78312: F, t78314: F, t78318: F, t78320: F, t78327: F, t78329: F, t78331: F, t78333: F, t50834: F, t71335: F, t71337: F, t77959: F, t77963: F, t77967: F, t77971: F, t77975: F, t77979: F, t77983: F, t77989: F, t77992: F, t77995: F, t77998: F, t63332: F, t63334: F, t63888: F, t63893: F, t63911: F, t71142: F, t71144: F, t71146: F, t71152: F, t71154: F, t71156: F, t71408: F, t78002: F, t78005: F, t44249: F, t50846: F, t71470: F, t71472: F, t71474: F, t78026: F, t78029: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F, t78078: F, t78080: F, t44275: F, t63361: F, t78057: F, t78084: F, t78087: F, t78090: F, t78093: F, t78095: F, t78097: F, t78100: F, t78103: F, t78105: F, t78107: F, t78109: F, t6036: F, t1129: F, t11365: F, t1137: F, t1156: F, t15126: F, t21947: F, t3376: F, t3401: F, t3403: F, t44177: F, t44179: F, t78243: F, t78287: F, t11285: F, t11350: F, t11352: F, t1148: F, t1683: F, t1695: F, t18840: F, t18899: F, t21855: F, t21887: F, t21890: F, t21939: F, t21942: F, t3359: F, t43692: F, t44155: F, t44223: F, t44361: F, t4797: F, t4835: F, t51376: F, t51427: F, t51604: F, t6037: F, t6053: F, t6056: F, t6085: F, t6088: F, t63602: F, t64103: F, t64292: F, t71860: F, t71863: F, t78114: F, t11310: F, t11420: F, t15146: F, t15207: F, t1682: F, t1694: F, t18622: F, t21839: F, t21842: F, t21845: F, t3332: F, t6052: F, t6069: F, t6084: F, t71672: F, t78225: F, t78335: F, t78355: F, t44320: F, t15136: F, t18650: F, t21836: F, t21907: F, t21952: F, t3357: F, t436: F, t51680: F, t63454: F, t71729: F, t78359: F, t78361: F, t78364: F, t78367: F, t78370: F, t78373: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t78757, t78775) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1451::<F>(t6218, t11668, t11779, t1214, t1227, t15615, t1735, t1748, t19033, t21745, t21749, t22197, t22208, t248, t3506, t3508, t3577, t3578, t4582, t47, t471, t479, t488, t5005, t6207, t65600, t65605, t72255, t72352, t72366, t77606, t77957, t8025);
        let (t78791, t78792) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1452::<F>(t78118, t78120, t78122, t78125, t78128, t78132, t78196, t78199, t78227, t78229, t78232, t78236, t78239, t78242, t78247, t78250, t78254, t78281, t78283, t78286, t78291, t78294, t78296);
        let t78794 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1453::<F>(t78298, t78302, t78304, t78310, t78312, t78314, t78318, t78320, t78327, t78329, t78331, t78333);
        let t78809 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1454::<F>(t50834, t71335, t71337, t77959, t77963, t77967, t77971, t77975, t77979, t77983, t77989, t77992, t77995, t77998);
        let t78824 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1455::<F>(t63332, t63334, t63888, t63893, t63911, t71142, t71144, t71146, t71152, t71154, t71156, t71408, t78002, t78005);
        let t78839 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1456::<F>(t44249, t50846, t71470, t71472, t71474, t78026, t78029, t78033, t78037, t78041, t78045, t78049, t78078, t78080);
        let t78853 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1457::<F>(t44275, t63361, t78057, t78084, t78087, t78090, t78093, t78095, t78097, t78100, t78103, t78105, t78107, t78109);
        let (t78859, t78874) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1458::<F>(t6036, t1129, t11365, t1137, t1156, t15126, t21947, t3376, t3401, t3403, t44177, t44179, t78132, t78196, t78199, t78229, t78232, t78236, t78239, t78243, t78281, t78283, t78286, t78287, t78298, t78809, t78824, t78839, t78853);
        let t78914 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1459::<F>(t11285, t11350, t11352, t1148, t1156, t1683, t1695, t18840, t18899, t21855, t21887, t21890, t21939, t21942, t3359, t43692, t44155, t44223, t44361, t4797, t4835, t51376, t51427, t51604, t6037, t6053, t6056, t6085, t6088, t63602, t64103, t64292, t71860, t71863, t78114, t78287, t78859);
        let t78944 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1460::<F>(t11310, t11365, t11420, t15126, t15146, t15207, t1682, t1694, t18622, t21839, t21842, t21845, t21887, t21939, t3332, t3376, t3401, t6052, t6056, t6069, t6084, t6088, t71672, t78225, t78327, t78329, t78331, t78333, t78335, t78355);
        let (t78961, t78973) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1461::<F>(t63332, t63334, t63361, t71142, t71144, t71146, t71152, t77989, t77992, t77995, t78057, t44320, t50834, t71154, t71156, t77998, t78002, t78005, t78033, t78037, t78041, t78045, t78049);
        let t79002 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1462::<F>(t6052, t11310, t11350, t1137, t11420, t15136, t15146, t1682, t18650, t21836, t21907, t21952, t3332, t3357, t3359, t3403, t436, t51680, t6037, t6069, t63454, t71729, t78287, t78359, t78361, t78364, t78367, t78370, t78373, t78859, t78961, t78973);
    (t78757, t78775, t78791, t78792, t78794, t78874, t78914, t78944, t79002)
}
