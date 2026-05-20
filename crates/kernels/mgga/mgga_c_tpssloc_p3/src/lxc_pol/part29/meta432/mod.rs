//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1727;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta432<F: Float>(t22666: F, t6907: F, t1985: F, t225: F, t6956: F, t562: F, t794: F, t6897: F, t12030: F, t12444: F, t1375: F, t1386: F, t2016: F, t22622: F, t22624: F, t22630: F, t22639: F, t22646: F, t22650: F, t22653: F, t22656: F, t22664: F, t3882: F, t3912: F, t568: F, t6958: F, t6963: F, t6993: F) -> (F, F, F, F, F, F) {
        let (t22667, t22668, t22670, t22674) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1727::<F>(t22666, t6907, t1985, t225, t6956, t562, t794);
        let (t22675, t22676, t22680) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1728::<F>(t22674, t6907, t6897, t12030, t12444, t1375, t1386, t2016, t22622, t22624, t22630, t22639, t22646, t22650, t22653, t22656, t22664, t22668, t22670, t3882, t3912, t568, t6958, t6963, t6993);
    (t22667, t22670, t22674, t22675, t22676, t22680)
}
