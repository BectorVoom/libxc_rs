//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta657 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2428;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2429;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2430;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2431;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2432;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2433;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2434;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2435;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2436;
use chunk9::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2437;
use chunk10::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2438;
use chunk11::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta657<F: Float>(t13998: F, t2960: F, t42875: F, t4338: F, t973: F, t10422: F, t14040: F, t3070: F, t10516: F, t4640: F, t10403: F, t14121: F, t13748: F, t1025: F, t10884: F, t10937: F, t14041: F, t1539: F, t2780: F, t3071: F, t42483: F, t42552: F, t42557: F, t42578: F, t42582: F, t4650: F, t13965: F, t3114: F, t14202: F, t3117: F, t10423: F, t13995: F, t10413: F, t14221: F, t10949: F, t14025: F, t10195: F, t10408: F, t10433: F, t10965: F, t13991: F, t14215: F, t14511: F, t1616: F, t42541: F, t42565: F, t42570: F, t42586: F, t42861: F, t4596: F, t4636: F, t47679: F, t10883: F, t13969: F, t14106: F, t13559: F, t2970: F, t1036: F, t13942: F, t3047: F, t4616: F, t10890: F, t14507: F, t1041: F, t14188: F, t1046: F, t10898: F, t13977: F, t13982: F, t13987: F, t1618: F, t3043: F, t42595: F, t43120: F, t43322: F, t43343: F, t4652: F, t1020: F, t14489: F, t248: F, t3101: F, t3038: F, t49650: F, t1022: F, t10480: F, t10876: F, t13975: F, t13985: F, t14143: F, t14180: F, t14211: F, t14218: F, t2244: F, t2775: F, t2776: F, t3132: F, t360: F, t42610: F, t42613: F, t42619: F, t42622: F, t42651: F, t4582: F, t13611: F, t3051: F, t14137: F, t14125: F, t4571: F, t10508: F, t10962: F, t4630: F, t13961: F, t10863: F, t14126: F, t14213: F, t14491: F, t17732: F, t3109: F, t42508: F, t43358: F, t4575: F, t884: F, t10957: F, t13950: F, t3048: F, t14173: F, t247: F, t677: F, t4589: F, t10969: F, t41687: F, t42600: F, t42721: F, t42729: F, t42731: F, t4583: F, t4588: F, t45993: F, t4600: F, t46006: F, t48497: F, t1009: F, t13939: F, t1011: F, t1019: F, t10868: F, t4347: F, t14134: F, t14102: F, t3039: F, t13990: F, t14093: F, t42735: F, t42752: F, t43094: F, t43097: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t49658, t49662, t49666, t49678, t49682) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2428::<F>(t13998, t2960, t42875, t4338, t973, t10422, t14040, t3070, t10516, t4640, t10403, t14121);
        let t49688 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2429::<F>(t13748, t2960, t1025, t10884, t10937, t14041, t1539, t2780, t3070, t3071, t42483, t42552, t42557, t42578, t42582, t4650, t49658, t49662, t49666, t49678, t49682);
        let t49718 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2430::<F>(t13965, t3114, t14202, t3117, t10423, t13995, t10413, t10422, t14221, t10949, t14025, t10195, t10408, t10433, t10965, t13991, t14215, t14511, t1616, t3070, t42541, t42565, t42570, t42586, t42861, t4596, t4636, t47679, t973);
        let (t49721, t49732, t49734, t49740, t49743) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2431::<F>(t10883, t13969, t14106, t13559, t2970, t973, t1036, t13942, t3047, t4616, t10890, t14507);
        let t49750 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2432::<F>(t1041, t13969, t14188, t1046, t10898, t10949, t13977, t13982, t13987, t1618, t3043, t42595, t43120, t43322, t43343, t4596, t4652, t49721, t49732, t49734, t49740, t49743);
        let t49786 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2433::<F>(t1020, t14489, t248, t3101, t3038, t49650, t1022, t10403, t10413, t10480, t10876, t13975, t13985, t14143, t14180, t14211, t14218, t2244, t2775, t2776, t3043, t3071, t3117, t3132, t360, t42610, t42613, t42619, t42622, t42651, t4582);
        let (t49799, t49801, t49808, t49810, t49818) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2434::<F>(t1041, t13611, t248, t3051, t14137, t3117, t10413, t10422, t14125, t10965, t4571, t1020, t10508, t4650);
        let t49824 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2435::<F>(t49818, t10962, t4630, t13961, t3114, t10403, t10863, t14126, t14213, t14489, t14491, t17732, t3070, t3071, t3109, t42508, t43358, t4575, t4636, t49799, t49801, t49808, t49810, t884);
        let (t49827, t49829, t49832, t49846, t49850) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2436::<F>(t10957, t4571, t13950, t3048, t13965, t3109, t1041, t13969, t14173, t247, t677);
        let t49860 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2437::<F>(t1041, t4589, t49850, t10969, t41687, t42600, t42721, t42729, t42731, t4582, t4583, t4588, t45993, t4600, t46006, t48497, t49827, t49829, t49832, t49846);
        let (t49864, t49866, t49872, t49873, t49877) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2438::<F>(t1009, t13939, t1011, t1019, t1041, t10868, t248, t4347, t14134, t3117, t10863, t4571);
        let t49891 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2439::<F>(t13969, t14102, t3039, t10876, t13990, t14134, t3048, t1025, t10957, t14093, t42735, t42752, t43094, t43097, t4636, t49866, t49872, t49873, t49877);
    (t49688, t49718, t49750, t49786, t49824, t49850, t49860, t49864, t49891)
}
