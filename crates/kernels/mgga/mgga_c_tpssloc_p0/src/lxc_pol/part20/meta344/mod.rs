//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1645;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta344<F: Float>(t2663: F, t3814: F, t3681: F, t67: F, t758: F, t1294: F, t9905: F, t9892: F, t3826: F, t588: F, t3684: F, t9467: F, t118: F, t1284: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12097, t12098, t12099, t12100, t12101, t12103, t12105, t12107, t12109) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1645::<F>(t2663, t3814, t3681, t67, t758, t1294, t9905, t9892, t3826, t588, t3684, t9467);
        let t12110 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1646::<F>(t118, t1284);
    (t12097, t12098, t12099, t12100, t12101, t12103, t12105, t12107, t12109, t12110)
}
