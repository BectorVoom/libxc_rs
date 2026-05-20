//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1249;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1250;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta275<F: Float>(t1873: F, t7676: F, t1268: F, t7467: F, t1778: F, t191: F, t192: F, t2020: F, t1390: F, t1799: F, t6878: F, t1983: F, t6890: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t7678, t7680, t7684, t7685) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1249::<F>(t1873, t7676, t1268, t7467, t1778, t191, t192);
        let (t7686, t7687, t7688, t7690, t7691) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1250::<F>(t2020, t7685, t1390, t1799, t6878, t1983, t6890);
    (t7678, t7680, t7684, t7685, t7686, t7687, t7688, t7690, t7691)
}
