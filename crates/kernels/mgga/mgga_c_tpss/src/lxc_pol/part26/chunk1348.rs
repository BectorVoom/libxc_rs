//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1348/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1348<F: Float>(t1163: F, t22110: F, t22176: F, t25354: F, t3538: F, t485: F, t544: F, t69375: F, t69377: F, t69379: F, t69382: F, t69385: F, t69388: F, t69392: F, t69394: F, t69397: F, t69401: F, t69403: F, t69420: F, t69422: F, t73086: F, t73089: F, t73096: F, t73114: F) -> (F,) {
    let t73117 = -t69375 - t69377 - t69379 - t69382 - t69385 - t69388 - t69392 - t69394 - t69397 - t69401 - t73086 * t485 - t22176 * t1163 - 2.0 * t73089 * t485 - 2.0 * t22110 * t1163 + t69403 - 4.0 * t25354 * t3538 + (t73096 + t73114) * t544 - t69420 + t69422;
    (t73117,)
}
