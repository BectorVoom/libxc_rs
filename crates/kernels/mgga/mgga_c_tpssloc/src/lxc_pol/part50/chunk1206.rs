//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1206/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1206<F: Float>(t30840: F, t4571: F, t30829: F, t4630: F, t113380: F, t25638: F, t113381: F, t113388: F, t113397: F, t113400: F, t113454: F, t1618: F, t1622: F, t25585: F, t25601: F, t30817: F, t4636: F, t8384: F) -> F {
    let t119312 = t30840 * t4571;
    let t119316 = t30829 * t4630;
    let t119322 = t25638 * t113380;
    let t119324 = -F::cast_from(0.40372756094140390856e-3_f64) * t113381 - F::cast_from(0.32298204875312312685e-2_f64) * t25585 * t8384 + t30840 * t4636 / F::new(2304.0) - t113454 * t1622 / F::new(432.0) + t119312 / F::new(3456.0) - t113397 * t1618 / F::new(288.0) + t119316 / F::new(2304.0) + t113388 / F::new(2304.0) + t113400 / F::new(3456.0) - F::cast_from(0.40372756094140390856e-3_f64) * t25601 * t30817 - F::cast_from(0.40372756094140390856e-3_f64) * t119322;
    t119324
}
