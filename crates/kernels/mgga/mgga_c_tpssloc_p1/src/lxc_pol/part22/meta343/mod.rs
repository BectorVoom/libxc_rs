//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1543;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1544;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta343<F: Float>(t16710: F, t758: F, t3966: F, t4195: F, t4194: F, t184: F, t5392: F, t607: F, t12939: F, t13121: F, t16699: F, t16700: F, t16703: F, t16705: F, t16707: F, t16708: F, t16709: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F, t16684: F, t16686: F, t16698: F, t225: F, t1504: F, t68: F, t1891: F, t5527: F, t776: F, t4119: F, t4226: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16711, t16712, t16713, t16715, t16716, t16717, t16719, t16720) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1543::<F>(t16710, t758, t3966, t4195, t4194, t184, t5392, t607, t12939, t13121, t16699, t16700, t16703, t16705, t16707, t16708, t16709, t9853, t9859, t9894, t9907, t9921);
        let (t16723, t16729, t16736, t16737, t16740) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1544::<F>(t16684, t16686, t16698, t16720, t225, t1504, t68, t1891, t5527, t776, t4119, t4226);
    (t16711, t16712, t16713, t16715, t16716, t16717, t16719, t16723, t16729, t16736, t16737, t16740)
}
