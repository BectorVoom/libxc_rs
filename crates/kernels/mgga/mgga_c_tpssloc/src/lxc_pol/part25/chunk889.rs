//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 889/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk889<F: Float>(t2303: F, t71: F, t33: F, t9228: F, t2235: F, t608: F, t641: F, t645: F, t72: F, t2307: F, t79: F, t2244: F, t605: F, t2251: F, t2241: F, t2240: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22489 = t71 * t2303;
    let t22493 = t9228 * t33;
    let t22519 = t2235 * t608;
    let t22527 = t72 * t641 * t645;
    let t22530 = t79 * t2307;
    let t22531 = t72 * t22530;
    let t22534 = t605 * t2244;
    let t22537 = t605 * t2251;
    let t22546 = t72 * t79 * t2241;
    let t22549 = t2240 * t608;
    (t22489, t22493, t22519, t22527, t22530, t22531, t22534, t22537, t22546, t22549)
}
