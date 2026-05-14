//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 572/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk572<F: Float>(t3375: F, t440: F, t3236: F, t3293: F, t1146: F) -> (F, F, F, F, F) {
    let t3376 = t440 * t3375;
    let t3383 = 0.40256666666666666667e0 * t3236;
    let t3390 = 0.137975e0 * t3293;
    let t3399 = t1146 * t1146;
    let t3400 = 1.0 / t3399;
    (t3376, t3383, t3390, t3399, t3400)
}
