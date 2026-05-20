//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1817;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta498<F: Float>(t22591: F, t7687: F, t1983: F, t1307: F, t1845: F, t8643: F, t22574: F, t15868: F, t2019: F, t1774: F, t6534: F, t652: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25985, t25987, t25988, t25989, t25991, t25992, t25993, t25994, t25996) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1817::<F>(t22591, t7687, t1983, t1307, t1845, t8643, t22574, t15868, t2019, t1774, t6534, t652);
    (t25985, t25987, t25988, t25989, t25991, t25992, t25993, t25994, t25996)
}
