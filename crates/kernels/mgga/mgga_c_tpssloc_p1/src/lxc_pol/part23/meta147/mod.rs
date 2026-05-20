//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta147 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk690;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk691;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk692;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk693;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk694;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk695;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta147<F: Float>(t1055: F, t5943: F, t1052: F, t1635: F, t388: F, t4557: F, t4660: F, t5849: F, t5851: F, t5915: F, t5920: F, t1637: F, t1070: F, t193: F, t3216: F, t336: F, t5691: F, t5693: F, t5697: F, t5729: F, t5732: F, t5798: F, t5800: F, t5802: F, t5806: F, t5810: F, t5814: F, t25: F, t265: F, t394: F, t5669: F, t1408: F, t1409: F, t1534: F, t1642: F, t396: F, t40: F, t5397: F, t5398: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3242: F, t5392: F, t3240: F, t123: F, t3247: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5944, t5946, t5950) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk690::<F>(t1055, t5943, t1052, t1635, t388, t4557, t4660, t5849, t5851, t5915, t5920, t1637);
        let t5954 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk691::<F>(t1070, t193, t3216, t336, t5691, t5693, t5697, t5729, t5732, t5798, t5800, t5802, t5806, t5810, t5814, t5946, t5950);
        let (t5955, t5962) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk692::<F>(t25, t265, t394, t5669, t5954, t1408, t1409, t1534, t1642, t396, t40, t5397, t5398, dens_threshold, rho0, zeta_threshold);
        let t5966 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk693::<F>(t5397);
        let t5971 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk694::<F>(t3242, t5392);
        let (t5972, t5973, t5975) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk695::<F>(t3240, t5971, t123, t3247, t5392);
    (t5944, t5946, t5950, t5955, t5962, t5966, t5971, t5972, t5973, t5975)
}
