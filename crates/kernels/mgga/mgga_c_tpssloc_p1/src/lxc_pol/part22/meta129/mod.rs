//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta129 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk859;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk860;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk861;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk862;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk863;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk864;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta129<F: Float>(t1373: F, t225: F, t1376: F, t566: F, t68: F, t3787: F, t562: F, t1338: F, t1372: F, t193: F, t532: F, t1388: F, t1390: F, t531: F, t571: F, t112: F, t1395: F, t111: F, t576: F, t2218: F, t2221: F, t2225: F, t2232: F, t1406: F, t604: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t3882 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk859::<F>(t1373, t225);
        let t3887 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk860::<F>(t1376, t566, t68);
        let (t3897, t3901) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk861::<F>(t3787, t562, t1338, t1372);
        let t3918 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk862::<F>(t193, t532);
        let t3919 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk863::<F>(t1388, t1390);
        let (t3924, t3938, t3941) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk864::<F>(t531, t571, t112, t1395, t111, t576);
        let (t3951, t3953) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk865::<F>(t2218, t2221, t2225, t2232, t1406, t604);
    (t3882, t3887, t3897, t3901, t3918, t3919, t3924, t3938, t3941, t3951, t3953)
}
