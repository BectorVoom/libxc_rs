//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 526/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk526<F: Float>(t2383: F, t803: F, t206: F, t237: F, t235: F, t72: F, t219: F, t807: F, t251: F, t810: F, t73: F, t2157: F, t246: F, t768: F, t806: F, t255: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2384 = t2383 * t803;
    let t2387 = 1.0 / t237 / t206;
    let t2388 = t235 * t2387;
    let t2389 = t2388 * t72;
    let t2401 = t807 * t219;
    let t2405 = 1.0 / t810 / t251;
    let t2406 = t73 * t2405;
    let t2411 = t2157 * t246;
    let t2415 = t768 * t806;
    let t2435 = t255 * t255;
    (t2384, t2387, t2389, t2401, t2405, t2406, t2411, t2415, t2435)
}
