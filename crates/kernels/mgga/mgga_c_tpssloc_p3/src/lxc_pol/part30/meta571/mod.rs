//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta571<F: Float>(t28609: F, t6784: F, t5685: F, t6785: F, t5681: F, t5936: F, t6800: F, t6799: F, t5932: F, t1948: F, t5914: F, t345: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28610, t28613, t28614, t28617, t28618, t28621, t28622, t28625, t28626, t28630, t28631) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1941::<F>(t28609, t6784, t5685, t6785, t5681, t5936, t6800, t6799, t5932, t1948, t5914, t345);
    (t28610, t28613, t28614, t28617, t28618, t28621, t28622, t28625, t28626, t28630, t28631)
}
