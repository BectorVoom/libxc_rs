//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2140/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2140<F: Float>(t87583: F, t234: F, t4265: F, t6552: F, t6637: F, t776: F, t23110: F, t23185: F, t25237: F, t23168: F, t25307: F, t13263: F, t13397: F, t25261: F, t2633: F, t2679: F, t4182: F, t4281: F, t4291: F, t81656: F, t81670: F, t81689: F, t81691: F, t829: F, t87566: F, t87567: F, t87575: F, t87578: F, t87582: F) -> F {
    let t87584 = F::cast_from(0.76763589786250567036e-1_f64) * t87583;
    let t87586 = t234 * t4265;
    let t87589 = t6552 * t6637 * t87586 * t776;
    let t87601 = t23185 * t23110 * t25237;
    let t87602 = F::cast_from(0.82246703342411321824e-2_f64) * t87601;
    let t87603 = t23168 * t25307;
    let t87604 = F::cast_from(0.76763589786250567036e-1_f64) * t87603;
    let t87606 = -t87566 + F::new(4.0) * t4281 * t87567 * t4182 - t4291 * t25261 * t2679 + F::cast_from(0.16449340668482264365e-1_f64) * t81656 - F::cast_from(0.16449340668482264365e-1_f64) * t87575 - F::cast_from(0.82246703342411321825e-2_f64) * t87578 + t87582 - t87584 + F::cast_from(0.82246703342411321824e-2_f64) * t81670 - F::cast_from(0.3289868133696452873e-1_f64) * t87589 - F::new(6.0) * t13397 * t25261 * t13263 + F::new(6.0) * t4281 * t25261 * t2633 - F::new(2.0) * t4291 * t87567 * t829 + t87602 + t87604 - t81689 + F::cast_from(0.41123351671205660912e-2_f64) * t81691;
    t87606
}
