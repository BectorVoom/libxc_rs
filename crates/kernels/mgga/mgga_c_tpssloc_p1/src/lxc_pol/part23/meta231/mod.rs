//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta231 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta231<F: Float>(t5519: F, t706: F, t13115: F, t157: F, t5398: F, t751: F, t707: F, t5522: F, t67: F, t758: F, t184: F, t5392: F) -> (F, F, F, F, F, F, F) {
        let (t16689, t16693, t16701, t16702, t16710, t16711, t16716) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk881::<F>(t5519, t706, t13115, t157, t5398, t751, t707, t5522, t67, t758, t184, t5392);
    (t16689, t16693, t16701, t16702, t16710, t16711, t16716)
}
