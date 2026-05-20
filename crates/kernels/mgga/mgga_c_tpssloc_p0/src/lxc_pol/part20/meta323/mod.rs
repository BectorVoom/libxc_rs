//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1595;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta323<F: Float>(t11638: F, t491: F, t1246: F, t1222: F, t3567: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F, t485: F, t248: F, t3516: F, t3570: F, t3515: F, t11154: F, t3585: F, t3493: F, t4978: F, t4582: F, t3576: F, t3604: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11640, t11642, t11644, t11647, t11649, t11651) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1595::<F>(t11638, t491, t1246, t1222, t3567, t1203, t3540, t2393, t374, t486, t485, t248, t3516, t3570);
        let (t11652, t11655, t11660, t11661, t11662, t11665) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1596::<F>(t11651, t3515, t11154, t248, t3585, t3493, t486, t4978, t4582, t3576, t3604);
    (t11640, t11642, t11644, t11647, t11649, t11651, t11652, t11655, t11660, t11661, t11662, t11665)
}
