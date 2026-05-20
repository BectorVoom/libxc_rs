//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2395;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta619<F: Float>(t12365: F, t3858: F, t12384: F, t3777: F, t12282: F, t12328: F, t1333: F, t1336: F, t2690: F, t3788: F, t3795: F, t67: F, t6924: F) -> (F, F, F, F, F, F, F) {
        let (t40126, t40130, t40138, t40145, t40159, t40160, t40167) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2395::<F>(t12365, t3858, t12384, t3777, t12282, t12328, t1333, t1336, t2690, t3788, t3795, t67, t6924);
    (t40126, t40130, t40138, t40145, t40159, t40160, t40167)
}
