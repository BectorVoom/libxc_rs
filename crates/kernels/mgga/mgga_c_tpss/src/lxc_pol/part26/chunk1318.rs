//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1318/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1318<F: Float>(t70038: F, t70098: F, t70160: F, t70210: F, t823: F, t4578: F, t750: F, t4802: F, t580: F, t1288: F, t8096: F, t19818: F, t821: F, t19817: F, t4806: F, t64247: F) -> (F, F, F, F, F, F, F, F, F) {
    let t70212 = t70038 + t70098 + t70160 + t70210;
    let t70213 = t70212 * t823;
    let t70221 = t4578 * t750;
    let t70227 = t580 * t4802;
    let t70236 = t8096 * t1288;
    let t70237 = t70236 * t19818;
    let t70240 = t4802 * t821;
    let t70241 = t19817 * t70240;
    let t70243 = t4806 * t821;
    let t70244 = t64247 * t70243;
    (t70212, t70213, t70221, t70227, t70237, t70240, t70241, t70243, t70244)
}
