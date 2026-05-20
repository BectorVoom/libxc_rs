//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta239 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1121;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1122;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1123;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1124;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1125;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta239<F: Float>(t1458: F, t671: F, t1401: F, t3938: F, t3941: F, t4072: F, t5363: F, t5371: F, t577: F, t2235: F, t33: F, t645: F, t79: F, t72: F, t605: F, t608: F, t625: F, t641: F, t71: F, t1874: F, t2314: F, t4034: F, t1266: F, t1873: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5376, t5381, t6486) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1121::<F>(t1458, t671, t1401, t3938, t3941, t4072, t5363, t5371, t577, t2235, t33);
        let t6492 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1122::<F>(t645, t79, t72);
        let t6495 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1123::<F>(t605, t608);
        let (t6503, t6509) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1124::<F>(t625, t641, t71);
        let (t6522, t6524, t6525) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1125::<F>(t1874, t2314, t4034, t1266, t1873);
    (t5376, t5381, t6486, t6492, t6495, t6503, t6509, t6522, t6524, t6525)
}
