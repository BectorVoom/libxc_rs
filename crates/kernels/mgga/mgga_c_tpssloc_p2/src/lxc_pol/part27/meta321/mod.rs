//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1394;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta321<F: Float>(t11135: F, t11203: F, t135: F, t3477: F, t1174: F, t1176: F, t698: F, t1179: F, t3431: F, t3460: F, t3456: F, t3439: F) -> (F, F, F, F, F, F, F, F) {
        let (t11459, t11487, t11514, t11529, t11531, t11534, t11537, t11539) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1394::<F>(t11135, t11203, t135, t3477, t1174, t1176, t698, t1179, t3431, t3460, t3456, t3439);
    (t11459, t11487, t11514, t11529, t11531, t11534, t11537, t11539)
}
