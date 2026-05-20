//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2434;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta643<F: Float>(t42340: F, t42341: F, t3034: F, t368: F, t3128: F, t10882: F, t42333: F, t1015: F, t10477: F, t67: F, t3067: F, t11059: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t42342, t42344, t42345, t42347, t42354, t42358, t42386, t42387, t42388) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2434::<F>(t42340, t42341, t3034, t368, t3128, t10882, t42333, t1015, t10477, t67, t3067, t11059);
    (t42342, t42344, t42345, t42347, t42354, t42358, t42386, t42387, t42388)
}
