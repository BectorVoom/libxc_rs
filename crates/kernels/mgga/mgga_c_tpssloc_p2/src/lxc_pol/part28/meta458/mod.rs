//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1665;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta458<F: Float>(t23956: F, t24446: F, t3: F, t112: F, t7222: F, t111: F, t2098: F, t671: F, t7056: F, t2039: F, t2363: F, t12521: F, t12524: F, t1401: F, t16535: F, t2319: F, t23917: F, t3938: F, t3941: F, t577: F, t7230: F, t7235: F, t191: F, t192: F, t5118: F, t1390: F, t5187: F, t531: F, t1982: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24447, t24448, t24462, t24465, t24478, t24481, t24486) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1665::<F>(t23956, t24446, t3, t112, t7222, t111, t2098, t671, t7056, t2039, t2363, t12521, t12524, t1401, t16535, t2319, t23917, t3938, t3941, t577, t7230, t7235);
        let (t24987, t24990, t24994, t24995) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1666::<F>(t191, t192, t5118, t1390, t5187, t531, t1982);
    (t24447, t24448, t24462, t24465, t24478, t24481, t24486, t24987, t24990, t24994, t24995)
}
