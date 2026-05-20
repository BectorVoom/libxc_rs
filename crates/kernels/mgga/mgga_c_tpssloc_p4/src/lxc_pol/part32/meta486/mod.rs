//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1792;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1793;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta486<F: Float>(t4300: F, t6571: F, t6553: F, t1880: F, t1902: F, t4142: F, t1492: F, t6624: F, t1519: F, t214: F, t6572: F, t13053: F, t1528: F, t1912: F, t23235: F, t23281: F, t25200: F, t25206: F, t25209: F, t25211: F, t25214: F, t259: F, t2713: F, t7538: F, t855: F) -> (F, F, F, F, F, F, F) {
        let (t25216, t25217, t25218, t25220, t25222, t25224) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1792::<F>(t4300, t6571, t6553, t1880, t1902, t4142, t1492, t6624, t1519, t214);
        let (t25225, t25228) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1793::<F>(t25224, t6572, t1880, t13053, t1528, t1912, t23235, t23281, t25200, t25206, t25209, t25211, t25214, t25218, t25220, t25222, t259, t2713, t7538, t855);
    (t25216, t25217, t25220, t25222, t25224, t25225, t25228)
}
