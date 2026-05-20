//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta845 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3056;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3057;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3058;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3059;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta845<F: Float>(t2244: F, t43763: F, t5392: F, t123: F, t43809: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t3265: F, t3313: F, t6021: F, t11190: F, t5989: F, t14850: F, t14937: F, t3375: F, t6063: F, t1136: F, t11365: F, t14829: F, t15153: F, t15165: F, t15179: F, t15219: F, t1695: F, t18615: F, t18622: F, t3376: F, t3377: F, t3378: F, t3395: F, t3401: F, t3403: F, t436: F, t44155: F, t51382: F, t51389: F, t51392: F, t51486: F, t51521: F, t51727: F, t6085: F, t6088: F, t63280: F, t63283: F, t63290: F, t63325: F, t63346: F, t63376: F, t18893: F, t3359: F, t11303: F, t11350: F, t11415: F, t11420: F, t15117: F, t15146: F, t15159: F, t15168: F, t15172: F, t1683: F, t18631: F, t18634: F, t18637: F, t18640: F, t18643: F, t18644: F, t18650: F, t18894: F, t3332: F, t3333: F, t3351: F, t3357: F, t44214: F, t44361: F, t4824: F, t51427: F, t51599: F, t51604: F, t6037: F, t6053: F, t6056: F, t11285: F, t6084: F, t18785: F, t3307: F, t11275: F, t6024: F, t11310: F, t11361: F, t1155: F, t1156: F, t15126: F, t15136: F, t15182: F, t15185: F, t15210: F, t15222: F, t15226: F, t18619: F, t18623: F, t43692: F, t44220: F, t44223: F, t4861: F, t51376: F, t51680: F, t6068: F, t6069: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t63420, t63422) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3056::<F>(t2244, t43763, t5392, t123, t43809);
        let t63424 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3057::<F>(t63380, t63382, t63384, t63388, t63392, t63396, t63398, t63400, t63404, t63408, t63412, t63417, t63422);
        let (t63446, t63449, t63451, t63457) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3058::<F>(t3265, t3313, t6021, t11190, t5989, t14850, t14937, t3375, t6063, t1136, t11365, t14829, t15153, t15165, t15179, t15219, t1695, t18615, t18622, t3376, t3377, t3378, t3395, t3401, t3403, t436, t44155, t51382, t51389, t51392, t51486, t51521, t51727, t6085, t6088, t63280, t63283, t63290, t63325, t63346, t63376, t63424);
        let t63506 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3059::<F>(t18893, t3359, t11303, t11350, t1136, t11415, t11420, t15117, t15146, t15159, t15165, t15168, t15172, t1683, t18631, t18634, t18637, t18640, t18643, t18644, t18650, t18894, t3332, t3333, t3351, t3357, t44214, t44361, t4824, t51427, t51599, t51604, t6037, t6053, t6056);
        let (t63557, t63560, t63561) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3060::<F>(t11285, t6084, t18785, t3403, t3307, t3313, t5989, t11275, t3265, t6024, t11310, t11361, t1155, t1156, t14829, t15126, t15136, t15182, t15185, t15210, t15222, t15226, t18615, t18619, t18622, t18623, t18643, t3351, t3357, t3376, t3377, t3395, t3401, t43692, t44220, t44223, t4861, t51376, t51680, t6068, t6069, t6088, t63283);
    (t63420, t63422, t63446, t63449, t63451, t63457, t63506, t63557, t63560, t63561)
}
