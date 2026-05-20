//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta117 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk669;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk670;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta117<F: Float>(t67: F, t753: F, t758: F, t185: F, t2250: F, t707: F, t152: F, t32: F, t2244: F, t181: F, t204: F, t686: F, t756: F, t2373: F, t2377: F, t2408: F, t2417: F, t2423: F, t2426: F, t2429: F, t2432: F, t2450: F, t2486: F, t2518: F, t2520: F, t2530: F, t2533: F, t2537: F, t2539: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t2652, t2653, t2654, t2655, t2657, t2658, t2659, t2661, t2663) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk669::<F>(t67, t753, t758, t185, t2250, t707, t152, t32, t2244, t181, t204, t686);
        let (t2665, t2666) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk670::<F>(t2663, t756, t2373, t2377, t2408, t2417, t2423, t2426, t2429, t2432, t2450, t2486, t2518, t2520, t2530, t2533, t2537, t2539, t2654, t2657, t2661);
    (t2652, t2653, t2654, t2655, t2657, t2658, t2659, t2661, t2663, t2665, t2666)
}
