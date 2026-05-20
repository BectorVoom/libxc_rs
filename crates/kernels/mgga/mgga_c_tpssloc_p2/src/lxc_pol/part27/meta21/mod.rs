//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta21 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk158;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk159;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk160;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk161;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk162;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk163;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta21<F: Float>(t120: F, t61: F, t283: F, t374: F, t339: F, t350: F, t370: F, t349: F, t362: F, t68: F, t353: F, t254: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t375, t376) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk158::<F>(t120, t61, t283);
        let t378 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk159::<F>(t374, t375, t376);
        let t381 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk160::<F>(t339, t350, t370, t378);
        let (t382, t383) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk161::<F>(t349, t381, t362, t68);
        let t384 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk162::<F>(t381, t383);
        let (t386, t387) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk163::<F>(t353, t384);
        let t388 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk164::<F>(t254, t387);
    (t375, t376, t378, t381, t382, t383, t384, t386, t387, t388)
}
