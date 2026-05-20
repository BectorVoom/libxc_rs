//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta212 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1214;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1215;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1216;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1217;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1218;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1219;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1220;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1221;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1222;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1223;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1224;
use chunk11::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1225;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta212<F: Float>(t1230: F, t248: F, t5979: F, t5975: F, t5985: F, t5987: F, t5991: F, t6023: F, t6026: F, t6092: F, t6094: F, t6096: F, t6100: F, t6104: F, t6108: F, t475: F, t1214: F, t1734: F, t3508: F, t1213: F, t1227: F, t1737: F, t1748: F, t3506: F, t3515: F, t3542: F, t3547: F, t467: F, t5005: F, t5019: F, t5024: F, t5036: F, t5041: F, t6109: F, t6203: F, t6197: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t6207 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1214::<F>(t1230, t248, t5979);
        let t6211 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1215::<F>(t1230, t248, t5975);
        let t6218 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1216::<F>(t5985, t5987, t5991, t6023, t6026, t6092, t6094, t6096, t6100, t6104, t6108);
        let t6219 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1217::<F>(t475, t6218);
        let t6221 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1218::<F>(t1214, t248, t6219);
        let t6224 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1219::<F>(t1734);
        let t6225 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1220::<F>(t3508, t6224);
        let t6227 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1221::<F>(t1214, t248, t6225);
        let t6230 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1222::<F>(t475, t6224);
        let t6232 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1223::<F>(t1214, t248, t6230);
        let t6237 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1224::<F>(t1213, t1227, t1737, t1748, t3506, t3515, t3542, t3547, t467, t5005, t5019, t5024, t5036, t5041, t6109, t6203, t6207, t6211, t6221, t6227, t6232);
        let t6238 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1225::<F>(t6197, t6237);
    (t6207, t6211, t6218, t6219, t6221, t6224, t6225, t6227, t6230, t6232, t6238)
}
