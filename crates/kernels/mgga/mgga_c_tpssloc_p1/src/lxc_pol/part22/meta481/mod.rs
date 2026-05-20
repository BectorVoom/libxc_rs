//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta481 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1888;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta481<F: Float>(t21089: F, t2929: F, t951: F, t959: F, t10523: F, t2932: F, t1589: F, t17934: F, t10629: F, t10632: F, t4483: F, t5808: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21091, t21093, t21094, t21095, t21097, t21099, t21100, t21101, t21103, t21105) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1888::<F>(t21089, t2929, t951, t959, t10523, t2932, t1589, t17934, t10629, t10632, t4483, t5808);
    (t21091, t21093, t21094, t21095, t21097, t21099, t21100, t21101, t21103, t21105)
}
