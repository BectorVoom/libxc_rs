//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1197/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1197<F: Float>(t2108: F, t2240: F, t2244: F, t39049: F, t7245: F, t24525: F, t9231: F, t24503: F, t33: F, t2110: F, t22519: F, t22527: F, t24505: F, t24508: F, t24520: F, t24526: F, t6492: F, t6495: F, t7256: F, t7259: F, t83748: F) -> (F,) {
    let t85507 = t2240 * t2244 * t2108;
    let t85510 = t39049 * t7245;
    let t85514 = t9231 * t24525;
    let t85524 = t2240 * t33 * t24503;
    let t85532 = -5.0 * t85507 * t6492 + 5.0 / 2.0 * t85510 * t6492 + t83748 * t2110 + 5.0 * t85514 * t6492 + 2.0 * t22519 * t7256 + 5.0 * t24520 * t22527 + 2.0 * t22519 * t7259 + 5.0 / 2.0 * t85524 * t6492 + t6495 * t24505 + 5.0 * t24526 * t22527 + 2.0 * t6495 * t24508;
    (t85532,)
}
