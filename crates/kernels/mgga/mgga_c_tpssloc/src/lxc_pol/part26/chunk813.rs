//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 813/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk813<F: Float>(t10191: F, t2986: F, t2770: F, t607: F, t2250: F, t4510: F, t2980: F, t9288: F, t977: F, t9258: F, t978: F, t3008: F, t343: F, t984: F, t4546: F, t271: F, t2775: F) -> (F, F, F, F, F, F, F) {
    let t10192 = t2986 * t10191;
    let t10194 = t2770 * t607;
    let t10195 = t10194 * t2250;
    let t10196 = t4510 * t10195;
    let t10199 = t2980 * t9288;
    let t10200 = t977 * t10199;
    let t10203 = t978 * t9258;
    let t10204 = t977 * t10203;
    let t10208 = t3008 * t984 * t343;
    let t10209 = t4546 * t10208;
    let t10213 = 1.0 / t271 / t2775;
    (t10192, t10195, t10196, t10200, t10204, t10209, t10213)
}
