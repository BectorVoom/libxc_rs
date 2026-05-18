//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1309/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1309<F: Float>(t13760: F, t19476: F, t13695: F, t18454: F, t13682: F, t13749: F, t13756: F, t13765: F, t5389: F, t60738: F, t13700: F, t13687: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69503 = t19476 * t13760;
    let t69505 = t18454 * t13695;
    let t69507 = t18454 * t13682;
    let t69510 = t18454 * t13749;
    let t69512 = t18454 * t13756;
    let t69515 = t19476 * t13765;
    let t69517 = t60738 * t5389;
    let t69519 = t19476 * t13700;
    let t69521 = t18454 * t13687;
    (t69503, t69505, t69507, t69510, t69512, t69515, t69517, t69519, t69521)
}
