//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1255/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1255<F: Float>(t10969: F, t154: F, t2769: F, t39097: F, t123: F) -> (F, F, F) {
    let t41664 = t154 * t10969;
    let t41665 = t2769 * t2769;
    let t41666 = F::new(1.0) / t41665;
    let t41667 = t41666 * t39097;
    let t41669 = t123 * t41664 * t41667;
    (t41666, t41667, t41669)
}
