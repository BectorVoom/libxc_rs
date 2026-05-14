//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 553/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk553<F: Float>(t25: F, t28: F, t1268: F, t2312: F, t2314: F, t2319: F, t2363: F, t671: F, t88: F, t526: F, t606: F, t2249: F, t514: F, t528: F, t1081: F, t3231: F, t517: F, t157: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t3660 = 2.0 * t1268 * t2363 + 4.0 * t2314 * t671 + 2.0 * t2319 * t88 + t2312;
    let t3664 = 1.0 / t526;
    let t3665 = t606 * t606;
    let t3671 = piecewise3(t26, 0.0, 4.0 / 9.0 * t3664 * t3665 + 4.0 / 3.0 * t514 * t2249);
    let t3672 = 1.0 / t528;
    let t3673 = t1081 * t1081;
    let t3679 = piecewise3(t29, 0.0, 4.0 / 9.0 * t3672 * t3673 + 4.0 / 3.0 * t517 * t3231);
    let t3681 = (t3671 + t3679) * t157;
    (t3660, t3664, t3665, t3672, t3673, t3681)
}
