//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 564/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk564<F: Float>(t3508: F, t6224: F, t1214: F, t248: F, t475: F, t1213: F, t1227: F, t1737: F, t1748: F, t3506: F, t3515: F, t3542: F, t3547: F, t467: F, t5005: F, t5019: F, t5024: F, t5036: F, t5041: F, t6109: F, t6203: F, t6207: F, t6211: F, t6221: F) -> (F, F, F, F, F) {
    let t6225 = t6224 * t3508;
    let t6227 = t248 * t1214 * t6225;
    let t6230 = t6224 * t475;
    let t6232 = t248 * t1214 * t6230;
    let t6237 = -t5005 * t1748 / 2304.0 - t5019 * t1737 / 288.0 + 5.0 / 13824.0 * t1227 * t6203 - t1227 * t6207 / 4608.0 - t1227 * t6211 / 2304.0 - t5036 / 54.0 + 11.0 / 108.0 * t6109 * t467 - t5041 / 432.0 - t3542 + t1213 * t6221 / 3072.0 + t3506 * t6227 / 1536.0 - t3515 * t6232 / 3072.0 + t5024 * t1748 / 432.0 - t3547;
    (t6225, t6227, t6230, t6232, t6237)
}
