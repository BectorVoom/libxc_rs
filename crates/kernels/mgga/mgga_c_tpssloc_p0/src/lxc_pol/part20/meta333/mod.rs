//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta333<F: Float>(t11552: F, t221: F, t456: F, t1197: F, t698: F, t1174: F, t135: F, t3551: F, t3556: F, t1196: F, t9258: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11832, t11834, t11835, t11836, t11838, t11839, t11841, t11842, t11844, t11845) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1622::<F>(t11552, t221, t456, t1197, t698, t1174, t135, t3551, t3556, t1196, t9258, t974);
    (t11832, t11834, t11835, t11836, t11838, t11839, t11841, t11842, t11844, t11845)
}
