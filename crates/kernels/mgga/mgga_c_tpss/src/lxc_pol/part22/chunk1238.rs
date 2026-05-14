//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1238/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1238<F: Float>(t65600: F, t65572: F, t65574: F, t65576: F, t65578: F, t65580: F, t65582: F, t65584: F, t65586: F, t65588: F, t65597: F, t67150: F, t67160: F, t65616: F, t65624: F, t65628: F) -> (F, F, F, F) {
    let t67162 = 7.0 / 12.0 * t65600;
    let t67163 = t67150 - t65572 / 24.0 + t65574 / 192.0 - t65576 / 384.0 - t65578 / 768.0 + t65580 / 128.0 + t65582 / 96.0 + t65584 / 192.0 - t65586 / 96.0 - 5.0 / 192.0 * t65588 - t67160 - t65597 / 2.0 - t67162;
    let t67169 = 35.0 / 144.0 * t65616;
    let t67173 = 119.0 / 3456.0 * t65624;
    let t67175 = 7.0 / 576.0 * t65628;
    (t67163, t67169, t67173, t67175)
}
