//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta728 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2583;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta728<F: Float>(t11292: F, t1687: F, t11365: F, t1694: F, t3331: F, t4794: F, t14933: F, t300: F, t3401: F, t11310: F, t15823: F, t225: F) -> (F, F, F, F, F, F, F, F) {
        let (t51680, t51727, t51730, t51807, t51810, t51819, t51848, t51925) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2583::<F>(t11292, t1687, t11365, t1694, t3331, t4794, t14933, t300, t3401, t11310, t15823, t225);
    (t51680, t51727, t51730, t51807, t51810, t51819, t51848, t51925)
}
