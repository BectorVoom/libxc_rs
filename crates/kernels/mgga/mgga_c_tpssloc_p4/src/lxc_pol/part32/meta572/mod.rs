//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta572<F: Float>(t5161: F, t7753: F, t1983: F, t26167: F, t7687: F, t191: F, t192: F, t6295: F, t2020: F, t20085: F, t2019: F, t1390: F, t6330: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28813, t28815, t28817, t28819, t28821, t28822, t28823, t28825, t28826) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1947::<F>(t5161, t7753, t1983, t26167, t7687, t191, t192, t6295, t2020, t20085, t2019, t1390, t6330);
    (t28813, t28815, t28817, t28819, t28821, t28822, t28823, t28825, t28826)
}
