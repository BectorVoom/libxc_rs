//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta289 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta289<F: Float>(t13360: F, t849: F, t13176: F, t842: F, t1516: F, t9601: F, t1509: F, t852: F, t252: F, t4233: F, t4290: F, t808: F) -> (F, F, F, F, F, F) {
        let (t13362, t13365, t13368, t13380, t13384, t13390) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1444::<F>(t13360, t849, t13176, t842, t1516, t9601, t1509, t852, t252, t4233, t4290, t808);
    (t13362, t13365, t13368, t13380, t13384, t13390)
}
