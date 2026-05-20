//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2879/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2879<F: Float>(t4370: F, t2798: F, t17292: F, t699: F, t136: F, t59682: F, t908: F, t2403: F, t5720: F, t59690: F, t5723: F, t60149: F, t894: F) -> (F, F, F, F, F, F, F, F) {
    let t60160 = t4370 * t4370;
    let t60161 = t2798 * t60160;
    let t60163 = t699 * t17292;
    let t60166 = t136 * t908 * t59682;
    let t60168 = t2403 * t5720;
    let t60171 = t136 * t908 * t59690;
    let t60173 = t2403 * t5723;
    let t60176 = t894 * t60149;
    (t60160, t60161, t60163, t60166, t60168, t60171, t60173, t60176)
}
