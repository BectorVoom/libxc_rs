//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1691;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta496<F: Float>(t5493: F, t88: F, t22473: F, t5464: F, t5488: F, t6530: F, t89: F, t3788: F, t6388: F, t6936: F, t1339: F, t6420: F) -> (F, F, F, F, F, F, F) {
        let (t28007, t28012, t28014, t28030, t28057, t28058, t28060) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1691::<F>(t5493, t88, t22473, t5464, t5488, t6530, t89, t3788, t6388, t6936, t1339, t6420);
    (t28007, t28012, t28014, t28030, t28057, t28058, t28060)
}
