//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1858;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1859;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta501<F: Float>(t25084: F, t4184: F, t23146: F, t4191: F, t4240: F, t4250: F, t13228: F, t828: F, t2628: F, t6605: F, t13351: F, t232: F, t815: F, t23097: F, t23096: F, t23106: F, t23108: F, t23114: F, t23119: F, t1894: F, t236: F, t4119: F, t6591: F, t23062: F, t7497: F, t1510: F, t776: F, t13223: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25085, t25087, t25089, t25091, t25093, t25094, t25095, t25097) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1858::<F>(t25084, t4184, t23146, t4191, t4240, t4250, t13228, t828, t2628, t6605, t13351, t232);
        let (t25098, t25103) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1859::<F>(t25097, t815, t23097, t23096, t23106, t23108, t23114, t23119, t25085, t25087, t25089, t25091, t25095);
        let (t25106, t25107, t25109, t25111, t25112, t25113, t25115) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1860::<F>(t1894, t236, t4119, t6591, t23062, t7497, t1510, t776, t815, t23097, t13223, t232);
    (t25093, t25094, t25097, t25098, t25103, t25106, t25107, t25109, t25111, t25112, t25113, t25115)
}
