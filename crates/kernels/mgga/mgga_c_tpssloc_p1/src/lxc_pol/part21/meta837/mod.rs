//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta837 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2978;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2979;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2980;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2981;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2982;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2983;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2984;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2985;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2986;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2987;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2988;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta837<F: Float>(t13965: F, t4641: F, t17659: F, t3048: F, t14207: F, t4630: F, t13969: F, t17717: F, t3039: F, t1020: F, t10508: F, t248: F, t5867: F, t5878: F, t1041: F, t10863: F, t13980: F, t14085: F, t14107: F, t14180: F, t17693: F, t17712: F, t3117: F, t3130: F, t4582: F, t4585: F, t4644: F, t49734: F, t49748: F, t49854: F, t50193: F, t5861: F, t61855: F, t17696: F, t1021: F, t10390: F, t10413: F, t14211: F, t17681: F, t17688: F, t17925: F, t17976: F, t17991: F, t2780: F, t2960: F, t2986: F, t3071: F, t360: F, t42546: F, t42610: F, t42613: F, t43361: F, t48477: F, t48611: F, t49757: F, t50366: F, t55677: F, t55716: F, t59659: F, t61719: F, t973: F, t974: F, t977: F, t998: F, t10422: F, t17648: F, t3070: F, t10214: F, t1031: F, t17701: F, t17877: F, t18036: F, t2979: F, t378: F, t42508: F, t42541: F, t49799: F, t49801: F, t49808: F, t49810: F, t49818: F, t49820: F, t59668: F, t59672: F, t59696: F, t59725: F, t59742: F, t14080: F, t4571: F, t14202: F, t1043: F, t1615: F, t375: F, t10408: F, t1044: F, t10957: F, t10965: F, t14229: F, t17890: F, t2771: F, t42721: F, t49822: F, t49827: F, t49829: F, t49831: F, t49846: F, t5857: F, t59682: F, t59690: F, t62064: F, t17700: F, t1023: F, t10403: F, t13611: F, t1616: F, t42397: F, t42735: F, t42752: F, t4600: F, t48607: F, t49743: F, t49852: F, t49871: F, t49873: F, t49877: F, t49884: F, t49887: F, t5873: F, t61524: F, t62091: F, t1036: F, t17878: F, t17631: F, t3082: F, t5905: F, t10937: F, t10952: F, t17632: F, t17677: F, t17960: F, t43110: F, t48585: F, t49889: F, t49892: F, t49894: F, t49897: F, t49906: F, t49922: F, t50370: F, t884: F, t10480: F, t10883: F, t13985: F, t17670: F, t17705: F, t17980: F, t2776: F, t3041: F, t3121: F, t3132: F, t42347: F, t42354: F, t42358: F, t42496: F, t49940: F, t49945: F, t49957: F, t49959: F, t49964: F, t49966: F, t5909: F, t18035: F, t10904: F, t13995: F, t14033: F, t14037: F, t14174: F, t17734: F, t17988: F, t18021: F, t42505: F, t43114: F, t4590: F, t49972: F, t49987: F, t49989: F, t49993: F, t5681: F, t17906: F, t43338: F, t5677: F, t1022: F, t13532: F, t13537: F, t13542: F, t17593: F, t17923: F, t18016: F, t18025: F, t18030: F, t2775: F, t3123: F, t3131: F, t4347: F, t49616: F, t50027: F, t5900: F, t62055: F, t62059: F, t43198: F, t5908: F, t18041: F, t17636: F, t13528: F, t14143: F, t14147: F, t14184: F, t14489: F, t1622: F, t17718: F, t17738: F, t43358: F, t4593: F, t48432: F, t50047: F, t50056: F, t17642: F, t1618: F, t17920: F, t42511: F, t43155: F, t43157: F, t43161: F, t4596: F, t50062: F, t50077: F, t50302: F, t50445: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t62148, t62150, t62152, t62164, t62177) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2978::<F>(t13965, t4641, t17659, t3048, t14207, t4630, t13969, t17717, t3039, t1020, t10508, t248, t5867);
        let t62185 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2979::<F>(t10508, t248, t3039, t5878, t1041, t10863, t13980, t14085, t14107, t14180, t17693, t17712, t3117, t3130, t4582, t4585, t4644, t49734, t49748, t49854, t50193, t5861, t61855, t62148, t62150, t62152, t62164, t62177);
        let t62225 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2980::<F>(t1041, t13969, t17696, t1021, t10390, t10413, t14211, t17681, t17688, t17925, t17976, t17991, t248, t2780, t2960, t2986, t3039, t3071, t3117, t360, t42546, t42610, t42613, t43361, t48477, t48611, t49757, t50366, t55677, t55716, t5878, t59659, t61719, t973, t974, t977, t998);
        let t62258 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2981::<F>(t10422, t17648, t3070, t10214, t1031, t17701, t17877, t18036, t2979, t378, t42508, t42541, t49799, t49801, t49808, t49810, t49818, t49820, t59668, t59672, t59696, t59725, t59742, t973, t977);
        let (t62291, t62296) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2982::<F>(t14080, t4571, t14202, t4644, t1043, t1615, t375, t10408, t1041, t1044, t10957, t10965, t14229, t17890, t248, t2771, t2780, t3070, t3071, t3117, t42721, t49822, t49827, t49829, t49831, t49846, t5857, t5861, t5867, t59682, t59690, t62064);
        let t62333 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2983::<F>(t10413, t10422, t17700, t1023, t10403, t10408, t13611, t1616, t2771, t2780, t3039, t3070, t3071, t42397, t42735, t42752, t4582, t4600, t48607, t49743, t49852, t49871, t49873, t49877, t49884, t49887, t5873, t61524, t62091);
        let t62362 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2984::<F>(t1036, t17878, t13969, t17631, t3039, t3082, t5905, t10937, t10952, t17632, t17677, t17960, t2986, t3070, t3071, t43110, t48585, t49889, t49892, t49894, t49897, t49906, t49922, t50370, t55716, t884);
        let t62398 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2985::<F>(t10390, t10403, t10480, t10883, t13985, t17670, t17677, t17705, t17712, t17980, t2776, t3041, t3071, t3121, t3132, t42347, t42354, t42358, t42496, t4582, t49940, t49945, t49957, t49959, t49964, t49966, t5873, t5909);
        let t62427 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2986::<F>(t10403, t10422, t18035, t10904, t10937, t13995, t14033, t14037, t14085, t14174, t17734, t17988, t18021, t18036, t2960, t3070, t3071, t3121, t42505, t43114, t4590, t4644, t49972, t49987, t49989, t49993, t5681);
        let t62475 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2987::<F>(t17906, t3048, t1041, t248, t43338, t5677, t1022, t10403, t10408, t10413, t10937, t10957, t13532, t13537, t13542, t14211, t1616, t17593, t17923, t18016, t18025, t18030, t2775, t2960, t3070, t3071, t3123, t3131, t42397, t42505, t42541, t4347, t49616, t50027, t5900, t62055, t62059, t62291);
        let t62512 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2988::<F>(t3070, t43198, t5908, t10937, t18041, t1041, t13969, t17636, t10408, t10413, t10952, t13528, t14143, t14147, t14184, t14489, t1616, t1622, t17718, t17738, t2776, t2960, t3039, t3071, t43358, t4582, t4593, t4644, t48432, t50047, t50056, t5878, t5909);
        let t62544 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2989::<F>(t1041, t13969, t17642, t17906, t3117, t10390, t10403, t10413, t10965, t1618, t17920, t17976, t3041, t3048, t3071, t3132, t42511, t43155, t43157, t43161, t4596, t50062, t50077, t50302, t50445, t5681, t5900, t5909);
    (t62185, t62225, t62258, t62296, t62333, t62362, t62398, t62427, t62475, t62512, t62544)
}
