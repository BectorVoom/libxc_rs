//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta631 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1891;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta631<F: Float>(t19996: F, t6952: F, t26257: F, t5310: F, t1358: F, t28088: F, t22852: F, t3792: F, t80798: F, t97312: F, t22705: F, t236: F, t550: F, t6414: F, t22765: F, t6417: F, t6390: F, t80997: F, t81000: F, t1351: F, t3788: F, t6388: F, t6936: F, t19958: F, t22833: F, t22797: F, t6375: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97359, t97361, t97363, t97367, t97372) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1891::<F>(t19996, t6952, t26257, t5310, t1358, t28088, t22852, t3792, t80798, t97312, t22705, t236, t550, t6414);
        let (t97378, t97380, t97382, t97387, t97389, t97394) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1892::<F>(t22765, t6417, t6390, t80997, t81000, t1351, t3788, t6388, t6936, t19958, t22833, t22797, t6375);
    (t97359, t97361, t97363, t97367, t97372, t97378, t97380, t97382, t97387, t97389, t97394)
}
