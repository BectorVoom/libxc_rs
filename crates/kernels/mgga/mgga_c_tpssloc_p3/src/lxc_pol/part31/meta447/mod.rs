//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta447<F: Float>(t112: F, t7222: F, t111: F, t2098: F, t191: F, t192: F, t5118: F, t1390: F, t5187: F, t531: F, t1982: F) -> (F, F, F, F, F, F) {
        let (t24462, t24465, t24987, t24990, t24994, t24995) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1597::<F>(t112, t7222, t111, t2098, t191, t192, t5118, t1390, t5187, t531, t1982);
    (t24462, t24465, t24987, t24990, t24994, t24995)
}
