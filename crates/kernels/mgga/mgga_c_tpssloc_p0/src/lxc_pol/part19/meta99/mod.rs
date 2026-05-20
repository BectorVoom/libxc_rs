//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta99 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk551;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk552;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk553;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk554;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta99<F: Float>(t290: F, t2793: F, t2842: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F, t919: F, t923: F, t307: F, t922: F, t302: F, t931: F, t932: F, t2822: F, t2800: F, t2808: F, t2816: F, t2818: F, t2824: F, t2828: F, t2831: F, t2834: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2843, t2844) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk551::<F>(t290);
        let (t2845, t2847, t2853, t2856, t2859) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk552::<F>(t2793, t2844, t2842, t2764, t2766, t2773, t2778, t2782, t919, t923, t307, t922);
        let (t2860, t2861, t2862) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk553::<F>(t2859, t302, t931);
        let (t2863, t2880) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk554::<F>(t2862, t932, t2764, t2822, t2766, t2773, t2778, t2782, t2800, t2808, t2816, t2818, t2824, t2828, t2831, t2834);
    (t2843, t2844, t2845, t2847, t2853, t2856, t2859, t2860, t2861, t2862, t2863, t2880)
}
