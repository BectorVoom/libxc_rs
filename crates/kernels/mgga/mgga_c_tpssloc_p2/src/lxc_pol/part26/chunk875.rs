//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 875/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk875<F: Float>(t2830: F, t699: F, t2833: F, t241: F, t2978: F, t10216: F, t9288: F, t136: F, t10277: F, t2826: F, t10195: F, t2770: F) -> (F, F, F, F, F, F, F, F) {
    let t10300 = t699 * t2830;
    let t10302 = t699 * t2833;
    let t10304 = t241 * t2978;
    let t10305 = t10216 * t9288;
    let t10306 = t10304 * t10305;
    let t10307 = t136 * t10306;
    let t10309 = t10277 * t9288;
    let t10310 = t2826 * t10309;
    let t10311 = t136 * t10310;
    let t10313 = t2826 * t10195;
    let t10314 = t136 * t10313;
    let t10316 = t2770 * t9288;
    (t10300, t10302, t10305, t10307, t10309, t10311, t10314, t10316)
}
