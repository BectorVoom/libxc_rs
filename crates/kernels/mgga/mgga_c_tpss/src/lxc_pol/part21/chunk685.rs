//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 685/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk685<F: Float>(t1139: F, t3144: F, t1136: F, t1149: F, t3111: F, t3113: F, t3120: F, t473: F, t1151: F, t475: F, t1153: F, t198: F, t2856: F, t2859: F, t2866: F, t2908: F, t2916: F, t3006: F, t3008: F, t3011: F, t3015: F, t3019: F, t3023: F, t330: F) -> (F, F, F, F, F, F) {
    let t3145 = t1139 * t3144;
    let t3147 = 2.0 * t1136 * t3120 - t1136 * t3145 - 2.0 * t1149 * t3113 + t3111 * t473;
    let t3151 = t1151 * t1151;
    let t3153 = t475 * t475;
    let t3154 = 1.0 / t3153;
    let t3157 = t1153 * t198 * t3147 * t330 - t198 * t3151 * t3154 * t330 - t2856 + t2859 - t2866 + t2908 + t2916 + t3006 + t3008 - t3011 + t3015 - t3019 - t3023;
    (t3145, t3147, t3151, t3153, t3154, t3157)
}
