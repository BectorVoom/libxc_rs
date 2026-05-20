//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2141;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta613<F: Float>(t50919: F, t50948: F, t3356: F, t4794: F, t11349: F, t1675: F, t11285: F, t4857: F, t1670: F, t3313: F, t3375: F, t4832: F) -> (F, F, F, F, F, F, F) {
        let (t51565, t51574, t51599, t51604, t51651, t51667, t51677) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2141::<F>(t50919, t50948, t3356, t4794, t11349, t1675, t11285, t4857, t1670, t3313, t3375, t4832);
    (t51565, t51574, t51599, t51604, t51651, t51667, t51677)
}
