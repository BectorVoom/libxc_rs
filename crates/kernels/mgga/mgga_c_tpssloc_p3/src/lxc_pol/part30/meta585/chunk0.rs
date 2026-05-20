//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1964/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1964<F: Float>(t20063: F, t3701: F, t1484: F, t2752: F, t17083: F, t225: F, t5584: F, t852: F, t1509: F, t4265: F, t1519: F, t4233: F) -> (F, F, F, F, F, F) {
    let t57806 = t20063 * t3701;
    let t57911 = t2752 * t1484;
    let t58143 = t17083 * t225;
    let t58166 = t852 * t5584;
    let t58204 = t4265 * t1509;
    let t58226 = t1519 * t4233;
    (t57806, t57911, t58143, t58166, t58204, t58226)
}
