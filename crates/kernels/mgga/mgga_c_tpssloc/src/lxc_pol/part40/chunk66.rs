//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 66/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk66<F: Float>(t123: F, t126: F, t129: F, t136: F) -> (F, F, F, F) {
    let t164 = 0.705945e1 * t126 + 0.1549425e1 * t123 + 0.420775e0 * t129 + 0.1562925e0 * t136;
    let t167 = 1.0 + 0.32163958997385070134e2 / t164;
    let t168 = f64::ln(t167);
    let t172 = 1.0 + 0.278125e-1 * t123;
    (t164, t167, t168, t172)
}
