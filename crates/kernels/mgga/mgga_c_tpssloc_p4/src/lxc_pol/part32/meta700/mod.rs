//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2193;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2194;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2195;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta700<F: Float>(t28200: F, t6883: F, t225: F, t28053: F, t6888: F, t7691: F, t90739: F, t1375: F, t1386: F, t20025: F, t2016: F, t26224: F, t26225: F, t26366: F, t3887: F, t5210: F, t5354: F, t539: F, t56422: F, t568: F, t6460: F, t6992: F, t7722: F, t81399: F, t93906: F, t97468: F, t12020: F, t1378: F, t1390: F, t16022: F, t16030: F, t16439: F, t1807: F, t1843: F, t19648: F, t1983: F, t20022: F, t20023: F, t20029: F, t20051: F, t20060: F, t2015: F, t26226: F, t26328: F, t26348: F, t26371: F, t28111: F, t28220: F, t28224: F, t3758: F, t3882: F, t5215: F, t5321: F, t5325: F, t5326: F, t533: F, t56434: F, t56580: F, t56596: F, t56607: F, t6958: F, t6963: F, t7729: F, t7749: F, t7750: F, t80711: F, t81267: F, t81282: F, t81318: F, t81375: F, t90512: F, t90515: F, t90521: F, t90585: F, t90687: F, t91441: F, t91488: F, t93335: F, t93368: F, t93387: F, t93446: F, t93899: F, t96885: F, t96893: F, t96896: F, t96900: F, t96917: F, t96920: F, t96925: F, t96929: F, t96960: F, t96999: F, t97032: F, t97075: F, t97116: F, t97154: F, t97196: F, t97496: F, t97503: F, t97519: F, t97524: F, t97527: F, t97529: F, t97552: F, t97607: F, t97611: F, t97616: F, t97619: F, t97624: F, t97626: F, t97666: F, t97717: F, t97724: F, t97729: F, t97732: F, t97740: F, t24987: F, t7756: F, t2314: F, t28025: F, t4034: F, t1266: F, t28017: F, t652: F, t1845: F, t5187: F, t22574: F, t8643: F, t7688: F, t1874: F, t75560: F, t19451: F, t6525: F, t25994: F, t4028: F, t55943: F, t191: F, t192: F, t19537: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t97770 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2193::<F>(t28200, t6883, t225, t28053, t6888, t7691, t90739, t1375, t1386, t20025, t2016, t26224, t26225, t26366, t3887, t5210, t5354, t539, t56422, t568, t6460, t6992, t7722, t81399, t93906, t97468);
        let t97777 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2194::<F>(t12020, t1375, t1378, t1390, t16022, t16030, t16439, t1807, t1843, t19648, t1983, t20022, t20023, t20029, t20051, t20060, t2015, t2016, t26224, t26226, t26328, t26348, t26366, t26371, t28111, t28220, t28224, t3758, t3882, t3887, t5215, t5321, t5325, t5326, t533, t56434, t56580, t56596, t56607, t568, t6958, t6963, t7729, t7749, t7750, t80711, t81267, t81282, t81318, t81375, t90512, t90515, t90521, t90585, t90687, t91441, t91488, t93335, t93368, t93387, t93446, t93899, t96885, t96893, t96896, t96900, t96917, t96920, t96925, t96929, t96960, t96999, t97032, t97075, t97116, t97154, t97196, t97496, t97503, t97519, t97524, t97527, t97529, t97552, t97607, t97611, t97616, t97619, t97624, t97626, t97666, t97717, t97724, t97729, t97732, t97740, t97770);
        let (t97779, t97783, t97785, t97788, t97792) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2195::<F>(t24987, t7756, t2314, t28025, t4034, t1266, t28017, t652, t1845, t5187, t22574, t8643);
        let (t97794, t97796, t97798, t97800, t97802, t97804) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2196::<F>(t24987, t7688, t1874, t75560, t19451, t6525, t25994, t4028, t55943, t191, t192, t19537);
    (t97777, t97779, t97783, t97785, t97788, t97792, t97794, t97796, t97798, t97800, t97802, t97804)
}
