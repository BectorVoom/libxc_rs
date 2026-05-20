//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta584<F: Float>(t24815: F, t6252: F, t24814: F, t24821: F, t24820: F, t5979: F, t7363: F, t7362: F, t5975: F, t29664: F, t493: F, t5971: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29749, t29750, t29753, t29754, t29758, t29759, t29762, t29763, t29773, t29776) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1967::<F>(t24815, t6252, t24814, t24821, t24820, t5979, t7363, t7362, t5975, t29664, t493, t5971);
    (t29749, t29750, t29753, t29754, t29758, t29759, t29762, t29763, t29773, t29776)
}
