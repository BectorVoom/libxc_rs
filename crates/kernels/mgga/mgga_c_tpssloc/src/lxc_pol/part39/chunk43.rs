//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 43/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk43<F: Float>(t100: F, t104: F, t92: F, t96: F, t64: F) -> (F, F, F) {
    let t106 = t100 * t104 + t92 * t96;
    let t107 = 1.0 / t106;
    let t109 = t64 * t107 / 8.0;
    let t110 = 1.0 < t109;
    let t111 = piecewise3(t110, 1.0, t109);
    (t106, t107, t111)
}
