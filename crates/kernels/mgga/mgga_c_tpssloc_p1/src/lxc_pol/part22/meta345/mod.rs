//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1547;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1548;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta345<F: Float>(t213: F, t5527: F, t221: F, t776: F, t4119: F, t4128: F, t12986: F, t13002: F, t13005: F, t13010: F, t16769: F, t4127: F, t9526: F, t9540: F, t9542: F, t9547: F, t9572: F, t118: F, t794: F, t9549: F, t16662: F, t210: F, t214: F, t5544: F, t2576: F, t2563: F, t5555: F, t13014: F, t13020: F, t13022: F, t13027: F, t787: F, t9579: F, t9583: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16771, t16773, t16777, t16781) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1547::<F>(t213, t5527, t221, t776, t4119, t4128, t12986, t13002, t13005, t13010, t16769, t4127, t9526, t9540, t9542, t9547, t9572);
        let (t16783, t16784, t16787, t16791, t16792, t16794, t16796) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1548::<F>(t118, t5527, t794, t9549, t16662, t210, t214, t5544, t2576, t2563, t5555, t213);
        let (t16798, t16803) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1549::<F>(t16796, t221, t776, t13014, t13020, t13022, t13027, t16784, t16787, t16792, t16794, t4127, t787, t9579, t9583);
    (t16771, t16773, t16777, t16781, t16783, t16784, t16787, t16791, t16792, t16794, t16798, t16803)
}
