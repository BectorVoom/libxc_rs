//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta88 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk617;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk618;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk619;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk620;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk621;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk622;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk623;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta88<F: Float>(t2223: F, t14: F, t21: F, t594: F, t598: F, t15: F, t19: F, t2218: F, t2220: F, t2222: F, t601: F, t604: F, t84: F, t85: F, t24: F, t645: F, t607: F, t65: F, t11: F, t2219: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2224, t2225) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk617::<F>(t2223, t14, t21);
        let (t2226, t2228, t2229) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk618::<F>(t2225, t594, t598, t15);
        let (t2230, t2232, t2233, t2235) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk619::<F>(t2229, t19, t2218, t2220, t2222, t2224, t2226, t2228, t601, t604);
        let t2239 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk620::<F>(t84, t85);
        let t2240 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk621::<F>(t2239, t24);
        let t2241 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk622::<F>(t645);
        let t2244 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk623::<F>(t607);
        let (t2245, t2248) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk624::<F>(t2244, t65, t11, t2219);
    (t2225, t2229, t2230, t2232, t2233, t2235, t2239, t2240, t2241, t2244, t2245, t2248)
}
