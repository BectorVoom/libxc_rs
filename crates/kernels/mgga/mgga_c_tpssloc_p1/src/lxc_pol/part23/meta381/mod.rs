//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta381 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1183;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta381<F: Float>(t1020: F, t1616: F, t248: F, t43216: F, t10882: F, t48569: F, t10875: F, t1606: F, t2402: F, t973: F, t1654: F, t9698: F) -> (F, F, F, F, F) {
        let (t50181, t50193, t50265, t50425, t50834) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1183::<F>(t1020, t1616, t248, t43216, t10882, t48569, t10875, t1606, t2402, t973, t1654, t9698);
    (t50181, t50193, t50265, t50425, t50834)
}
