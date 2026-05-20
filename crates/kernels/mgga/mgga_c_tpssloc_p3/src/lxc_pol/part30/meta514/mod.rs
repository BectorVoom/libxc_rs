//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1841;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta514<F: Float>(t25: F, t1409: F, t1965: F, t25398: F, t25883: F, t3966: F, t40: F, t607: F, t6835: F, t7643: F, t28: F, t870: F, t4255: F, dens_threshold: F, rho0: F, zeta_threshold: F, t16596: F, t23788: F, t1081: F, t1484: F, t4119: F, t25365: F, t10143: F) -> (F, F, F, F, F, F, F, F) {
        let (t25890, t25891, t25892) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1841::<F>(t25, t1409, t1965, t25398, t25883, t3966, t40, t607, t6835, t7643, t28, t870, t4255, dens_threshold, rho0, zeta_threshold);
        let (t25898, t25901, t25905, t25921, t25927) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1842::<F>(t16596, t23788, t1081, t1484, t28, t4119, t25365, t10143);
    (t25890, t25891, t25892, t25898, t25901, t25905, t25921, t25927)
}
