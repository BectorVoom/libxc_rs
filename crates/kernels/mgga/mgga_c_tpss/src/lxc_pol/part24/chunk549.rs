//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 549/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk549<F: Float>(t821: F, t823: F, t1693: F, t262: F, t265: F, t664: F, t838: F) -> (F, F, F, F) {
    let t2440 = t821 * t823;
    let t2453 = t262 * t1693 * t265;
    let t2454 = 0.23744444444444444444e-1 * t2453;
    let t2455 = t664 * t838;
    (t2440, t2453, t2454, t2455)
}
