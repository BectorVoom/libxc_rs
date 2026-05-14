//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 226/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk226<F: Float>(t19: F, t598: F, t582: F, t586: F, t589: F, t593: F, t596: F, t83: F, t85: F, t24: F, t4: F, t581: F) -> (F, F, F, F) {
    let t600 = 0.1356e2 * t19 * t598;
    let t601 = t582 - t586 + t589 - t593 + t596 - t600;
    let t604 = 1.0 / t85 / t83;
    let t605 = t24 * t604;
    let t606 = t4 - t581;
    (t601, t604, t605, t606)
}
