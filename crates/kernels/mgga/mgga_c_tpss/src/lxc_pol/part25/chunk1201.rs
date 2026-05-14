//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1201/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1201<F: Float>(t5389: F, t60738: F, t13700: F, t19476: F, t13687: F, t18454: F, t13691: F, t5373: F, t60724: F, t18436: F, t5377: F, t18450: F, t5410: F, t5383: F, t60695: F, t5415: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69517 = t60738 * t5389;
    let t69519 = t19476 * t13700;
    let t69521 = t18454 * t13687;
    let t69523 = t18454 * t13691;
    let t69525 = t60724 * t5373;
    let t69527 = t18436 * t5377;
    let t69531 = t18450 * t5410;
    let t69533 = t60695 * t5383;
    let t69535 = t18450 * t5415;
    (t69517, t69519, t69521, t69523, t69525, t69527, t69531, t69533, t69535)
}
