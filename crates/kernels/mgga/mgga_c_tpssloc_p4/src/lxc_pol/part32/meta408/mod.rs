//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1576;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta408<F: Float>(t4781: F, t4785: F, t3313: F, t11277: F, t5988: F, t1117: F, t11275: F, t3411: F, t6106: F, t1157: F, t6105: F, t1164: F, t11282: F, t6068: F, t11285: F, t1155: F, t11292: F, t4883: F, t15218: F, t4882: F, t1190: F, t6238: F, t1743: F, t4965: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18264, t18268, t18270, t18273) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1576::<F>(t4781, t4785, t3313, t11277, t5988, t1117, t11275, t3411, t6106, t1157, t6105, t1164);
        let (t18278, t18282, t18285, t18287, t18297) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1577::<F>(t11282, t6068, t11285, t1155, t1164, t11292, t4883, t15218, t4882, t1190, t6238, t1743, t4965);
    (t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18287, t18297)
}
