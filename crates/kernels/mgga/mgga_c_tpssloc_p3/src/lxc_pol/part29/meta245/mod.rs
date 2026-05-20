//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1148;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1149;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1150;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1151;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1152;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1153;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1154;
use chunk7::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta245<F: Float>(t6613: F, t812: F, t831: F, t1899: F, t838: F, t234: F, t59: F, t240: F, t849: F, t6580: F, t6582: F, t6587: F, t6594: F, t6603: F, t6607: F, t6610: F, t218: F, t1903: F, t225: F, t1911: F, t865: F, t2718: F, t1906: F, t6547: F, t214: F, t252: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t6614 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1148::<F>(t6613, t812);
        let (t6615, t6618, t6619, t6620) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1149::<F>(t6614, t831, t1899, t838, t234, t59, t240);
        let t6621 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1150::<F>(t6620, t812);
        let t6624 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1151::<F>(t6621, t849, t6580, t6582, t6587, t6594, t6603, t6607, t6610, t6615, t6618);
        let (t6625, t6627) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1152::<F>(t218, t6624, t1903, t225);
        let t6632 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1153::<F>(t1911, t865, t2718);
        let (t6636, t6637) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1154::<F>(t1906, t6547, t214, t225);
        let t6638 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1155::<F>(t234, t252);
    (t6614, t6618, t6619, t6620, t6621, t6624, t6625, t6627, t6632, t6636, t6637, t6638)
}
