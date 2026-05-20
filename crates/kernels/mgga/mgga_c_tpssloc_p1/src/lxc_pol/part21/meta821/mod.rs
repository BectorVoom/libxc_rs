//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta821 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2886;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2887;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta821<F: Float>(t17271: F, t2815: F, t896: F, t17210: F, t2807: F, t13615: F, t4362: F, t17215: F, t17218: F, t17255: F, t699: F, t136: F, t59730: F, t908: F, t59698: F, t60243: F, t60245: F, t60248: F, t60251: F, t60254: F, t60257: F, t60260: F, t59696: F, t2826: F, t59742: F, t47787: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t60263, t60265, t60267, t60269, t60271, t60274, t60277) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2886::<F>(t17271, t2815, t896, t17210, t2807, t13615, t4362, t17215, t17218, t17255, t699, t136, t59730, t908);
        let t60279 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2887::<F>(t59698, t60243, t60245, t60248, t60251, t60254, t60257, t60260, t60263, t60265, t60267, t60269, t60271, t60274, t60277);
        let (t60282, t60296, t60300) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2888::<F>(t136, t59696, t908, t2826, t59742, t47787, t59700, t59702, t59704, t59708, t59713, t59717, t59721, t59727, t59732, t59735, t59738, t59744);
    (t60263, t60265, t60267, t60269, t60271, t60274, t60277, t60279, t60282, t60296, t60300)
}
