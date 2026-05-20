//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1857;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta612<F: Float>(t91486: F, t1404: F, t7945: F, t2105: F, t5363: F, t2098: F, t5381: F, t27286: F, t576: F, t112: F, t27240: F, t111: F) -> (F, F, F, F, F, F, F) {
        let (t93873, t94113, t94118, t94120, t94122, t94127, t94170) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1857::<F>(t91486, t1404, t7945, t2105, t5363, t2098, t5381, t27286, t576, t112, t27240, t111);
    (t93873, t94113, t94118, t94120, t94122, t94127, t94170)
}
