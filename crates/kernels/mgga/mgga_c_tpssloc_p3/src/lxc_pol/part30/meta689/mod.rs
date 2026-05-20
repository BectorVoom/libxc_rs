//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2195;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2196;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2197;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta689<F: Float>(t24996: F, t97890: F, t28860: F, t6876: F, t1307: F, t6324: F, t22574: F, t26162: F, t28835: F, t28830: F, t24995: F, t8643: F, t1442: F, t1869: F, t19289: F, t25958: F, t33085: F, t4073: F, t6287: F, t6515: F, t672: F, t96686: F, t97862: F, t97865: F, t97869: F, t97871: F, t97874: F, t97878: F, t97880: F, t97887: F, t97889: F, t74060: F, t1388: F, t1983: F, t28238: F, t6999: F, t75214: F, t12461: F, t7752: F, t26161: F, t26163: F, t24991: F, t7685: F, t25988: F, t33136: F, t28823: F, t1874: F, t96709: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t97892, t97893, t97897, t97899, t97905) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2195::<F>(t24996, t97890, t28860, t6876, t1307, t6324, t22574, t26162, t28835, t28830, t24995, t8643);
        let t97906 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2196::<F>(t1442, t1869, t19289, t25958, t33085, t4073, t6287, t6515, t672, t96686, t97862, t97865, t97869, t97871, t97874, t97878, t97880, t97887, t97889, t97892, t97893, t97897, t97899, t97905);
        let (t97910, t97914, t97916, t97919, t97920) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2197::<F>(t22574, t74060, t8643, t1388, t28830, t26162, t1983, t28238, t6999, t75214, t12461, t7752);
        let (t97923, t97925, t97928, t97930, t97932) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2198::<F>(t26161, t26163, t97920, t24991, t7685, t22574, t25988, t33136, t28823, t6876, t1874, t96709);
    (t97906, t97910, t97914, t97916, t97919, t97923, t97925, t97928, t97930, t97932)
}
