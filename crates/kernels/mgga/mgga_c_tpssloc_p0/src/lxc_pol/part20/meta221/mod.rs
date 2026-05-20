//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1291;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1292;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1293;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1294;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta221<F: Float>(t9212: F, t591: F, t9: F, t21: F, t587: F, t14: F, t598: F, t2230: F, t594: F, t2229: F, t3: F, t19: F, t9211: F, t2233: F, t604: F) -> (F, F, F, F, F, F, F, F) {
        let (t9213, t9214) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1291::<F>(t9212, t591, t9);
        let (t9215, t9216) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1292::<F>(t9214, t21, t587);
        let (t9217, t9218) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1293::<F>(t9216, t14, t598);
        let (t9220, t9223, t9225, t9226, t9228) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1294::<F>(t9218, t2230, t594, t2229, t3, t19, t9211, t9213, t9215, t9217, t2233, t604);
    (t9214, t9216, t9218, t9220, t9223, t9225, t9226, t9228)
}
