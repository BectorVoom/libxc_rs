//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta703 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2285;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2286;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2287;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2288;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2289;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2290;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2291;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2292;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2293;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2294;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2295;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta703<F: Float>(t1036: F, t28572: F, t1015: F, t1022: F, t17841: F, t1935: F, t23564: F, t23604: F, t25645: F, t25652: F, t25658: F, t25679: F, t28558: F, t28582: F, t28587: F, t3032: F, t343: F, t360: F, t4649: F, t5872: F, t6730: F, t6734: F, t7583: F, t82911: F, t88341: F, t88362: F, t88367: F, t88385: F, t88537: F, t17624: F, t6717: F, t1933: F, t1937: F, t5398: F, t10475: F, t17738: F, t23422: F, t23678: F, t25609: F, t25653: F, t25654: F, t28578: F, t3128: F, t5866: F, t5885: F, t7574: F, t82516: F, t82542: F, t88286: F, t88415: F, t40: F, t5842: F, t23479: F, t17701: F, t17877: F, t18021: F, t1941: F, t23419: F, t28525: F, t378: F, t4579: F, t6722: F, t83117: F, t83215: F, t88422: F, t88425: F, t88428: F, t88440: F, t88453: F, t88513: F, t1409: F, t1597: F, t23562: F, t5836: F, t18041: F, t17649: F, t17998: F, t6747: F, t83025: F, t83028: F, t88348: F, t88479: F, t88488: F, t17611: F, t6755: F, t1934: F, t17659: F, t6765: F, t17178: F, t17183: F, t18036: F, t1920: F, t23437: F, t23529: F, t2987: F, t4509: F, t5857: F, t5869: F, t5909: F, t6735: F, t7573: F, t83016: F, t83220: F, t88503: F, t88517: F, t344: F, t6740: F, t5904: F, t6764: F, t1046: F, t17681: F, t17890: F, t23483: F, t23544: F, t28526: F, t5861: F, t83121: F, t88548: F, t13797: F, t17152: F, t17161: F, t17920: F, t18016: F, t18025: F, t25585: F, t25601: F, t6758: F, t7578: F, t83080: F, t88566: F, t88569: F, t16558: F, t3: F, t17677: F, t17705: F, t88575: F, t88577: F, t88582: F, t88604: F, t88622: F, t88625: F, t88636: F, t88645: F, t23472: F, t28586: F, t17615: F, t17620: F, t28566: F, t5890: F, t5894: F, t6723: F, t83008: F, t88648: F, t88689: F, t88692: F, t28581: F, t82895: F, t28577: F, t25641: F, t88451: F, t1615: F, t17157: F, t17167: F, t17171: F, t25683: F, t363: F, t6800: F, t88351: F, t88354: F, t88372: F, t88430: F, t88431: F, t88704: F, t88383: F, t25650: F, t3030: F, t88449: F, t17714: F, t17959: F, t23489: F, t23537: F, t25655: F, t25660: F, t25661: F, t6742: F, t6744: F, t68: F, t83157: F, t88290: F, t88407: F, t88723: F, t99492: F, t99514: F, t99535: F, t99556: F, t99571: F) -> F {
        let t99600 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2285::<F>(t1036, t28572, t1015, t1022, t17841, t1935, t23564, t23604, t25645, t25652, t25658, t25679, t28558, t28582, t28587, t3032, t343, t360, t4649, t5872, t6730, t6734, t7583, t82911, t88341, t88362, t88367, t88385, t88537);
        let t99635 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2286::<F>(t17624, t6717, t1933, t1937, t5398, t1022, t10475, t17738, t23422, t23678, t25609, t25652, t25653, t25654, t28578, t3128, t4649, t5866, t5872, t5885, t7574, t7583, t82516, t82542, t82911, t88286, t88415, t88537);
        let (t99645, t99654) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2287::<F>(t40, t5842, t1933, t23479, t17701, t17877, t18021, t1937, t1941, t23419, t28525, t28582, t378, t4579, t6722, t83117, t83215, t88422, t88425, t88428, t88440, t88453, t88513);
        let (t99660, t99665, t99682) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2288::<F>(t1409, t1597, t23562, t343, t40, t5836, t99645, t18041, t23419, t17649, t17998, t6747, t7583, t83025, t83028, t88348, t88479, t88488);
        let t99709 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2289::<F>(t17611, t6755, t1933, t1934, t5836, t17659, t6765, t1597, t17178, t17183, t18036, t1920, t23437, t23529, t2987, t4509, t5857, t5869, t5909, t6735, t7573, t83016, t83220, t88503, t88517);
        let t99736 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2290::<F>(t28525, t344, t6740, t5904, t6764, t1046, t17681, t17890, t23419, t23483, t23544, t28526, t28578, t28582, t28587, t5857, t5861, t6735, t6747, t6765, t83117, t83121, t88548);
        let t99760 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2291::<F>(t13797, t17152, t17161, t17920, t18016, t18025, t1920, t1933, t1934, t23419, t25585, t25601, t25609, t378, t4509, t5842, t5904, t6735, t6758, t7578, t83016, t83080, t88566, t88569);
        let t99772 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2292::<F>(t16558, t3, t17677, t17705, t1933, t1937, t23419, t88575, t88577, t88582, t88604, t88622, t88625, t88636, t88645);
        let t99793 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2293::<F>(t1933, t23479, t99665, t1015, t23472, t28586, t17615, t6717, t17620, t23422, t28558, t28566, t5890, t5894, t5909, t6723, t83008, t88648, t88689, t88692);
        let t99826 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2294::<F>(t1933, t23479, t99660, t1015, t28581, t82895, t28577, t3128, t25641, t88451, t1615, t17157, t17167, t17171, t1920, t25679, t25683, t2987, t363, t4509, t6800, t88351, t88354, t88372, t88430, t88431, t88704);
        let t99855 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2295::<F>(t7583, t88383, t25650, t3030, t88449, t1015, t17714, t17959, t23489, t23537, t25652, t25655, t25660, t25661, t28566, t28587, t360, t5866, t6730, t6742, t6744, t68, t83157, t88290, t88407, t88723);
        let t99859 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2296::<F>(t99492, t99514, t99535, t99556, t99571, t99600, t99635, t99654, t99682, t99709, t99736, t99760, t99772, t99793, t99826, t99855);
    t99859
}
