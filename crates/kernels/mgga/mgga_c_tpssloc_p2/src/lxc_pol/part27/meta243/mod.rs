//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta243 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1166;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1167;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1168;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta243<F: Float>(t1911: F, t865: F, t2718: F, t1906: F, t6547: F, t214: F, t225: F, t234: F, t252: F, t776: F, t6552: F, t1905: F, t794: F, t6562: F, t6604: F, t814: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t6632 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1166::<F>(t1911, t865, t2718);
        let (t6636, t6637) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1167::<F>(t1906, t6547, t214, t225);
        let t6638 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1168::<F>(t234, t252);
        let (t6639, t6640, t6641, t6643, t6645, t6646) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1169::<F>(t6638, t776, t6637, t6552, t1905, t794, t6562, t6604, t814);
    (t6632, t6636, t6637, t6638, t6639, t6640, t6641, t6643, t6645, t6646)
}
