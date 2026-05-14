//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1069/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1069<F: Float>(t3139: F, t4245: F, t1113: F, t450: F, t9080: F, t1141: F, t1143: F, t1581: F, t15930: F, t15956: F, t15960: F, t15964: F, t15968: F, t15975: F, t15979: F, t15989: F, t15992: F, t220: F, t3124: F, t3126: F, t3138: F, t4303: F, t4314: F, t468: F, t5270: F, t5279: F, t5283: F, t5287: F, t9749: F, t9764: F, t9787: F) -> (F,) {
    let t15999 = t3139 * t4245;
    let t16004 = t9080 * t1113 * t450;
    let t16012 = t1113 * t1141 * t1143 * t5270 + 2.0 * t1141 * t1143 * t15975 + 2.0 * t1141 * t1143 * t15979 + t1141 * t1143 * t15989 + t1141 * t1143 * t15992 + 4.0 * t1581 * t15964 * t3124 - 2.0 * t1581 * t15999 * t3138 + t15930 * t220 * t468 + 6.0 * t15956 * t5279 * t9749 + 2.0 * t15960 * t3124 * t3126 - t15960 * t3138 * t3139 - 6.0 * t15968 * t5279 * t9764 + t16004 * t5279 * t9787 + 4.0 * t3124 * t4303 * t5283 + 2.0 * t3124 * t4303 * t5287 - 2.0 * t3138 * t4314 * t5283 - t3138 * t4314 * t5287;
    (t16012,)
}
