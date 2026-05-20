//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk755;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk756;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk757;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk758;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta133<F: Float>(t3030: F, t466: F, t3032: F, t1208: F, t476: F, t478: F, t3036: F, t483: F, t475: F, t1210: F, t121: F, t1229: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3499, t3500, t3502, t3503, t3504, t3505, t3506) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk755::<F>(t3030, t466, t3032, t1208, t476, t478, t3036, t483);
        let t3508 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk756::<F>(t475);
        let (t3514, t3515) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk757::<F>(t1210, t3504, t3500);
        let t3521 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk758::<F>(t121, t1229);
    (t3499, t3500, t3502, t3503, t3505, t3506, t3508, t3514, t3515, t3521)
}
