//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 299/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk299<F: Float>(t225: F, t492: F, t496: F, t68: F, t1011: F, t1209: F, t1206: F) -> (F, F, F, F, F) {
    let t1238 = t492 * t225;
    let t1239 = t496 * t496;
    let t1240 = 1.0 / t1239;
    let t1241 = t68 * t1240;
    let t1243 = t1011 * t1209;
    let t1244 = t1206 * t1243;
    (t1238, t1239, t1241, t1243, t1244)
}
