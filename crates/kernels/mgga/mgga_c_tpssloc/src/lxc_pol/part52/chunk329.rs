//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 329/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk329<F: Float>(t1323: F, t562: F, t541: F, t801: F, t119: F, t1307: F, t210: F, t225: F) -> (F, F, F, F) {
    let t1324 = t1323 * t562;
    let t1327 = 7.0 / 288.0 * t801 * t541;
    let t1328 = t119 * t1307;
    let t1329 = t210 * t1328;
    let t1332 = t1323 * t225;
    (t1324, t1327, t1329, t1332)
}
