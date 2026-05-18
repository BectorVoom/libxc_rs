//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1279/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1279<F: Float>(t31455: F, t5784: F, t18669: F, t7682: F, t1675: F, t1679: F, t72: F, t789: F, t1981: F, t234: F, t38: F, t5489: F) -> (F, F, F, F, F) {
    let t62277 = t31455 * t5784;
    let t62280 = t7682 * t18669;
    let t62294 = F::new(1232.0) / F::new(81.0) * t1675 * t789 * t72 * t1679;
    let t62306 = t1981 * t38 * t234;
    let t62307 = t62306 * t5489;
    (t62277, t62280, t62294, t62306, t62307)
}
