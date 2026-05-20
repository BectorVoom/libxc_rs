//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta98 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk629;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk630;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk631;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk632;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk633;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk634;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk635;
use chunk7::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk636;
use chunk8::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk637;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta98<F: Float>(t2144: F, t466: F, t225: F, t477: F, t491: F, t462: F, t493: F, t2121: F, t470: F, t1241: F, t265: F, t504: F, t1238: F, t2124: F, t498: F, t1256: F, t193: F, t1964: F, t336: F, t28: F, t1971: F, t52: F, t2119: F, dens_threshold: F, rho1: F, zeta_threshold: F, t1979: F, t2114: F, t113: F, t1876: F, t2021: F, t510: F, t574: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2145, t2147) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk629::<F>(t2144, t466, t225, t477);
        let t2148 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk630::<F>(t2147, t491);
        let (t2149, t2152) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk631::<F>(t2148, t462, t2144, t493);
        let t2154 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk632::<F>(t2121, t2149, t2152, t470);
        let t2155 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk633::<F>(t1241, t2154);
        let (t2157, t2161) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk634::<F>(t265, t504, t1238, t2121, t2124, t2145, t2155, t498, t1256, t193, t1964, t336);
        let t2165 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk635::<F>(t28, t1971, t2161, t52, t2119, dens_threshold, rho1, zeta_threshold);
        let t2167 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk636::<F>(t1979, t2114);
        let t2169 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk637::<F>(t113, t1876, t2021, t2114, t2165, t2167, t510, t574);
    (t2145, t2147, t2148, t2149, t2152, t2154, t2155, t2157, t2161, t2165, t2167, t2169)
}
