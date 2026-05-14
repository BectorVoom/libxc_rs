//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 789/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk789<F: Float>(t1830: F, t645: F, t5545: F, t5555: F, t5548: F, t5553: F, t5560: F) -> (F, F, F, F) {
    let t5820 = t1830 * t645;
    let t5826 = 7.0 / 144.0 * t5545;
    let t5829 = 7.0 / 1152.0 * t5555;
    let t5831 = -t5826 - t5548 / 24.0 - t5553 / 768.0 - t5829 - t5560 / 192.0;
    (t5820, t5826, t5829, t5831)
}
