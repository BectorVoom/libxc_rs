//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2110;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta496<F: Float>(t17030: F, t4182: F, t1499: F, t4280: F, t16935: F, t4282: F, t13433: F, t1510: F, t829: F, t13397: F, t16817: F, t16820: F, t16823: F, t16825: F, t16828: F, t16830: F, t17023: F, t17028: F, t226: F, t2617: F, t4166: F, t4281: F, t4283: F, t4288: F, t4291: F, t4292: F, t5575: F, t5651: F, t5655: F, t808: F, t812: F, t863: F, t16814: F, t858: F, t225: F, t5559: F, t5657: F, t865: F, t2718: F, t17022: F, t218: F, t5636: F, t10110: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17031, t17034, t17037, t17041, t17046, t17048) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2110::<F>(t17030, t4182, t1499, t4280, t16935, t4282, t13433, t1510, t829, t13397, t16817, t16820, t16823, t16825, t16828, t16830, t17023, t17028, t226, t2617, t4166, t4281, t4283, t4288, t4291, t4292, t5575, t5651, t5655, t808, t812, t863);
        let (t17049, t17050, t17052, t17057, t17060, t17064) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2111::<F>(t16814, t17048, t858, t225, t5559, t5657, t865, t2718, t17022, t218, t5636, t10110);
    (t17031, t17034, t17037, t17041, t17046, t17049, t17050, t17052, t17057, t17060, t17064)
}
