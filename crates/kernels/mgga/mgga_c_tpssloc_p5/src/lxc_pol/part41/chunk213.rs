//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 213/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk213<F: Float>(t607: F, t634: F, t638: F, t72: F, t609: F, t629: F, t66: F, t80: F) -> (F, F) {
    let t641 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t634 * t607 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t638 * t607;
    let t642 = t72 * t641;
    let t645 = -t609 * t80 / F::cast_from(12.0_f64) + t629 * t80 / F::cast_from(24.0_f64) + t66 * t642 / F::cast_from(24.0_f64);
    (t642, t645)
}
