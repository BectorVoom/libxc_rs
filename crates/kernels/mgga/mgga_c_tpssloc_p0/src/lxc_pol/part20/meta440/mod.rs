//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta440 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta440<F: Float>(t1113: F, t14753: F, t136: F, t14744: F, t11265: F, t1661: F, t3271: F, t11243: F, t3270: F, t4756: F, t1102: F, t3279: F, t4748: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14795, t14796, t14798, t14799, t14801, t14802, t14804, t14805, t14808, t14809, t14811) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1883::<F>(t1113, t14753, t136, t14744, t11265, t1661, t3271, t11243, t3270, t4756, t1102, t3279, t4748);
    (t14795, t14796, t14798, t14799, t14801, t14802, t14804, t14805, t14808, t14809, t14811)
}
