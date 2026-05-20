//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta104 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk697;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk698;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta104<F: Float>(t116: F, t206: F, t212: F, t2586: F, t2562: F, t2564: F, t2569: F, t2571: F, t2573: F, t2579: F, t2582: F, t787: F, t252: F, t798: F, t852: F, t225: F, t799: F, t154: F, t2559: F, t222: F, t2563: F, t805: F, t119: F, t2379: F, t210: F, t2553: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2588, t2590, t2591) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk697::<F>(t116, t206, t212, t2586, t2562, t2564, t2569, t2571, t2573, t2579, t2582, t787);
        let (t2592, t2594, t2597) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk698::<F>(t252, t2591, t798, t852, t225, t799);
        let (t2600, t2602, t2603, t2606, t2610, t2613) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk699::<F>(t154, t2559, t222, t2563, t805, t119, t2379, t210, t2553, t225, t2591);
    (t2588, t2590, t2591, t2592, t2594, t2597, t2600, t2602, t2603, t2606, t2610, t2613)
}
