//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2136;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta573<F: Float>(t2402: F, t976: F, t973: F, t979: F, t2955: F, t2967: F, t986: F, t3010: F, t698: F, t10327: F, t135: F, t10286: F, t2960: F) -> (F, F, F, F, F, F, F) {
        let (t42891, t42893, t42895, t42903, t42906, t42909, t42911) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2136::<F>(t2402, t976, t973, t979, t2955, t2967, t986, t3010, t698, t10327, t135, t10286, t2960);
    (t42891, t42893, t42895, t42903, t42906, t42909, t42911)
}
