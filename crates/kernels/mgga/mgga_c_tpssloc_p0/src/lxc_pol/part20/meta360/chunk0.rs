//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1684/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1684<F: Float>(t1388: F, t3698: F, t3700: F, t570: F, t11976: F, t11978: F, t11980: F, t11982: F, t11984: F, t12012: F, t12044: F, t12046: F, t12156: F, t12451: F, t1297: F, t1390: F, t193: F, t533: F, t571: F, t9457: F, t9476: F, t9484: F, t9780: F) -> (F, F, F) {
    let t12458 = t3698 * t1388;
    let t12461 = F::cast_from(1.0_f64) / t3700 / t570;
    let t12465 = t12451 * t1390 * t193 * t533 + F::cast_from(2.0_f64) * t12458 * t12461 * t193 * t533 + F::cast_from(3.0_f64) * t12012 * t1297 * t193 + F::cast_from(6.0_f64) * t12156 * t193 * t571 + t11976 - t11978 - t11980 - t11982 - t11984 + t12044 - t12046 - t9457 + t9476 + t9484 + t9780;
    (t12458, t12461, t12465)
}
