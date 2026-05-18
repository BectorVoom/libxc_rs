//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1179/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1179<F: Float>(t1527: F, t23270: F, t25038: F, t98224: F, t1484: F, t5664: F, t25373: F, t5397: F, t1408: F, t5544: F, t5660: F, t22960: F) -> (F, F, F, F, F, F, F) {
    let t105698 = t25038 * t23270 * t98224 * t1527;
    let t105731 = t1484 * t5664;
    let t105732 = t25373 * t105731;
    let t105741 = t5397 * t1484;
    let t105745 = t1408 * t5544;
    let t105754 = t1484 * t5660;
    let t105755 = t22960 * t105754;
    (t105698, t105731, t105732, t105741, t105745, t105754, t105755)
}
