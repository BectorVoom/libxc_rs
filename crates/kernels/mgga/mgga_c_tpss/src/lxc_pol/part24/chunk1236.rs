//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1236/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1236<F: Float>(t18169: F, t2775: F, t1677: F, t36: F, t1206: F, t1777: F, t1338: F, t1683: F, t3205: F, t6273: F, t1289: F, t1270: F, t8549: F, t3204: F, t10178: F, t536: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23310 = t18169 * t2775;
    let t23510 = t1677 * t36;
    let t23794 = t1777 * t1206;
    let t24587 = t1683 * t1338;
    let t24790 = t3205 * t6273;
    let t25122 = t23510 * t1289;
    let t26620 = t6273 * t1270;
    let t27754 = t8549 * t2775;
    let t30366 = t3204 * t3204;
    let t30367 = 1.0 / t30366;
    let t31297 = 1.0 / t10178 / t536;
    (t23310, t23794, t24587, t24790, t25122, t26620, t27754, t30367, t31297)
}
