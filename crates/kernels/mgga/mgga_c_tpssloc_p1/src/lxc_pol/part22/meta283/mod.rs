//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta283<F: Float>(t13012: F, t4130: F, t2563: F, t4138: F, t4134: F, t9546: F, t118: F, t4119: F, t794: F, t2576: F, t225: F, t4266: F) -> (F, F, F, F, F, F) {
        let (t13014, t13020, t13022, t13025, t13027, t13042) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1432::<F>(t13012, t4130, t2563, t4138, t4134, t9546, t118, t4119, t794, t2576, t225, t4266);
    (t13014, t13020, t13022, t13025, t13027, t13042)
}
