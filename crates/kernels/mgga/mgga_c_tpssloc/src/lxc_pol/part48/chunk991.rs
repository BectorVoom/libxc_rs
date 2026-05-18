//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 991/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk991<F: Float>(t115454: F, t115468: F, t1985: F, t1998: F, t214: F, t24063: F, t114081: F, t114085: F, t114098: F, t114102: F, t114104: F, t114106: F, t115439: F, t1332: F, t1336: F, t31636: F, t31639: F, t3856: F, t544: F, t553: F) -> (F, F) {
    let t115469 = t115454 + t115468;
    let t115474 = t1985 * t214 * t1998 * t24063;
    let t115480 = t114081 - F::new(0.82246703342411321824e-2) * t115439 + t544 * t553 * t115469 + F::new(0.82246703342411321825e-2) * t115474 - t114085 - t1336 * t31636 * t3856 - t114098 + t114102 + t114104 + t114106 + F::new(2.0) * t1332 * t31639;
    (t115469, t115480)
}
