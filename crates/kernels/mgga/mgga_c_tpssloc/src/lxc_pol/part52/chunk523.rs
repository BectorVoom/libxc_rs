//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 523/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk523<F: Float>(t3033: F, t3129: F, t360: F, t135: F, t999: F, t973: F, t1005: F, t1036: F, t221: F, t2965: F, t339: F, t964: F, t995: F, t1050: F, t225: F, t1053: F, t386: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3130 = t3033 * t3129;
    let t3131 = t360 * t360;
    let t3139 = t135 * t999;
    let t3140 = t973 * t3139;
    let t3156 = t1005 * t1036;
    let t3158 = t221 * t2965;
    let t3160 = t339 * t3158 / 432.0;
    let t3163 = t964 * t995;
    let t3169 = t1050 * t225;
    let t3173 = 1.0 / t1053 / t386;
    (t3130, t3131, t3139, t3140, t3156, t3158, t3160, t3163, t3169, t3173)
}
