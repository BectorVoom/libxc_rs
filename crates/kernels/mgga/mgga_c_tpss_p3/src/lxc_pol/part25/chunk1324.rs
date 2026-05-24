//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1324/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1324<F: Float>(t226: F, t4799: F, t782: F, t4716: F, t818: F, t10584: F, t3664: F, t4783: F, t1378: F, t3721: F, t4759: F, t1379: F) -> (F, F, F, F, F, F, F) {
    let t70070 = t4799 * t782 * t226;
    let t70074 = t4716 * t818;
    let t70094 = t10584 * t3664;
    let t70103 = t4783 * t782 * t226;
    let t70113 = t3721 * t1378 * t226;
    let t70123 = t4759 * t818;
    let t70130 = t1379 * t3664;
    (t70070, t70074, t70094, t70103, t70113, t70123, t70130)
}
