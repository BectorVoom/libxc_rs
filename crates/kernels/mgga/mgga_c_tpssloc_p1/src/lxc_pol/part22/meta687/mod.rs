//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2264;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta687<F: Float>(t2403: F, t6014: F, t6017: F, t18502: F, t699: F, t18499: F, t18509: F, t18507: F, t3356: F, t6031: F, t1128: F, t18668: F) -> (F, F, F, F, F, F, F, F) {
        let (t63893, t63911, t64074, t64076, t64087, t64089, t64103, t64254) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2264::<F>(t2403, t6014, t6017, t18502, t699, t18499, t18509, t18507, t3356, t6031, t1128, t18668);
    (t63893, t63911, t64074, t64076, t64087, t64089, t64103, t64254)
}
