//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk778;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk779;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta137<F: Float>(t3040: F, t360: F, t1021: F, t248: F, t1030: F, t372: F, t364: F, t354: F, t1043: F, t121: F, t884: F, t1041: F, t1044: F, t2780: F, t283: F, t883: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3041, t3043, t3047, t3048) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk778::<F>(t3040, t360, t1021, t248, t1030, t372, t364, t354);
        let t3051 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk779::<F>(t1043, t121);
        let (t3053, t3054, t3057, t3061) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk780::<F>(t248, t3051, t884, t1041, t1044, t2780, t283, t883);
    (t3041, t3043, t3047, t3048, t3051, t3053, t3054, t3057, t3061)
}
