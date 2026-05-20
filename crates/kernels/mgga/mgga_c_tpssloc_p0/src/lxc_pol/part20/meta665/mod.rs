//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta665 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2492;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2493;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2494;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2495;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2496;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2497;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2498;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2499;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta665<F: Float>(t1409: F, t43791: F, t9288: F, t11145: F, t123: F, t2394: F, t4725: F, t14727: F, t690: F, t43763: F, t43809: F, t12606: F, t3247: F, t607: F, t1088: F, t50865: F, t50869: F, t50873: F, t50903: F, t50905: F, t50907: F, t50912: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50915, t50917) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2492::<F>(t1409, t43791, t9288, t11145, t123);
        let t50919 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2493::<F>(t2394, t4725);
        let t50921 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2494::<F>(t14727, t690);
        let (t50924, t50926) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2495::<F>(t1409, t43763, t9288, t123, t43809);
        let (t50929, t50931) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2496::<F>(t12606, t3247, t607, t1088, t123);
        let t50934 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2497::<F>(t1088, t123, t50865);
        let t50937 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2498::<F>(t1088, t123, t50869);
        let t50940 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2499::<F>(t1088, t123, t50873);
        let t50942 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2500::<F>(t50903, t50905, t50907, t50912, t50917, t50919, t50921, t50926, t50931, t50934, t50937, t50940);
    (t50915, t50917, t50919, t50921, t50924, t50926, t50929, t50931, t50934, t50937, t50940, t50942)
}
