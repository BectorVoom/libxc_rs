//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1435;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1436;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta285<F: Float>(t13123: F, t2375: F, t184: F, t3966: F, t4094: F, t706: F, t68: F, t822: F, t1484: F, t1891: F, t4119: F, t845: F, t4162: F) -> (F, F, F, F, F, F, F) {
        let (t13124, t13126, t13133, t13151, t13156, t13160) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1435::<F>(t13123, t2375, t184, t3966, t4094, t706, t68, t822, t1484, t1891, t4119, t845);
        let t13176 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1436::<F>(t4162, t68);
    (t13124, t13126, t13133, t13151, t13156, t13160, t13176)
}
