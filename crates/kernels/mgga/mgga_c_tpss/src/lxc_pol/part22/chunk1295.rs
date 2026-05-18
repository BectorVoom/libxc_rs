//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1295/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1295<F: Float>(t1695: F, t212: F, t60720: F, t17974: F, t2395: F, t2376: F, t339: F, t5557: F, t803: F, t2391: F, t17990: F, t5570: F) -> (F, F, F, F, F, F) {
    let t61079 = t60720 * t212 * t1695;
    let t61081 = t17974 * t2395;
    let t61086 = t339 * t5557 * t2376;
    let t61087 = t61086 * t803;
    let t61089 = t17974 * t2391;
    let t61183 = t17990 * t5570;
    (t61079, t61081, t61086, t61087, t61089, t61183)
}
