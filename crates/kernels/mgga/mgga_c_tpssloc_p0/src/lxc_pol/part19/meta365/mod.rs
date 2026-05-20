//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1330;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta365<F: Float>(t10454: F, t3117: F, t10891: F, t10895: F, t10508: F, t248: F, t3039: F, t3041: F, t3020: F, t3030: F, t3032: F, t3038: F, t10360: F, t1040: F, t1043: F, t204: F, t1041: F, t884: F, t1009: F, t10358: F, t1011: F, t1019: F, t338: F, t39177: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t42729, t42731, t42735, t42741, t42742, t42743) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1330::<F>(t10454, t3117, t10891, t10895, t10508, t248, t3039, t3041, t3020, t3030, t3032, t3038);
        let (t42746, t42752, t42754, t42756, t42759) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1331::<F>(t10360, t1040, t1043, t204, t1041, t248, t884, t1009, t10358, t1011, t1019, t338, t39177);
    (t42729, t42731, t42735, t42741, t42742, t42743, t42746, t42752, t42754, t42756, t42759)
}
