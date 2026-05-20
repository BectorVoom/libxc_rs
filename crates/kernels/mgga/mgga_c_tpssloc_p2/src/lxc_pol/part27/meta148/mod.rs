//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta148 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk838;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk839;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk840;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk841;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta148<F: Float>(t1088: F, t3248: F, t123: F, t1089: F, t2250: F, t3237: F, t3238: F, t3245: F, t423: F, t1094: F, t1098: F, t1119: F, t1097: F, t419: F, t409: F, t1117: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3249, t3250) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk838::<F>(t1088, t3248, t123);
        let t3252 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk839::<F>(t1089, t2250);
        let (t3253, t3254) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk840::<F>(t1088, t3252, t123);
        let (t3256, t3258, t3259, t3261, t3263, t3264) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk841::<F>(t3237, t3238, t3245, t3250, t3254, t423, t1094, t1098, t1119, t1097, t419, t409);
        let t3265 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk842::<F>(t1117);
    (t3249, t3250, t3252, t3253, t3254, t3256, t3258, t3259, t3261, t3263, t3264, t3265)
}
