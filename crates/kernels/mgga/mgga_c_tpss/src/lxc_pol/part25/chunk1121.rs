//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1121/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1121<F: Float>(t1043: F, t15408: F, t1024: F, t5117: F, t9504: F, t2998: F, t5177: F, t4206: F, t1089: F, t5161: F, t9347: F, t9172: F) -> (F, F, F, F, F) {
    let t15409 = t15408 * t1043;
    let t15411 = F::new(1.0) * t1024 * t15409;
    let t15413 = F::new(0.16081979498692535067e2) * t9504 * t5117;
    let t15414 = t2998 * t5177;
    let t15415 = t15414 * t4206;
    let t15417 = F::new(0.17315859105681463759e2) * t1089 * t15415;
    let t15418 = t9347 * t5161;
    let t15419 = t15418 * t4206;
    let t15421 = F::new(0.10389515463408878255e3) * t1089 * t15419;
    let t15422 = t9172 * t5161;
    (t15411, t15413, t15417, t15421, t15422)
}
