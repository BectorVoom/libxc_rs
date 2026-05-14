//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1212/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1212<F: Float>(t20903: F, t6032: F, t20891: F, t4314: F, t1148: F, t6025: F, t6509: F, t1107: F) -> (F, F, F, F) {
    let t20904 = t6032 * t20903;
    let t20906 = t20891 * t4314;
    let t20910 = t6025 * t6509 * t1148;
    let t20913 = t1107 * t6509;
    (t20904, t20906, t20910, t20913)
}
