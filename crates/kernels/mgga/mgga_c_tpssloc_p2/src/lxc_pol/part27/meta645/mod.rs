//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta645 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2207;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2208;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2209;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2210;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2211;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2212;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2213;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2214;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2215;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2216;
use chunk10::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2217;
use chunk11::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2218;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta645<F: Float>(t23422: F, t4603: F, t14159: F, t6717: F, t14137: F, t6765: F, t7583: F, t83138: F, t23509: F, t25682: F, t25644: F, t82926: F, t23512: F, t25486: F, t23519: F, t25492: F, t1607: F, t23515: F, t23521: F, t23529: F, t4636: F, t6747: F, t82911: F, t82951: F, t82953: F, t83092: F, t1597: F, t607: F, t23562: F, t343: F, t40: F, t4540: F, t25650: F, t6740: F, t14206: F, t6754: F, t12606: F, t3: F, t1025: F, t1933: F, t1937: F, t23453: F, t23504: F, t25588: F, t25645: F, t6722: F, t7573: F, t82927: F, t82961: F, t83111: F, t25651: F, t83120: F, t1409: F, t984: F, t1036: F, t25622: F, t14134: F, t23479: F, t25637: F, t1014: F, t82654: F, t1022: F, t14037: F, t1611: F, t23419: F, t23556: F, t25655: F, t25661: F, t363: F, t378: F, t6800: F, t82971: F, t82996: F, t83085: F, t344: F, t1009: F, t23473: F, t13528: F, t13542: F, t13931: F, t14130: F, t1618: F, t1920: F, t1934: F, t1935: F, t23414: F, t23495: F, t25601: F, t25609: F, t2987: F, t4509: F, t6730: F, t6734: F, t6735: F, t7578: F, t82880: F, t83004: F, t83025: F, t83028: F, t3082: F, t7586: F, t25641: F, t82892: F, t25638: F, t13532: F, t13537: F, t13797: F, t13941: F, t14122: F, t14126: F, t1941: F, t23548: F, t23564: F, t25679: F, t7574: F, t82918: F, t82923: F, t83016: F, t83034: F, t83215: F, t23418: F, t4669: F, t13765: F, t14033: F, t14069: F, t14488: F, t23457: F, t25585: F, t25589: F, t3073: F, t360: F, t4575: F, t6723: F, t6742: F, t6744: F, t68: F, t83041: F, t83046: F, t83220: F, t10469: F, t23470: F, t82986: F, t23437: F, t4630: F, t82943: F, t3966: F, t14222: F, t1622: F, t23544: F, t25580: F, t25600: F, t25658: F, t3032: F, t3040: F, t3098: F, t4579: F, t6729: F, t83071: F, t83075: F, t82895: F, t25664: F, t23528: F, t23436: F, t4640: F, t14507: F, t23536: F, t1046: F, t25683: F, t3057: F, t3134: F, t4616: F, t6758: F, t82868: F, t83080: F, t83082: F, t83098: F, t23540: F, t23433: F, t10189: F, t4343: F, t13783: F, t4338: F, t13546: F, t13555: F, t13559: F, t14099: F, t14103: F, t14167: F, t23541: F, t25571: F, t25574: F, t3043: F, t6680: F, t82964: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t88335, t88336, t88339, t88341, t88342, t88348) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2207::<F>(t23422, t4603, t14159, t6717, t14137, t6765, t7583, t83138, t23509, t25682, t25644, t82926);
        let t88358 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2208::<F>(t23512, t25486, t23519, t25492, t1607, t23515, t23521, t23529, t4636, t6747, t82911, t82951, t82953, t83092, t88335, t88336, t88339, t88341, t88342, t88348);
        let (t88360, t88362, t88365, t88367, t88372, t88385, t88388) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2209::<F>(t1597, t607, t23562, t343, t40, t4540, t25644, t25650, t6740, t6747, t14206, t6754);
        let t88397 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2210::<F>(t12606, t3, t1025, t1933, t1937, t23453, t23504, t23515, t23521, t25588, t25645, t6722, t6747, t7573, t7583, t82927, t82961, t83111, t88362, t88367, t88372, t88385, t88388);
        let (t88400, t88405, t88407, t88415, t88422, t88425) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2211::<F>(t25651, t3, t83120, t1409, t984, t23562, t343, t1036, t25622, t14134, t6765, t1933, t23479, t88360);
        let t88437 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2212::<F>(t1933, t23479, t88365, t23562, t25637, t984, t1014, t82654, t1022, t14037, t1611, t23419, t23556, t25655, t25661, t363, t378, t6747, t6800, t7583, t82971, t82996, t83085, t88400, t88407, t88415, t88422, t88425);
        let t88472 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2213::<F>(t23479, t25637, t6722, t1409, t344, t1009, t6740, t23473, t13528, t13542, t13931, t14130, t1618, t1920, t1933, t1934, t1935, t23414, t23419, t23495, t25601, t25609, t2987, t343, t4509, t4540, t6730, t6734, t6735, t7578, t82880, t83004, t83025, t83028);
        let t88504 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2214::<F>(t3082, t7586, t25641, t82892, t25638, t6735, t13532, t13537, t13797, t13941, t14122, t14126, t1920, t1941, t23548, t23564, t25679, t378, t4509, t7574, t7583, t82918, t82923, t83016, t83034, t83215);
        let t88533 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2215::<F>(t23418, t4669, t13765, t23419, t14033, t14069, t14488, t23457, t23495, t25585, t25589, t25609, t3073, t360, t4575, t6723, t6735, t6742, t6744, t68, t7574, t7578, t83041, t83046, t83220);
        let (t88537, t88570) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2216::<F>(t10469, t23470, t3, t82986, t23437, t4630, t25641, t82943, t1933, t1937, t3966, t14222, t1597, t1622, t23544, t23548, t25580, t25600, t25601, t25658, t3032, t3040, t3098, t360, t4579, t4636, t6722, t6729, t6735, t83071, t83075, t83215, t83220);
        let t88597 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2217::<F>(t25655, t82895, t25661, t1036, t25664, t1611, t23528, t23436, t4640, t14507, t23536, t1025, t1046, t1622, t23504, t25580, t25683, t3057, t3134, t378, t4616, t6758, t82868, t83080, t83082, t83098);
        let t88632 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2218::<F>(t14507, t23540, t23433, t4630, t10189, t1920, t4343, t13783, t4338, t13546, t13555, t13559, t14099, t14103, t14167, t1618, t23541, t25571, t25574, t2987, t3043, t4509, t6680, t6765, t82964);
    (t88358, t88397, t88405, t88437, t88472, t88504, t88533, t88537, t88570, t88597, t88632)
}
