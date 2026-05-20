//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1558;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1559;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1560;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1561;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1562;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1563;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1564;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1565;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1566;
use chunk9::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1567;
use chunk10::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1568;
use chunk11::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1569;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta406<F: Float>(t11153: F, t5392: F, t607: F, t3240: F, t123: F, t3966: F, t4723: F, t5976: F, t690: F, t5971: F, t1088: F, t4728: F, t5980: F, t3242: F, t5398: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t18211 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1558::<F>(t11153, t5392, t607);
        let t18213 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1559::<F>(t18211, t3240, t123);
        let t18215 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1560::<F>(t3966, t4723);
        let t18217 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1561::<F>(t18215, t3240, t123);
        let t18219 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1562::<F>(t5976, t690);
        let t18221 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1563::<F>(t5971, t607);
        let t18223 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1564::<F>(t1088, t18221, t123);
        let t18225 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1565::<F>(t3966, t4728);
        let t18227 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1566::<F>(t1088, t18225, t123);
        let t18229 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1567::<F>(t5980, t690);
        let t18232 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1568::<F>(t3242, t5398, t607);
        let t18234 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1569::<F>(t18232, t3240, t123);
    (t18211, t18213, t18215, t18217, t18219, t18221, t18223, t18225, t18227, t18229, t18232, t18234)
}
