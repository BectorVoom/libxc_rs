//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1505;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1506;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta321<F: Float>(t1213: F, t15730: F, t11789: F, t1653: F, t248: F, t1227: F, t15437: F, t3505: F, t3576: F, t5064: F) -> (F, F, F, F, F) {
        let (t15731, t15734, t15735, t15737) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1505::<F>(t1213, t15730, t11789, t1653, t248, t1227, t15437, t3505);
        let t15740 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1506::<F>(t3576, t5064);
    (t15731, t15734, t15735, t15737, t15740)
}
