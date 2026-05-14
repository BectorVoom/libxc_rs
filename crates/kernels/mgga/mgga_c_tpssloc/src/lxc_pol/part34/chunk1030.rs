//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1030/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1030<F: Float>(t1484: F, t23097: F, t5612: F, t815: F, t2628: F, t5585: F, t20887: F, t23146: F, t5593: F, t87199: F, t20882: F, t20988: F, t25084: F, t20891: F, t1898: F, t20937: F, t249: F) -> (F, F, F, F, F, F, F, F) {
    let t105282 = t23097 * t815 * t5612 * t1484;
    let t105286 = t23097 * t2628 * t5585 * t1484;
    let t105288 = t23146 * t20887;
    let t105290 = t87199 * t5593;
    let t105292 = t23146 * t20882;
    let t105294 = t25084 * t20988;
    let t105296 = t23146 * t20891;
    let t105299 = t20937 * t1898 * t249;
    (t105282, t105286, t105288, t105290, t105292, t105294, t105296, t105299)
}
