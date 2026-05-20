//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta278<F: Float>(t185: F, t20217: F, t707: F, t13115: F, t5499: F, t20777: F, t20815: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F, t9894: F) -> (F, F, F, F) {
        let (t20816, t20818, t20820, t20821) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk972::<F>(t185, t20217, t707, t13115, t5499, t20777, t20815, t9820, t9824, t9876, t9884, t9887, t9890, t9894);
    (t20816, t20818, t20820, t20821)
}
