//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 33/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk33<F: Float>(t66: F, t80: F) -> (F, F, F, F) {
    let t83 = F::cast_from(1.0_f64) + t66 * t80 / F::cast_from(24.0_f64);
    let t84 = t83 * t83;
    let t85 = t84 * t84;
    let t86 = F::cast_from(1.0_f64) / t85;
    (t83, t84, t85, t86)
}
