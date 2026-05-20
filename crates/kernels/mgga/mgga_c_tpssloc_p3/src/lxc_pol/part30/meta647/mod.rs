//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2061;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta647<F: Float>(t1955: F, t43603: F, t4657: F, t6688: F, t7566: F, t82632: F, t23384: F, t25400: F, t25416: F, t82431: F, t1921: F, t88804: F) -> (F, F, F, F, F, F) {
        let (t88851, t88868, t88882, t88889, t88915, t88932) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2061::<F>(t1955, t43603, t4657, t6688, t7566, t82632, t23384, t25400, t25416, t82431, t1921, t88804);
    (t88851, t88868, t88882, t88889, t88915, t88932)
}
