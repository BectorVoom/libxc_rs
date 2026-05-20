//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2391;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta616<F: Float>(t12267: F, t3789: F, t3798: F, t3802: F, t3734: F, t3792: F, t3719: F, t1314: F, t9569: F, t1329: F, t12189: F, t3770: F, t2559: F, t3732: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t39952, t39955, t39975, t39978, t39993, t40005, t40006, t40008) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2391::<F>(t12267, t3789, t3798, t3802, t3734, t3792, t3719, t1314, t9569, t1329, t12189, t3770);
        let t40018 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2392::<F>(t2559, t3732);
    (t39952, t39955, t39975, t39978, t39993, t40005, t40006, t40008, t40018)
}
