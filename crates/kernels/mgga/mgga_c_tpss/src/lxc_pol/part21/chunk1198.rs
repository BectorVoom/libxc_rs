//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1198/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1198<F: Float>(t18596: F, t547: F, t117: F, t18403: F, t1281: F, t1784: F, t18575: F, t18584: F, t18586: F, t18588: F, t18591: F, t18595: F, t3407: F, t3410: F, t548: F, t5766: F) -> (F, F) {
    let t18598 = 6.0 * t547 * t18596;
    let t18599 = t117 * t18403;
    let t18601 = 3.0 * t547 * t18599;
    let t18602 = 6.0 * t1281 * t5766 + 6.0 * t1784 * t3407 + 3.0 * t1784 * t3410 + t18575 * t548 + t18584 + t18586 + t18588 + t18591 + t18595 + t18598 + t18601;
    (t18599, t18602)
}
