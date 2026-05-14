//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1138/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1138<F: Float>(t39549: F, t40790: F, t40793: F, t40795: F, t40797: F, t40799: F, t40801: F, t40803: F, t40805: F, t40807: F, t40809: F, t40811: F, t2427: F, t9909: F, t39568: F, t761: F) -> (F, F, F) {
    let t41249 = t40790 + t40793 + t40795 + t40797 + t40799 + t40801 - t40803 - t40805 + t40807 + t40809 + t40811 + t39549;
    let t41251 = t2427 * t9909;
    let t41252 = 48.0 * t41251;
    let t41254 = 0.14035736694323150897e2 * t761 * t39568;
    (t41249, t41252, t41254)
}
