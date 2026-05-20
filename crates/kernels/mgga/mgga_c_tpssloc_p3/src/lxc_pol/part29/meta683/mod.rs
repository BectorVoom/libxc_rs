//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta683 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2314;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2315;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2316;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2317;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2318;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2319;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2320;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2321;
use chunk8::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2322;
use chunk9::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2323;
use chunk10::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2324;
use chunk11::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta683<F: Float>(t1222: F, t27589: F, t1184: F, t1409: F, t24682: F, t460: F, t461: F, t1009: F, t7324: F, t24722: F, t15548: F, t24733: F, t27598: F, t3535: F, t1218: F, t14731: F, t14736: F, t14740: F, t15663: F, t15750: F, t2121: F, t24736: F, t24741: F, t4899: F, t4989: F, t7331: F, t8040: F, t86204: F, t86324: F, t2132: F, t24746: F, t3545: F, t8020: F, t1202: F, t27603: F, t4993: F, t15486: F, t7345: F, t27599: F, t3572: F, t27604: F, t3523: F, t1232: F, t1748: F, t2136: F, t3587: F, t86129: F, t86228: F, t86248: F, t88391: F, t24683: F, t8027: F, t4928: F, t52: F, t86292: F, t15564: F, t23413: F, t86262: F, t86266: F, t86269: F, t86273: F, t86275: F, t86278: F, t86327: F, t15689: F, t7310: F, t27674: F, t3548: F, t15753: F, t27608: F, t7321: F, t27586: F, t15357: F, t15560: F, t2134: F, t24650: F, t27580: F, t27692: F, t27714: F, t7320: F, t86282: F, t86296: F, t3540: F, t8049: F, t3966: F, t24716: F, t4997: F, t15459: F, t15463: F, t15470: F, t15710: F, t24706: F, t3562: F, t5030: F, t8031: F, t86293: F, t86299: F, t15492: F, t7339: F, t15734: F, t25588: F, t1244: F, t1742: F, t3068: F, t1210: F, t24721: F, t27691: F, sigma2: F, t27700: F, t86261: F, t15239: F, t15474: F, t15541: F, t15761: F, t27617: F, t3580: F, t475: F, t68: F, t7326: F, t7328: F, t86313: F, t15418: F, t4724: F, t24720: F, t27710: F, t11588: F, t4729: F, t14749: F, t14753: F, t15455: F, t15764: F, t2140: F, t3448: F, t488: F, t86341: F, t86343: F, t86348: F, t86350: F, t15572: F, t15501: F, t24727: F, t3500: F, t7337: F, t15478: F, t15527: F, t15656: F, t15714: F, t24699: F, t24815: F, t27636: F, t27637: F, t3493: F, t3496: F, t3511: F, t3518: F, t8028: F, t86354: F, t3: F, t24684: F, t15608: F, t11716: F, t14744: F, t1714: F, t1734: F, t27642: F, t27644: F, t27704: F, t3507: F, t4950: F, t5011: F, t6729: F, t85827: F, t85966: F, t85972: F, t86194: F, t86330: F, t86357: F, t95396: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t95410, t95413, t95415, t95424, t95435) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2314::<F>(t1222, t27589, t1184, t1409, t24682, t460, t461, t1009, t7324, t24722, t15548, t24733);
        let t95443 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2315::<F>(t27598, t3535, t1218, t14731, t14736, t14740, t15663, t15750, t2121, t24736, t24741, t4899, t4989, t7331, t8040, t86204, t86324, t95410, t95415, t95424, t95435);
        let (t95446, t95450, t95452, t95456, t95459, t95463) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2316::<F>(t2132, t24746, t95413, t3545, t8020, t1202, t27603, t24736, t4993, t15486, t7345, t27599, t3572);
        let t95469 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2317::<F>(t27604, t3523, t1232, t1748, t2132, t2136, t3587, t86129, t86228, t86248, t88391, t95446, t95450, t95452, t95456, t95459, t95463);
        let (t95484, t95492) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2318::<F>(t24683, t24746, t8027, t4928, t52, t2132, t8040, t86292, t15564, t2136, t23413, t86262, t86266, t86269, t86273, t86275, t86278, t86327);
        let t95518 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2319::<F>(t15689, t7310, t27674, t3548, t15753, t27608, t7321, t1222, t27586, t15357, t15560, t2134, t24650, t27580, t27692, t27714, t460, t7320, t8040, t86282, t86296, t86324);
        let t95543 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2320::<F>(t3540, t8049, t2132, t2136, t3966, t24716, t4997, t15459, t15463, t15470, t15710, t24706, t24736, t24741, t27674, t3562, t5030, t8031, t86293, t86299);
        let (t95545, t95550, t95556, t95566, t95571) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2321::<F>(t15492, t7339, t15734, t7345, t25588, t461, t7324, t1244, t1742, t3068, t1210, t24721, t27691, sigma2);
        let t95576 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2322::<F>(t27700, t86261, t15239, t15474, t15541, t15761, t24741, t27617, t3580, t3587, t475, t68, t7326, t7328, t7331, t7345, t86313, t95545, t95550, t95556, t95566, t95571);
        let t95603 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2323::<F>(t15418, t2121, t4724, t24720, t27710, t24722, t11588, t4729, t14749, t14753, t15455, t15764, t2140, t3448, t488, t7345, t86341, t86343, t86348, t86350);
        let t95633 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2324::<F>(t15572, t24741, t15501, t24727, t3500, t7337, t15478, t15527, t15656, t15714, t24699, t24706, t24815, t27599, t27636, t27637, t3493, t3496, t3511, t3518, t7339, t7345, t8028, t8031, t86354);
        let t95672 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2325::<F>(t27710, t3, t24684, t15608, t24741, t11716, t1210, t14744, t1714, t1734, t2121, t2132, t24699, t27636, t27637, t27642, t27644, t27704, t3448, t3507, t475, t4950, t5011, t6729, t7321, t7331, t8040, t85827, t85966, t85972, t86194, t86330, t86357, t95396);
    (t95443, t95469, t95484, t95492, t95518, t95543, t95576, t95603, t95633, t95672)
}
