//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1042;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta278<F: Float>(t3824: F, t588: F, t1287: F, t2225: F, t12083: F, t184: F, t17: F, t3681: F, t750: F, t1284: F, t2516: F, t521: F, t9861: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12121, t12123, t12124, t12125, t12126, t12128, t12129, t12131, t12132) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1042::<F>(t3824, t588, t1287, t2225, t12083, t184, t17, t3681, t750, t1284, t2516, t521, t9861);
    (t12121, t12123, t12124, t12125, t12126, t12128, t12129, t12131, t12132)
}
