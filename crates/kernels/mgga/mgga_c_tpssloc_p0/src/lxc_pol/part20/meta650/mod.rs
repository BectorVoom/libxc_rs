//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2390;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2391;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2392;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2393;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta650<F: Float>(t13634: F, t49039: F, t13615: F, t2798: F, t896: F, t2815: F, t13623: F, t2807: F, t10588: F, t4378: F, t13629: F, t48981: F, t894: F, t42086: F, t42087: F, t47781: F, t47785: F, t47787: F, t48907: F, t48920: F, t48934: F, t48990: F, t49004: F, t49026: F, t49042: F, t893: F, t913: F, t14388: F, t2836: F, t2842: F, t10704: F, t4395: F, t10702: F, t2793: F, t10524: F, t10603: F, t10717: F, t10724: F, t10734: F, t10756: F, t10765: F, t14271: F, t14276: F, t14337: F, t14369: F, t14459: F, t14466: F, t1580: F, t2906: F, t2924: F, t2930: F, t41826: F, t41981: F, t42111: F, t42113: F, t42123: F, t4416: F, t4438: F, t4475: F, t48883: F, t48890: F, t950: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49043, t49049, t49052, t49054, t49056, t49058, t49060) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2390::<F>(t13634, t49039, t13615, t2798, t896, t2815, t13623, t2807, t10588, t4378, t13629, t48981, t894);
        let t49062 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2391::<F>(t42086, t42087, t47781, t47785, t47787, t49043, t49049, t49052, t49054, t49056, t49058, t49060);
        let (t49068, t49071) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2392::<F>(t48907, t48920, t48934, t48990, t49004, t49026, t49042, t49062, t893, t913, t14388, t2836, t2842);
        let (t49075, t49076) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2393::<F>(t10704, t4395, t10702, t2793, t10524, t10603, t10717, t10724, t10734, t10756, t10765, t14271, t14276, t14337, t14369, t14459, t14466, t1580, t2906, t2924, t2930, t41826, t41981, t42111, t42113, t42123, t4416, t4438, t4475, t48883, t48890, t49068, t49071, t950);
    (t49043, t49049, t49052, t49054, t49056, t49058, t49060, t49068, t49071, t49075, t49076)
}
