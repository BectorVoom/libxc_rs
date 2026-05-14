//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1106/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1106<F: Float>(t15266: F, t4219: F, t15275: F, t4223: F, t15271: F, t4597: F, t924: F, t140: F, t5210: F, t1098: F, t5214: F, t1095: F, t5223: F, t1103: F, t12361: F, t12368: F, t12371: F, t12385: F) -> (F, F, F) {
    let t15544 = t4219 * t15266;
    let t15547 = t4223 * t15275;
    let t15550 = t4223 * t15271;
    let t15554 = t4597 * t924;
    let t15557 = t140 * t5210;
    let t15558 = t1098 * t15557;
    let t15560 = t140 * t5214;
    let t15561 = t1098 * t15560;
    let t15564 = t5223 * t1095;
    let t15566 = t1098 * t15544 / 108.0 - t1098 * t15547 / 72.0 - t1098 * t15550 / 48.0 - t12361 + t12368 / 10368.0 - t12371 - 11.0 / 324.0 * t15554 * t1103 - t15558 / 432.0 + t15561 / 648.0 + t12385 / 648.0 + 11.0 / 324.0 * t15564;
    (t15557, t15560, t15566)
}
