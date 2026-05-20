//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta161 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk874;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta161<F: Float>(t3507: F, t3508: F, t1214: F, t248: F, t1210: F, t3504: F, t3500: F, t475: F, t121: F, t1229: F) -> (F, F, F, F, F, F, F) {
        let (t3509, t3511, t3514, t3515) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk874::<F>(t3507, t3508, t1214, t248, t1210, t3504, t3500);
        let (t3516, t3518, t3521) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk875::<F>(t3507, t475, t1214, t248, t121, t1229);
    (t3509, t3511, t3514, t3515, t3516, t3518, t3521)
}
