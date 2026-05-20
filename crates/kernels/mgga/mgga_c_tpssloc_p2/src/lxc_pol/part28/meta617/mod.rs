//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1934;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta617<F: Float>(t7712: F, t80939: F, t22683: F, t26285: F, t6546: F, t16148: F, t221: F, t26284: F, t16153: F, t26289: F, t6604: F, t80887: F, t16217: F, t6952: F, t1827: F, t80910: F, t22756: F, t5289: F, t16208: F, t6945: F, t16060: F, t6951: F, t1369: F, t1878: F, t80730: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t91167, t91170, t91173, t91176, t91179) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1934::<F>(t7712, t80939, t22683, t26285, t6546, t16148, t221, t26284, t16153, t26289, t6604, t80887);
        let (t91183, t91185, t91187, t91189, t91192, t91194) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1935::<F>(t16217, t6952, t1827, t80910, t22756, t5289, t16208, t6945, t16060, t6951, t1369, t1878, t80730);
    (t91167, t91170, t91173, t91176, t91179, t91183, t91185, t91187, t91189, t91192, t91194)
}
