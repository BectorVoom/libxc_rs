//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2149;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2150;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2151;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta637<F: Float>(t23133: F, t4257: F, t1496: F, t81942: F, t7497: F, t81933: F, t25098: F, t81835: F, t13228: F, t2628: F, t2678: F, t6605: F, t23097: F, t4234: F, t776: F, t815: F, t81877: F, t81883: F, t13176: F, t6620: F, t849: F, t81857: F, t81859: F, t81874: F, t87287: F, t87289: F, t87292: F, t87293: F, t87296: F, t87298: F, t25097: F, t81782: F, t81783: F, t1516: F, t81769: F, t4261: F, t25111: F, t25115: F, t87229: F, t23132: F, t4166: F, t25068: F, t2707: F, t81763: F, t23083: F, t25094: F, t1510: F, t2379: F, t25119: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87301, t87304, t87306, t87308, t87312) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2149::<F>(t23133, t4257, t1496, t81942, t7497, t81933, t25098, t81835, t13228, t2628, t2678, t6605);
        let t87324 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2150::<F>(t23097, t4234, t776, t815, t81877, t81883, t13176, t6620, t849, t81857, t81859, t81874, t87287, t87289, t87292, t87293, t87296, t87298, t87301, t87304, t87306, t87308, t87312);
        let (t87329, t87331, t87333, t87336, t87339, t87340) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2151::<F>(t25097, t81782, t81783, t1516, t81769, t23133, t4261, t25111, t25115, t87229, t23132, t4166);
        let (t87342, t87343, t87345, t87348, t87351) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2152::<F>(t849, t87340, t25068, t2707, t1516, t81763, t23083, t25094, t1510, t2379, t25119, t815);
    (t87324, t87329, t87331, t87333, t87336, t87339, t87342, t87343, t87345, t87348, t87351)
}
