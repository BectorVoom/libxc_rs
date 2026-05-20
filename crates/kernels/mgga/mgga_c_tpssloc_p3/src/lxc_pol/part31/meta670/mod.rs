//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta670 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1989;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1990;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1991;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1992;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1993;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1994;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1995;
use chunk7::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1996;
use chunk8::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1997;
use chunk9::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1998;
use chunk10::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1999;
use chunk11::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta670<F: Float>(t101832: F, t870: F, t193: F, t7859: F, t16557: F, t1877: F, t2057: F, t24191: F, t24339: F, t25: F, t25024: F, t2522: F, t25375: F, t25377: F, t25385: F, t26563: F, t26744: F, t28256: F, t28459: F, t29106: F, t4314: F, t606: F, t7110: F, t7114: F, t7845: F, t97950: F, t97953: F, t97985: F, t98015: F, t98034: F, t98075: F, t100562: F, t16662: F, t16944: F, t16949: F, t17109: F, t24344: F, t25365: F, t25374: F, t4119: F, t5527: F, t5544: F, t5664: F, t67128: F, t67164: F, t84800: F, t93000: F, t98007: F, t98011: F, t98030: F, t100572: F, t101226: F, t1484: F, t1530: F, t16596: F, t202: F, t26740: F, t28248: F, t29125: F, t4255: F, t4303: F, t46341: F, t5660: F, t67123: F, t776: F, t84766: F, t868: F, t92276: F, t97999: F, t98003: F, t98102: F, t265: F, t394: F, t101209: F, t101248: F, t101283: F, t1409: F, t16558: F, t2064: F, t26807: F, t29149: F, t3966: F, t40: F, t5398: F, t607: F, t7131: F, t7865: F, dens_threshold: F, rho0: F, zeta_threshold: F, t100638: F, t100641: F, t100644: F, t100656: F, t100669: F, t100696: F, t100731: F, t100769: F, t100791: F, t1081: F, t25928: F, t25930: F, t26756: F, t28: F, t28771: F, t84797: F, t100651: F, t100682: F, t100692: F, t100713: F, t100743: F, t101196: F, t101211: F, t101220: F, t101241: F, t1649: F, t25892: F, t25905: F, t25921: F, t28774: F, t28778: F, t7649: F, t92319: F, t100664: F, t100705: F, t100708: F, t100766: F, t100788: F, t18196: F, t25898: F, t25901: F, t25938: F, t28795: F, t29157: F, t6841: F, t6848: F, t7656: F, t100646: F, t100659: F, t100689: F, t100718: F, t100734: F, t100747: F, t100759: F, t100772: F, t100780: F, t25934: F, t25945: F, t28764: F, t28789: F, t28792: F, t5966: F, t504: F, t2071: F, t26862: F, t29189: F, t52: F, t7150: F, t7884: F, rho1: F, t101138: F, t101150: F, t113: F, t1442: F, t15868: F, t19451: F, t1983: F, t22574: F, t24175: F, t26161: F, t26163: F, t26558: F, t26559: F, t26870: F, t26902: F, t26906: F, t26974: F, t28821: F, t28834: F, t28969: F, t29197: F, t29377: F, t29378: F, t5107: F, t650: F, t6876: F, t6879: F, t6999: F, t7050: F, t7218: F, t7685: F, t7787: F, t7940: F, t91655: F, t92169: F, t96797: F, t97875: F, t97894: F, t26959: F, t7428: F, t27979: F, t7032: F, t1860: F, t27956: F, t7031: F, t91890: F, t91894: F, t91896: F, t91898: F, t91900: F, t91904: F, t91905: F, t91913: F, t91921: F, t2031: F, t96461: F, t96469: F, t22549: F, t23963: F, t26009: F, t26016: F, t26954: F, t34125: F, t84216: F, t84229: F, t90101: F, t90104: F, t91922: F, t92040: F, t92052: F, t9239: F, t96418: F, t96458: F, t96466: F) -> (F, F, F) {
        let (t101833, t101840, t101843) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1989::<F>(t101832, t870, t193, t7859, t16557, t1877, t2057, t24191, t24339, t25, t25024, t2522, t25375, t25377, t25385, t26563, t26744, t28256, t28459, t29106, t4314, t606, t7110, t7114, t7845, t97950, t97953, t97985, t98015, t98034, t98075);
        let t101892 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1990::<F>(t100562, t16662, t16944, t16949, t17109, t1877, t2057, t24344, t2522, t25365, t25374, t26563, t26744, t4119, t4314, t5527, t5544, t5664, t67128, t67164, t7110, t7114, t7845, t84800, t93000, t98007, t98011, t98030);
        let t101937 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1991::<F>(t100572, t101226, t101832, t1484, t1530, t16596, t1877, t193, t202, t24191, t24339, t24344, t2522, t26740, t26744, t28248, t29106, t29125, t4255, t4303, t4314, t46341, t5660, t67123, t7114, t776, t7845, t84766, t868, t870, t92276, t97999, t98003, t98102);
        let (t101938, t101951) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1992::<F>(t25, t265, t394, t101892, t101937, t101209, t101248, t101283, t101843, t1409, t16558, t2064, t26807, t29149, t3966, t40, t5398, t607, t7131, t7865, dens_threshold, rho0, zeta_threshold);
        let t101981 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1993::<F>(t100638, t100641, t100644, t100656, t100669, t100696, t100731, t100769, t100791, t101833, t101840, t1081, t1877, t24191, t24344, t25928, t25930, t26563, t26744, t26756, t28, t28771, t29106, t7114, t84797);
        let t102012 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1994::<F>(t100651, t100682, t100692, t100713, t100743, t101196, t101211, t101220, t101241, t1649, t1877, t2057, t24191, t2522, t25892, t25905, t25921, t26563, t26740, t26756, t28774, t28778, t7110, t7649, t7845, t92319);
        let t102048 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1995::<F>(t100664, t100705, t100708, t100766, t100788, t101226, t18196, t1877, t2057, t24191, t24339, t2522, t25898, t25901, t25938, t26563, t28795, t29106, t29157, t46341, t6841, t6848, t7656, t7845, t92276, t92319);
        let t102087 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1996::<F>(t100646, t100659, t100689, t100718, t100734, t100747, t100759, t100772, t100780, t1877, t2057, t24191, t24339, t2522, t25934, t25945, t26744, t26756, t28764, t28789, t28792, t4314, t5966, t7110, t7114, t84800);
        let t102102 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1997::<F>(t28, t265, t504, t101938, t101981, t102012, t102048, t102087, t1409, t16558, t2071, t26862, t29189, t3966, t52, t5398, t607, t7150, t7884, dens_threshold, rho1, zeta_threshold);
        let t102105 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1998::<F>(t101138, t101150, t101951, t102102, t113, t1442, t15868, t19451, t1983, t22574, t24175, t26161, t26163, t26558, t26559, t26870, t26902, t26906, t26974, t28821, t28834, t28969, t29197, t29377, t29378, t5107, t650, t6876, t6879, t6999, t7050, t7218, t7685, t7787, t7940, t91655, t92169, t96797, t97875, t97894);
        let t102145 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1999::<F>(t26959, t7428, t27979, t7032, t1860, t27956, t7031, t91890, t91894, t91896, t91898, t91900, t91904, t91905, t91913, t91921);
        let t102171 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2000::<F>(t2031, t96461, t96469, t22549, t23963, t26009, t26016, t26954, t34125, t84216, t84229, t90101, t90104, t91922, t92040, t92052, t9239, t96418, t96458, t96466);
    (t102105, t102145, t102171)
}
