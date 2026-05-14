//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 675/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk675<F: Float>(t193: F, t200: F, t2056: F, t25049: F, t25277: F, t25077: F, t25080: F, t25140: F, t25144: F, t1509: F, t2047: F, t7823: F, t814: F, t25293: F, t25317: F, t225: F, t7824: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26563 = t193 * t200 * t2056;
    let t26591 = 0.38381794893125283518e-1 * t25049;
    let t26613 = 0.38381794893125283518e-1 * t25277;
    let t26619 = 7.0 / 288.0 * t25077;
    let t26621 = 7.0 / 1152.0 * t25080;
    let t26644 = 7.0 / 72.0 * t25140;
    let t26646 = 7.0 / 1152.0 * t25144;
    let t26656 = t2047 * t1509;
    let t26661 = t814 * t7823;
    let t26667 = 0.38381794893125283518e-1 * t25293;
    let t26673 = 0.16449340668482264365e-1 * t25317;
    let t26700 = t7824 * t225;
    (t26563, t26591, t26613, t26619, t26621, t26644, t26646, t26656, t26661, t26667, t26673, t26700)
}
