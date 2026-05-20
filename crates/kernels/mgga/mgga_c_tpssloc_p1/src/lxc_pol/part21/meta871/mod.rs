//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta871 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3200;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3201;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3202;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3203;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3204;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3205;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3206;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3207;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3208;
use chunk9::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3209;
use chunk10::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3210;
use chunk11::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3211;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta871<F: Float>(t11697: F, t18968: F, t3577: F, t11539: F, t1174: F, t18232: F, t18215: F, t1734: F, t3584: F, t375: F, t11665: F, t18371: F, t15569: F, t15572: F, t15714: F, t15740: F, t15749: F, t18364: F, t3242: F, t3508: F, t3578: F, t45250: F, t4733: F, t4950: F, t5012: F, t52615: F, t53433: F, t53440: F, t53452: F, t66372: F, t66378: F, t66380: F, t1244: F, t3068: F, t478: F, t6163: F, t11734: F, t1227: F, t15498: F, t15525: F, t15541: F, t19072: F, t3515: F, t3580: F, t4582: F, t4977: F, t4989: F, t5024: F, t52919: F, t53456: F, t53468: F, t53470: F, t53476: F, t53481: F, t53490: F, t53494: F, t53496: F, t53498: F, t61855: F, t18386: F, t15608: F, t225: F, t65165: F, t6183: F, t698: F, t1216: F, t15470: F, t15474: F, t15700: F, t18241: F, t18383: F, t18965: F, t45119: F, t45134: F, t45266: F, t45296: F, t484: F, t488: F, t4954: F, t52893: F, t52897: F, t53322: F, t53515: F, t53519: F, t64874: F, t68: F, t65463: F, t65518: F, t65565: F, t65610: F, t65653: F, t65685: F, t65716: F, t65764: F, t65802: F, t65835: F, t65883: F, t65925: F, t65954: F, t65990: F, t66029: F, t66067: F, t66111: F, t66157: F, t66185: F, t66219: F, t66254: F, t66282: F, t66326: F, t66353: F, t66400: F, t66442: F, t66480: F, t66528: F, t66564: F, t11871: F, t11881: F, t11888: F, t1201: F, t15022: F, t15247: F, t15426: F, t1758: F, t18301: F, t19169: F, t19174: F, t19197: F, t3507: F, t3604: F, t3610: F, t3624: F, t3625: F, t44724: F, t44726: F, t44730: F, t470: F, t493: F, t5011: F, t5079: F, t52479: F, t52480: F, t6252: F, t6256: F, t6260: F, t65265: F, t1246: F, t15001: F, t15019: F, t15027: F, t15032: F, t15245: F, t15253: F, t15257: F, t19129: F, t19179: F, t3590: F, t44698: F, t44701: F, t44741: F, t45320: F, t4978: F, t5068: F, t5073: F, t53613: F, t53646: F, t6218: F, t6253: F, t11914: F, t14992: F, t15429: F, t15772: F, t1729: F, t19145: F, t19153: F, t19156: F, t19170: F, t3565: F, t45323: F, t4964: F, t5076: F, t5086: F, t53545: F, t6263: F, t6265: F, t1243: F, t65955: F, t11904: F, t1247: F, t1249: F, t15000: F, t15016: F, t15241: F, t1756: F, t18572: F, t19142: F, t19157: F, t19180: F, t19203: F, t23508: F, t3612: F, t3628: F, t44691: F, t44785: F, t475: F, t494: F, t5064: F, t5072: F, t52447: F, t6168: F, t65347: F, t19253: F, t5088: F, t11925: F, t1238: F, t1241: F, t1251: F, t1252: F, t14980: F, t15786: F, t15803: F, t15820: F, t1760: F, t1761: F, t19208: F, t19220: F, t19234: F, t3593: F, t3598: F, t3599: F, t3631: F, t45350: F, t466: F, t498: F, t5055: F, t5060: F, t51925: F, t51928: F, t6243: F, t6268: F, t65208: F, t65249: F, t65343: F, t65374: F, t65408: F, t19121: F, t19259: F, t11613: F, t14972: F, t15425: F, t15787: F, t15794: F, t15797: F, t1751: F, t19209: F, t19232: F, t19249: F, t3481: F, t3487: F, t3600: F, t4940: F, t4945: F, t5052: F, t5089: F, t53658: F, t6238: F, t1254: F, t1256: F, t15834: F, t193: F, t336: F, t4700: F, t5095: F, t63714: F, t63717: F, t63720: F, t63722: F, t63725: F, t63729: F, t64548: F, t64558: F, t64562: F, t64564: F, t64566: F, t64602: F, t65206: F) -> F {
        let (t66566, t66571, t66575, t66583, t66597) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3200::<F>(t11697, t18968, t3577, t11539, t1174, t18232, t18215, t1734, t3584, t375, t11665, t18371);
        let t66601 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3201::<F>(t15569, t15572, t11665, t15714, t15740, t15749, t18364, t3242, t3508, t3577, t3578, t45250, t4733, t4950, t5012, t52615, t53433, t53440, t53452, t66372, t66378, t66380, t66566, t66571, t66575, t66583, t66597);
        let t66631 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3202::<F>(t1244, t3068, t478, t6163, t11734, t1227, t15498, t15525, t15541, t19072, t3515, t3580, t4582, t4977, t4989, t5024, t52919, t53456, t53468, t53470, t53476, t53481, t53490, t53494, t53496, t53498, t61855);
        let (t66662, t66670) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3203::<F>(t11697, t18386, t3577, t15608, t15740, t225, t65165, t1174, t6183, t698, t11665, t1216, t15470, t15474, t15569, t15700, t18241, t18383, t18965, t3578, t45119, t45134, t45266, t45296, t484, t488, t4954, t5012, t52893, t52897, t53322, t53515, t53519, t64874, t68);
        let t66675 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3204::<F>(t65463, t65518, t65565, t65610, t65653, t65685, t65716, t65764, t65802, t65835, t65883, t65925, t65954, t65990, t66029, t66067, t66111, t66157, t66185, t66219, t66254, t66282, t66326, t66353, t66400, t66442, t66480, t66528, t66564, t66601, t66631, t66670);
        let t66702 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3205::<F>(t11871, t11881, t11888, t1201, t15022, t15247, t15426, t1758, t18301, t19169, t19174, t19197, t3507, t3604, t3610, t3624, t3625, t44724, t44726, t44730, t470, t493, t5011, t5079, t52479, t52480, t6252, t6256, t6260, t65265, t66675);
        let t66737 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3206::<F>(t11871, t11888, t1244, t1246, t15001, t15019, t15027, t15032, t15245, t15253, t15257, t19129, t19179, t3507, t3590, t3604, t3610, t44698, t44701, t44741, t45320, t4978, t5011, t5068, t5073, t52480, t53613, t53646, t6218, t6252, t6253, t6256);
        let t66769 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3207::<F>(t11888, t11914, t1216, t14992, t15032, t15245, t15429, t15772, t1729, t19145, t19153, t19156, t19169, t19170, t3565, t3604, t3610, t45323, t4964, t5011, t5068, t5076, t5086, t52480, t53545, t6256, t6260, t6263, t6265);
        let t66802 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3208::<F>(t1243, t65955, t11881, t11904, t1247, t1249, t15000, t15016, t15241, t1756, t18572, t19142, t19157, t19180, t19203, t23508, t3507, t3604, t3610, t3612, t3628, t44691, t44785, t475, t494, t5064, t5072, t52447, t6168, t6252, t6256, t65347, t66662);
        let t66842 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3209::<F>(t19253, t225, t5088, t11925, t1238, t1241, t1251, t1252, t14980, t15786, t15803, t15820, t1760, t1761, t19208, t19220, t19234, t3593, t3598, t3599, t3631, t45350, t466, t498, t5055, t5060, t51925, t51928, t6243, t6268, t65208, t65249, t65343, t65374, t65408, t66675, t66702, t66737, t66769, t66802);
        let t66879 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3210::<F>(t19121, t225, t19259, t11613, t1252, t14972, t15425, t15787, t15794, t15797, t1751, t1761, t19209, t19220, t19232, t19234, t19249, t3481, t3487, t3600, t3631, t4940, t4945, t498, t5052, t5055, t5060, t5089, t53658, t6238, t6268);
        let t66885 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3211::<F>(t1254, t1256, t15834, t193, t336, t4700, t5095, t63714, t63717, t63720, t63722, t63725, t63729, t64548, t64558, t64562, t64564, t64566, t64602, t65206, t66842, t66879);
    t66885
}
