//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk853;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk854;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk855;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta192<F: Float>(t2749: F, t868: F, t261: F, t2751: F, t193: F, t202: F, t9793: F, t9797: F, t9820: F, t9824: F, t9872: F, t9876: F, t9881: F, t9884: F, t9887: F, t9890: F, t9894: F, t9896: F, t9853: F, t9859: F, t9900: F, t9903: F, t9907: F, t9911: F, t9914: F, t9917: F, t9921: F, t9923: F, t9925: F, t9928: F, t9931: F, t9934: F, t10125: F, t10138: F, t225: F, t3023: F, t1053: F, t68: F, t1065: F, t3175: F, t3021: F, t3206: F, t3174: F, t1887: F, t337: F, t615: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10140, t10143, t10147) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk853::<F>(t2749, t868, t261, t2751, t193, t202, t9793, t9797, t9820, t9824, t9872, t9876, t9881, t9884, t9887, t9890, t9894, t9896);
        let t10148 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk854::<F>(t9853, t9859, t9900, t9903, t9907, t9911, t9914, t9917, t9921, t9923, t9925, t9928, t9931, t9934);
        let t10150 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk855::<F>(t10125, t10138, t10147, t10148);
        let (t10160, t10163, t10165, t10167, t10170, t10182, t10186) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk856::<F>(t225, t3023, t1053, t68, t1065, t3175, t3021, t3206, t3174, t1887, t337, t615);
    (t10140, t10143, t10150, t10160, t10163, t10165, t10167, t10170, t10182, t10186)
}
