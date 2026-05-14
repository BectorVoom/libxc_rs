//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1210/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1210<F: Float>(t29827: F, t3640: F, t2109: F, t96461: F, t96469: F, t96425: F, t26012: F, t7974: F, t5415: F, t55: F, t2108: F, t2240: F, t5392: F, t1409: F, t605: F, t55921: F, t7245: F) -> (F, F, F, F, F, F, F, F, F) {
    let t104677 = t29827 * t3640;
    let t104735 = t2109 * t96461;
    let t104740 = t2109 * t96469;
    let t104749 = t2109 * t96425;
    let t104787 = t7974 * t26012;
    let t104818 = t5415 * t55;
    let t104907 = t2240 * t5392 * t2108;
    let t104911 = t605 * t1409 * t2108;
    let t104953 = t55921 * t7245;
    (t104677, t104735, t104740, t104749, t104787, t104818, t104907, t104911, t104953)
}
