//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1562/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1562<F: Float>(t22473: F, t2332: F, t2358: F, t6530: F, t2303: F, t71: F, t33: F, t9228: F, t2235: F, t608: F, t641: F, t645: F, t72: F) -> (F, F, F, F, F, F) {
    let t22474 = t22473 * t2332;
    let t22476 = t6530 * t2358;
    let t22489 = t71 * t2303;
    let t22493 = t9228 * t33;
    let t22519 = t2235 * t608;
    let t22527 = t72 * t641 * t645;
    (t22474, t22476, t22489, t22493, t22519, t22527)
}
