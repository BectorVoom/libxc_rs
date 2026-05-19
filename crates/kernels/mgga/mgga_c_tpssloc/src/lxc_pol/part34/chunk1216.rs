//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1216/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1216<F: Float>(t107377: F, t107381: F, t107385: F, t107389: F, t107397: F, t107402: F, t107406: F, t1336: F, t19815: F, t20568: F, t29343: F, t29345: F, t5234: F, t6378: F, t7208: F, t7932: F, t7934: F, t84595: F, t84597: F, t97148: F, t97161: F, t97179: F, t97200: F) -> F {
    let t107951 = F::cast_from(0.11514538467937585055e0_f64) * t97148 - F::cast_from(0.14804406601634037928e0_f64) * t97161 - F::cast_from(0.3289868133696452873e-1_f64) * t107377 - F::new(3.0) * t5234 * t29343 - F::cast_from(0.19739208802178717238e0_f64) * t107381 - t84595 - F::cast_from(0.29608813203268075857e0_f64) * t107385 + F::cast_from(0.9869604401089358619e-1_f64) * t107389 + t84597 - t1336 * t7208 * t20568 + F::new(3.0) * t6378 * t7934 - F::new(3.0) * t19815 * t7932 - F::cast_from(0.49348022005446793095e-1_f64) * t107397 - F::cast_from(0.69087230807625510332e0_f64) * t97179 + F::cast_from(0.16449340668482264365e-1_f64) * t107402 - F::new(3.0) * t5234 * t29345 - F::cast_from(0.16449340668482264365e-1_f64) * t107406 - F::cast_from(0.11514538467937585055e0_f64) * t97200;
    t107951
}
