//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta360 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1778;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1779;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1780;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta360<F: Float>(t13360: F, t849: F, t13176: F, t842: F, t1516: F, t9601: F, t10012: F, t10014: F, t10026: F, t10029: F, t10030: F, t10036: F, t10038: F, t13333: F, t13337: F, t13345: F, t13347: F, t13353: F, t13359: F, t249: F, t2623: F, t2643: F, t2703: F, t2707: F, t4172: F, t4178: F, t4261: F, t843: F, t9990: F, t13213: F, t13268: F, t13331: F, t218: F, t1509: F, t852: F, t829: F, t252: F, t4233: F, t4182: F, t2684: F, t4282: F, t4290: F, t808: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t13362, t13365, t13368, t13375) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1778::<F>(t13360, t849, t13176, t842, t1516, t9601, t10012, t10014, t10026, t10029, t10030, t10036, t10038, t13333, t13337, t13345, t13347, t13353, t13359, t249, t2623, t2643, t2703, t2707, t4172, t4178, t4261, t843, t9990);
        let (t13377, t13378, t13380, t13381, t13384, t13385, t13388) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1779::<F>(t13213, t13268, t13331, t13375, t218, t1509, t852, t829, t252, t4233, t4182, t2684, t4282);
        let t13390 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1780::<F>(t4290, t808);
    (t13362, t13365, t13368, t13377, t13378, t13380, t13381, t13384, t13385, t13388, t13390)
}
