//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1165/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1165<F: Float>(t20882: F, t23146: F, t20988: F, t25084: F, t20891: F, t1898: F, t20937: F, t249: F, t20983: F, t25146: F, t5619: F, t5587: F, t87218: F) -> (F, F, F, F, F, F, F) {
    let t105292 = t23146 * t20882;
    let t105294 = t25084 * t20988;
    let t105296 = t23146 * t20891;
    let t105299 = t20937 * t1898 * t249;
    let t105304 = t25084 * t20983;
    let t105309 = t25146 * t5619;
    let t105311 = t87218 * t5587;
    (t105292, t105294, t105296, t105299, t105304, t105309, t105311)
}
