//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1318/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1318<F: Float>(t1509: F, t232: F, t25119: F, t5527: F, t815: F, t20947: F, t841: F, t20870: F, t6605: F, t20896: F, t6621: F, t20963: F, t23048: F) -> (F, F, F, F, F) {
    let t105387 = t25119 * t815 * t5527 * t1509 * t232;
    let t105390 = t25119 * t841 * t20947;
    let t105393 = t6605 * t815 * t20870;
    let t105396 = t6621 * t20896;
    let t105402 = t23048 * t20963;
    (t105387, t105390, t105393, t105396, t105402)
}
