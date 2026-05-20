//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta633 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2315;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2316;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2317;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2318;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2319;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2320;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2321;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2322;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta633<F: Float>(t13583: F, t690: F, t13586: F, t10216: F, t2244: F, t3966: F, t10564: F, t123: F, t13571: F, t13536: F, t9288: F, t2768: F, t2394: F, t4348: F, t13612: F, t45872: F, t883: F, t882: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t47715 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2315::<F>(t13583, t690);
        let t47717 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2316::<F>(t13586, t690);
        let (t47720, t47722) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2317::<F>(t10216, t2244, t3966, t10564, t123);
        let t47724 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2318::<F>(t13571, t690);
        let (t47726, t47728) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2319::<F>(t13536, t9288, t123, t2768);
        let t47730 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2320::<F>(t2394, t4348);
        let (t47731, t47732) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2321::<F>(t47730, t13612, t690);
        let (t47734, t47736) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2322::<F>(t45872, t883, t123, t882);
    (t47715, t47717, t47720, t47722, t47724, t47726, t47728, t47730, t47731, t47732, t47734, t47736)
}
