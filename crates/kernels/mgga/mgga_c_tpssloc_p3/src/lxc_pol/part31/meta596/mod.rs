//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta596<F: Float>(t26028: F, t7032: F, t26016: F, t84173: F, t26959: F, t6486: F, t1860: F, t26024: F, t7031: F, t2031: F, t90090: F, t26012: F) -> (F, F, F, F, F, F) {
        let (t92008, t92012, t92031, t92034, t92040, t92047) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1841::<F>(t26028, t7032, t26016, t84173, t26959, t6486, t1860, t26024, t7031, t2031, t90090, t26012);
    (t92008, t92012, t92031, t92034, t92040, t92047)
}
