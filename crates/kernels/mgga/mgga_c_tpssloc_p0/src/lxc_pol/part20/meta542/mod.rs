//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta542 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2083;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta542<F: Float>(t40961: F, t849: F, t10021: F, t812: F, t841: F, t23076: F, t241: F, t67: F, t2707: F, t9601: F, t2697: F, t9997: F) -> (F, F, F, F, F, F) {
        let (t40962, t40965, t40966, t40971, t40982, t40984) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2083::<F>(t40961, t849, t10021, t812, t841, t23076, t241, t67, t2707, t9601, t2697, t9997);
    (t40962, t40965, t40966, t40971, t40982, t40984)
}
