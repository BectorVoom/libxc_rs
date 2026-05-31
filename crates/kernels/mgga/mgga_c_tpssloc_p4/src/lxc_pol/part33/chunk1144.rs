//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1144/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1144<F: Float>(t25277: F, t25077: F, t25080: F, t25140: F, t25144: F, t25293: F, t25317: F, t25211: F, t25346: F, t26198: F, t26200: F, t26231: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26613 = F::cast_from(0.38381794893125283518e-1_f64) * t25277;
    let t26619 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t25077;
    let t26621 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t25080;
    let t26644 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t25140;
    let t26646 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t25144;
    let t26667 = F::cast_from(0.38381794893125283518e-1_f64) * t25293;
    let t26673 = F::cast_from(0.16449340668482264365e-1_f64) * t25317;
    let t26712 = F::cast_from(0.38381794893125283518e-1_f64) * t25211;
    let t26726 = F::cast_from(0.16449340668482264365e-1_f64) * t25346;
    let t26988 = F::cast_from(0.16449340668482264365e-1_f64) * t26198;
    let t26993 = F::cast_from(0.38381794893125283518e-1_f64) * t26200;
    let t27012 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t26231;
    (t26613, t26619, t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988, t26993, t27012)
}
