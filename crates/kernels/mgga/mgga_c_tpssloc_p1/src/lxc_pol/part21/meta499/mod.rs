//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta499 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2122;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2123;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2124;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2125;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2126;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2127;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2128;
use chunk7::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2129;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta499<F: Float>(t17157: F, t2768: F, t123: F, t3966: F, t4337: F, t5682: F, t690: F, t5677: F, t607: F, t882: F, t4342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17158, t17159) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2122::<F>(t17157, t2768, t123);
        let t17161 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2123::<F>(t3966, t4337);
        let (t17162, t17163) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2124::<F>(t17161, t2768, t123);
        let t17165 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2125::<F>(t5682, t690);
        let t17167 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2126::<F>(t5677, t607);
        let (t17168, t17169) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2127::<F>(t17167, t882, t123);
        let t17171 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2128::<F>(t3966, t4342);
        let (t17172, t17173) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2129::<F>(t17171, t882, t123);
    (t17158, t17159, t17161, t17162, t17163, t17165, t17167, t17168, t17169, t17171, t17172, t17173)
}
