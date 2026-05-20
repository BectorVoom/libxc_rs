//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta706 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta706<F: Float>(t10771: F, t1568: F, t10756: F, t1580: F, t2930: F, t2885: F, t4408: F, t10813: F, t4433: F, t13716: F, t2932: F, t10632: F, t4471: F) -> (F, F, F, F, F, F, F) {
        let (t48776, t48779, t48783, t48789, t48854, t48883, t48890) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2539::<F>(t10771, t1568, t10756, t1580, t2930, t2885, t4408, t10813, t4433, t13716, t2932, t10632, t4471);
    (t48776, t48779, t48783, t48789, t48854, t48883, t48890)
}
