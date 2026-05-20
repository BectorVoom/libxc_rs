//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta531 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2004;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta531<F: Float>(t2509: F, t2512: F, t745: F, t9711: F, t1294: F, t2504: F, t9493: F, t2369: F, t9489: F, t116: F, t4: F, t126: F, t268: F, t8705: F) -> (F, F, F, F, F, F, F) {
        let (t39259, t39261, t39263, t39264, t39266, t39267, t39273) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2004::<F>(t2509, t2512, t745, t9711, t1294, t2504, t9493, t2369, t9489, t116, t4, t126, t268, t8705);
    (t39259, t39261, t39263, t39264, t39266, t39267, t39273)
}
