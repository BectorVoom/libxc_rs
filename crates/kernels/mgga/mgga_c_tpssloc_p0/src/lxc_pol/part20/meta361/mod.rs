//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1689;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta361<F: Float>(t12512: F, t3: F, t112: F, t3931: F, t111: F, t1395: F, t2319: F, t671: F, t2363: F, t1401: F, t3938: F, t3941: F, t576: F, t577: F, t9416: F) -> (F, F, F, F, F, F) {
        let (t12513, t12521, t12524, t12529, t12532, t12537) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1689::<F>(t12512, t3, t112, t3931, t111, t1395, t2319, t671, t2363, t1401, t3938, t3941, t576, t577, t9416);
    (t12513, t12521, t12524, t12529, t12532, t12537)
}
