//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 442/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk442<F: Float>(t3031: F, t3032: F, t371: F, t335: F, t368: F, t1015: F, t1043: F, t121: F, t283: F, t883: F, t61: F, t363: F, t1017: F, t67: F, t1058: F, t1044: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3033 = t3031 * t3032;
    let t3034 = t371 * t371;
    let t3036 = 1.0 / t3034 / t335;
    let t3037 = t368 * t3036;
    let t3038 = t1015 * t3037;
    let t3039 = t3033 * t3038;
    let t3051 = t121 * t1043;
    let t3061 = 1.0 / t283 / t883;
    let t3062 = t61 * t3061;
    let t3067 = t363 * t368;
    let t3068 = t1017 * t67;
    let t3069 = t3067 * t3068;
    let t3070 = t1058 * t3069;
    let t3071 = t820 * t1044;
    (t3033, t3034, t3036, t3037, t3038, t3039, t3051, t3061, t3062, t3067, t3068, t3069, t3070, t3071)
}
