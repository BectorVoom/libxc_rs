//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 875/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk875<F: Float>(t12116: F, t12118: F, t12121: F, t12123: F, t12133: F, t12141: F, t20526: F, t20527: F, t20528: F, t20529: F, t20530: F, t20532: F, t3918: F, t5122: F, t6347: F, t9853: F, t9859: F) -> (F,) {
    let t20696 = 9.0 * t3918 * t5122 * t6347 + t12116 + t12118 - t12121 + t12123 + t12133 - t12141 + t20526 + t20527 + t20528 + t20529 + t20530 + t20532 + t9853 + t9859;
    (t20696,)
}
