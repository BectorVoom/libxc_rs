//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1188/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1188<F: Float>(t20447: F, t219: F, t1805: F, t8275: F, t1219: F, t6419: F, t10085: F, t1838: F, t1656: F, t18967: F, t20155: F, t65551: F, t65561: F, t65570: F, t65592: F, t65600: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t66525 = t20447 * t219;
    let t66559 = t8275 * t1805;
    let t66970 = t1219 * t6419;
    let t67006 = t10085 * t1838;
    let t67061 = t18967 * t1656;
    let t67083 = t20155 * t219;
    let t67138 = 7.0 / 576.0 * t65551;
    let t67143 = 7.0 / 144.0 * t65561;
    let t67150 = 7.0 / 36.0 * t65570;
    let t67160 = 7.0 / 288.0 * t65592;
    let t67162 = 7.0 / 12.0 * t65600;
    (t66525, t66559, t66970, t67006, t67061, t67083, t67138, t67143, t67150, t67160, t67162)
}
