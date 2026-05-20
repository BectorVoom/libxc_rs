//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1280;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1281;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta276<F: Float>(t3941: F, t7769: F, t1401: F, t7467: F, t1409: F, t1419: F, t56: F, t6503: F, t7251: F, t67: F, t1864: F, t2109: F, t7445: F, t5: F, t1860: F, t2110: F, t7246: F, t7428: F, t7432: F, t7435: F, t112: F, t1458: F, t2165: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7771, t7773, t7973, t7974, t7975, t7978) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1280::<F>(t3941, t7769, t1401, t7467, t1409, t1419, t56, t6503, t7251, t67, t1864, t2109, t7445);
        let (t7982, t7983, t7989) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1281::<F>(t5, t1860, t2110, t7246, t7428, t7432, t7435, t7975, t7978, t112, t1458, t2165);
    (t7771, t7773, t7973, t7974, t7975, t7978, t7982, t7983, t7989)
}
