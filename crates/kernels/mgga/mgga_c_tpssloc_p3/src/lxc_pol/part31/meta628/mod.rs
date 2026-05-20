//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1885;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta628<F: Float>(t22765: F, t6422: F, t19921: F, t6952: F, t19926: F, t22756: F, t22783: F, t6431: F, t1831: F, t91160: F, t19815: F, t6951: F, t1369: F, t1339: F, t1824: F, t22827: F, t5187: F, t550: F, t74677: F, t1307: F, t3788: F, t6388: F, t6427: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97253, t97255, t97257, t97259, t97261, t97263, t97265) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1885::<F>(t22765, t6422, t19921, t6952, t19926, t22756, t22783, t6431, t1831, t91160, t19815, t6951);
        let (t97266, t97273, t97277, t97281, t97283) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1886::<F>(t1369, t97265, t1339, t1824, t22827, t5187, t550, t74677, t1307, t3788, t6388, t22783, t6427);
    (t97253, t97255, t97257, t97259, t97261, t97263, t97266, t97273, t97277, t97281, t97283)
}
