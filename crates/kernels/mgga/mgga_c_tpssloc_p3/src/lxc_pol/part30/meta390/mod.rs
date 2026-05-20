//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta390 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1478;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1479;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1480;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1481;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1482;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1483;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1484;
use chunk7::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1485;
use chunk8::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1486;
use chunk9::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1487;
use chunk10::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1488;
use chunk11::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1489;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta390<F: Float>(t10564: F, t17152: F, t123: F, t10277: F, t5392: F, t607: F, t2768: F, t3966: F, t4337: F, t5682: F, t690: F, t5677: F, t882: F, t4342: F, t5686: F, t2770: F, t5398: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t17154 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1478::<F>(t10564, t17152, t123);
        let t17157 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1479::<F>(t10277, t5392, t607);
        let t17159 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1480::<F>(t17157, t2768, t123);
        let t17161 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1481::<F>(t3966, t4337);
        let t17163 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1482::<F>(t17161, t2768, t123);
        let t17165 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1483::<F>(t5682, t690);
        let t17167 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1484::<F>(t5677, t607);
        let t17169 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1485::<F>(t17167, t882, t123);
        let t17171 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1486::<F>(t3966, t4342);
        let t17173 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1487::<F>(t17171, t882, t123);
        let t17175 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1488::<F>(t5686, t690);
        let t17178 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1489::<F>(t2770, t5398, t607);
    (t17154, t17157, t17159, t17161, t17163, t17165, t17167, t17169, t17171, t17173, t17175, t17178)
}
