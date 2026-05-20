//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta407<F: Float>(t5914: F, t990: F, t17875: F, t381: F, t1049: F, t5848: F, t1065: F, t5943: F, t3174: F, t1625: F, t4552: F, t5919: F) -> (F, F, F, F, F, F, F) {
        let (t18053, t18057, t18059, t18061, t18062, t18065, t18070) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1546::<F>(t5914, t990, t17875, t381, t1049, t5848, t1065, t5943, t3174, t1625, t4552, t5919);
    (t18053, t18057, t18059, t18061, t18062, t18065, t18070)
}
