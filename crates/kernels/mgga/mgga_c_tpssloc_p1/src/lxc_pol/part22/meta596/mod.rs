//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta596<F: Float>(t2770: F, t340: F, t2403: F, t4389: F, t4386: F, t344: F, t42308: F, t60: F, t10213: F, t134: F, t4509: F, t4540: F) -> (F, F, F, F, F, F, F, F) {
        let (t48143, t48155, t48156, t48157, t48158, t48180, t48213, t48217) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2117::<F>(t2770, t340, t2403, t4389, t4386, t344, t42308, t60, t10213, t134, t4509, t4540);
    (t48143, t48155, t48156, t48157, t48158, t48180, t48213, t48217)
}
