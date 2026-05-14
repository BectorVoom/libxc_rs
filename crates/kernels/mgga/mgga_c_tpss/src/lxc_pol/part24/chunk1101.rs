//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1101/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1101<F: Float>(t1089: F, t15635: F, t3009: F, t5191: F, t12264: F, t1531: F, t15361: F, t15363: F, t15365: F, t15411: F, t15413: F, t15441: F, t15446: F, t15448: F, t15465: F, t4120: F, t4143: F, t5130: F, t9471: F) -> (F, F, F) {
    let t15637 = 0.23392894490538584828e1 * t1089 * t15635;
    let t15639 = 0.11696447245269292414e1 * t3009 * t5191;
    let t15647 = -0.19751673498613801407e-1 * t15441 - t15361 + t15363 - t15365 - t15411 - t15413 - t15446 - t15448 + t15465 + 2.0 * t12264 * t1531 + 2.0 * t4120 * t4143 - 2.0 * t9471 * t5130;
    (t15637, t15639, t15647)
}
