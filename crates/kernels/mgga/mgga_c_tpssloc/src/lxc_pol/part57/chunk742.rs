//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 742/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk742<F: Float>(t25144: F, t1509: F, t2047: F, t7823: F, t814: F, t25293: F, t25317: F, t225: F, t7824: F, t25211: F, t7815: F, t25346: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26646 = F::new(7.0) / F::new(1152.0) * t25144;
    let t26656 = t2047 * t1509;
    let t26661 = t814 * t7823;
    let t26667 = F::new(0.38381794893125283518e-1) * t25293;
    let t26673 = F::new(0.16449340668482264365e-1) * t25317;
    let t26700 = t7824 * t225;
    let t26712 = F::new(0.38381794893125283518e-1) * t25211;
    let t26713 = t7815 * t225;
    let t26726 = F::new(0.16449340668482264365e-1) * t25346;
    (t26646, t26656, t26661, t26667, t26673, t26700, t26712, t26713, t26726)
}
