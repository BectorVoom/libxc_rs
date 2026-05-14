//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1334/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1334<F: Float>(t5: F, t65185: F, t65220: F, t65249: F, t65275: F, t65311: F, t65342: F, t65387: F, t65424: F, t117: F, t12841: F, t19620: F, t7310: F, t61871: F, t1333: F, t61870: F, t19590: F, t61873: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t65428 = piecewise3(t8, 0.0, t65185 + t65220 + t65249 + t65275 + t65311 + t65342 + t65387 + t65424);
    let t65429 = t65428 * t117;
    let t65436 = 6.0 * t19620 * t7310 * t12841;
    let t65437 = 22.0 / 9.0 * t61871;
    let t65440 = t61870 * t1333;
    let t65442 = t61873 * t19590;
    (t65429, t65436, t65437, t65440, t65442)
}
