//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta123 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk682;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk683;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk684;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta123<F: Float>(t1932: F, t3131: F, t1014: F, t3032: F, t3031: F, t360: F, t390: F, t1878: F, t268: F, t405: F, t1091: F, t690: F, t1229: F, t154: F, t636: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3188, t3199, t3200) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk682::<F>(t1932, t3131, t1014, t3032, t3031);
        let (t3201, t3215, t3216, t3236, t3237, t3238) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk683::<F>(t1932, t360, t390, t1878, t268, t405, t1091, t690);
        let t3240 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk684::<F>(t1229, t154);
        let (t3241, t3242) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk685::<F>(t636);
    (t3188, t3199, t3200, t3201, t3215, t3216, t3236, t3237, t3238, t3240, t3241, t3242)
}
