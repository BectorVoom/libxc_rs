//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 775/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk775<F: Float>(t3054: F, t5248: F, t1112: F, t242: F, t450: F, t1501: F, t1562: F, t3068: F, t3097: F, t5064: F, t1111: F, t1125: F, t1575: F, t3052: F, t3067: F, t3080: F, t4210: F, t4265: F, t444: F, t463: F, t5223: F, t5231: F, t5235: F, t5239: F, t5245: F) -> (F, F, F, F, F, F, F, F) {
    let t5249 = t5248 * t3054;
    let t5250 = t1112 * t5249;
    let t5251 = t242 * t5250;
    let t5254 = t5248 * t450;
    let t5255 = t1112 * t5254;
    let t5256 = t242 * t5255;
    let t5261 = t1562 * t1501;
    let t5262 = t3068 * t5261;
    let t5265 = t3097 * t5064;
    let t5266 = t242 * t5265;
    let t5269 = 11.0 / 108.0 * t5223 * t444 - t4210 / 54.0 + 19.0 / 1728.0 * t5231 * t463 - t1125 * t5235 / 4608.0 - t1125 * t5239 / 2304.0 + t1111 * t5245 / 3072.0 + t3052 * t5251 / 1536.0 - t3080 * t5256 / 3072.0 + t4265 * t1575 / 432.0 - t3067 * t5262 / 2304.0 + 5.0 / 13824.0 * t1125 * t5266;
    (t5249, t5251, t5254, t5256, t5261, t5262, t5266, t5269)
}
