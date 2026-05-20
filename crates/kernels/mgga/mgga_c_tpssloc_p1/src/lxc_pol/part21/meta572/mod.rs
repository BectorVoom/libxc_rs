//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2286;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2287;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta572<F: Float>(t5427: F, t608: F, t5392: F, t9287: F, t607: F, t3966: F, t3981: F, t2267: F, t5398: F, t16558: F, t43: F, t9300: F, t3990: F, t2274: F, t55: F, t1420: F, t39: F, t3991: F, t3994: F, t51: F, t5408: F, t5411: F, t5416: F, t615: F, t621: F, t9311: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19363, t19368, t19369, t19372, t19377, t19378, t19381, t19390) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2286::<F>(t5427, t608, t5392, t9287, t607, t3966, t3981, t2267, t5398, t16558, t43, t9300);
        let (t19397, t19404) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2287::<F>(t19390, t607, t3966, t3990, t2274, t5398, t16558, t55, t1420, t19369, t19372, t19378, t19381, t39, t3991, t3994, t51, t5408, t5411, t5416, t615, t621, t9311);
    (t19363, t19368, t19369, t19372, t19377, t19378, t19381, t19390, t19397, t19404)
}
