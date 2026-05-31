//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1008/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1008<F: Float>(t31376: F, t5544: F, t6552: F, t6637: F, t101708: F, t1888: F, t232: F, t6646: F, t101715: F, t22996: F, t2632: F, t121574: F, t126481: F, t126484: F, t126488: F, t126492: F, t127917: F, t1499: F, t226: F, t235: F, t33396: F) -> F {
    let t128001 = t6552 * t6637 * t31376 * t5544;
    let t128007 = t1888 * t6646 * t101708 * t232;
    let t128011 = t1888 * t6646 * t101715 * t232;
    let t128015 = t1888 * t22996 * t101715 * t2632;
    let t128020 = -F::cast_from(0.16449340668482264365e-1_f64) * t128001 + t226 * t235 * t127917 - F::cast_from(0.82246703342411321825e-2_f64) * t128007 - F::cast_from(0.82246703342411321825e-2_f64) * t128011 - t126481 + F::cast_from(0.16449340668482264365e-1_f64) * t128015 + t126484 - F::cast_from(0.38381794893125283518e-1_f64) * t121574 - t126488 + t126492 + F::cast_from(2.0_f64) * t1499 * t33396;
    t128020
}
