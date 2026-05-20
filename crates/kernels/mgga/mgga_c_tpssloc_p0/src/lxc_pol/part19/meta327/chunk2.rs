//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1164/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1164<F: Float>(t12392: F, t3799: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39306: F, t39309: F, t39312: F, t39316: F, t39320: F, t39324: F, t39327: F) -> (F, F) {
    let t40206 = t3799 * t12392;
    let t40210 = -t39249 - t39256 - t39261 - t39266 - t39304 + t39306 - t39309 + t39312 + t39316 + t39320 - t39324 + t39327;
    (t40206, t40210)
}
