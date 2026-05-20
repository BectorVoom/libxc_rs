//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2429;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta639<F: Float>(t273: F, t41654: F, t242: F, t281: F, t283: F, t2853: F, t2860: F, t10770: F, t919: F, t2897: F, t2904: F, t10701: F, t888: F) -> (F, F, F, F, F, F, F, F) {
        let (t41942, t41959, t41961, t41962, t41981, t41984, t42020, t42023) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2429::<F>(t273, t41654, t242, t281, t283, t2853, t2860, t10770, t919, t2897, t2904, t10701, t888);
    (t41942, t41959, t41961, t41962, t41981, t41984, t42020, t42023)
}
