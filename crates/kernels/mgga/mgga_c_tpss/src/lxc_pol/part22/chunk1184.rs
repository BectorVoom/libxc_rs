//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1184/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1184<F: Float>(t18660: F, t5492: F, t31455: F, t5784: F, t18669: F, t7682: F, t5489: F, t18356: F, t18670: F, t1675: F, t1679: F, t72: F, t789: F, t1981: F, t234: F, t38: F) -> (F, F, F, F, F, F, F) {
    let t62275 = t5492 * t18660;
    let t62277 = t31455 * t5784;
    let t62280 = t7682 * t18669;
    let t62281 = t62280 * t5489;
    let t62285 = t18670 * t18356;
    let t62294 = 1232.0 / 81.0 * t1675 * t789 * t72 * t1679;
    let t62306 = t1981 * t38 * t234;
    (t62275, t62277, t62280, t62281, t62285, t62294, t62306)
}
