//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 804/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk804<F: Float>(t30: F, t821: F, t33: F, t750: F, t1168: F, t196: F, t197: F) -> (F, F, F, F, F) {
    let t5591 = t30 * t821;
    let t5671 = t33 * t750;
    let t5678 = t33 * t821;
    let t5705 = t1168 * t196;
    let t5706 = t5705 * t197;
    (t5591, t5671, t5678, t5705, t5706)
}
