//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 682/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk682<F: Float>(t605: F, t608: F, t38: F, t43: F, t625: F, t44: F, t607: F, t614: F, t67: F, t1864: F) -> (F, F, F, F, F) {
    let t6495 = t605 * t608;
    let t6500 = t38 * t43;
    let t6503 = 8.0 / 3.0 * t625;
    let t6504 = -8.0 / 3.0 * t614 * t44 + 5.0 / 6.0 * t6500 * t607 + t6503;
    let t6505 = t6504 * t67;
    let t6506 = t6505 * t1864;
    (t6495, t6500, t6504, t6505, t6506)
}
