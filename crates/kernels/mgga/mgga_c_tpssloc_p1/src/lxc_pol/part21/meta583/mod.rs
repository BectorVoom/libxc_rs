//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2311;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2312;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2313;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta583<F: Float>(t19676: F, t19679: F, t19688: F, t19699: F, t225: F, t1819: F, t68: F, t1995: F, t6330: F, t1307: F, t5187: F, t5279: F, t1365: F, t6347: F, t1347: F, t19631: F, t1345: F, t1348: F, t1821: F, t5272: F, t5278: F, t5280: F, t5283: F, t546: F, t548: F, t6404: F, t6408: F, t6411: F, t550: F, t1380: F, t3792: F, t5286: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19702, t19708, t19715, t19716, t19719) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2311::<F>(t19676, t19679, t19688, t19699, t225, t1819, t68, t1995, t6330, t1307, t5187, t5279);
        let (t19724, t19725, t19728, t19731) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2312::<F>(t1365, t6347, t1307, t1347, t19631, t1345, t1348, t1819, t1821, t19702, t19708, t19716, t19719, t5272, t5278, t5280, t5283, t546, t548, t6404, t6408, t6411);
        let (t19732, t19733, t19735) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2313::<F>(t19731, t550, t1380, t3792, t5286);
    (t19702, t19708, t19715, t19716, t19719, t19724, t19725, t19728, t19731, t19732, t19733, t19735)
}
