//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta836 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2970;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2971;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2972;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2973;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2974;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2975;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2976;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta836<F: Float>(t17997: F, t3070: F, t42488: F, t1041: F, t13969: F, t17975: F, t10413: F, t10876: F, t10937: F, t14080: F, t1409: F, t14167: F, t14172: F, t14218: F, t14219: F, t17649: F, t17712: F, t17920: F, t17923: F, t3071: F, t3131: F, t3132: F, t3966: F, t42483: F, t43361: F, t4579: F, t4582: F, t4590: F, t4644: F, t49604: F, t49607: F, t49621: F, t49629: F, t49984: F, t61910: F, t883: F, t17687: F, t14085: F, t4571: F, t13765: F, t13995: F, t18086: F, t3069: F, t10403: F, t10891: F, t14041: F, t14130: F, t17718: F, t2776: F, t3041: F, t3073: F, t3121: F, t4650: F, t47779: F, t48611: F, t49658: F, t49661: F, t49666: F, t5685: F, t5867: F, t61855: F, t10952: F, t17655: F, t17659: F, t3117: F, t17187: F, t248: F, t3051: F, t10390: F, t10480: F, t10904: F, t13762: F, t14488: F, t17670: F, t17714: F, t17998: F, t3040: F, t3130: F, t42552: F, t42573: F, t43291: F, t43292: F, t4593: F, t4596: F, t48607: F, t49651: F, t49682: F, t49684: F, t50510: F, t5880: F, t61078: F, t10422: F, t17704: F, t17680: F, t17692: F, t10408: F, t17697: F, t17705: F, t17984: F, t2771: F, t3048: F, t42334: F, t42388: F, t42586: F, t4575: F, t4600: F, t48477: F, t48612: F, t49690: F, t49692: F, t49697: F, t49771: F, t5878: F, t61098: F, t2244: F, t5398: F, t14077: F, t4630: F, t10401: F, t246: F, t3067: F, t3186: F, t1615: F, t3061: F, t375: F, t1022: F, t3961: F, t3200: F, t10482: F, t5872: F, t17924: F, t17959: F, t376: F, t17672: F, t14164: F, t14207: F, t14213: F, t14228: F, t14234: F, t17151: F, t17177: F, t17182: F, t17673: F, t17925: F, t2770: F, t42397: F, t42508: F, t43322: F, t4594: F, t4652: F, t49702: F, t10868: F, t5681: F, t10949: F, t14187: F, t1622: F, t17643: F, t17734: F, t17972: F, t4583: F, t4588: F, t4636: F, t49716: F, t49721: F, t49732: F, t49740: F, t50334: F, t55662: F, t55666: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t61921 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2970::<F>(t17997, t3070, t42488, t1041, t13969, t17975, t10413, t10876, t10937, t14080, t1409, t14167, t14172, t14218, t14219, t17649, t17712, t17920, t17923, t3071, t3131, t3132, t3966, t42483, t43361, t4579, t4582, t4590, t4644, t49604, t49607, t49621, t49629, t49984, t61910, t883);
        let t61965 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2971::<F>(t1041, t13969, t17687, t14085, t4571, t13765, t13995, t18086, t3069, t10403, t10413, t10891, t14041, t14130, t14218, t17718, t2776, t3041, t3070, t3071, t3073, t3121, t3132, t42483, t4582, t4650, t47779, t48611, t49658, t49661, t49666, t5685, t5867, t61855);
        let t62007 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2972::<F>(t10952, t17655, t17659, t3117, t1041, t17187, t248, t3051, t10390, t10480, t10904, t13762, t13995, t14488, t17670, t17714, t17998, t3040, t3071, t3130, t3131, t42552, t42573, t43291, t43292, t4582, t4593, t4596, t48607, t49651, t49682, t49684, t50510, t5880, t61078);
        let t62042 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2973::<F>(t10422, t17704, t3070, t17680, t1041, t13969, t17692, t10408, t10413, t10937, t17697, t17705, t17984, t2771, t3048, t42334, t42388, t42586, t4575, t4600, t48477, t48607, t48611, t48612, t49690, t49692, t49697, t49771, t49984, t5878, t61098);
        let t62044 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2974::<F>(t2244, t5398);
        let (t62049, t62053, t62055, t62057, t62059, t62064) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2975::<F>(t14077, t4630, t10401, t246, t3067, t3186, t1615, t3061, t375, t1022, t3961, t3200);
        let (t62091, t62101) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2976::<F>(t10482, t5872, t10413, t10422, t17924, t17959, t376, t10480, t13969, t17672, t10408, t1041, t14164, t14207, t14213, t14228, t14234, t17151, t17177, t17182, t17673, t17925, t2770, t3070, t3071, t3130, t3131, t42388, t42397, t42508, t43322, t4582, t4594, t4652, t49702, t62044, t62049, t62055, t62057, t62059, t62064);
        let t62145 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2977::<F>(t1041, t10868, t248, t5681, t10949, t14080, t14172, t14187, t1622, t17643, t17734, t17972, t3117, t4582, t4583, t4588, t4636, t49716, t49721, t49732, t49740, t50334, t55662, t55666, t62044);
    (t61921, t61965, t62007, t62042, t62044, t62053, t62055, t62059, t62064, t62091, t62101, t62145)
}
