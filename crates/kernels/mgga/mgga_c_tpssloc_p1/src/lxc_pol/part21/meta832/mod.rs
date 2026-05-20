//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta832 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2932;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2933;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2934;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2935;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2936;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2937;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2938;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2939;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta832<F: Float>(t17579: F, t225: F, t18048: F, t210: F, t974: F, t2985: F, t1597: F, t976: F, t17826: F, t2960: F, t12652: F, t4337: F, t10236: F, t17686: F, t43070: F, t10254: F, t17635: F, t13554: F, t10263: F, t13835: F, t1539: F, t17844: F, t23547: F, t2986: F, t2988: F, t43069: F, t4510: F, t4518: F, t4531: F, t47907: F, t5845: F, t984: F, t17691: F, t41831: F, t41863: F, t41870: F, t41872: F, t48087: F, t48096: F, t48098: F, t48103: F, t48116: F, t60091: F, t60153: F, t60156: F, t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t60163: F, t60166: F, t60168: F, t60171: F, t60173: F, t60189: F, t60192: F, t60194: F, t60197: F, t60200: F, t60202: F, t60204: F, t60207: F, t60223: F, t60226: F, t60229: F, t60232: F, t60235: F, t43002: F, t60274: F, t60277: F, t60282: F, t60296: F, t60308: F, t60310: F, t60312: F, t60315: F, t60318: F, t60321: F, t60324: F, t60327: F, t135: F, t17843: F, t973: F, t13831: F, t17804: F, t340: F, t343: F, t42811: F, t42817: F, t42873: F, t42877: F, t42893: F, t42895: F, t47887: F, t47938: F, t10189: F, t5842: F, t2990: F, t13847: F, t13861: F, t17841: F, t2987: F, t10186: F, t13812: F, t17788: F, t17805: F, t17811: F, t17814: F, t17818: F, t17854: F, t17867: F, t47966: F, t48184: F, t13798: F, t13851: F, t17791: F, t17821: F, t42903: F, t42906: F, t42911: F, t42914: F, t59668: F, t59672: F, t59696: F, t59725: F, t59742: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t61058, t61061, t61064, t61065, t61066, t61074, t61078) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2932::<F>(t17579, t225, t18048, t210, t974, t2985, t1597, t976, t17826, t2960, t12652, t4337);
        let (t61098, t61102) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2933::<F>(t10236, t17686, t43070, t10254, t17635, t12652, t13554, t10263, t13835, t1539, t17844, t23547, t2960, t2986, t2988, t43069, t4510, t4518, t4531, t47907, t5845, t61065, t61066, t61074, t61078, t984);
        let (t61103, t61124) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2934::<F>(t10254, t17691, t41831, t41863, t41870, t41872, t48087, t48096, t48098, t48103, t48116, t60091, t60153, t60156);
        let t61138 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2935::<F>(t48155, t48157, t48159, t48161, t48163, t48165, t48167, t60163, t60166, t60168, t60171, t60173, t60189);
        let t61150 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2936::<F>(t60192, t60194, t60197, t60200, t60202, t60204, t60207, t60223, t60226, t60229, t60232, t60235);
        let t61163 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2937::<F>(t43002, t60274, t60277, t60282, t60296, t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327);
        let t61181 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2938::<F>(t135, t17843, t973, t13831, t17804, t2986, t2988, t340, t343, t42811, t42817, t42873, t42877, t42893, t42895, t4531, t47887, t47938, t61103, t61124, t61138, t61150, t61163, t974);
        let t61214 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2939::<F>(t10189, t5842, t2986, t2990, t13847, t13861, t17841, t2987, t10186, t13812, t17788, t17805, t17811, t17814, t17818, t17854, t17867, t4531, t47966, t48184);
        let t61241 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2940::<F>(t10186, t13798, t13851, t13861, t17791, t17821, t2986, t42903, t42906, t42911, t42914, t4510, t4518, t59668, t59672, t59696, t59725, t59742);
    (t61058, t61061, t61064, t61065, t61078, t61098, t61102, t61181, t61214, t61241)
}
