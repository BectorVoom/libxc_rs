//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1287;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta443<F: Float>(t12283: F, t20450: F, t20595: F, t68: F, t1340: F, t20556: F, t3799: F, t20570: F, t1362: F, t20512: F, t40021: F, t16288: F, t6422: F) -> (F, F, F, F, F, F, F, F) {
        let (t74276, t74289, t74290, t74297, t74299, t74311, t74360, t74376) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1287::<F>(t12283, t20450, t20595, t68, t1340, t20556, t3799, t20570, t1362, t20512, t40021, t16288, t6422);
    (t74276, t74289, t74290, t74297, t74299, t74311, t74360, t74376)
}
