//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1633;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta473<F: Float>(t22705: F, t26243: F, t550: F, t22852: F, t2002: F, t5230: F, t559: F, t1358: F, t7715: F, t1831: F, t22783: F, t5234: F, t6951: F) -> (F, F, F, F, F, F, F) {
        let (t26245, t26246, t26248, t26249, t26251, t26255, t26257) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1633::<F>(t22705, t26243, t550, t22852, t2002, t5230, t559, t1358, t7715, t1831, t22783, t5234, t6951);
    (t26245, t26246, t26248, t26249, t26251, t26255, t26257)
}
