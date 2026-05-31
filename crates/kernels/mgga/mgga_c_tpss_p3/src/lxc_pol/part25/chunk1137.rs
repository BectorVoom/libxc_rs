//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1137/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1137<F: Float>(t12264: F, t1531: F, t15361: F, t15363: F, t15365: F, t15411: F, t15413: F, t15441: F, t15446: F, t15448: F, t15465: F, t4120: F, t4143: F, t5130: F, t9471: F) -> F {
    let t15647 = -F::cast_from(0.19751673498613801407e-1_f64) * t15441 - t15361 + t15363 - t15365 - t15411 - t15413 - t15446 - t15448 + t15465 + F::cast_from(2.0_f64) * t12264 * t1531 + F::cast_from(2.0_f64) * t4120 * t4143 - F::cast_from(2.0_f64) * t9471 * t5130;
    t15647
}
