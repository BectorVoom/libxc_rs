//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta113 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk681;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk682;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk683;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk684;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk685;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta113<F: Float>(t815: F, t835: F, t812: F, t831: F, t242: F, t67: F, t845: F, t246: F, t232: F, t776: F, t753: F, t758: F, t152: F, t32: F, t181: F, t204: F, t686: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2638, t2639) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk681::<F>(t815, t835, t812);
        let (t2640, t2642, t2643) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk682::<F>(t2639, t831, t242, t815, t812);
        let (t2644, t2645) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk683::<F>(t67, t845, t246);
        let t2647 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk684::<F>(t232, t776);
        let (t2652, t2653, t2658, t2663) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk685::<F>(t67, t753, t758, t152, t32, t181, t204, t686);
    (t2638, t2639, t2640, t2642, t2643, t2644, t2645, t2647, t2652, t2653, t2658, t2663)
}
