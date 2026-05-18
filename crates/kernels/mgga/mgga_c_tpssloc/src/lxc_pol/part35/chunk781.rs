//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 781/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk781<F: Float>(t1873: F, t7676: F, t1268: F, t7467: F, t1778: F, t191: F, t192: F) -> (F, F, F, F) {
    let t7678 = F::new(2.0) * t7676 * t1873;
    let t7680 = F::new(2.0) * t1268 * t7467;
    let t7684 = t1778 * t191;
    let t7685 = t7684 * t192;
    (t7678, t7680, t7684, t7685)
}
