//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1212/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1212<F: Float>(t1379: F, t3664: F, t226: F, t44960: F, t14349: F, t1705: F, t935: F, t4578: F, t750: F, t4802: F, t580: F, t1288: F, t8096: F, t19818: F, t821: F, t19817: F) -> (F, F, F, F, F, F, F, F) {
    let t70130 = t1379 * t3664;
    let t70134 = t44960 * t226;
    let t70189 = t1705 * t14349 * t935;
    let t70221 = t4578 * t750;
    let t70227 = t580 * t4802;
    let t70236 = t8096 * t1288;
    let t70237 = t70236 * t19818;
    let t70240 = t4802 * t821;
    let t70241 = t19817 * t70240;
    (t70130, t70134, t70189, t70221, t70227, t70237, t70240, t70241)
}
