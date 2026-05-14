//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 544/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk544<F: Float>(t235: F, t2376: F, t238: F, t242: F, t232: F, t339: F, t789: F, t795: F) -> (F, F, F, F) {
    let t2377 = t2376 * t235;
    let t2379 = t2377 * t238 * t242;
    let t2381 = 119.0 / 13824.0 * t232 * t2379;
    let t2383 = t339 * t795 * t789;
    (t2377, t2379, t2381, t2383)
}
