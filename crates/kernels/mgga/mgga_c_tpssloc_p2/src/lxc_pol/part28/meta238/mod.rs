//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta238 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1039;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1040;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1041;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1042;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1043;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1044;
use chunk6::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta238<F: Float>(t1458: F, t671: F, t1401: F, t3938: F, t3941: F, t4072: F, t5363: F, t5371: F, t577: F, t2235: F, t33: F, t645: F, t79: F, t72: F, t605: F, t608: F, t641: F, t71: F, t107: F, t625: F, t63: F, t656: F, t666: F, t25: F, t776: F, t154: F, t781: F, t1879: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5376, t5381, t6486) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1039::<F>(t1458, t671, t1401, t3938, t3941, t4072, t5363, t5371, t577, t2235, t33);
        let t6492 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1040::<F>(t645, t79, t72);
        let t6495 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1041::<F>(t605, t608);
        let t6509 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1042::<F>(t641, t71);
        let (t6528, t6530) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1043::<F>(t107, t625, t63, t656);
        let (t6531, t6542, t6546) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1044::<F>(t6530, t666, t25, t776, t154, t781);
        let t6547 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1045::<F>(t1879, t6546);
    (t5376, t5381, t6486, t6492, t6495, t6509, t6528, t6530, t6531, t6542, t6546, t6547)
}
