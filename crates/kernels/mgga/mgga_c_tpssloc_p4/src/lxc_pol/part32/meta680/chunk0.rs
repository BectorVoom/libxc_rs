//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2119/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2119<F: Float>(t12571: F, t24525: F, t27331: F, t9239: F, t2240: F, t27363: F, t33: F, t26012: F, t7255: F, t2109: F, t90090: F, t90094: F) -> (F, F, F, F, F, F) {
    let t96028 = t12571 * t24525;
    let t96045 = t9239 * t27331;
    let t96072 = t2240 * t33 * t27363;
    let t96102 = t7255 * t26012;
    let t96110 = t2109 * t90090;
    let t96115 = t2109 * t90094;
    (t96028, t96045, t96072, t96102, t96110, t96115)
}
