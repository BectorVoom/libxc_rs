//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1134/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1134<F: Float>(t114: F, t13215: F, t630: F, t13154: F, t13157: F, t13159: F, t13161: F, t13165: F, t13168: F, t69: F, t7587: F, t7588: F, t7590: F, t7592: F) -> (F,) {
    let t115 = 1.0 < t114;
    let t13216 = t630 * t13215;
    let t13219 = -t7587 - 22.0 / 9.0 * t7588 - 2.0 / 3.0 * t7590 + t7592 / 3.0 - 11.0 / 9.0 * t13154 - t13157 + t13159 - 3.0 / 4.0 * t69 * t13161 + t69 * t13165 / 2.0 + t69 * t13168 / 4.0 - t69 * t13216 / 8.0;
    let t13220 = piecewise3(t115, 0.0, t13219);
    (t13220,)
}
