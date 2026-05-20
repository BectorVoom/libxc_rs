//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta647<F: Float>(t12283: F, t19894: F, t19981: F, t19986: F, t19823: F, t40021: F, t12211: F, t19827: F, t19831: F, t1351: F, t6330: F, t19541: F, t2663: F) -> (F, F, F, F, F, F, F, F) {
        let (t57127, t57143, t57145, t57158, t57160, t57170, t57172, t57211) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2187::<F>(t12283, t19894, t19981, t19986, t19823, t40021, t12211, t19827, t19831, t1351, t6330, t19541, t2663);
    (t57127, t57143, t57145, t57158, t57160, t57170, t57172, t57211)
}
