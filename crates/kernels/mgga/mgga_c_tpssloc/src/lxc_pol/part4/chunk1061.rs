//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1061/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1061<F: Float>(t1434: F, t19322: F, t19323: F, t19326: F, t19331: F, t19335: F, t19338: F, t19343: F, t19346: F, t19349: F, t3962: F, t5393: F, t5400: F, t5403: F, t642: F, t80: F) -> (F,) {
    let t19356 = -t19322 * t19323 / 6.0 - t19326 * t80 / 12.0 - t5393 * t642 / 12.0 - t19331 * t80 / 12.0 - t19335 * t80 / 12.0 - t19338 * t80 / 12.0 - t5400 * t642 / 12.0 - t19343 * t80 / 6.0 - t19346 * t80 / 6.0 - t19349 * t80 / 6.0 - t5403 * t642 / 6.0 - t3962 * t1434 / 6.0;
    (t19356,)
}
