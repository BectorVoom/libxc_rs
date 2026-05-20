//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta481 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1439;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1440;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1441;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1442;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1443;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1444;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1445;
use chunk7::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1446;
use chunk8::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1447;
use chunk9::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1448;
use chunk10::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1449;
use chunk11::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1450;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta481<F: Float>(t15376: F, t22069: F, t3447: F, t4908: F, t6123: F, t64811: F, t73274: F, t73276: F, t73279: F, t73287: F, t73290: F, t73307: F, t73314: F, t78043: F, t78047: F, t4900: F, t4904: F, t64821: F, t73169: F, t73330: F, t73386: F, t73389: F, t73395: F, t73417: F, t73420: F, t73424: F, t78031: F, t78039: F, t15390: F, t18409: F, t18420: F, t18427: F, t18469: F, t22072: F, t22075: F, t22090: F, t22095: F, t4919: F, t52081: F, t64648: F, t73181: F, t73201: F, t73405: F, t73427: F, t1740: F, t48: F, t338: F, t11546: F, t1174: F, t18321: F, t44566: F, t463: F, t52124: F, t6127: F, t64878: F, t64881: F, t64885: F, t64979: F, t73433: F, t73444: F, t73451: F, t75836: F, sigma2: F, t1177: F, t1714: F, t22032: F, t22047: F, t22052: F, t22082: F, t3440: F, t3441: F, t3455: F, t44487: F, t44621: F, t44622: F, t460: F, t4889: F, t4934: F, t6120: F, t65002: F, t65023: F, t73491: F, t75847: F, t6144: F, t6138: F, t1409: F, t1710: F, t22035: F, t22041: F, t22056: F, t22060: F, t3450: F, t457: F, t6131: F, t65112: F, t65126: F, t73113: F, t974: F, t50846: F, t63888: F, t63893: F, t63911: F, t71335: F, t71337: F, t71408: F, t77959: F, t77963: F, t77967: F, t78084: F, t44466: F, t71470: F, t71472: F, t71474: F, t77971: F, t77975: F, t77979: F, t77983: F, t78087: F, t78090: F, t78093: F, t78100: F, t11516: F, t11547: F, t1178: F, t1717: F, t29614: F, t52281: F, t6141: F, t6147: F, t73523: F, t73535: F, t73541: F, t75912: F, t78423: F, t1238: F, t1751: F, t1760: F, t1761: F, t19232: F, t19234: F, t22004: F, t22008: F, t22113: F, t22393: F, t22394: F, t3598: F, t491: F, t4945: F, t498: F, t5055: F, t6150: F, t6238: F, t6244: F, t6268: F, t73900: F, t78379: F, t11678: F, t1227: F, t15507: F, t15654: F, t1653: F, t1734: F, t1737: F, t1748: F, t19033: F, t22275: F, t22301: F, t3578: F, t4582: F, t4972: F, t53087: F, t6211: F, t65444: F, t65464: F, t72161: F, t72181: F, t72183: F, t72389: F, t72398: F, t72967: F, t77606: F, t77621: F, t11692: F, t18395: F, t19047: F, t22208: F, t22246: F, t22258: F, t22314: F, t5005: F, t5019: F, t5024: F, t53083: F, t6221: F, t65528: F, t72223: F, t72225: F, t72229: F, t72248: F, t72251: F, t72253: F, t72384: F, t72767: F, t15453: F, t1730: F, t22174: F, t488: F, t6232: F, t65552: F, t65558: F, t65581: F, t65706: F, t72273: F, t72285: F, t72287: F, t72289: F, t72293: F, t72297: F, t72302: F) -> (F, F, F, F, F, F, F) {
        let t78441 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1439::<F>(t15376, t22069, t3447, t4908, t6123, t64811, t73274, t73276, t73279, t73287, t73290, t73307, t73314, t78043, t78047);
        let t78460 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1440::<F>(t3447, t4900, t4904, t64821, t73169, t73330, t73386, t73389, t73395, t73417, t73420, t73424, t78031, t78039);
        let t78489 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1441::<F>(t15376, t15390, t18409, t18420, t18427, t18469, t22072, t22075, t22090, t22095, t3447, t4904, t4919, t52081, t64648, t73181, t73201, t73405, t73427);
        let (t78505, t78506, t78516) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1442::<F>(t1740, t48, t338, t11546, t1174, t15390, t18321, t3447, t44566, t463, t4919, t52124, t6127, t64878, t64881, t64885, t64979, t73433, t73444, t73451, t75836, sigma2);
        let t78545 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1443::<F>(t1174, t1177, t1714, t18321, t22032, t22047, t22052, t22082, t3440, t3441, t3455, t44487, t44621, t44622, t460, t4889, t4934, t6120, t65002, t65023, t73491, t75836, t75847);
        let t78578 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1444::<F>(t6144, t6138, t1174, t1409, t1710, t18321, t22035, t22041, t22056, t22060, t3447, t3450, t457, t460, t4889, t4919, t6131, t65112, t65126, t73113, t974);
        let (t78596, t78607) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1445::<F>(t50846, t63888, t63893, t63911, t71335, t71337, t71408, t77959, t77963, t77967, t78084, t44466, t71470, t71472, t71474, t77971, t77975, t77979, t77983, t78087, t78090, t78093, t78100);
        let t78634 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1446::<F>(t11516, t11547, t1174, t1177, t1178, t1717, t18321, t29614, t3440, t457, t460, t4934, t52281, t6138, t6141, t6147, t73113, t73523, t73535, t73541, t75836, t75912, t78596, t78607, t974);
        let (t78637, t78646) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1447::<F>(t78423, t78441, t78460, t78489, t78516, t78545, t78578, t78634, t1238, t1751, t1760, t1761, t19232, t19234, t22004, t22008, t22113, t22393, t22394, t3598, t491, t4945, t498, t5055, t6150, t6238, t6244, t6268, t73900, t78379);
        let t78689 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1448::<F>(t11678, t1227, t15507, t15654, t1653, t1734, t1737, t1748, t19033, t22275, t22301, t3578, t4582, t4972, t53087, t6211, t65444, t65464, t72161, t72181, t72183, t72389, t72398, t72967, t77606, t77621);
        let t78713 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1449::<F>(t11692, t1748, t18395, t19047, t22208, t22246, t22258, t22314, t3578, t5005, t5019, t5024, t53083, t6221, t65528, t72223, t72225, t72229, t72248, t72251, t72253, t72384, t72767);
        let t78734 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1450::<F>(t1227, t15453, t1730, t22174, t4582, t488, t6232, t65552, t65558, t65581, t65706, t72273, t72285, t72287, t72289, t72293, t72297, t72302, t77606);
    (t78505, t78506, t78637, t78646, t78689, t78713, t78734)
}
