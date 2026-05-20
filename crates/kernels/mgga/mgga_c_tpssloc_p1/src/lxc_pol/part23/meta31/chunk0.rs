//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 226/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk226<F: Float>(t19: F, t598: F, t83: F, t85: F, t24: F, t583: F, t61: F) -> (F, F, F, F) {
    let t600 = F::new(0.1356e2) * t19 * t598;
    let t604 = F::new(1.0) / t85 / t83;
    let t605 = t24 * t604;
    let t625 = F::new(1.0) / t61 / t583;
    (t600, t604, t605, t625)
}
