//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta827 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2918;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta827<F: Float>(t14473: F, t4489: F, t2906: F, t42110: F, t42113: F, t5774: F, t959: F, t10629: F, t14259: F, t5790: F, t10623: F, t5812: F, t17951: F, t2940: F, t14260: F, t4483: F, t2925: F, t5811: F, t14480: F, t10723: F, t17947: F, t59637: F, t60810: F, t60812: F, t60814: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t60816, t60821, t60825, t60827) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2918::<F>(t14473, t4489, t2906, t42110, t42113, t5774, t959, t10629, t14259, t5790, t10623, t5812);
        let (t60829, t60831, t60834, t60836, t60839, t60840) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2919::<F>(t17951, t2940, t14260, t4483, t2925, t5811, t959, t14480, t10723, t17947, t59637, t60810, t60812, t60814, t60816, t60821, t60825, t60827);
    (t60816, t60821, t60825, t60827, t60829, t60831, t60834, t60836, t60839, t60840)
}
