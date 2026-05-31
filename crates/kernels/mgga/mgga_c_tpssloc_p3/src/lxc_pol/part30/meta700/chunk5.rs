//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2258/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2258<F: Float>(t23110: F, t23185: F, t28321: F, t16805: F, t1909: F, t226: F, t235: F, t25256: F, t28407: F, t4166: F, t4291: F, t808: F, t812: F, t82032: F, t82039: F, t82047: F, t829: F, t87710: F, t87714: F, t87730: F, t87734: F, t92817: F, t98524: F, t98592: F, t98601: F, t98608: F, t98876: F, t98881: F) -> F {
    let t98884 = t23185 * t23110 * t28321;
    let t98886 = -F::cast_from(2.0_f64) * t4291 * t98524 * t829 + t808 * t28407 - t812 * t98592 * t829 - F::cast_from(2.0_f64) * t4166 * t25256 - F::cast_from(0.26044789391763585244e-1_f64) * t82032 - F::cast_from(0.16449340668482264365e-1_f64) * t98601 - F::cast_from(0.52089578783527170488e-1_f64) * t82039 + t87710 - F::cast_from(0.49348022005446793095e-1_f64) * t87714 + t16805 * t1909 - t82047 + F::cast_from(0.3289868133696452873e-1_f64) * t98608 - t92817 + t226 * t235 * t98876 + t87730 + F::cast_from(0.49348022005446793095e-1_f64) * t98881 + F::cast_from(0.41123351671205660912e-2_f64) * t98884 - t87734;
    t98886
}
