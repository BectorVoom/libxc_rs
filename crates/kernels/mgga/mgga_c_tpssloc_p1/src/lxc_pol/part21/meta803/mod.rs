//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta803 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2791;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta803<F: Float>(t46445: F, t2517: F, t2658: F, t5392: F, t47160: F, t41291: F, t16634: F, t2427: F, t47163: F, t47165: F, t12923: F, t3966: F, t4194: F) -> (F, F, F, F, F, F, F, F) {
        let (t59011, t59014, t59015, t59016, t59018, t59019, t59020, t59022) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2791::<F>(t46445, t2517, t2658, t5392, t47160, t41291, t16634, t2427, t47163, t47165, t12923, t3966, t4194);
    (t59011, t59014, t59015, t59016, t59018, t59019, t59020, t59022)
}
