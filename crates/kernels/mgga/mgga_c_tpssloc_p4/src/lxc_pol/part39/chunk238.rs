//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 238/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk238<F: Float>(t172: F, t739: F, t688: F, t690: F, t694: F, t699: F) -> (F, F) {
    let t740 = t172 * t739;
    let t745 = -F::cast_from(0.86308333333333333334e0_f64) * t688 - F::cast_from(0.301925e0_f64) * t690 - F::cast_from(0.5501625e-1_f64) * t694 - F::cast_from(0.82785e-1_f64) * t699;
    (t740, t745)
}
