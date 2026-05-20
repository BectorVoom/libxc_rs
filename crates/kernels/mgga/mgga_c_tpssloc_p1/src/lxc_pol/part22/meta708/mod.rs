//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta708 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2302;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2303;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta708<F: Float>(t52: F, t12874: F, t12877: F, t16558: F, t16563: F, t17635: F, t20217: F, t20234: F, t2440: F, t3966: F, t40647: F, t4087: F, t5398: F, t607: F, t67060: F, t76: F, zeta_threshold: F, t67064: F, t157: F, t182: F, t46130: F, t57887: F, t46132: F, t46134: F, t57897: F, t40667: F, t40682: F, t172: F, t20742: F, t763: F, t39309: F, t39312: F, t39316: F, t39320: F, t40673: F, t40679: F, t40685: F, t16693: F, t16713: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t67082 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2302::<F>(t52, t12874, t12877, t16558, t16563, t17635, t20217, t20234, t2440, t3966, t40647, t4087, t5398, t607, t67060, t76, zeta_threshold);
        let (t67083, t67086, t67087, t67088, t67089, t67090, t67095, t67096, t67097, t67099) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2303::<F>(t67064, t67082, t157, t182, t46130, t57887, t46132, t46134, t57897, t40667, t40682, t172, t20742, t763);
        let (t67100, t67101, t67104) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2304::<F>(t67099, t39309, t39312, t39316, t39320, t40673, t40679, t40685, t67095, t67096, t67097, t16693, t16713);
    (t67083, t67086, t67087, t67088, t67089, t67090, t67095, t67096, t67097, t67100, t67101, t67104)
}
