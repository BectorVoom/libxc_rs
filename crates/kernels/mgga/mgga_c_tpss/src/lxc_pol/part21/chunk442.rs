//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 442/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk442<F: Float>(t1474: F, t1464: F, t366: F, t220: F, t368: F, t983: F, t985: F) -> (F, F, F) {
    let t1475 = param_beta * t1474;
    let t1477 = t366 * t1464;
    let t1482 = t1474 * t220 * t368 + t1477 * t983 * t985;
    (t1475, t1477, t1482)
}
