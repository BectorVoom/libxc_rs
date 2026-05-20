//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta257<F: Float>(t248: F, t3516: F, t3570: F, t3515: F, t11154: F, t3585: F, t3493: F, t486: F, t4978: F, t4582: F, t3576: F, t3604: F) -> (F, F, F, F, F, F, F) {
        let (t11651, t11652, t11655, t11660, t11661, t11662, t11665) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1003::<F>(t248, t3516, t3570, t3515, t11154, t3585, t3493, t486, t4978, t4582, t3576, t3604);
    (t11651, t11652, t11655, t11660, t11661, t11662, t11665)
}
