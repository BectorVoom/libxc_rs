//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1166/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1166<F: Float>(t25080: F, t25140: F, t25144: F, t25293: F, t25317: F, t25211: F, t25346: F, t26198: F, t26200: F, t26231: F, t26251: F, t26255: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26621 = F::new(7.0) / F::new(1152.0) * t25080;
    let t26644 = F::new(7.0) / F::new(72.0) * t25140;
    let t26646 = F::new(7.0) / F::new(1152.0) * t25144;
    let t26667 = F::new(0.38381794893125283518e-1) * t25293;
    let t26673 = F::new(0.16449340668482264365e-1) * t25317;
    let t26712 = F::new(0.38381794893125283518e-1) * t25211;
    let t26726 = F::new(0.16449340668482264365e-1) * t25346;
    let t26988 = F::new(0.16449340668482264365e-1) * t26198;
    let t26993 = F::new(0.38381794893125283518e-1) * t26200;
    let t27012 = F::new(7.0) / F::new(1152.0) * t26231;
    let t27019 = F::new(7.0) / F::new(1152.0) * t26251;
    let t27022 = F::new(7.0) / F::new(288.0) * t26255;
    (t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988, t26993, t27012, t27019, t27022)
}
