//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta512 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta512<F: Float>(t343: F, t381: F, t6690: F, t25712: F, t4347: F, t6689: F, t7561: F, t968: F, t1920: F, t1625: F, t6688: F, t6691: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25796, t25797, t25798, t25801, t25802, t25806, t25807, t25810, t25811) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1836::<F>(t343, t381, t6690, t25712, t4347, t6689, t7561, t968, t1920, t1625, t6688, t6691);
    (t25796, t25797, t25798, t25801, t25802, t25806, t25807, t25810, t25811)
}
