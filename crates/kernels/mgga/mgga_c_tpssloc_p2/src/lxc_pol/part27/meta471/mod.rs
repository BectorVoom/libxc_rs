//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1833;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1834;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta471<F: Float>(t6712: F, t995: F, t1941: F, t3077: F, t1942: F, t3082: F, t344: F, t40: F, t1009: F, t6740: F, t1015: F, t6746: F, t984: F, t1933: F, t225: F, t343: F, t364: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23463, t23465, t23469, t23470, t23471, t23472, t23473) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1833::<F>(t6712, t995, t1941, t3077, t1942, t3082, t344, t40, t1009, t6740, t1015, t6746);
        let (t23474, t23476, t23477, t23478, t23479) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1834::<F>(t23472, t23473, t40, t984, t1933, t225, t343, t364);
    (t23463, t23465, t23469, t23470, t23471, t23472, t23473, t23474, t23476, t23477, t23478, t23479)
}
