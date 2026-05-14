//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1268/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1268<F: Float>(t1219: F, t6255: F, t19498: F, t219: F, t1656: F, t18497: F, t18495: F, t6259: F, t10085: F, t1768: F, t1777: F, t30367: F, t116: F, t19596: F, t63945: F, t63957: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65706 = t1219 * t6255;
    let t65747 = t19498 * t219;
    let t65788 = t18497 * t1656;
    let t65871 = t6259 * t18495;
    let t65877 = t10085 * t1768;
    let t65898 = t1777 * t30367;
    let t66108 = t116 * t19596;
    let t66410 = 119.0 / 3456.0 * t63945;
    let t66418 = 35.0 / 108.0 * t63957;
    (t65706, t65747, t65788, t65871, t65877, t65898, t66108, t66410, t66418)
}
