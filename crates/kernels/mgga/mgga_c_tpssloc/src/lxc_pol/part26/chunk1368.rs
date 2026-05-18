//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1368/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1368<F: Float>(t461: F, t607: F, t1009: F, t7324: F, t24722: F, t24658: F, t27635: F, t24663: F, t3503: F, t1210: F, t24669: F, t1222: F, t24677: F) -> (F, F, F, F) {
    let t86259 = t607 * t461;
    let t86261 = t7324 * t86259 * t1009;
    let t86262 = t86261 * t24722;
    let t86264 = t24658 * t27635;
    let t86266 = t86264 * t3503 * t24663;
    let t86269 = t86264 * t1210 * t24669;
    let t86273 = t24677 * t1222;
    (t86262, t86266, t86269, t86273)
}
