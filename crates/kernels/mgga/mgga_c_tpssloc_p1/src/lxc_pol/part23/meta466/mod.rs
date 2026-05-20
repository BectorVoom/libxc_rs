//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1364;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1365;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1366;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta466<F: Float>(t42086: F, t59688: F, t59694: F, t76574: F, t76578: F, t76583: F, t76591: F, t76599: F, t76614: F, t76622: F, t76893: F, t76896: F, t76909: F, t76915: F, t77072: F, t894: F, t2798: F, t77041: F, t41942: F, t77075: F, t42087: F, t47787: F, t76587: F, t76595: F, t76610: F, t76618: F, t76626: F, t76899: F, t76903: F, t76906: F, t76912: F, t77037: F, t77082: F, t893: F, t913: F, t5791: F, t5811: F, t959: F, t13727: F, t21315: F, t2842: F, t5695: F, t5726: F, t1557: F, t21299: F, t2792: F, t10661: F, t5730: F, t13520: F, t21318: F, t1556: F, t69347: F, t5790: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t77097 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1364::<F>(t42086, t59688, t59694, t76574, t76578, t76583, t76591, t76599, t76614, t76622, t76893, t76896, t76909, t76915);
        let (t77102, t77105, t77107, t77114) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1365::<F>(t77072, t894, t2798, t77041, t41942, t77075, t42087, t47787, t76587, t76595, t76610, t76618, t76626, t76899, t76903, t76906, t76912);
        let (t77119, t77122, t77124, t77127) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1366::<F>(t77037, t77082, t77097, t77114, t893, t913, t5791, t5811, t959, t13727, t21315, t2842, t5695, t5726);
        let (t77130, t77133, t77135, t77138, t77139) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1367::<F>(t1557, t21299, t2792, t10661, t5726, t5730, t13520, t21318, t1556, t2842, t69347, t5790);
    (t77102, t77105, t77107, t77119, t77122, t77124, t77127, t77130, t77133, t77135, t77138, t77139)
}
