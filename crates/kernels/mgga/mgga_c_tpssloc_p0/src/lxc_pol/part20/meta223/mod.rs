//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta223 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1298;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta223<F: Float>(t31: F, t9258: F, t65: F, t2251: F, t628: F, t2283: F, t608: F, t36: F, t366: F, t41: F, t42: F, t2244: F, t607: F, sigma0: F) -> (F, F, F, F, F, F, F) {
        let (t9259, t9260, t9263, t9268, t9277, t9287, t9288) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1298::<F>(t31, t9258, t65, t2251, t628, t2283, t608, t36, t366, t41, t42, t2244, t607, sigma0);
    (t9259, t9260, t9263, t9268, t9277, t9287, t9288)
}
