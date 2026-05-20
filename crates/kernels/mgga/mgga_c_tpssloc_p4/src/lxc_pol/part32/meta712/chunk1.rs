//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2234/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2234<F: Float>(t16830: F, t25255: F, t25262: F, t2617: F, t28413: F, t4234: F, t4291: F, t5585: F, t812: F, t81679: F, t829: F, t87154: F, t92516: F, t98461: F, t98464: F, t98467: F, t98471: F, t98475: F, t98478: F, t98482: F, t98486: F, t98488: F, t98490: F, t98494: F) -> F {
    let t98497 = F::new(2.0) * t2617 * t28413 + F::new(2.0) * t812 * t81679 * t5585 - F::new(2.0) * t812 * t25255 * t4234 + F::cast_from(0.3289868133696452873e-1_f64) * t98461 + F::cast_from(0.3289868133696452873e-1_f64) * t98464 + F::cast_from(0.16449340668482264365e-1_f64) * t98467 - t87154 + t92516 + F::cast_from(0.3289868133696452873e-1_f64) * t98471 - F::cast_from(0.3289868133696452873e-1_f64) * t98475 + F::cast_from(0.3289868133696452873e-1_f64) * t98478 - F::cast_from(0.16449340668482264365e-1_f64) * t98482 + F::cast_from(0.16449340668482264365e-1_f64) * t98486 + F::cast_from(0.19190897446562641759e-1_f64) * t98488 - F::cast_from(0.38381794893125283518e-1_f64) * t98490 - F::new(2.0) * t16830 * t25262 - t4291 * t98494 * t829;
    t98497
}
