//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1239/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1239<F: Float>(t62390: F, t65604: F, t65608: F, t65611: F, t65614: F, t65618: F, t65620: F, t65622: F, t65626: F, t65630: F, t67169: F, t67173: F, t67175: F, t65639: F, t65643: F, t65647: F) -> (F, F, F, F) {
    let t67177 = t65604 / 96.0 - t65608 / 128.0 + t65611 / 4.0 + t65614 / 8.0 - t67169 + 5.0 / 96.0 * t65618 + 5.0 / 192.0 * t65620 - t65622 / 768.0 - t62390 - t67173 - 5.0 / 32.0 * t65626 + t67175 - t65630 / 768.0;
    let t67183 = 7.0 / 144.0 * t65639;
    let t67185 = 7.0 / 144.0 * t65643;
    let t67187 = 119.0 / 864.0 * t65647;
    (t67177, t67183, t67185, t67187)
}
