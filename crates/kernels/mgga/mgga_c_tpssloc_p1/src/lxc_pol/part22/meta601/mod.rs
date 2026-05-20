//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta601<F: Float>(t10523: F, t1573: F, t10629: F, t48096: F, t47730: F, t48155: F, t1556: F, t2842: F, t10828: F, t1580: F, t2841: F, t4351: F) -> (F, F, F, F, F, F, F, F) {
        let (t49099, t49104, t49139, t49144, t49200, t49226, t49263, t49269) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2123::<F>(t10523, t1573, t10629, t48096, t47730, t48155, t1556, t2842, t10828, t1580, t2841, t4351);
    (t49099, t49104, t49139, t49144, t49200, t49226, t49263, t49269)
}
