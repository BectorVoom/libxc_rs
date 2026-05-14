//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 115/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk115<F: Float>(t269: F, t285: F, t153: F, t159: F, t162: F, zeta_threshold: F) -> (F, F) {
    let t287 = 0.621814e-1 * t269 * t285;
    let t288 = 2.0 <= zeta_threshold;
    let t290 = piecewise3(t288, t153, 2.0 * t159);
    let t291 = 0.0 <= zeta_threshold;
    let t292 = piecewise3(t291, t153, 0.0);
    let t294 = (t290 + t292 - 2.0) * t162;
    (t287, t294)
}
