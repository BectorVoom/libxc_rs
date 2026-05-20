//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta329<F: Float>(t1287: F, t2225: F, t3681: F, t750: F, t17: F, t1284: F, t2516: F, t521: F, t9861: F, t3826: F, t592: F, t1285: F) -> (F, F, F, F, F, F) {
        let (t12123, t12127, t12130, t12133, t12134, t12136) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1260::<F>(t1287, t2225, t3681, t750, t17, t1284, t2516, t521, t9861, t3826, t592, t1285);
    (t12123, t12127, t12130, t12133, t12134, t12136)
}
