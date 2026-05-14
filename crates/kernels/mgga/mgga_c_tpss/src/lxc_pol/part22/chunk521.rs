//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 521/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk521<F: Float>(t2187: F, t2190: F, t2193: F, t2197: F, t2199: F, t2202: F) -> (F,) {
    let t2204 = -0.57538888888888888889e0 * t2187 + 0.11507777777777777778e1 * t2190 + 0.40256666666666666667e0 * t2193 + 0.366775e-1 * t2197 + 0.73355e-1 * t2199 + 0.137975e0 * t2202;
    (t2204,)
}
