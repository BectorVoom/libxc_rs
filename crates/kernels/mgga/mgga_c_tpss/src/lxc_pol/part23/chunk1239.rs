//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1239/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1239<F: Float>(t3205: F, t5753: F, t18169: F, t3117: F, t6273: F, t1270: F, t8549: F, t3204: F, t10178: F, t536: F, t1974: F, t1980: F, t574: F, t7689: F, t90: F, t29: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22964 = t3205 * t5753;
    let t24476 = t18169 * t3117;
    let t24790 = t3205 * t6273;
    let t26009 = t5753 * t1270;
    let t28778 = t8549 * t3117;
    let t30366 = t3204 * t3204;
    let t30367 = 1.0 / t30366;
    let t31297 = 1.0 / t10178 / t536;
    let t31450 = t1974 * t1980;
    let t31455 = t574 * t7689;
    let t31462 = t90 * t90;
    let t31464 = t29 / t31462;
    (t22964, t24476, t24790, t26009, t28778, t30367, t31297, t31450, t31455, t31464)
}
