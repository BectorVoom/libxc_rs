//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1105;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1106;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1107;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1108;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1109;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta255<F: Float>(t6635: F, t6644: F, t2047: F, t814: F, t829: F, t235: F, t7084: F, t2051: F, t226: F, t6641: F, t6650: F, t6654: F, t808: F, t812: F, t858: F, t2054: F, t259: F, t2597: F, t2713: F, t6557: F, t6569: F, t6574: F, t7067: F, t7069: F, t7072: F, t7085: F, t7087: F, t7092: F, t855: F, t866: F, t870: F, t2056: F, t2752: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t7095, t7097, t7101) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1105::<F>(t6635, t6644, t2047, t814);
        let (t7102, t7104, t7106) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1106::<F>(t7101, t829, t235, t7084, t2051, t226, t6641, t6650, t6654, t7095, t7097, t808, t812);
        let t7107 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1107::<F>(t7106, t858);
        let t7109 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1108::<F>(t2054, t259, t2597, t2713, t6557, t6569, t6574, t7067, t7069, t7072, t7085, t7087, t7092, t7107, t855, t866);
        let t7110 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1109::<F>(t7109, t870);
        let t7114 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1110::<F>(t2056, t2752);
    (t7095, t7097, t7101, t7102, t7104, t7106, t7107, t7109, t7110, t7114)
}
