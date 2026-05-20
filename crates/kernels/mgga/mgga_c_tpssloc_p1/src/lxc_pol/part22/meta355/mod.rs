//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1573;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1574;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1575;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1576;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1577;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1578;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta355<F: Float>(t10143: F, t5664: F, t12895: F, t13121: F, t1484: F, t16697: F, t16699: F, t16700: F, t16703: F, t16705: F, t16707: F, t16708: F, t16709: F, t16712: F, t16715: F, t16719: F, t1877: F, t193: F, t2522: F, t262: F, t5527: F, t776: F, t868: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F, t16610: F, t16666: F, t17119: F, t1534: F, t2: F, t584: F, t5678: F, t690: F, t10216: F, t5392: F, t607: F, t10564: F, t123: F, t10277: F, t2768: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17120, t17131) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1573::<F>(t10143, t5664, t12895, t13121, t1484, t16697, t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16712, t16715, t16719, t1877, t193, t2522, t262, t5527, t776, t868, t9853, t9859, t9894, t9907, t9921);
        let t17133 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1574::<F>(t16610, t16666, t17119, t17131);
        let (t17141, t17149) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1575::<F>(t1534, t2, t584, t5678, t690);
        let (t17151, t17152) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1576::<F>(t10216, t5392, t607);
        let (t17153, t17154) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1577::<F>(t10564, t17152, t123);
        let (t17156, t17157) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1578::<F>(t10277, t5392, t607);
        let (t17158, t17159) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1579::<F>(t17157, t2768, t123);
    (t17120, t17133, t17141, t17149, t17151, t17152, t17153, t17154, t17156, t17157, t17158, t17159)
}
