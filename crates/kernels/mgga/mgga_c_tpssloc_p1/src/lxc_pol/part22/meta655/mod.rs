//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta655<F: Float>(t16673: F, t2642: F, t41424: F, t5587: F, t13278: F, t4236: F, t5584: F, t828: F, t16946: F, t2697: F, t16951: F, t5614: F, t9671: F) -> (F, F, F, F, F, F, F) {
        let (t58642, t58668, t58670, t58688, t58705, t58709, t58723) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2197::<F>(t16673, t2642, t41424, t5587, t13278, t4236, t5584, t828, t16946, t2697, t16951, t5614, t9671);
    (t58642, t58668, t58670, t58688, t58705, t58709, t58723)
}
