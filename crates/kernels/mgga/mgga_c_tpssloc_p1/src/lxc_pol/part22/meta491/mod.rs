//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1915;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1916;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1917;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta491<F: Float>(t21256: F, t21363: F, t300: F, t21348: F, t4483: F, t5804: F, t17954: F, t4475: F, t959: F, t4488: F, t5791: F, t1637: F, t5950: F, t11094: F, t17202: F, t193: F, t21093: F, t21097: F, t21099: F, t21103: F, t21105: F, t21107: F, t336: F, t4700: F, t1615: F, t5872: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t21365, t21367, t21369, t21370, t21372, t21373, t21375, t21376) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1915::<F>(t21256, t21363, t300, t21348, t4483, t5804, t17954, t4475, t959, t4488, t5791, t1637, t5950);
        let t21381 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1916::<F>(t11094, t1637, t17202, t193, t21093, t21097, t21099, t21103, t21105, t21107, t21365, t21367, t21369, t21372, t21375, t21376, t336, t4700);
        let t21390 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1917::<F>(t1615, t5872);
    (t21365, t21367, t21369, t21370, t21372, t21373, t21375, t21376, t21381, t21390)
}
