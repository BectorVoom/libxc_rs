//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1567;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1568;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1569;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1570;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1571;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta282<F: Float>(t10213: F, t974: F, t2769: F, t632: F, t698: F, t976: F, t979: F, t973: F, t2970: F, t2999: F, t135: F, t2978: F, t2981: F, t4509: F, t984: F, t2770: F, t343: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t10214 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1567::<F>(t10213, t974);
        let t10216 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1568::<F>(t2769, t632);
        let t10224 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1569::<F>(t698, t976);
        let (t10225, t10226, t10229, t10231) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1570::<F>(t10224, t979, t973, t2970, t2999, t135, t2978);
        let (t10233, t10235) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1571::<F>(t10231, t2981, t973, t4509, t984);
        let t10236 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1572::<F>(t2770, t343);
    (t10214, t10216, t10224, t10225, t10226, t10229, t10231, t10233, t10235, t10236)
}
