//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta51 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk326;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk327;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta51<F: Float>(t500: F, t111: F, t88: F, t522: F, t588: F, t592: F, t521: F, t750: F, t17: F, t67: F, t758: F, t172: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t1256, t1268, t1274, t1276, t1287) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk326::<F>(t500, t111, t88, t522, t588, t592, t521, t750);
        let (t1288, t1291, t1293, t1294) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk327::<F>(t1287, t17, t521, t67, t758, t172);
    (t1256, t1268, t1274, t1276, t1287, t1288, t1291, t1293, t1294)
}
