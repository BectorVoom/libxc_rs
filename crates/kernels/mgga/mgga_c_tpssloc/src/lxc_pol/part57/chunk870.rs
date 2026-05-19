//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 870/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk870<F: Float>(t214: F, t33284: F, t1985: F, t1825: F, t31636: F, t33266: F, t553: F, t1336: F, t1814: F, t31192: F, t31200: F, t31617: F, t31625: F, t32743: F, t32747: F, t32751: F, t33278: F, t33282: F, t544: F, t8634: F) -> (F, F, F, F) {
    let t33285 = t214 * t33284;
    let t33286 = t1985 * t33285;
    let t33289 = t31636 * t1825;
    let t33291 = t553 * t33266;
    let t33293 = -t31192 - t32743 - t31200 - t32747 + t32751 - t31617 - F::cast_from(0.16449340668482264365e-1_f64) * t33278 - t31625 - F::cast_from(0.82246703342411321825e-2_f64) * t33282 + F::cast_from(0.82246703342411321825e-2_f64) * t33286 + t1814 * t8634 - t1336 * t33289 + t544 * t33291;
    (t33285, t33289, t33291, t33293)
}
