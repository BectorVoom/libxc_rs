//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1252/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1252<F: Float>(t5: F, t67342: F, t67362: F, t67387: F, t67407: F, t67434: F, t67462: F, t67489: F, t67514: F, t117: F, t65440: F, t65442: F, t65444: F, t61871: F, t61874: F, t61876: F, t63006: F, t65447: F, t65450: F, t65453: F, t65455: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t67518 = piecewise3(t8, 0.0, t67342 + t67362 + t67387 + t67407 + t67434 + t67462 + t67489 + t67514);
    let t67519 = t67518 * t117;
    let t67531 = 22.0 / 9.0 * t65440;
    let t67532 = 8.0 / 3.0 * t65442;
    let t67533 = 4.0 / 3.0 * t65444;
    let t67537 = -t63006 - 44.0 / 9.0 * t61871 - 4.0 / 3.0 * t61874 + 2.0 / 3.0 * t61876 - t67531 - t67532 + t67533 - 3.0 / 2.0 * t65447 + t65450 + t65453 / 2.0 - t65455 / 4.0;
    (t67519, t67537)
}
