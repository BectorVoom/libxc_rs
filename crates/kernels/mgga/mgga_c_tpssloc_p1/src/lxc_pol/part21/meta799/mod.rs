//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta799 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2779;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2780;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2781;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2782;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2783;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2784;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2785;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2786;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta799<F: Float>(t16673: F, t2696: F, t849: F, t13360: F, t4261: F, t5584: F, t9975: F, t5619: F, t9674: F, t4250: F, t46657: F, t16907: F, t9638: F, t119: F, t12971: F, t13222: F, t13229: F, t13254: F, t13262: F, t13347: F, t13365: F, t1484: F, t1516: F, t16901: F, t16932: F, t16937: F, t16946: F, t210: F, t2553: F, t2623: F, t2643: F, t2645: F, t2684: F, t2701: F, t4172: F, t4191: F, t46570: F, t47037: F, t47044: F, t5527: F, t58139: F, t787: F, t820: F, t843: F, t9607: F, t17013: F, t13258: F, t16845: F, t13261: F, t4166: F, t13151: F, t13156: F, t13164: F, t13191: F, t16723: F, t16729: F, t16737: F, t16749: F, t1891: F, t228: F, t2379: F, t2667: F, t2671: F, t2675: F, t4219: F, t4225: F, t4227: F, t4230: F, t5544: F, t5601: F, t5605: F, t5608: F, t58090: F, t68: F, t822: F, t824: F, t825: F, t39249: F, t39256: F, t39309: F, t39312: F, t39316: F, t39320: F, t40626: F, t57877: F, t57879: F, t57880: F, t57885: F, t57886: F, t57888: F, t57889: F, t57891: F, t57892: F, t57897: F, t57898: F, t57899: F, t39373: F, t39397: F, t39400: F, t39408: F, t39411: F, t40679: F, t40685: F, t40708: F, t40714: F, t40716: F, t57900: F, t57903: F, t57907: F, t57908: F, t57936: F, t57939: F, t57943: F, t57946: F, t57948: F, t39463: F, t39468: F, t39472: F, t39476: F, t40721: F, t40732: F, t57959: F, t57961: F, t57962: F, t57966: F, t57970: F, t57972: F, t57975: F, t57983: F, t57986: F, t57987: F, t57988: F, t57989: F, t57990: F, t39483: F, t40741: F, t40743: F, t40748: F, t40760: F, t57993: F, t57996: F, t58005: F, t58008: F, t58020: F, t58022: F, t58023: F, t58025: F, t58026: F, t58027: F, t58028: F, t58030: F, t58032: F, t58033: F, t58034: F, t39529: F, t40764: F, t40766: F, t40779: F, t40784: F, t40790: F, t40793: F, t40797: F, t40799: F, t58035: F, t58040: F, t58042: F, t58046: F, t58048: F, t58053: F, t58055: F, t58056: F, t58058: F, t58059: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t58845, t58847, t58853, t58859, t58873, t58885) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2779::<F>(t16673, t2696, t849, t13360, t4261, t5584, t9975, t5619, t9674, t4250, t46657, t16907, t9638);
        let t58887 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2780::<F>(t119, t12971, t13222, t13229, t13254, t13262, t13347, t13365, t1484, t1516, t16901, t16932, t16937, t16946, t210, t2553, t2623, t2643, t2645, t2684, t2701, t4172, t4191, t4261, t46570, t47037, t47044, t5527, t58139, t58845, t58847, t58853, t58859, t58873, t58885, t787, t820, t843, t9607);
        let (t58890, t58900, t58904, t58947) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2781::<F>(t17013, t9638, t13258, t16845, t13261, t4166, t13151, t13156, t13164, t13191, t16723, t16729, t16737, t16749, t1891, t228, t2379, t2667, t2671, t2675, t4219, t4225, t4227, t4230, t5544, t5601, t5605, t5608, t58090, t58139, t68, t822, t824, t825);
        let t58963 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2782::<F>(t39249, t39256, t39309, t39312, t39316, t39320, t40626, t57877, t57879, t57880, t57885, t57886, t57888, t57889, t57891, t57892, t57897, t57898, t57899);
        let t58964 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2783::<F>(t39373, t39397, t39400, t39408, t39411, t40679, t40685, t40708, t40714, t40716, t57900, t57903, t57907, t57908, t57936, t57939, t57943, t57946, t57948);
        let t58966 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2784::<F>(t39463, t39468, t39472, t39476, t40721, t40732, t57959, t57961, t57962, t57966, t57970, t57972, t57975, t57983, t57986, t57987, t57988, t57989, t57990);
        let t58967 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2785::<F>(t39483, t40741, t40743, t40748, t40760, t57993, t57996, t58005, t58008, t58020, t58022, t58023, t58025, t58026, t58027, t58028, t58030, t58032, t58033, t58034);
        let t58970 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2786::<F>(t39529, t40764, t40766, t40779, t40784, t40790, t40793, t40797, t40799, t58035, t58040, t58042, t58046, t58048, t58053, t58055, t58056, t58058, t58059);
    (t58887, t58890, t58900, t58904, t58947, t58963, t58964, t58966, t58967, t58970)
}
