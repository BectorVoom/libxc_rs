//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1034/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1034<F: Float>(t1894: F, t20800: F, t236: F, t6591: F, t20974: F, t23146: F, t1509: F, t232: F, t25119: F, t5527: F, t815: F, t20947: F, t841: F, t20870: F, t6605: F, t20896: F, t6621: F) -> (F, F, F, F, F, F) {
    let t105376 = t6591 * t1894 * t236 * t20800;
    let t105381 = t23146 * t20974;
    let t105387 = t25119 * t815 * t5527 * t1509 * t232;
    let t105390 = t25119 * t841 * t20947;
    let t105393 = t6605 * t815 * t20870;
    let t105396 = t6621 * t20896;
    (t105376, t105381, t105387, t105390, t105393, t105396)
}
