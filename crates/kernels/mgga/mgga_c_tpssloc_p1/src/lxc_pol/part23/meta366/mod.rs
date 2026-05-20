//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta366<F: Float>(t43819: F, t3262: F, t3311: F, t409: F, t43776: F, t3314: F, t3374: F, t3399: F, t440: F, t3355: F, t427: F, t3358: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43895, t43942, t43969, t44027, t44053, t44075, t44077, t44154, t44155, t44177, t44178) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1166::<F>(t43819, t3262, t3311, t409, t43776, t3314, t3374, t3399, t440, t3355, t427, t3358);
    (t43895, t43942, t43969, t44027, t44053, t44075, t44077, t44154, t44155, t44177, t44178)
}
