//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta755 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2537;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta755<F: Float>(t71371: F, t71389: F, t1107: F, t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63893: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t136: F, t43761: F, t71164: F, t1100: F, t1113: F, t71148: F, t21794: F, t699: F, t11219: F, t71158: F, t71133: F) -> (F, F, F, F, F, F, F, F) {
        let (t71390, t71391, t71396) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2537::<F>(t71371, t71389, t1107, t63332, t63334, t63336, t63886, t63888, t63893, t71124, t71130, t71135, t71140, t71142);
        let (t71400, t71403, t71406, t71408, t71411, t71414) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2538::<F>(t136, t43761, t71164, t1100, t71390, t1113, t71148, t21794, t699, t11219, t71158, t71133);
    (t71391, t71396, t71400, t71403, t71406, t71408, t71411, t71414)
}
