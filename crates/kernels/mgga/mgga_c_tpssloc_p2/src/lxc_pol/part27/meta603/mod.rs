//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta603<F: Float>(t23610: F, t23665: F, t3032: F, t3131: F, t23614: F, t82431: F, t23384: F, t23693: F, t23698: F, t3166: F, t362: F, t23383: F, t6712: F) -> (F, F, F, F, F, F, F) {
        let (t82539, t82542, t82555, t82562, t82564, t82566, t82573) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2073::<F>(t23610, t23665, t3032, t3131, t23614, t82431, t23384, t23693, t23698, t3166, t362, t23383, t6712);
    (t82539, t82542, t82555, t82562, t82564, t82566, t82573)
}
