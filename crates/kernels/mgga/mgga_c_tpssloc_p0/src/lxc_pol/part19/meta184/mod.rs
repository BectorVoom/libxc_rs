//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk835;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta184<F: Float>(t761: F, t9919: F, t2531: F, t2535: F, t2427: F, t2430: F, t185: F, t9258: F, t707: F, t32: F, t717: F, t2659: F, t2244: F, t751: F, t2658: F, t9853: F, t9859: F, t9911: F, t9914: F, t9917: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t9921, t9923, t9925, t9926, t9928, t9929, t9931) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk835::<F>(t761, t9919, t2531, t2535, t2427, t2430, t185, t9258, t707, t32, t717, t2659);
        let (t9932, t9934, t9935) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk836::<F>(t2244, t751, t2658, t9853, t9859, t9911, t9914, t9917, t9921, t9923, t9925, t9928, t9931);
    (t9921, t9923, t9925, t9926, t9928, t9929, t9931, t9932, t9934, t9935)
}
