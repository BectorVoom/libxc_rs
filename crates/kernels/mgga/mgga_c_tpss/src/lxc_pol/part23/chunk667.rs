//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 667/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk667<F: Float>(t3096: F, t66: F, t2841: F, t242: F, t1128: F, t2846: F, t2850: F, t1098: F, t1111: F, t1125: F, t3027: F, t3029: F, t3035: F, t3040: F, t3044: F, t3052: F, t3057: F, t3063: F, t3067: F, t3070: F, t3076: F, t3080: F, t3083: F, t3089: F, t3093: F) -> (F, F, F, F, F) {
    let t3097 = t66 * t3096;
    let t3098 = t3097 * t2841;
    let t3099 = t242 * t3098;
    let t3102 = t1128 * t2846;
    let t3103 = t242 * t3102;
    let t3106 = t1128 * t2850;
    let t3107 = t242 * t3106;
    let t3110 = -t3027 - t3029 / 432.0 + t1098 * t3035 / 216.0 - t1098 * t3040 / 144.0 - t1098 * t3044 / 288.0 + t3052 * t3057 / 1536.0 + t3063 / 2304.0 - t3067 * t3070 / 2304.0 + t1111 * t3076 / 3072.0 - t3080 * t3083 / 3072.0 - t3089 - t3093 / 3456.0 + 5.0 / 13824.0 * t1125 * t3099 - t1125 * t3103 / 2304.0 - t1125 * t3107 / 4608.0;
    (t3097, t3099, t3103, t3107, t3110)
}
