//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1171/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1171<F: Float>(t22986: F, t23270: F, t30622: F, t4119: F, t32814: F, t81651: F, t82074: F, t30623: F, t86873: F, t1903: F, t254: F, t23168: F, t32789: F, t112873: F, t1527: F, t1888: F) -> (F, F, F, F, F, F) {
    let t118630 = 0.3289868133696452873e-1 * t22986 * t23270 * t30622 * t4119;
    let t118632 = t81651 * t82074 * t32814;
    let t118633 = 0.16449340668482264365e-1 * t118632;
    let t118639 = 0.3289868133696452873e-1 * t22986 * t86873 * t30623;
    let t118640 = t1903 * t254;
    let t118649 = t23168 * t32789;
    let t118650 = 0.76763589786250567037e-1 * t118649;
    let t118654 = 0.3289868133696452873e-1 * t1888 * t23270 * t112873 * t1527;
    (t118630, t118633, t118639, t118640, t118650, t118654)
}
