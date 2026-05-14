//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 922/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk922<F: Float>(t114: F, t13541: F, t630: F, t13154: F, t13157: F, t13159: F, t13483: F, t13486: F, t13489: F, t13492: F, t13495: F, t69: F, t7587: F, t7588: F) -> (F,) {
    let t115 = 1.0 < t114;
    let t13542 = t630 * t13541;
    let t13545 = -t7587 - 11.0 / 9.0 * t7588 - 22.0 / 9.0 * t13154 - t13157 + t13159 - 2.0 / 3.0 * t13483 - 3.0 / 4.0 * t69 * t13486 + t69 * t13489 / 2.0 + t13492 / 3.0 + t69 * t13495 / 4.0 - t69 * t13542 / 8.0;
    let t13546 = piecewise3(t115, 0.0, t13545);
    (t13546,)
}
