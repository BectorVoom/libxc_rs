//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta849 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3072;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3073;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3074;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3075;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3076;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta849<F: Float>(t1671: F, t51397: F, t18786: F, t3371: F, t63717: F, t63720: F, t63722: F, t63725: F, t63729: F, t63731: F, t63733: F, t63735: F, t63737: F, t63739: F, t63741: F, t63743: F, t63745: F, t63747: F, t63752: F, t63754: F, t63757: F, t14845: F, t4782: F, t14914: F, t4740: F, t44159: F, t5989: F, t11180: F, t6021: F, t18835: F, t3259: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63323: F, t43748: F, t50903: F, t50905: F, t50907: F, t50919: F, t50921: F, t50948: F, t50950: F, t63327: F, t63330: F, t63332: F, t63334: F, t63336: F, t43780: F, t43782: F, t43816: F, t44348: F, t50952: F, t50954: F, t63355: F, t63359: F, t63361: F, t63365: F, t63370: F, t63374: F, t63380: F, t63382: F, t63384: F, t63388: F, t63392: F, t63396: F, t63398: F, t63400: F, t63404: F, t63408: F, t63412: F, t63417: F, t63422: F, t423: F, t18496: F, t699: F, t18517: F, t18514: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63759, t63760) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3072::<F>(t1671, t51397, t18786, t3371, t63717, t63720, t63722, t63725, t63729, t63731, t63733, t63735, t63737, t63739, t63741, t63743, t63745, t63747, t63752, t63754, t63757);
        let (t63763, t63765, t63767, t63769, t63771, t63784) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3073::<F>(t14845, t4782, t14914, t4740, t44159, t5989, t11180, t6021, t18835, t3259, t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317, t63323);
        let t63798 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3074::<F>(t43748, t50903, t50905, t50907, t50919, t50921, t50948, t50950, t63327, t63330, t63332, t63334, t63336);
        let t63811 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3075::<F>(t43780, t43782, t43816, t44348, t50952, t50954, t63355, t63359, t63361, t63365, t63370, t63374);
        let t63825 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3076::<F>(t63380, t63382, t63384, t63388, t63392, t63396, t63398, t63400, t63404, t63408, t63412, t63417, t63422);
        let (t63829, t63841, t63843, t63845, t63847) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3077::<F>(t423, t63784, t63798, t63811, t63825, t18496, t699, t18517, t18514, t50826, t50828, t50834, t63291, t63296, t63300, t63304, t63306, t63308, t63313, t63317);
    (t63759, t63760, t63763, t63765, t63767, t63769, t63771, t63829, t63841, t63843, t63845, t63847)
}
