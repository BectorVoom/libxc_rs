//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1373/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1373<F: Float>(t1791: F, t65157: F, t65165: F, t19342: F, t62348: F, t18350: F, t20264: F, t62024: F, t62259: F, t62262: F, t62264: F, t62266: F, t62270: F, t62273: F, t62275: F, t62345: F, t65182: F) -> F {
    let t67349 = t1791 * t65157;
    let t67352 = t1791 * t65165;
    let t67358 = F::cast_from(160.0_f64) / F::cast_from(3.0_f64) * t62348 * t19342;
    let t67362 = F::cast_from(176.0_f64) / F::cast_from(27.0_f64) * t62259 + F::cast_from(176.0_f64) / F::cast_from(27.0_f64) * t62262 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t62264 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t62266 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t62024 * t20264 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t18350 * t67349 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t18350 * t67352 - F::cast_from(70.0_f64) * t62345 * t65182 - t67358 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t62270 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t62273 + F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t62275;
    t67362
}
