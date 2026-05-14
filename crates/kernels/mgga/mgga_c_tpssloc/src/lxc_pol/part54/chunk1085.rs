//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1085/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1085<F: Float>(t31170: F, t32721: F, t1831: F, t8466: F, t31137: F, t7691: F, t6888: F, t7700: F, t1985: F, t1799: F, t31193: F, t6637: F, t26403: F, t550: F, t6976: F, t1992: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32722 = t31170 * t32721;
    let t32724 = t8466 * t1831;
    let t32731 = t31137 * t7691;
    let t32733 = 0.3289868133696452873e-1 * t6888 * t32731;
    let t32735 = t31137 * t7700;
    let t32737 = 0.16449340668482264365e-1 * t1985 * t32735;
    let t32740 = t31193 * t1799;
    let t32741 = t6637 * t32740;
    let t32743 = 0.3289868133696452873e-1 * t6888 * t32741;
    let t32744 = t26403 * t550;
    let t32745 = t6976 * t32744;
    let t32747 = 0.16449340668482264365e-1 * t1992 * t32745;
    (t32722, t32724, t32731, t32733, t32735, t32737, t32740, t32741, t32743, t32744, t32745, t32747)
}
