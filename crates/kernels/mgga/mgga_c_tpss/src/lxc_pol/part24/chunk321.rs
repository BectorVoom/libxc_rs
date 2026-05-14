//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 321/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk321<F: Float>(t1013: F, t1015: F, t128: F, t1012: F, t408: F, t404: F) -> (F, F, F, F, F, F) {
    let t1016 = t1013 * t1015;
    let t1017 = t128 * t1016;
    let t1019 = -t1012 + 0.17808333333333333333e-1 * t1017;
    let t1021 = 0.621814e-1 * t1019 * t408;
    let t1022 = t404 * t404;
    let t1023 = 1.0 / t1022;
    (t1016, t1017, t1019, t1021, t1022, t1023)
}
