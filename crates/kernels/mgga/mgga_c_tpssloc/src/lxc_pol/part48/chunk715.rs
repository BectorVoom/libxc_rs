//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 715/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk715<F: Float>(t33: F, t7254: F, t2240: F, t1860: F, t2110: F, t22493: F, t22519: F, t22527: F, t22531: F, t22534: F, t22537: F, t22546: F, t22549: F, t24505: F, t24508: F, t24511: F, t24514: F, t24517: F, t24520: F, t6486: F, t6492: F, t6495: F, t7246: F, t7256: F, t7259: F) -> (F, F) {
    let t24525 = t33 * t7254;
    let t24526 = t2240 * t24525;
    let t24541 = -t22493 * t2110 / 6.0 - t6486 * t7256 / 3.0 - t6486 * t7259 / 3.0 - t1860 * t24505 / 6.0 - t1860 * t24508 / 3.0 - t1860 * t24511 / 6.0 - 5.0 * t24514 * t22546 - 10.0 / 3.0 * t22549 * t24517 + 5.0 / 3.0 * t24520 * t6492 + 2.0 / 3.0 * t22519 * t2110 + 5.0 / 3.0 * t24526 * t6492 + 5.0 / 3.0 * t7246 * t22527 + 5.0 / 6.0 * t7246 * t22531 + t22534 * t2110 / 3.0 + t22537 * t2110 / 3.0 + 2.0 / 3.0 * t6495 * t7256 + 2.0 / 3.0 * t6495 * t7259;
    (t24525, t24541)
}
