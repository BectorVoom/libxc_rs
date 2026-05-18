//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 971/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk971<F: Float>(t4608: F, t582: F, t4573: F, t7737: F, t581: F, t3431: F, t3446: F, t2009: F, t4579: F, t13335: F, t48: F, t7750: F) -> (F, F, F, F, F, F) {
    let t13365 = t582 * t4608;
    let t13370 = t7737 * t4573;
    let t13371 = t13370 * t581;
    let t13374 = t3446 * t3431;
    let t13379 = t2009 * t4579;
    let t13380 = t13379 * t581;
    let t13383 = t48 * t13335;
    let t13392 = t7750 * t4573;
    (t13365, t13371, t13374, t13380, t13383, t13392)
}
