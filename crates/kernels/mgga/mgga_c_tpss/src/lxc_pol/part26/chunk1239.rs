//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1239/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1239<F: Float>(t1875: F, t339: F, t5229: F, t331: F, t4596: F, t136: F, t1558: F, t19077: F, t20800: F, t20802: F, t20844: F, t444: F, t463: F, t5207: F, t5211: F, t5235: F, t5239: F, t5245: F, t5251: F, t6002: F, t6007: F, t6013: F) -> (F, F, F, F) {
    let t21998 = t339 * t1875 * t5229;
    let t22011 = t4596 * t331;
    let t22012 = t22011 * t136;
    let t22020 = -t20844 / 1728.0 + 19.0 / 864.0 * t21998 * t463 + t20802 * t1558 / 54.0 - t6002 * t5207 / 288.0 - t6002 * t5211 / 144.0 - t6013 * t5235 / 2304.0 - t6013 * t5239 / 1152.0 + 11.0 / 108.0 * t22012 * t444 - t20800 / 54.0 + t6007 * t5245 / 1536.0 + t19077 * t5251 / 768.0;
    (t21998, t22011, t22012, t22020)
}
