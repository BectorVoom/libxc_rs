//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta264<F: Float>(t11778: F, t61: F, t11148: F, t248: F, t121: F, t3584: F, t3243: F, t1227: F, t1229: F, t676: F, t1090: F, t3536: F, t3572: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11779, t11781, t11784, t11786, t11787, t11789, t11791, t11792, t11794) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1015::<F>(t11778, t61, t11148, t248, t121, t3584, t3243, t1227, t1229, t676, t1090, t3536, t3572);
    (t11779, t11781, t11784, t11786, t11787, t11789, t11791, t11792, t11794)
}
