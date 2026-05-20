//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta291<F: Float>(t12168: F, t1343: F, t820: F, t3799: F, t3858: F, t12267: F, t1340: F, t120: F, t3850: F, t3805: F, t3807: F, t3719: F, t550: F) -> (F, F, F, F, F, F) {
        let (t12392, t12395, t12397, t12402, t12404, t12407) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1064::<F>(t12168, t1343, t820, t3799, t3858, t12267, t1340, t120, t3850, t3805, t3807, t3719, t550);
    (t12392, t12395, t12397, t12402, t12404, t12407)
}
