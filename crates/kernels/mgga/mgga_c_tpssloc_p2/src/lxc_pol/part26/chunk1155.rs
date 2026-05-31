//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1155/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1155<F: Float>(t2240: F, t24525: F, t1860: F, t2110: F, t22493: F, t22519: F, t22527: F, t22531: F, t22534: F, t22537: F, t22546: F, t22549: F, t24505: F, t24508: F, t24511: F, t24514: F, t24517: F, t24520: F, t6486: F, t6492: F, t6495: F, t7246: F, t7256: F, t7259: F) -> (F, F) {
    let t24526 = t2240 * t24525;
    let t24541 = -t22493 * t2110 / F::cast_from(6.0_f64) - t6486 * t7256 / F::cast_from(3.0_f64) - t6486 * t7259 / F::cast_from(3.0_f64) - t1860 * t24505 / F::cast_from(6.0_f64) - t1860 * t24508 / F::cast_from(3.0_f64) - t1860 * t24511 / F::cast_from(6.0_f64) - F::cast_from(5.0_f64) * t24514 * t22546 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t22549 * t24517 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24520 * t6492 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t22519 * t2110 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24526 * t6492 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7246 * t22527 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t22531 + t22534 * t2110 / F::cast_from(3.0_f64) + t22537 * t2110 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t7256 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t7259;
    (t24526, t24541)
}
