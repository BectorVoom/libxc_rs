//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1581;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta318<F: Float>(t11545: F, t974: F, t11147: F, t461: F, t9288: F, t457: F, t63: F, t221: F, t456: F, t1186: F, t698: F, t1174: F) -> (F, F, F, F, F, F, F, F) {
        let (t11546, t11548, t11549, t11552, t11554, t11556, t11557, t11558) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1581::<F>(t11545, t974, t11147, t461, t9288, t457, t63, t221, t456, t1186, t698, t1174);
    (t11546, t11548, t11549, t11552, t11554, t11556, t11557, t11558)
}
