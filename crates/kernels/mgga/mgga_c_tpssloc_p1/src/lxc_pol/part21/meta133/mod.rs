//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta133 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk888;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk889;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk890;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk891;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk892;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk893;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk894;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk895;
use chunk8::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta133<F: Float>(t1878: F, t268: F, t405: F, t1091: F, t690: F, t1229: F, t154: F, t636: F, t2244: F, t123: F, t2296: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t3236 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk888::<F>(t1878, t268, t405);
        let (t3237, t3238) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk889::<F>(t3236, t1091, t690);
        let t3240 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk890::<F>(t1229, t154);
        let t3241 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk891::<F>(t636);
        let t3242 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk892::<F>(t3241);
        let t3243 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk893::<F>(t2244, t3242);
        let (t3244, t3245) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk894::<F>(t3240, t3243, t123);
        let t3247 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk895::<F>(t2296);
        let t3248 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk896::<F>(t2244, t3247);
    (t3236, t3237, t3238, t3240, t3241, t3242, t3243, t3244, t3245, t3247, t3248)
}
