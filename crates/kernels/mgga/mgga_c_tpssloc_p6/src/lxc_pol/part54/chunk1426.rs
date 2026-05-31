//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1426/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1426<F: Float>(t1992: F, t33249: F, t80650: F, t114264: F, t115596: F, t115601: F, t120594: F, t1323: F, t1375: F, t16439: F, t2015: F, t22656: F, t24095: F, t26224: F, t26370: F, t26989: F, t27009: F, t27114: F, t33266: F, t33323: F, t3887: F, t568: F, t6963: F, t7750: F, t7925: F, t8627: F, t91505: F) -> F {
    let t122370 = t1992 * t80650 * t33249;
    let t122375 = F::cast_from(2.0_f64) * t22656 * t7925 + F::cast_from(2.0_f64) * t1375 * t3887 * t27114 * t2015 - F::cast_from(0.38381794893125283518e-1_f64) * t115596 + t114264 - F::cast_from(6.0_f64) * t91505 * t33323 - t120594 + t1323 * t33266 * t568 + F::cast_from(2.0_f64) * t27009 * t6963 - F::cast_from(6.0_f64) * t26224 * t26989 * t26370 + F::cast_from(0.41123351671205660912e-2_f64) * t115601 + F::cast_from(0.16449340668482264365e-1_f64) * t122370 + F::cast_from(2.0_f64) * t16439 * t8627 - t24095 * t7750;
    t122375
}
