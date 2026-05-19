//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1179/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1179<F: Float>(t1352: F, t31636: F, t31584: F, t553: F, t1332: F, t1336: F, t31192: F, t31197: F, t31200: F, t31205: F, t31209: F, t31617: F, t31621: F, t31625: F, t31629: F, t31633: F, t544: F, t8634: F) -> (F, F, F) {
    let t31637 = t31636 * t1352;
    let t31639 = t553 * t31584;
    let t31641 = -t31192 - t31197 - t31200 - t31205 + t31209 - t31617 - F::cast_from(0.16449340668482264365e-1_f64) * t31621 - t31625 - F::cast_from(0.82246703342411321825e-2_f64) * t31629 + F::cast_from(0.82246703342411321825e-2_f64) * t31633 + t1332 * t8634 - t1336 * t31637 + t544 * t31639;
    (t31637, t31639, t31641)
}
