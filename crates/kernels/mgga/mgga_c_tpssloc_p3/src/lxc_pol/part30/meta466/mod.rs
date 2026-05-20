//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta466<F: Float>(t1926: F, t3158: F, t40: F, t6722: F, t1937: F, t6712: F, t995: F, t1942: F, t3082: F, t344: F, t1009: F, t6740: F) -> (F, F, F, F, F, F, F) {
        let (t23447, t23449, t23463, t23469, t23470, t23471, t23472) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1751::<F>(t1926, t3158, t40, t6722, t1937, t6712, t995, t1942, t3082, t344, t1009, t6740);
    (t23447, t23449, t23463, t23469, t23470, t23471, t23472)
}
