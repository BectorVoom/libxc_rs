//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1068/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1068<F: Float>(t7696: F, t794: F, t6897: F, t225: F, t7704: F, t25049: F, t25277: F, t25077: F, t25080: F, t25140: F, t25144: F, t25293: F, t25317: F, t25211: F, t25346: F, t26198: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26477 = t7704 * t225;
    let t26591 = 0.38381794893125283518e-1 * t25049;
    let t26613 = 0.38381794893125283518e-1 * t25277;
    let t26619 = 7.0 / 288.0 * t25077;
    let t26621 = 7.0 / 1152.0 * t25080;
    let t26644 = 7.0 / 72.0 * t25140;
    let t26646 = 7.0 / 1152.0 * t25144;
    let t26667 = 0.38381794893125283518e-1 * t25293;
    let t26673 = 0.16449340668482264365e-1 * t25317;
    let t26712 = 0.38381794893125283518e-1 * t25211;
    let t26726 = 0.16449340668482264365e-1 * t25346;
    let t26988 = 0.16449340668482264365e-1 * t26198;
    (t26474, t26475, t26477, t26591, t26613, t26619, t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988)
}
