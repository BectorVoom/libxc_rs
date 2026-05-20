//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2239/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2239<F: Float>(t13397: F, t16816: F, t25261: F, t4182: F, t4234: F, t4281: F, t4291: F, t81633: F, t829: F, t87536: F, t87545: F, t87547: F, t87566: F, t87582: F, t87584: F, t87602: F, t98494: F, t98541: F, t98546: F, t98549: F, t98553: F, t98564: F) -> F {
    let t98566 = t87536 - t87545 - t87547 - F::cast_from(0.12793931631041761173e0_f64) * t81633 - t87566 - F::new(2.0) * t4291 * t25261 * t4234 - t4291 * t98541 * t829 - F::cast_from(0.16449340668482264365e-1_f64) * t98546 + F::cast_from(0.82246703342411321825e-2_f64) * t98549 - F::cast_from(0.82246703342411321825e-2_f64) * t98553 + F::new(2.0) * t4281 * t98494 * t4182 + t87582 - t87584 + F::new(6.0) * t4281 * t98541 * t4182 - F::new(6.0) * t13397 * t98541 * t16816 + F::cast_from(0.38381794893125283518e-1_f64) * t98564 + t87602;
    t98566
}
