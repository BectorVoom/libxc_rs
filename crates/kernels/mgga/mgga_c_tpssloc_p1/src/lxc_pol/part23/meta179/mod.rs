//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk806;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta179<F: Float>(t761: F, t9892: F, t152: F, t31: F, t2368: F, t2505: F, t745: F, t2509: F, t746: F, t9490: F) -> (F, F, F, F, F) {
        let (t9894, t9897, t9905) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk806::<F>(t761, t9892, t152, t31, t2368, t2505, t745);
        let (t9907, t9919) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk807::<F>(t761, t9905, t2509, t746, t9490);
    (t9894, t9897, t9905, t9907, t9919)
}
