//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1959/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1959<F: Float>(t4993: F, t7345: F, t5040: F, t7310: F, t27607: F, t460: F, t24682: F, t24658: F, t3: F, t24719: F, t3030: F, t1734: F, t3503: F) -> (F, F, F, F, F, F, F, F) {
    let t27622 = t7345 * t4993;
    let t27626 = t7310 * t5040;
    let t27628 = t27607 * t460;
    let t27629 = t24682 * t27628;
    let t27634 = t24658 * t3;
    let t27635 = t24719 * t3030;
    let t27636 = t27634 * t27635;
    let t27637 = t3503 * t1734;
    (t27622, t27626, t27628, t27629, t27634, t27635, t27636, t27637)
}
