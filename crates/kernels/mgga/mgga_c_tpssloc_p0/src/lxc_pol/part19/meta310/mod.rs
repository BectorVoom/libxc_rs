//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta310 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta310<F: Float>(t268: F, t521: F, t9799: F, t9847: F, t677: F, t9494: F, t3684: F, t12110: F, t9885: F, t12099: F, t2663: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39306: F, t39309: F, t39312: F, t39316: F, t39320: F) -> (F, F, F, F, F, F, F, F) {
        let (t39321, t39322, t39324, t39325, t39327, t39329, t39331, t39332) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1110::<F>(t268, t521, t9799, t9847, t677, t9494, t3684, t12110, t9885, t12099, t2663, t39249, t39256, t39261, t39266, t39304, t39306, t39309, t39312, t39316, t39320);
    (t39321, t39322, t39324, t39325, t39327, t39329, t39331, t39332)
}
