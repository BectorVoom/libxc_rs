//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 739/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk739<F: Float>(t10277: F, t9288: F, t2826: F, t136: F, t10195: F, t2770: F, t908: F, t10250: F, t883: F, t9258: F, t10295: F, t10296: F, t10298: F, t10300: F, t10302: F, t10307: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10309 = t10277 * t9288;
    let t10310 = t2826 * t10309;
    let t10311 = t136 * t10310;
    let t10313 = t2826 * t10195;
    let t10314 = t136 * t10313;
    let t10316 = t2770 * t9288;
    let t10317 = t908 * t10316;
    let t10318 = t136 * t10317;
    let t10319 = t908 * t10250;
    let t10320 = t136 * t10319;
    let t10321 = t883 * t9258;
    let t10322 = t908 * t10321;
    let t10323 = t136 * t10322;
    let t10325 = t10295 + 5.0 / 9.0 * t10296 - t10298 / 9.0 + 2.0 / 3.0 * t10300 - t10302 / 3.0 + 2.0 / 27.0 * t10307 - t10311 / 3.0 + t10314 / 6.0 + t10318 - t10320 + t10323 / 6.0;
    (t10309, t10311, t10314, t10316, t10318, t10320, t10321, t10323, t10325)
}
