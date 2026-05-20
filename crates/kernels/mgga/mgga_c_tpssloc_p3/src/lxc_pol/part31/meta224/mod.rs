//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk961;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk962;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk963;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk964;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta224<F: Float>(t25: F, t265: F, t394: F, t5669: F, t5954: F, t1408: F, t1409: F, t1534: F, t1642: F, t396: F, t40: F, t5397: F, t5398: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3242: F, t5392: F, t3240: F, t123: F, t3247: F, t1088: F, t1089: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5955, t5962) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk961::<F>(t25, t265, t394, t5669, t5954, t1408, t1409, t1534, t1642, t396, t40, t5397, t5398, dens_threshold, rho0, zeta_threshold);
        let t5966 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk962::<F>(t5397);
        let t5971 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk963::<F>(t3242, t5392);
        let (t5972, t5973, t5975) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk964::<F>(t3240, t5971, t123, t3247, t5392);
        let (t5976, t5977, t5979) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk965::<F>(t1088, t5975, t123, t1089, t5398);
    (t5955, t5962, t5966, t5971, t5972, t5973, t5975, t5976, t5977, t5979)
}
