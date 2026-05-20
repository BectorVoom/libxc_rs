//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2469;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2470;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta668<F: Float>(t42341: F, t44696: F, t42344: F, t483: F, t1210: F, t1174: F, t3561: F, t698: F, t10471: F, t44690: F, t11727: F, t44722: F, t478: F, t11818: F, t1213: F, t248: F, t3494: F, t3506: F, t3509: F, t3515: F, t3516: F, t11718: F, t11721: F, t3493: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44833, t44834, t44836, t44847, t44857, t44858, t44863) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2469::<F>(t42341, t44696, t42344, t483, t1210, t1174, t3561, t698, t10471, t44690, t11727, t44722, t478);
        let (t44886, t44890, t44894, t44896, t44906) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2470::<F>(t11818, t1213, t248, t3494, t3506, t3509, t3515, t3516, t11718, t44857, t11721, t3493);
    (t44833, t44834, t44836, t44847, t44857, t44858, t44863, t44886, t44890, t44894, t44896, t44906)
}
