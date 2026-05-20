//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta709 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2543;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta709<F: Float>(t10756: F, t300: F, t10828: F, t2930: F, t10390: F, t14501: F, t10422: F, t13761: F, t3070: F, t1615: F, t3120: F, t3040: F) -> (F, F, F, F, F, F, F) {
        let (t49513, t49532, t49541, t49604, t49607, t49616, t49621) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2543::<F>(t10756, t300, t10828, t2930, t10390, t14501, t10422, t13761, t3070, t1615, t3120, t3040);
    (t49513, t49532, t49541, t49604, t49607, t49616, t49621)
}
