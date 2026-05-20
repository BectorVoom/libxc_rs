//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2305/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2305<F: Float>(t11881: F, t15000: F, t1653: F, t1716: F, t24778: F, t24795: F, t24829: F, t27406: F, t27531: F, t3243: F, t4964: F, t7283: F, t7362: F, t7373: F, t7376: F, t7389: F, t8073: F, t8082: F, t85814: F, t85947: F, t86076: F, t95192: F, t95194: F, t95197: F, t95201: F, t95213: F) -> F {
    let t95224 = -F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7362 * t85947 * t1653 + F::cast_from(0.73108180748810063843e-2_f64) * t27406 * t24795 - t95192 - F::cast_from(0.3289868133696452873e-1_f64) * t95194 * t95197 + F::cast_from(0.16449340668482264365e-1_f64) * t95194 * t95201 - F::cast_from(0.82246703342411321825e-2_f64) * t7373 * t85814 * t8073 + F::cast_from(0.36554090374405031923e-2_f64) * t86076 * t27531 * t7376 * t3243 + t95213 + F::new(6.0) * t11881 * t8082 * t15000 - F::cast_from(0.9747757433174675179e-2_f64) * t27406 * t24778 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t1716 * t24829 + F::new(2.0) * t4964 * t7389;
    t95224
}
