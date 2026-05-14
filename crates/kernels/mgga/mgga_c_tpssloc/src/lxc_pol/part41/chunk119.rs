//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 119/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk119<F: Float>(t273: F, t276: F, t279: F, t285: F) -> (F, F, F) {
    let t320 = 0.51785e1 * t276 + 0.905775e0 * t273 + 0.1100325e0 * t279 + 0.1241775e0 * t285;
    let t323 = 1.0 + 0.29608749977793437516e2 / t320;
    let t324 = f64::ln(t323);
    (t320, t323, t324)
}
