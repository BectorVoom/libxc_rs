//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1895;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta520<F: Float>(t2314: F, t7468: F, t4034: F, t1266: F, t7467: F, t652: F, t6876: F, t7756: F, t645: F, t72: F, t7431: F, t1437: F, t1864: F) -> (F, F, F, F, F, F, F) {
        let (t25998, t26002, t26003, t26005, t26006, t26009, t26012) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1895::<F>(t2314, t7468, t4034, t1266, t7467, t652, t6876, t7756, t645, t72, t7431, t1437, t1864);
    (t25998, t26002, t26003, t26005, t26006, t26009, t26012)
}
