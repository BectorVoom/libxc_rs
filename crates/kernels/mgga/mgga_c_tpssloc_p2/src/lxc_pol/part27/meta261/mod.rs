//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1259;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1260;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta261<F: Float>(t1437: F, t79: F, t72: F, t1410: F, t605: F, t1409: F, t6500: F, t6503: F, t67: F, t1864: F, t1433: F, t71: F, t1863: F, t5: F, t1860: F, t1865: F, t6490: F, t7428: F, t112: F, t1874: F, t4028: F, t1458: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t7431, t7432, t7435, t7440, t7441, t7442) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1259::<F>(t1437, t79, t72, t1410, t605, t1409, t6500, t6503, t67, t1864);
        let (t7445, t7446) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1260::<F>(t1433, t71, t1863);
        let (t7450, t7451, t7457, t7458) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1261::<F>(t5, t1860, t1865, t6490, t7428, t7432, t7435, t7442, t7446, t112, t1874, t4028, t1458, t89);
    (t7431, t7432, t7435, t7440, t7441, t7442, t7445, t7446, t7450, t7451, t7457, t7458)
}
