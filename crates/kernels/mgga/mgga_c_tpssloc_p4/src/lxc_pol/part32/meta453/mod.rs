//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1722;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1723;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta453<F: Float>(t22751: F, t6892: F, t6883: F, t6908: F, t22674: F, t6891: F, t22892: F, t1988: F, t22716: F, t22724: F, t6898: F, t6902: F, t794: F, t6897: F, t225: F, t3886: F, t6903: F, t1914: F, t193: F, t201: F, t25: F, t2752: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22907, t22909, t22920, t22921, t22924, t22926, t22927) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1722::<F>(t22751, t6892, t6883, t6908, t22674, t6891, t22892, t1988, t22716, t22724, t6898, t6902, t794);
        let (t22928, t22933, t22940, t22959) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1723::<F>(t22927, t6897, t225, t3886, t6883, t6903, t1914, t193, t201);
        let t22960 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1724::<F>(t25, t2752);
    (t22907, t22909, t22920, t22921, t22924, t22926, t22927, t22928, t22933, t22940, t22959, t22960)
}
