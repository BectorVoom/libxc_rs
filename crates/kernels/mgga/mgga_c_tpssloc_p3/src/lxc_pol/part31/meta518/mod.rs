//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1720;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1721;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1722;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1723;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta518<F: Float>(t29009: F, t29054: F, t858: F, t2053: F, t2718: F, t5657: F, t218: F, t29040: F, t1528: F, t17090: F, t2054: F, t25036: F, t25049: F, t259: F, t26713: F, t28265: F, t28269: F, t28274: F, t28278: F, t28289: F, t28296: F, t28300: F, t4147: F, t4268: F, t5637: F, t7087: F, t7830: F, t855: F, t1527: F, t7841: F, t10110: F, t5636: F, t2047: F, t5558: F, t1492: F, t7823: F, t17052: F, t17092: F, t24291: F, t24318: F, t24321: F, t25206: F, t25209: F, t25211: F, t25346: F, t26700: F, t28440: F, t5658: F, t7842: F, t870: F, t1408: F, t1877: F, t2057: F, t24191: F, t24344: F, t25: F, t2522: F, t26744: F, t28249: F, t28252: F, t28256: F, t28456: F, t28459: F, t28462: F, t28972: F, t4314: F, t5397: F, t7114: F, t7475: F, t7545: F, t7845: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t29055, t29056, t29060, t29071, t29075) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1720::<F>(t29009, t29054, t858, t2053, t2718, t5657, t218, t29040, t1528, t17090, t2054, t25036, t25049, t259, t26713, t28265, t28269, t28274, t28278, t28289, t28296, t28300, t4147, t4268, t5637, t7087, t7830, t855);
        let (t29080, t29091, t29095, t29099, t29104) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1721::<F>(t1527, t7841, t2718, t10110, t2053, t5636, t2047, t5558, t1492, t7823, t1528, t17052, t17092, t2054, t24291, t24318, t24321, t25206, t25209, t25211, t25346, t259, t26700, t28440, t4147, t4268, t5658, t7087, t7842, t855);
        let (t29105, t29106) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1722::<F>(t29075, t29104, t870);
        let t29124 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1723::<F>(t1408, t1877, t2057, t24191, t24344, t25, t2522, t26744, t28249, t28252, t28256, t28456, t28459, t28462, t28972, t29106, t4314, t5397, t7114, t7475, t7545, t7845);
    (t29055, t29056, t29060, t29071, t29080, t29091, t29095, t29099, t29105, t29106, t29124)
}
