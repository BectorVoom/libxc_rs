//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 673/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk673<F: Float>(t1139: F, t3144: F, t1136: F, t1149: F, t3111: F, t3113: F, t3120: F, t473: F, t1151: F, t475: F) -> (F, F, F, F, F) {
    let t3145 = t1139 * t3144;
    let t3147 = 2.0 * t1136 * t3120 - t1136 * t3145 - 2.0 * t1149 * t3113 + t3111 * t473;
    let t3151 = t1151 * t1151;
    let t3153 = t475 * t475;
    let t3154 = 1.0 / t3153;
    (t3145, t3147, t3151, t3153, t3154)
}
