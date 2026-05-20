//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta735 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2413;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2414;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta735<F: Float>(t21238: F, t2929: F, t4497: F, t959: F, t17934: F, t4489: F, t4498: F, t17565: F, t21089: F, t41825: F, t17951: F, t4483: F, t17566: F, t4475: F, t60963: F, t21334: F, t892: F, t914: F, t1580: F, t49513: F, t60722: F, t950: F, t1637: F, t17198: F, t4696: F, t4700: F, t60867: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t68905, t68910, t68912, t68916, t68918) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2413::<F>(t21238, t2929, t4497, t959, t17934, t4489, t4498, t17565, t21089, t41825, t17951, t4483);
        let (t68920, t68923, t68926, t68930) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2414::<F>(t17566, t4483, t4475, t60963, t959, t21334, t892, t914, t1580, t49513, t60722, t950);
        let t68931 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2415::<F>(t1637, t17198, t4696, t4700, t60867, t68905, t68910, t68912, t68916, t68918, t68920, t68923, t68926, t68930);
    (t68905, t68910, t68912, t68916, t68918, t68920, t68923, t68926, t68930, t68931)
}
