//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1550;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1551;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta346<F: Float>(t16781: F, t16803: F, t225: F, t10054: F, t5585: F, t13176: F, t1499: F, t1523: F, t1525: F, t16673: F, t16679: F, t16754: F, t16756: F, t16759: F, t16762: F, t255: F, t2617: F, t4162: F, t4166: F, t4286: F, t4291: F, t4296: F, t4298: F, t5645: F, t5648: F, t5653: F, t812: F, t861: F, t252: F, t5584: F, t828: F, t9975: F) -> (F, F, F, F, F, F) {
        let (t16804, t16805, t16811, t16814) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1550::<F>(t16781, t16803, t225, t10054, t5585, t13176, t1499, t1523, t1525, t16673, t16679, t16754, t16756, t16759, t16762, t255, t2617, t4162, t4166, t4286, t4291, t4296, t4298, t5645, t5648, t5653, t812, t861);
        let t16815 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1551::<F>(t252, t5584);
        let t16816 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1552::<F>(t828, t9975);
    (t16804, t16805, t16811, t16814, t16815, t16816)
}
