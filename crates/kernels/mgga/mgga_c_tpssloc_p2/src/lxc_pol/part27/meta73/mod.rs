//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta73 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk498;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk499;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk500;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk501;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta73<F: Float>(t1409: F, t55: F, t1414: F, t1420: F, t39: F, t51: F, t56: F, t627: F, t33: F, t634: F, t638: F, t72: F, t1411: F, t66: F, t80: F, t5: F, t1406: F, t605: F, t86: F, t112: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t1426, t1427, t1430, t1431, t1433) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk498::<F>(t1409, t55, t1414, t1420, t39, t51, t56, t627, t33, t634, t638);
        let (t1434, t1437) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk499::<F>(t1433, t72, t1411, t1427, t66, t80);
        let t1441 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk500::<F>(t5, t1406, t1437, t605, t86);
        let t1442 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk501::<F>(t112, t1441);
    (t1426, t1427, t1430, t1431, t1433, t1434, t1437, t1441, t1442)
}
