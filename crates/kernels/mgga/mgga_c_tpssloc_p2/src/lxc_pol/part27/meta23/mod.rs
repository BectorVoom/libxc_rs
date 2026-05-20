//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta23 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk172;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk173;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk174;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk175;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk176;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta23<F: Float>(t407: F, t410: F, t413: F, t417: F, t409: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t419, t422, t423) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk172::<F>(t407, t410, t413, t417);
        let (t425, t427) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk173::<F>(t409, t423, t407);
        let (t432, t435, t436) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk174::<F>(t407, t410, t413, t417);
        let t440 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk175::<F>(t407);
        let (t445, t448, t449) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk176::<F>(t407, t410, t413, t417);
    (t419, t422, t423, t425, t427, t432, t435, t436, t440, t445, t448, t449)
}
