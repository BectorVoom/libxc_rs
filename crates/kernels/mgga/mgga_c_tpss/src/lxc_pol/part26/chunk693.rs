//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 693/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk693<F: Float>(t1114: F, t1501: F, t3068: F, t3090: F, t242: F, t1125: F, t2840: F, t3096: F, t3426: F, t3931: F, t1127: F, t2845: F, t1128: F, t4056: F, t1116: F, t1130: F, t3063: F, t3067: F, t3080: F, t3089: F, t3093: F, t4253: F, t4258: F, t4261: F, t4265: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4270 = t1501 * t1114;
    let t4271 = t3068 * t4270;
    let t4274 = t3090 * t1501;
    let t4275 = t242 * t4274;
    let t4276 = t1125 * t4275;
    let t4278 = t3096 * t2840;
    let t4279 = t4278 * t3426;
    let t4280 = t3931 * t4279;
    let t4283 = t1127 * t2845;
    let t4284 = t4283 * t3426;
    let t4285 = t3931 * t4284;
    let t4288 = t1128 * t4056;
    let t4289 = t242 * t4288;
    let t4292 = -t3080 * t4253 / 3072.0 - t4258 * t1116 / 576.0 - t4261 / 864.0 + t4265 * t1130 / 864.0 + t3063 / 4608.0 - t3089 - t3093 / 6912.0 - t3067 * t4271 / 4608.0 - t4276 / 6912.0 + 5.0 / 13824.0 * t1125 * t4280 - t1125 * t4285 / 2304.0 - t1125 * t4289 / 4608.0;
    (t4270, t4271, t4275, t4276, t4278, t4279, t4280, t4283, t4284, t4285, t4289, t4292)
}
