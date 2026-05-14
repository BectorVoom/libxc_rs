//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 333/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk333<F: Float>(t1011: F, t1036: F, t1017: F, t1028: F, t1033: F, t1040: F) -> (F, F, F) {
    let t1075 = 0.301925e0 * t1011;
    let t1078 = 0.82785e-1 * t1036;
    let t1080 = 0.258925e1 * t1028 - t1075 + 0.301925e0 * t1017 + 0.16504875e0 * t1033 - t1078 + 0.82785e-1 * t1040;
    (t1075, t1078, t1080)
}
