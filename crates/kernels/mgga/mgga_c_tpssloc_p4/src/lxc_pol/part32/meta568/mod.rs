//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1937;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1938;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1939;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta568<F: Float>(t5612: F, t815: F, t6605: F, t1898: F, t5575: F, t249: F, t5628: F, t6621: F, t5619: F, t6614: F, t23048: F, t5587: F, t1512: F, t25146: F, t5614: F, t5617: F, t2628: F, t5585: F, t23096: F, t23106: F, t23108: F, t25065: F, t26619: F, t26621: F, t23146: F, t5593: F, t1894: F, t236: F, t5544: F, t6591: F, t23056: F, t5568: F, t5527: F, t23078: F, t1484: F, t1509: F, t232: F, t23097: F, t1516: F, t25068: F, t5624: F, t5572: F, t6581: F, t23141: F, t23144: F, t25109: F, t25126: F, t25133: F, t26644: F, t26646: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28356, t28357, t28359, t28360, t28362, t28364, t28366) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1936::<F>(t5612, t815, t6605, t1898, t5575, t249, t5628, t6621, t5619, t6614, t23048, t5587);
        let (t28372, t28375, t28378) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1937::<F>(t1512, t25146, t5614, t6614, t5617, t815, t6605, t2628, t5585, t23096, t23106, t23108, t25065, t26619, t26621, t28357, t28360, t28362, t28364, t28366);
        let (t28380, t28383, t28384, t28386, t28389, t28390, t28395) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1938::<F>(t23146, t5593, t1894, t236, t5544, t6591, t23056, t5568, t5527, t23078, t1484, t1509, t232);
        let (t28396, t28405) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1939::<F>(t28395, t815, t23097, t1516, t25068, t5624, t6621, t5572, t6581, t23141, t23144, t25109, t25126, t25133, t26644, t26646, t28380, t28384, t28386, t28390);
    (t28356, t28359, t28372, t28375, t28378, t28383, t28389, t28395, t28396, t28405)
}
