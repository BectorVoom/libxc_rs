//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta83 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk482;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk483;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk484;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk485;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk486;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta83<F: Float>(t193: F, t201: F, t868: F, t870: F, t2369: F, t2509: F, t2512: F, t761: F, t172: F, t753: F, t763: F, t2504: F, t739: F, t746: F, t40: F, t52: F, t718: F, t751: F, t2244: F, t2250: F, t75: F, t767: F, t771: F, t78: F, zeta_threshold: F, t15: F, t60: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2522 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk482::<F>(t193, t201);
        let (t2523, t2527, t2528) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk483::<F>(t868, t870, t2369, t2509, t2512);
        let (t2530, t2531) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk484::<F>(t2528, t761, t172, t753);
        let (t2533, t2535) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk485::<F>(t2531, t763, t2504, t739, t746);
        let (t2537, t2539, t2553) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk486::<F>(t40, t52, t2535, t761, t718, t751, t2244, t2250, t75, t767, t771, t78, zeta_threshold);
        let t2558 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk487::<F>(t15, t60);
    (t2522, t2523, t2527, t2528, t2530, t2531, t2533, t2535, t2537, t2539, t2553, t2558)
}
