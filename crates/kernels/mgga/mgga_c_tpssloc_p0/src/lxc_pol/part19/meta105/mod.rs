//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta105 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk577;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk578;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta105<F: Float>(t2988: F, t2990: F, t2775: F, t344: F, t2244: F, t977: F, t2250: F, t978: F, t2822: F, t2824: F, t2828: F, t2831: F, t2834: F, t340: F, t343: F, t974: F, t984: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2991, t2994, t2995, t2996, t2999, t3000, t3008) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk577::<F>(t2988, t2990, t2775, t344, t2244, t977, t2250, t978, t2822, t2824, t2828, t2831, t2834);
        let (t3010, t3011, t3014) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk578::<F>(t3008, t340, t343, t974, t984);
    (t2991, t2994, t2995, t2996, t2999, t3000, t3008, t3010, t3011, t3014)
}
