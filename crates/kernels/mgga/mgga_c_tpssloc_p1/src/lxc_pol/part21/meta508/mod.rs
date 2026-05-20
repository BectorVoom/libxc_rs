//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2154;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta508<F: Float>(t5695: F, t912: F, t2842: F, t1557: F, t4395: F, t2792: F, t5730: F, t10661: F, t10756: F, t10828: F, t17192: F, t17451: F, t17454: F, t17471: F, t17490: F, t17493: F, t17496: F, t17500: F, t17504: F, t17506: F, t2905: F, t2930: F, t311: F) -> (F, F, F, F, F, F, F) {
        let (t17507, t17509, t17510, t17512, t17513, t17515, t17516) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2154::<F>(t5695, t912, t2842, t1557, t4395, t2792, t5730, t10661, t10756, t10828, t17192, t17451, t17454, t17471, t17490, t17493, t17496, t17500, t17504, t17506, t2905, t2930, t311);
    (t17507, t17509, t17510, t17512, t17513, t17515, t17516)
}
