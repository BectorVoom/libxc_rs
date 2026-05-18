//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 901/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk901<F: Float>(t2020: F, t8690: F, t113: F, t1869: F, t1976: F, t2114: F, t2165: F, t510: F, t574: F, t8315: F, t8322: F, t8324: F, t8329: F, t8451: F, t8491: F, t8495: F, t8667: F, t8669: F, t8676: F, t8682: F, t8687: F) -> F {
    let t8691 = t8690 * t2020;
    let t8692 = -t113 * t8682 - t1869 * t2165 - t1976 * t2114 - t510 * t8667 + t574 * t8687 - F::new(2.0) * t8315 - t8322 - F::new(2.0) * t8324 - t8329 + t8451 + t8491 - t8495 - F::new(2.0) * t8669 - F::new(2.0) * t8676 + t8691;
    t8692
}
