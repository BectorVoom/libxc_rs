//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 72/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk72<F: Float>(t128: F, t131: F, t134: F, t141: F) -> (F, F, F) {
    let t182 = 0.51785e1 * t131 + 0.905775e0 * t128 + 0.1100325e0 * t134 + 0.1241775e0 * t141;
    let t185 = 1.0 + 0.29608749977793437516e2 / t182;
    let t186 = f64::ln(t185);
    (t182, t185, t186)
}
