//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1885/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1885<F: Float>(t23237: F, t25341: F, t6552: F, t23204: F, t25216: F, t6562: F, t1519: F, t212: F, t23171: F, t6554: F, t23270: F, t25038: F, t258: F, t4119: F, t776: F) -> (F, F, F, F) {
    let t87907 = t6552 * t23237 * t25341;
    let t87910 = t6562 * t23204 * t25216;
    let t87915 = t23171 * t212 * t1519 * t6554;
    let t87920 = t25038 * t23270 * t258 * t4119 * t776;
    (t87907, t87910, t87915, t87920)
}
