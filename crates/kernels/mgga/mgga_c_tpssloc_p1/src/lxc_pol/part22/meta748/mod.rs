//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta748 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2500;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2501;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta748<F: Float>(t25: F, t265: F, t394: F, t68418: F, t68765: F, t68897: F, t68931: F, t68999: F, t69031: F, t69462: F, t69464: F, t71055: F, t1074: F, t1408: F, t1409: F, t1534: F, t1642: F, t16557: F, t16558: F, t17133: F, t18176: F, t20216: F, t20217: F, t21076: F, t21703: F, t396: F, t3966: F, t40: F, t4324: F, t4705: F, t5397: F, t5398: F, t5955: F, t606: F, t607: F, t67059: F, t67060: F, t68427: F, t873: F, dens_threshold: F, rho0: F, zeta_threshold: F, t18255: F, t51667: F, t18259: F, t50819: F, t22408: F, t3640: F, t1164: F, t15218: F, t18279: F, t18910: F, t18274: F, t51651: F, t18915: F, t4875: F, t1155: F, t1694: F, t18615: F, t51848: F, t47774: F, t51002: F, t68513: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t71077 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2500::<F>(t25, t265, t394, t68418, t68765, t68897, t68931, t68999, t69031, t69462, t69464, t71055, t1074, t1408, t1409, t1534, t1642, t16557, t16558, t17133, t18176, t20216, t20217, t21076, t21703, t396, t3966, t40, t4324, t4705, t5397, t5398, t5955, t606, t607, t67059, t67060, t68427, t873, dens_threshold, rho0, zeta_threshold);
        let (t71090, t71095, t71097, t71101, t71106, t71109, t71112) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2501::<F>(t67059, t18255, t51667, t18259, t50819, t22408, t3640, t1164, t15218, t18279, t18910, t18274, t51651);
        let (t71114, t71115, t71118, t71124) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2502::<F>(t18915, t4875, t1155, t1694, t18615, t51848, t47774, t51002, t68513);
    (t71077, t71090, t71095, t71097, t71101, t71106, t71109, t71112, t71114, t71115, t71118, t71124)
}
