//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1639;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1640;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1641;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1642;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1643;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1644;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta424<F: Float>(t19482: F, t666: F, t5468: F, t9384: F, t659: F, t1444: F, t2: F, t584: F, t2341: F, t5396: F, t9212: F, t95: F, t5480: F, t9398: F, t662: F, t1449: F, t2349: F, t5484: F, t103: F, t100: F, t12774: F, t12795: F, t1447: F, t4060: F, t4064: F, t5469: F, t5472: F, t5475: F, t657: F, t663: F, t92: F, t656: F, t12747: F, t12750: F, t12752: F, t19471: F, t19474: F, t19477: F, t19480: F, t64: F, t9358: F, t9359: F, t109: F, t1268: F, t12725: F, t1458: F, t19450: F, t19451: F, t19456: F, t19461: F, t2314: F, t4028: F, t4072: F, t5113: F, t5493: F, t671: F, t7676: F, t25: F, t6320: F, t67: F, t758: F, t12061: F, t6305: F, t3664: F, t5397: F, t16557: F, t2219: F, t5134: F, t514: F, t606: F, zeta_threshold: F, t28: F, t12072: F, t6312: F, t3672: F, t5966: F, t1081: F, t18196: F, t5142: F, t517: F, t157: F, t184: F) -> (F, F, F, F, F, F) {
        let (t19483, t19489, t19493, t19499, t19503, t19504) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1639::<F>(t19482, t666, t5468, t9384, t659, t1444, t2, t584, t2341, t5396, t9212, t95);
        let t19529 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1640::<F>(t5480, t9398, t662, t1449, t2, t584, t2349, t5484, t19503, t103, t100, t12774, t12795, t1447, t19489, t19493, t19499, t19504, t4060, t4064, t5469, t5472, t5475, t657, t663, t92);
        let t19533 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1641::<F>(t19529, t656, t12747, t12750, t12752, t19471, t19474, t19477, t19480, t19483, t64, t9358, t9359);
        let t19534 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1642::<F>(t109, t19533);
        let t19537 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1643::<F>(t1268, t12725, t1458, t19450, t19451, t19456, t19461, t19534, t2314, t4028, t4072, t5113, t5493, t671, t7676);
        let (t19543, t19558) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1644::<F>(t25, t6320, t67, t758, t12061, t6305, t3664, t5397, t16557, t2219, t5134, t514, t606, zeta_threshold);
        let (t19572, t19573) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1645::<F>(t28, t12072, t6312, t3672, t5966, t1081, t18196, t2219, t5142, t517, t157, t19558, t184, zeta_threshold);
    (t19529, t19534, t19537, t19543, t19572, t19573)
}
