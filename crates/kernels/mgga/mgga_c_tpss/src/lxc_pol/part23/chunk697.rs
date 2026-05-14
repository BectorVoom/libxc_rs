//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 697/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk697<F: Float>(t1224: F, t3332: F, t774: F, t2377: F, t242: F, t527: F, t525: F, t1242: F, t339: F, t789: F) -> (F, F, F, F) {
    let t3334 = t1224 * t774 * t3332;
    let t3338 = t2377 * t527 * t242;
    let t3340 = 119.0 / 13824.0 * t525 * t3338;
    let t3342 = t339 * t1242 * t789;
    (t3334, t3338, t3340, t3342)
}
