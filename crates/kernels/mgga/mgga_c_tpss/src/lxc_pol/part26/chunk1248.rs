//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1248/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1248<F: Float>(t3205: F, t5753: F, t1206: F, t1777: F, t18169: F, t3117: F, t6273: F, t1338: F, t1864: F, t1270: F, t8549: F, t3204: F, t10178: F, t536: F, t574: F, t7689: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22964 = t3205 * t5753;
    let t23794 = t1777 * t1206;
    let t24476 = t18169 * t3117;
    let t24790 = t3205 * t6273;
    let t25354 = t1864 * t1338;
    let t26620 = t6273 * t1270;
    let t28778 = t8549 * t3117;
    let t30366 = t3204 * t3204;
    let t30367 = 1.0 / t30366;
    let t31297 = 1.0 / t10178 / t536;
    let t31455 = t574 * t7689;
    (t22964, t23794, t24476, t24790, t25354, t26620, t28778, t30367, t31297, t31455)
}
