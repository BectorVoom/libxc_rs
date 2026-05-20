//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta94 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk531;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk532;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk533;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk534;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk535;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk536;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk537;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk538;
use chunk8::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk539;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta94<F: Float>(t2764: F, t690: F, t885: F, t1043: F, t154: F, t632: F, t2244: F, t123: F, t2289: F, t882: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2765, t2766) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk531::<F>(t2764, t690, t885);
        let t2768 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk532::<F>(t1043, t154);
        let t2769 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk533::<F>(t632);
        let t2770 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk534::<F>(t2769);
        let t2771 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk535::<F>(t2244, t2770);
        let (t2772, t2773) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk536::<F>(t2768, t2771, t123);
        let t2775 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk537::<F>(t2289);
        let t2776 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk538::<F>(t2244, t2775);
        let (t2777, t2778) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk539::<F>(t2776, t882, t123);
    (t2765, t2766, t2768, t2769, t2770, t2771, t2772, t2773, t2775, t2776, t2777, t2778)
}
