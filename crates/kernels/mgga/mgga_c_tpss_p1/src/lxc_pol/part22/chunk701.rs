//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 701/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk701<F: Float>(t3210: F, t3395: F, t1163: F, t1168: F, t118: F, t1273: F, t2054: F, t2056: F, t2062: F, t2065: F, t2106: F, t3166: F, t3174: F, t485: F, t488: F, t544: F, t624: F, t626: F, t646: F) -> (F, F) {
    let t3396 = t3210 + t3395;
    let t3398 = -F::cast_from(2.0_f64) * t1163 * t624 + F::cast_from(2.0_f64) * t1168 * t1273 - t118 * t3166 - t2054 * t485 - F::cast_from(4.0_f64) * t2056 * t646 - F::cast_from(2.0_f64) * t2062 * t485 - F::cast_from(4.0_f64) * t2065 * t626 - F::cast_from(2.0_f64) * t2106 * t626 + t3174 * t544 + t3396 * t488;
    (t3396, t3398)
}
