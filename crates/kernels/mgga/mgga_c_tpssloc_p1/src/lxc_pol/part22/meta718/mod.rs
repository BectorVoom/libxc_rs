//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta718 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2326;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta718<F: Float>(t157: F, t4196: F, t57973: F, t46439: F, t59004: F, t59013: F, t41291: F, t59022: F, t59024: F, t59028: F, t59032: F, t59037: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t67494, t67495, t67496, t67497, t67498, t67499, t67500, t67501, t67502, t67503) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2326::<F>(t157, t4196, t57973, t46439, t59004, t59013, t41291, t59022, t59024, t59028, t59032, t59037);
    (t67494, t67495, t67496, t67497, t67498, t67499, t67500, t67501, t67502, t67503)
}
