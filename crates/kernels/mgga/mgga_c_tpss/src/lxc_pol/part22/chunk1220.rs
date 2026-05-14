//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1220/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1220<F: Float>(t61081: F, t61087: F, t61089: F, t63971: F, t63975: F, t63979: F, t63981: F, t63984: F, t63987: F, t63995: F, t66427: F, t66429: F, t66434: F, t66398: F, t66411: F, t66425: F) -> (F,) {
    let t66439 = -5.0 / 32.0 * t63971 + t66427 - t63975 / 768.0 - t66429 + 5.0 / 96.0 * t63979 + 5.0 / 192.0 * t63981 + t63984 / 4.0 + t63987 / 8.0 - t66434 - t63995 / 2.0 + 7.0 / 288.0 * t61081 - 119.0 / 432.0 * t61087 - 35.0 / 288.0 * t61089;
    let t66441 = t66398 + t66411 + t66425 + t66439;
    (t66441,)
}
