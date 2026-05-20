//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta699 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta699<F: Float>(t2770: F, t340: F, t2403: F, t4389: F, t4386: F, t13543: F, t699: F, t13547: F, t13556: F, t13529: F, t13533: F, t344: F, t42308: F, t60: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t48143, t48155, t48157, t48159, t48161, t48163, t48165, t48167, t48180) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2527::<F>(t2770, t340, t2403, t4389, t4386, t13543, t699, t13547, t13556, t13529, t13533, t344, t42308, t60);
    (t48143, t48155, t48157, t48159, t48161, t48163, t48165, t48167, t48180)
}
