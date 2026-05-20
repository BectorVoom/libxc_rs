//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2307;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2308;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2309;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2310;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2311;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2312;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2313;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta632<F: Float>(t13527: F, t2250: F, t123: F, t2768: F, t4337: F, t9258: F, t2244: F, t882: F, t2394: F, t4344: F, t4339: F, t13574: F, t690: F, t13577: F, t13568: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t47693, t47695) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2307::<F>(t13527, t2250, t123, t2768);
        let (t47697, t47699) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2308::<F>(t4337, t9258, t123, t2768);
        let (t47701, t47703) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2309::<F>(t13527, t2244, t123, t882);
        let t47705 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2310::<F>(t2394, t4344);
        let (t47706, t47707) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2311::<F>(t47705, t2394, t4339);
        let t47709 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2312::<F>(t13574, t690);
        let t47711 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2313::<F>(t13577, t690);
        let t47713 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2314::<F>(t13568, t690);
    (t47693, t47695, t47697, t47699, t47701, t47703, t47705, t47706, t47707, t47709, t47711, t47713)
}
