//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1386;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1387;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1388;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1389;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1390;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1391;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1392;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta233<F: Float>(t5727: F, t893: F, t2844: F, t5694: F, t2842: F, t2848: F, t4335: F, t5679: F, t5683: F, t5687: F, t1568: F, t932: F, t2868: F, t2875: F, t4384: F, t5699: F, t5706: F, t5712: F, t5714: F, t5718: F, t5721: F, t5724: F, t2888: F, t2892: F, t324: F, t1580: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5729, t5730) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1386::<F>(t5727, t893, t2844, t5694);
        let (t5732, t5737, t5742, t5743) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1387::<F>(t2842, t5730, t2848, t4335, t5679, t5683, t5687, t1568, t932);
        let t5758 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1388::<F>(t2868, t2875, t4335, t4384, t5679, t5683, t5687, t5699, t5706, t5712, t5714, t5718, t5721, t5724);
        let t5759 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1389::<F>(t5758, t932);
        let t5762 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1390::<F>(t2888, t5742);
        let t5769 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1391::<F>(t2892, t4335, t5679, t5683, t5687);
        let (t5770, t5774) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1392::<F>(t324, t5769, t1580);
    (t5729, t5730, t5732, t5737, t5742, t5743, t5758, t5759, t5762, t5769, t5770, t5774)
}
