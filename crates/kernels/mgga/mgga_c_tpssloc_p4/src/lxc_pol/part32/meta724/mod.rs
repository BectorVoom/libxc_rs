//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta724 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2318;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2319;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2320;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2321;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2322;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2323;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2324;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2325;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2326;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2327;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta724<F: Float>(t18329: F, t7310: F, t1734: F, t18303: F, t18948: F, t18955: F, t19058: F, t19062: F, t19072: F, t19077: F, t24729: F, t24733: F, t27604: F, t27617: F, t478: F, t4974: F, t4980: F, t4984: F, t4989: F, t7345: F, t7376: F, t86146: F, t86171: F, t95270: F, t95273: F, t95303: F, t95304: F, t18324: F, t18371: F, t24741: F, t29569: F, t29651: F, t4954: F, t7321: F, t86184: F, t95320: F, t95334: F, t95335: F, t95352: F, t95362: F, t95364: F, t95365: F, t95687: F, t19031: F, t2139: F, t471: F, t24746: F, t27607: F, t8027: F, t1409: F, t1714: F, t2132: F, t52: F, t6138: F, t1222: F, t29597: F, t29601: F, t1232: F, t18383: F, t18965: F, t2136: F, t24736: F, t25588: F, t29625: F, t6207: F, t7316: F, t86191: F, t86327: F, t95370: F, t1193: F, t29585: F, t29562: F, t27674: F, t5040: F, t18368: F, t27629: F, t27692: F, t8040: F, t86324: F, t95384: F, t95404: F, t95410: F, t95424: F, t95435: F, t95566: F, t95678: F, t29643: F, t3503: F, t86264: F, t1210: F, t29647: F, t95332: F, t29561: F, t6739: F, t7325: F, t1215: F, t15394: F, t18206: F, t18211: F, t18232: F, t18573: F, t2121: F, t2140: F, t24821: F, t27636: F, t27642: F, t27697: F, t488: F, t4899: F, t5011: F, t6224: F, t7331: F, t7999: F, t85972: F, t95396: F, t95446: F, t95450: F, t1202: F, t2133: F, t24815: F, t27637: F, t27655: F, t27704: F, t29600: F, t29615: F, t29644: F, t29648: F, t4950: F, t6144: F, t8028: F, t86149: F, t95456: F, t95459: F, t95463: F, t95465: F, t99767: F, t27628: F, t95648: F, t24682: F, t460: F, t27635: F, t3: F, t95326: F, t11716: F, t24685: F, t27638: F, t27639: F, t27644: F, t27645: F, t29594: F, t6218: F, t85966: F, t86234: F, t95415: F, t95649: F, t27634: F, t3030: F, t95420: F, t18387: F, t18969: F, t29563: F, t3032: F, t475: F, t4965: F, t8048: F, t86275: F, t86278: F, t95480: F, t95487: F, t95491: F, t18356: F, t27614: F, t4997: F, t1730: F, t27603: F, t27598: F, t5001: F, t1218: F, t1737: F, t18523: F, t19101: F, t2134: F, t5014: F, t6211: F, t6227: F, t7320: F, t86140: F, t95238: F, t95507: F, t95511: F, t95512: F, t18221: F, t18225: F, t18237: F, t18940: F, t24650: F, t3448: F, t6729: F, t68: F, t7326: F, t7328: F, t7573: F, t95340: F, t95346: F, t95387: F, t95515: F, t95517: F, t95520: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t104087 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2318::<F>(t18329, t7310, t1734, t18303, t18948, t18955, t19058, t19062, t19072, t19077, t24729, t24733, t27604, t27617, t478, t4974, t4980, t4984, t4989, t7345, t7376, t86146, t86171, t95270, t95273, t95303, t95304);
        let t104101 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2319::<F>(t18324, t7310, t18371, t24741, t29569, t29651, t4954, t7321, t86184, t95320, t95334, t95335, t95352, t95362, t95364, t95365, t95687);
        let (t104107, t104111, t104118, t104120, t104122, t104124, t104126) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2320::<F>(t19031, t2139, t471, t24746, t27607, t8027, t1409, t1714, t2132, t52, t6138, t1222, t29597);
        let t104134 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2321::<F>(t1222, t29601, t104107, t104111, t104120, t104124, t104126, t1232, t18383, t18965, t2136, t24736, t24741, t25588, t29625, t6207, t7316, t8027, t86191, t86327, t95370);
        let t104155 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2322::<F>(t1193, t29585, t2136, t29562, t52, t27674, t5040, t1409, t8027, t18368, t27629, t27692, t4954, t8040, t86324, t95384, t95404, t95410, t95424, t95435, t95566, t95678);
        let t104193 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2323::<F>(t29643, t3503, t86264, t1210, t29647, t8040, t95332, t29561, t6739, t7325, t1215, t15394, t18206, t18211, t18232, t18573, t2121, t2140, t24821, t27636, t27642, t27697, t488, t4899, t5011, t6224, t7331, t7999, t85972, t95396, t95446, t95450);
        let t104220 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2324::<F>(t1202, t2132, t2133, t2136, t24815, t27636, t27637, t27655, t27704, t29600, t29615, t29644, t29648, t488, t4950, t5011, t6144, t7316, t7321, t8028, t86149, t95456, t95459, t95463, t95465, t95687, t99767);
        let t104264 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2325::<F>(t27628, t95648, t104118, t24682, t460, t104122, t27635, t3, t95326, t11716, t1210, t1215, t24685, t27636, t27638, t27639, t27644, t27645, t29594, t29644, t29648, t3503, t6218, t6224, t7331, t8040, t85966, t86234, t95396, t95415, t95649);
        let (t104280, t104292) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2326::<F>(t27634, t3030, t95420, t52, t6144, t24682, t460, t1210, t1215, t18387, t18969, t24741, t27639, t27645, t29563, t3032, t475, t488, t4965, t6224, t7321, t7331, t8048, t86275, t86278, t95396, t95480, t95487, t95491);
        let t104319 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2327::<F>(t18356, t24729, t27614, t4997, t1730, t27603, t27598, t5001, t1218, t1232, t1737, t18523, t19101, t2134, t24736, t460, t5014, t6211, t6227, t7320, t7345, t86140, t95238, t95507, t95511, t95512);
        let t104351 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2328::<F>(t104280, t2132, t24746, t1714, t18221, t18225, t18237, t18940, t2121, t2136, t24650, t29562, t29594, t3448, t475, t6729, t68, t7321, t7326, t7328, t7573, t95340, t95346, t95387, t95515, t95517, t95520);
    (t104087, t104101, t104134, t104155, t104193, t104220, t104264, t104292, t104319, t104351)
}
