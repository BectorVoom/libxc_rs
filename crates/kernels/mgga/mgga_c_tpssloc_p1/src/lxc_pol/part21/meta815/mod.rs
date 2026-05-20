//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta815 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2871;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2872;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta815<F: Float>(t10817: F, t17510: F, t17513: F, t42143: F, t17517: F, t10771: F, t10811: F, t10828: F, t14271: F, t14328: F, t14337: F, t14439: F, t14443: F, t14463: F, t1569: F, t2861: F, t2862: F, t2880: F, t2886: F, t2906: F, t2930: F, t49285: F, t5743: F, t5759: F, t5762: F, t5775: F, t5791: F, t60006: F, t60008: F, t60010: F, t60016: F, t60021: F, t60023: F, t49072: F, t49240: F, t912: F, t13727: F, t14382: F, t14385: F, t49489: F, t13520: F, t14392: F, t14396: F, t49274: F, t2836: F, t2842: F, t5695: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t60025, t60027, t60029, t60030) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2871::<F>(t10817, t17510, t17513, t42143, t17517, t10771, t10811, t10828, t14271, t14328, t14337, t14439, t14443, t14463, t1569, t2861, t2862, t2880, t2886, t2906, t2930, t49285, t5743, t5759, t5762, t5775, t5791, t60006, t60008, t60010, t60016, t60021, t60023);
        let (t60033, t60035, t60037, t60039, t60041, t60044) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2872::<F>(t49072, t49240, t912, t13727, t14382, t14385, t49489, t13520, t14392, t14396, t49274, t2836, t2842, t5695);
    (t60025, t60027, t60029, t60030, t60033, t60035, t60037, t60039, t60041, t60044)
}
