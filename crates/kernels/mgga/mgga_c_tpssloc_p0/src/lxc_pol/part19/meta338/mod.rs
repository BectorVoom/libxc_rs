//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta338 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1204;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1205;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1206;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta338<F: Float>(t40: F, t52: F, t2244: F, t2250: F, t2291: F, t39097: F, t39103: F, t39110: F, t634: F, t75: F, t767: F, t9258: F, t9499: F, t2298: F, t638: F, t771: F, t78: F, t9508: F, zeta_threshold: F, t10047: F, t225: F, t2742: F, t9587: F, t9585: F, t10046: F, t10049: F, t10104: F, t10110: F, t10112: F, t10116: F, t259: F, t2591: F, t2710: F, t2713: F, t2718: F, t2719: F, t2720: F, t2743: F, t798: F, t855: F, t866: F, t9593: F, t10108: F, t257: F, t68: F, t2627: F, t10016: F, t252: F, t9957: F, t852: F, t9971: F, t2631: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40833, t40846) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1204::<F>(t40, t52, t2244, t2250, t2291, t39097, t39103, t39110, t634, t75, t767, t9258, t9499, t2298, t638, t771, t78, t9508, zeta_threshold);
        let t40848 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1205::<F>(t40833, t40846);
        let t40887 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1206::<F>(t10047, t225, t2742, t9587, t9585, t10046, t10049, t10104, t10110, t10112, t10116, t259, t2591, t2710, t2713, t2718, t2719, t2720, t2743, t798, t855, t866, t9593);
        let (t40890, t40891, t40895, t40904, t40909, t40917, t40925) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1207::<F>(t10108, t257, t68, t2719, t2627, t2710, t10016, t252, t9957, t852, t9971, t2631);
    (t40848, t40887, t40890, t40891, t40895, t40904, t40909, t40917, t40925)
}
