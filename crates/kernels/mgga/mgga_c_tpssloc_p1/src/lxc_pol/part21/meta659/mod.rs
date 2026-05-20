//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta659 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2460;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta659<F: Float>(t43819: F, t3311: F, t409: F, t3314: F, t3374: F, t3399: F, t440: F, t3256: F, t3263: F, t1094: F, t11189: F, t1124: F, t11349: F) -> (F, F, F, F, F, F, F, F) {
        let (t44053, t44075, t44077, t44154, t44155, t44159, t44162, t44172) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2460::<F>(t43819, t3311, t409, t3314, t3374, t3399, t440, t3256, t3263, t1094, t11189, t1124, t11349);
    (t44053, t44075, t44077, t44154, t44155, t44159, t44162, t44172)
}
