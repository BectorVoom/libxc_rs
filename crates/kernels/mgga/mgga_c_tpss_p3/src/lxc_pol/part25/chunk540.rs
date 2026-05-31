//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 540/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk540<F: Float>(t2576: F, t2453: F, t891: F, t895: F, t314: F, t894: F) -> (F, F, F, F) {
    let t2577 = F::cast_from(1.0_f64) / t2576;
    let t2581 = F::cast_from(0.12361111111111111111e-1_f64) * t2453;
    let t2589 = t891 * t895;
    let t2592 = t894 * t314;
    let t2593 = F::cast_from(1.0_f64) / t2592;
    (t2577, t2581, t2589, t2593)
}
