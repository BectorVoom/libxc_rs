//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1033/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1033<F: Float>(t1569: F, t2719: F, t2713: F, t3049: F, t1108: F, t3092: F, t4265: F, t242: F, t3060: F, t4246: F, t1111: F, t1562: F, t9540: F, t1571: F, t3087: F, t3074: F, t4231: F) -> (F, F, F, F, F, F, F) {
    let t12429 = t1569 * t2719;
    let t12431 = t2713 * t3049 * t12429;
    let t12435 = t2713 * t1108 * t12429;
    let t12439 = t4265 * t3092 / 648.0;
    let t12441 = t242 * t3060 * t4246;
    let t12443 = t1111 * t12441 / 2304.0;
    let t12445 = t242 * t9540 * t1562;
    let t12446 = t1111 * t12445;
    let t12448 = t1571 * t3087;
    let t12450 = t4231 * t3074;
    (t12431, t12435, t12439, t12443, t12446, t12448, t12450)
}
