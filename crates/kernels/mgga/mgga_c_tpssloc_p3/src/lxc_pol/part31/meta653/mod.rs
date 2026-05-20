//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta653 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1932;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1933;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta653<F: Float>(t16932: F, t25084: F, t16937: F, t16907: F, t23146: F, t17009: F, t17013: F, t25111: F, t7496: F, t87447: F, t22690: F, t23122: F, t5544: F, t841: F, t23097: F, t5617: F, t776: F, t815: F, t1510: F, t4233: F, t6605: F, t232: F, t58688: F, t5612: F, t1509: F, t4119: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98631, t98633, t98635, t98637, t98639, t98642, t98647) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1932::<F>(t16932, t25084, t16937, t16907, t23146, t17009, t17013, t25111, t7496, t87447, t22690, t23122, t5544, t841);
        let (t98651, t98655, t98659, t98663, t98668) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1933::<F>(t23097, t5617, t776, t815, t1510, t4233, t6605, t232, t58688, t5612, t1509, t4119);
    (t98631, t98633, t98635, t98637, t98639, t98642, t98647, t98651, t98655, t98659, t98663, t98668)
}
