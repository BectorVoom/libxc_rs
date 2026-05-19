//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1029/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1029<F: Float>(t1985: F, t28232: F, t31611: F, t115545: F, t22633: F, t28116: F, t113941: F, t115331: F, t122133: F, t127187: F, t127197: F, t127201: F, t127202: F, t20029: F, t20060: F, t2092: F, t26224: F, t26989: F, t28219: F, t31653: F, t33294: F, t33301: F, t33316: F, t5215: F, t6440: F, t8627: F, t96913: F) -> F {
    let t128656 = t1985 * t31611 * t28232;
    let t128659 = t22633 * t115545 * t28116;
    let t128663 = -t113941 - t96913 * t2092 + F::cast_from(0.38381794893125283518e-1_f64) * t122133 - t127187 - F::new(2.0) * t5215 * t33294 - t115331 + F::new(2.0) * t20060 * t8627 + F::new(4.0) * t20029 * t8627 + F::new(2.0) * t31653 * t6440 - F::new(12.0) * t26224 * t26989 * t28219 + F::new(4.0) * t5215 * t33301 - t127197 + F::cast_from(0.16449340668482264365e-1_f64) * t128656 - t127201 - t127202 + F::cast_from(0.3289868133696452873e-1_f64) * t128659 + F::new(4.0) * t5215 * t33316;
    t128663
}
