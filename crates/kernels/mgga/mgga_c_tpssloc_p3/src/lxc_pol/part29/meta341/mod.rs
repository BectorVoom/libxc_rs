//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1401;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta341<F: Float>(t28: F, t528: F, t1294: F, t9722: F, t172: F, t3681: F, t763: F, t2528: F, t3691: F, t9919: F, t2663: F, t3814: F, t67: F, t758: F, t9905: F, t9892: F, t3684: F, t9467: F, t118: F, t1284: F, t2375: F, t9882: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12072, t12087, t12089, t12091, t12094, t12097) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1401::<F>(t28, t528, t1294, t9722, t172, t3681, t763, t2528, t3691, t9919, t2663, t3814);
        let (t12100, t12103, t12105, t12109, t12111, t12114) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1402::<F>(t3681, t67, t758, t1294, t9905, t9892, t3684, t9467, t118, t1284, t2375, t9882);
    (t12072, t12087, t12089, t12091, t12094, t12097, t12100, t12103, t12105, t12109, t12111, t12114)
}
