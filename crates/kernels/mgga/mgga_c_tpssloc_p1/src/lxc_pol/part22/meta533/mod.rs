//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2007;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2008;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2009;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2010;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta533<F: Float>(t39300: F, t739: F, t746: F, t1294: F, t3691: F, t9722: F, t2483: F, t268: F, t9778: F, t2406: F, t9790: F, t204: F, t2410: F, t2415: F, t676: F, t9452: F, t9455: F) -> (F, F, F, F, F, F, F) {
        let (t39302, t39304, t39305, t39309) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2007::<F>(t39300, t739, t746, t1294, t3691, t9722, t2483, t268, t9778);
        let t39312 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2008::<F>(t2406, t268, t9790);
        let t39316 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2009::<F>(t204, t2410, t2415, t268);
        let t39320 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2010::<F>(t268, t676, t9452, t9455);
    (t39302, t39304, t39305, t39309, t39312, t39316, t39320)
}
