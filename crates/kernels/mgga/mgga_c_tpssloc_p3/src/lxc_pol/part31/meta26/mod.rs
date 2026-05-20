//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta26 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk184;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk185;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk186;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk187;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk188;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk189;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta26<F: Float>(t470: F, t494: F, t254: F, t492: F, t193: F, t336: F, t425: F, t453: F, t455: F, t265: F, t28: F, t52: F, t399: F, dens_threshold: F, rho1: F, zeta_threshold: F, t112: F, t88: F, t25: F, t148: F, t157: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t496, t498) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk184::<F>(t470, t494, t254);
        let (t500, t506, t504) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk185::<F>(t492, t498, t193, t336, t425, t453, t455, t265);
        let t510 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk186::<F>(t28, t265, t506, t52, t399, dens_threshold, rho1, zeta_threshold);
        let t513 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk187::<F>(t112, t88);
        let t514 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk188::<F>(t25);
        let (t515, t516, t517) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk189::<F>(t25, t514, t148, t28, zeta_threshold);
        let (t518, t521) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk190::<F>(t28, t517, t148, t516, t157, zeta_threshold);
    (t496, t498, t500, t506, t504, t510, t513, t514, t515, t517, t518, t521)
}
