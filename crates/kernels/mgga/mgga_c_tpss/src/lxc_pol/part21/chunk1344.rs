//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1344/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1344<F: Float>(t12851: F, t215: F, t65595: F, t19468: F, t19470: F, t5543: F, t65571: F, t65572: F, t65574: F, t65576: F, t65578: F, t65580: F, t65582: F, t65584: F, t65586: F, t65588: F, t65593: F) -> (F,) {
    let t65597 = t65595 * t215 * t12851;
    let t65600 = t5543 * t19468 * t19470;
    let t65601 = 7.0 / 24.0 * t65600;
    let t65602 = t65571 - t65572 / 48.0 + t65574 / 384.0 - t65576 / 768.0 - t65578 / 1536.0 + t65580 / 256.0 + t65582 / 192.0 + t65584 / 384.0 - t65586 / 192.0 - 5.0 / 384.0 * t65588 - t65593 - t65597 / 4.0 - t65601;
    (t65602,)
}
