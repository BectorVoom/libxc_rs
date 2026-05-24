//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 900/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk900<F: Float>(t7921: F, t7997: F, t162: F, t158: F, t2243: F, t725: F, t2206: F, t2218: F, t2433: F, t2436: F, t713: F, t720: F, t7870: F) -> (F, F, F, F, F, F) {
    let t7998 = t7921 + t7997;
    let t7999 = t162 * t7998;
    let t8000 = t158 * t7999;
    let t8001 = t2243 * t725;
    let t8006 = t2218 * t2206;
    let t8012 = t2433 * t2436;
    let t8017 = t713 * t7870 * t720;
    (t7998, t8000, t8001, t8006, t8012, t8017)
}
