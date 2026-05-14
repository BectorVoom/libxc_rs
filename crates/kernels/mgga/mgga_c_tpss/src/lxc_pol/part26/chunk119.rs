//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 119/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk119<F: Float>(t267: F, t270: F, t273: F, t279: F) -> (F, F, F) {
    let t314 = 0.51785e1 * t270 + 0.905775e0 * t267 + 0.1100325e0 * t273 + 0.1241775e0 * t279;
    let t317 = 1.0 + 0.29608749977793437516e2 / t314;
    let t318 = f64::ln(t317);
    (t314, t317, t318)
}
