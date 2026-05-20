//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta268 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1142;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1143;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1144;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1145;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1146;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta268<F: Float>(t2057: F, t7475: F, t1492: F, t2047: F, t7074: F, t7076: F, t7078: F, t7082: F, t7494: F, t7498: F, t7501: F, t7504: F, t7506: F, t7508: F, t218: F, t1527: F, t2053: F, t2718: F, t1510: F, t7101: F, t235: F, t1499: F, t2051: F, t226: F, t7095: F, t7097: F, t7522: F, t7526: F, t7530: F, t812: F, t858: F, t1528: F, t2054: F, t259: F, t4147: F, t4268: F, t7067: F, t7069: F, t7087: F, t7481: F, t7486: F, t7490: F, t855: F, t870: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7809, t7815, t7823) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1142::<F>(t2057, t7475, t1492, t2047, t7074, t7076, t7078, t7082, t7494, t7498, t7501, t7504, t7506, t7508);
        let (t7824, t7830, t7837, t7839, t7841) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1143::<F>(t218, t7823, t1527, t2053, t2718, t1510, t7101, t235, t1499, t2051, t226, t7095, t7097, t7522, t7526, t7530, t812);
        let t7842 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1144::<F>(t7841, t858);
        let t7844 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1145::<F>(t1528, t2054, t259, t4147, t4268, t7067, t7069, t7087, t7481, t7486, t7490, t7815, t7824, t7830, t7842, t855);
        let t7845 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1146::<F>(t7844, t870);
    (t7809, t7815, t7823, t7824, t7830, t7837, t7839, t7841, t7842, t7844, t7845)
}
