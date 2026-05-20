//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1826;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta585<F: Float>(t215: F, t6916: F, t225: F, t3787: F, t562: F, t22751: F, t26385: F, t81149: F, t81187: F, t81197: F, t26389: F, t26467: F, t6914: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91004, t91005, t91006, t91010, t91018, t91043, t91045, t91064, t91076) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1826::<F>(t215, t6916, t225, t3787, t562, t22751, t26385, t81149, t81187, t81197, t26389, t26467, t6914);
    (t91004, t91005, t91006, t91010, t91018, t91043, t91045, t91064, t91076)
}
