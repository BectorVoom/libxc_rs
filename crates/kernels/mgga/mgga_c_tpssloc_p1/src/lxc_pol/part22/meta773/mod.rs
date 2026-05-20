//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta773 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2644;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2645;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta773<F: Float>(t28: F, t265: F, t504: F, t68418: F, t71222: F, t71252: F, t72059: F, t72074: F, t72077: F, t72078: F, t72099: F, t73931: F, t1081: F, t1260: F, t1409: F, t1534: F, t1649: F, t16558: F, t17133: F, t1768: F, t18196: F, t19276: F, t20217: F, t20390: F, t21076: F, t22414: F, t3966: F, t4324: F, t506: F, t5099: F, t52: F, t5398: F, t5966: F, t607: F, t6279: F, t67060: F, t68427: F, t71090: F, t873: F, dens_threshold: F, rho1: F, zeta_threshold: F, t53777: F, t53779: F, t56099: F, t56102: F, t56104: F, t20396: F, t67: F, t758: F, t53798: F, t5397: F, t606: F, t584: F, t25: F, t15937: F, t15940: F, t16557: F, t19547: F, t20216: F, t20376: F, t2219: F, t3664: F, t39419: F, t5134: F, t514: F, t67059: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t73953 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2644::<F>(t28, t265, t504, t68418, t71222, t71252, t72059, t72074, t72077, t72078, t72099, t73931, t1081, t1260, t1409, t1534, t1649, t16558, t17133, t1768, t18196, t19276, t20217, t20390, t21076, t22414, t3966, t4324, t506, t5099, t52, t5398, t5966, t607, t6279, t67060, t68427, t71090, t873, dens_threshold, rho1, zeta_threshold);
        let (t73958, t73959, t73960, t73961, t73962, t73968, t73969, t73975, t73978) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2645::<F>(t53777, t53779, t56099, t56102, t56104, t20396, t67, t758, t53798, t5397, t606, t584);
        let t73989 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2646::<F>(t25, t15937, t15940, t16557, t19547, t20216, t20376, t2219, t3664, t39419, t5134, t514, t606, t67059, t73975, t73978, zeta_threshold);
    (t73953, t73958, t73959, t73960, t73961, t73962, t73968, t73969, t73975, t73978, t73989)
}
